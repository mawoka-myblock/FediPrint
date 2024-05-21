use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "job_status", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Unprocessed,
    Processing,
    Finished,
    WaitingForRetry,
    Failed,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "job_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobType {
    SendRegisterEmail,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct JobResponseSuccess {
    pub resp_data: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct JobResponseFailure {
    pub try_in: Option<i32>, // in seconds
    pub failure_message: String,
}

impl JobResponseFailure {
    #[allow(dead_code)]
    pub fn try_in_30(msg: &str) -> Self {
        JobResponseFailure {
            try_in: Some(30),
            failure_message: msg.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn never_try(msg: &str) -> Self {
        JobResponseFailure {
            try_in: None,
            failure_message: msg.to_string(),
        }
    }
}
