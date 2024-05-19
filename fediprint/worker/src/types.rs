use serde::{Deserialize, Serialize};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::Type, Clone)]
#[sqlx(type_name = "model_license", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum JobStatus {
    Unprocessed,
    Processing,
    Finished,
    WaitingForRetry
}

#[derive(FromPrimitive, ToPrimitive)]
pub enum JobType {
    SendRegisterEmail
}