use crate::models::types::hash::Hash;
use sqlx::{Error, Pool, Postgres};

pub async fn select_oldest_chain_block_blue_score(pool: &Pool<Postgres>) -> Result<(Hash, i64), Error> {
    sqlx::query_as::<_, (Hash, i64)>(
        "
        SELECT b.hash, b.blue_score FROM blocks b
        JOIN transactions_acceptances ta ON b.hash = ta.block_hash
        ORDER BY b.blue_score
        LIMIT 1
        ",
    )
    .fetch_one(pool)
    .await
}
