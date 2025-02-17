use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Clone, Debug, Serialize, Deserialize)]
#[command(name = "simply-kaspa-acceptance-tool", version = env!("VERGEN_GIT_DESCRIBE"))]
#[serde(rename_all = "camelCase")]
pub struct CliArgs {
    #[clap(
        short = 's',
        long,
        default_value = "wss://archival.kaspa.ws",
        help = "The url to a kaspad instance, e.g 'ws://localhost:17110'."
    )]
    pub rpc_url: Option<String>,
    #[clap(short, long, default_value = "mainnet", help = "The network type and suffix, e.g. 'testnet-11'")]
    pub network: String,
    #[clap(short, long, default_value = "postgres://postgres:postgres@localhost:5432/postgres", help = "PostgreSQL url")]
    pub database_url: String,
    #[clap(long, default_value = "info", help = "error, warn, info, debug, trace, off")]
    pub log_level: String,
    #[clap(long, help = "Disable colored output")]
    pub log_no_color: bool,
    #[clap(
        long,
        help = "Start block hash for virtual chain processing. If not specified the built-in default ccb8c53f3b0b742b4a8df654c29a852133cae8362d7f88efbddb0b2bf0da54e1 is used"
    )]
    pub start_hash: Option<String>,
    #[clap(long, help = "Max blue score for virtual chain processing. Leave empty to stop at the start of the last batch")]
    pub max_blue_score: Option<u64>,
}

impl CliArgs {
    pub fn version(&self) -> String {
        env!("VERGEN_GIT_DESCRIBE").to_string()
    }

    pub fn commit_id(&self) -> String {
        env!("VERGEN_GIT_SHA").to_string()
    }
}
