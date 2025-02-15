use itertools::Itertools;
use sqlx::{Error, Pool, Postgres};

use crate::models::transaction_acceptance::TransactionAcceptance;

pub async fn insert_transaction_acceptances(tx_acceptances: &[TransactionAcceptance], pool: &Pool<Postgres>) -> Result<u64, Error> {
    const COLS: usize = 2;
    let sql = format!(
        "INSERT INTO transactions_acceptances (transaction_id, block_hash) VALUES {} ON CONFLICT DO NOTHING",
        generate_placeholders(tx_acceptances.len(), COLS)
    );
    let mut query = sqlx::query(&sql);
    for ta in tx_acceptances {
        query = query.bind(&ta.transaction_id);
        query = query.bind(&ta.block_hash);
    }
    Ok(query.execute(pool).await?.rows_affected())
}

fn generate_placeholders(rows: usize, columns: usize) -> String {
    (0..rows).map(|i| format!("({})", (1..=columns).map(|c| format!("${}", c + i * columns)).join(", "))).join(", ")
}
