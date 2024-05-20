use crate::types::{JobStatus, JobType};
use chrono::{DateTime, Utc};
use num_traits::ToPrimitive;
use shared::db::account::FullAccount;
use sqlx::{Error, PgPool};
use uuid::Uuid;

mod types;

struct CreateRawJob<'a> {
    input_data: &'a str,
    max_tries: i32, // Should be 3
    job_type: JobType,
}
impl CreateRawJob<'_> {
    async fn create(self, pool: PgPool) -> Result<i32, Error> {
        sqlx::query_scalar!(r#"INSERT INTO jobs (input_data, max_tries, job_type) VALUES ($1, $2, $3) RETURNING id"#,
        self.input_data, self.max_tries, self.job_type.to_i32()
    ).fetch_one(&pool).await
    }
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
    job_type: JobType,
}

pub async fn send_register_confirm_email(to_user: &Uuid, pool: PgPool) -> Result<i32, Error> {
    let job = CreateRawJob {
        job_type: JobType::SendRegisterEmail,
        input_data: &to_user.to_string(),
        max_tries: 3,
    };
    let job_id = job.create(pool.clone()).await?;

    Ok(job_id)
}
