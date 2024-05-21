use anyhow::Result;
use chrono::{DateTime, TimeDelta, Utc};
use dotenvy::dotenv;
use shared::helpers::config::Config;
use sqlx::postgres::{PgListener, PgPoolOptions};
use sqlx::{Error, PgPool};
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tracing::debug;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use types::JobResponseFailure;

use crate::tasks::email::send_register_email;
use crate::types::JobStatus;
mod tasks;
pub mod types;

async fn save_failed_job(
    data: JobResponseFailure,
    t: f64,
    job_id: i32,
    pool: PgPool,
) -> Result<(), Error> {
    let mut retry_at: Option<DateTime<Utc>> = None;
    let mut status: JobStatus = JobStatus::Failed;
    if data.try_in.is_some() {
        retry_at = Some(Utc::now() + TimeDelta::milliseconds(data.try_in.unwrap().into()));
        status = JobStatus::WaitingForRetry;
    }
    sqlx::query!(
        r#"UPDATE jobs SET retry_at = $1, failure_log = array_append(failure_log, $2), processing_times = array_append(processing_times, $3), status = $4 WHERE id = $5"#,
        retry_at, data.failure_message, t, status as _, job_id
    ).execute(&pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "worker=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
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
        debug!(
            "Received notification: with {} on channel {}",
            notification.payload(),
            notification.channel()
        );
        let task_id: i32 = notification.payload().parse().unwrap();
        let lock_success: bool =
            sqlx::query_scalar!(r#"SELECT pg_try_advisory_lock($1)"#, i64::from(task_id))
                .fetch_one(&pool)
                .await
                .unwrap()
                .expect("DB query failed");
        if !lock_success {
            debug!("Could not acquire lock, waiting for next event");
            continue;
        }
        debug!("Yay! Acquired lock for job {task_id}");
        let job = sqlx::query_as!(
            types::FullJob,
            r#"SELECT
            id,
            created_at,
            started_at,
            status AS "status!: types::JobStatus",
            retry_at,
            finished_at,
            input_data,
            return_data,
            failure_log,
            tries,
            max_tries,
            processing_times,
            updated_at,
            job_type AS "job_type!: types::JobType"
        FROM
            jobs
        WHERE
            id = $1"#,
            task_id
        )
        .fetch_one(&pool)
        .await?;
        let start_time = Instant::now();
        let data = match job.job_type {
            types::JobType::SendRegisterEmail => {
                send_register_email(job, &config, pool.clone()).await
            }
        };
    }
}
