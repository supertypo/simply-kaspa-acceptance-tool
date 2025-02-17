use crate::virtual_chain::accept_transactions::accept_transactions;
use deadpool::managed::{Object, Pool};
use kaspa_hashes::Hash as KaspaHash;
use kaspa_rpc_core::api::rpc::RpcApi;
use log::{debug, error, info};
use simply_kaspa_acceptance_tool_database::client::KaspaDbClient;
use simply_kaspa_acceptance_tool_kaspad::pool::manager::KaspadManager;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub async fn process_virtual_chain(
    run: Arc<AtomicBool>,
    start_hash: KaspaHash,
    max_blue_score: u64,
    kaspad_pool: Pool<KaspadManager, Object<KaspadManager>>,
    database: KaspaDbClient,
) {
    let batch_scale = 1f64;
    let mut start_hash = start_hash;

    let start_time = Instant::now();
    let mut total_rows_added = 0;
    let mut finished = false;

    while run.load(Ordering::Relaxed) {
        debug!("Getting virtual chain from start_hash {}", start_hash.to_string());
        match kaspad_pool.get().await {
            Ok(kaspad) => match kaspad.get_virtual_chain_from_block(start_hash, true).await {
                Ok(res) => {
                    let added_blocks_count = res.added_chain_block_hashes.len();
                    if res.added_chain_block_hashes.len() > 0 {
                        let last_accepting_block =
                            kaspad.get_block(*res.added_chain_block_hashes.last().unwrap(), false).await.unwrap();
                        if last_accepting_block.header.blue_score < max_blue_score {
                            let rows_added = accept_transactions(batch_scale, &res.accepted_transaction_ids, &database).await;
                            info!(
                                "Committed {} accepted transactions. Last accepted: {} (bs: {})",
                                rows_added,
                                chrono::DateTime::from_timestamp_millis(last_accepting_block.header.timestamp as i64 / 1000 * 1000)
                                    .unwrap(),
                                last_accepting_block.header.blue_score
                            );
                            total_rows_added += rows_added;
                            start_hash = last_accepting_block.header.hash;
                        } else {
                            info!("Reached max blue score {max_blue_score}");
                            finished = true;
                        };
                    }
                    // Default batch size is 1800 on 1 bps:
                    if added_blocks_count < 200 || finished {
                        let time_to_sync = Instant::now().duration_since(start_time);
                        info!(
                            "\x1b[32mProcessing complete, added {} txs (in {}:{:0>2}:{:0>2}s)\x1b[0m",
                            total_rows_added,
                            time_to_sync.as_secs() / 3600,
                            time_to_sync.as_secs() % 3600 / 60,
                            time_to_sync.as_secs() % 60
                        );
                        return;
                    }
                }
                Err(e) => {
                    error!("Failed getting virtual chain from start_hash {}: {}", start_hash.to_string(), e);
                    sleep(Duration::from_secs(5)).await;
                }
            },
            Err(e) => {
                error!("Failed getting kaspad connection from pool: {}", e);
                sleep(Duration::from_secs(5)).await
            }
        }
    }
}
