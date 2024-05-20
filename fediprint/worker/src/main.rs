use anyhow::Result;
use dotenvy::dotenv;
use shared::helpers::config::Config;
use sqlx::postgres::{PgListener, PgPoolOptions};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("can't connect to database");
    let mut listener = PgListener::connect_with(&pool).await?;
    listener.listen_all(vec!["worker_update"]).await?;
    loop {
        let notification = listener.recv().await?;
        println!("[from recv]: {notification:?}");
    }
}
