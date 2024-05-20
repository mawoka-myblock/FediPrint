use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "job_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Unprocessed,
    Processing,
    Finished,
    WaitingForRetry,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "job_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobType {
    SendRegisterEmail,
}

pub struct FullJob {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub status: JobStatus,
    pub retry_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub input_data: Option<String>,
    pub return_data: Option<String>,
    pub failure_log: Vec<String>,
    pub tries: i32,
    pub max_tries: i32,
    pub processing_times: Vec<f64>,
    pub updated_at: DateTime<Utc>,
    pub job_type: JobType,
}

pub struct JobResponseSuccess<'a> {
    pub processing_time: f64,
    pub resp_data: Option<&'a str>,
}

pub struct JobResponseFailure<'a> {
    pub processing_time: f64,
    pub try_in: i32, // in seconds
    pub failure_message: &'a str,
}
