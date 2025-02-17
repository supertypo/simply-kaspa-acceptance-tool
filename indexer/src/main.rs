use clap::Parser;
use deadpool::managed::{Object, Pool};
use kaspa_hashes::Hash as KaspaHash;
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_wrpc_client::prelude::NetworkId;
use log::{info, trace, warn};
use simply_kaspa_acceptance_tool::signal::signal_handler::notify_on_signals;
use simply_kaspa_acceptance_tool::virtual_chain::process_virtual_chain::process_virtual_chain;
use simply_kaspa_acceptance_tool_cli::cli_args::CliArgs;
use simply_kaspa_acceptance_tool_database::client::KaspaDbClient;
use simply_kaspa_acceptance_tool_kaspad::pool::manager::KaspadManager;
use std::env;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::task;

#[tokio::main]
async fn main() {
    println!();
    println!("**************************************************************");
    println!("**************** Simply Kaspa Acceptance Tool ****************");
    println!("--------------------------------------------------------------");
    println!("- https://github.com/supertypo/simply-kaspa-acceptance-tool/ -");
    println!("--------------------------------------------------------------");
    let cli_args = CliArgs::parse();

    env::set_var("RUST_LOG", &cli_args.log_level);
    env::set_var("RUST_LOG_STYLE", if cli_args.log_no_color { "never" } else { "always" });
    env_logger::builder().target(env_logger::Target::Stdout).format_target(false).format_timestamp_millis().init();

    trace!("{:?}", cli_args);
    let rpc_url = cli_args.rpc_url.clone().unwrap_or("wss://archival.kaspa.ws".to_string());

    let network_id = NetworkId::from_str(&cli_args.network).unwrap();
    let kaspad_manager = KaspadManager { network_id, rpc_url: Some(rpc_url) };
    let kaspad_pool: Pool<KaspadManager> = Pool::builder(kaspad_manager).max_size(10).build().unwrap();

    let database = KaspaDbClient::new(&cli_args.database_url).await.expect("Database connection FAILED");

    start_processing(cli_args.start_hash, cli_args.max_blue_score, kaspad_pool, database).await.unwrap();
}

async fn start_processing(
    start_hash: Option<String>,
    max_blue_score: Option<u64>,
    kaspad_pool: Pool<KaspadManager, Object<KaspadManager>>,
    database: KaspaDbClient,
) -> Result<(), ()> {
    let run = Arc::new(AtomicBool::new(true));
    task::spawn(notify_on_signals(run.clone()));

    let start_hash = match start_hash {
        Some(start_hash) => {
            info!("Starting at user supplied block {start_hash}");
            KaspaHash::from_str(start_hash.as_str()).unwrap()
        }
        None => {
            let (hash, blue_score) = database.select_oldest_chain_block_blue_score().await.unwrap();
            if blue_score > 80921063 {
                info!("Starting at oldest db chain block {hash} (bs: {blue_score})");
                hash.into()
            } else {
                let hash = "ccb8c53f3b0b742b4a8df654c29a852133cae8362d7f88efbddb0b2bf0da54e1";
                info!("Starting at default chain block {hash} (bs: 80921063)");
                KaspaHash::from_str(hash).unwrap()
            }
        }
    };

    let max_blue_score = match max_blue_score {
        Some(max_blue_score) => {
            info!("Stopping at user supplied blue_score {max_blue_score}");
            max_blue_score
        }
        None => {
            let mut max_blue_score = 0;
            while run.load(Ordering::Relaxed) {
                match kaspad_pool.get().await {
                    Ok(kaspad) => {
                        max_blue_score = kaspad.get_sink_blue_score().await.unwrap();
                        break;
                    }
                    Err(e) => warn!("Failed to get sink blue_score: {e}"),
                }
            }
            info!("Stopping at current blue_score {max_blue_score}");
            max_blue_score
        }
    };

    process_virtual_chain(run.clone(), start_hash, max_blue_score, kaspad_pool.clone(), database.clone()).await;
    Ok(())
}
