pub mod db;
pub mod helpers;
pub mod models;
use helpers::config::Config;
use s3::Bucket;
use sqlx::PgPool;

pub struct AppState {
    pub env: Config,
    pub s3: Bucket,
    pub pool: PgPool,
    pub ms: meilisearch_sdk::Index,
    pub stripe: Option<stripe::Client>,
}
