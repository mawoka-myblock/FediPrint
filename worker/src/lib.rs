use sqlx::{Error, PgPool};
use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use uuid::Uuid;
use crate::types::{JobType, JobStatus};

mod types;



struct CreateRawJob<'a> {
    input_data: &'a str,
    max_tries: i8, // Should be 3
    job_type: JobType,
}

struct FullJob {
    id: i64,
    created_at: DateTime<Utc>,
    started_at: Option<DateTime<Utc>>,
    status: JobStatus,
    retry_at: Option<DateTime<Utc>>,
    finished_at: Option<DateTime<Utc>>,
    input_data: Option<String>,
    return_data: Option<String>,
    tries: i8,
    max_tries: i8,
    processing_time: Vec<f32>,
    updated_at: DateTime<Utc>,
    job_type: JobType
}

async fn create_raw_job(data: &str, max_tries: Option<i32>, job_type: JobType, pool: PgPool) -> Result<i32, Error> {
    sqlx::query_scalar!(r#"INSERT INTO jobs (input_data, max_tries, job_type) VALUES ($1, $2, $3) RETURNING id"#,
        data, max_tries, job_type.to_i32()
    ).fetch_one(&pool).await
}


pub async fn send_register_confirm_email(to_user: &Uuid, pool: PgPool) -> Result<i64, Error> {

}