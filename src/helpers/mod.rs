pub mod auth;

use std::sync::Arc;

use axum::{http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};
use crate::prisma::*;

pub type AppJsonResult<T> = AppResult<Json<T>>;

pub enum AppError {
    PrismaError(QueryError),
    NotFound,
}

pub type Database = Extension<Arc<PrismaClient>>;
pub type AppResult<T> = Result<T, AppError>;

impl From<QueryError> for AppError {
    fn from(error: QueryError) -> Self {
        match error {
            e if e.is_prisma_error::<RecordNotFound>() => AppError::NotFound,
            e => AppError::PrismaError(e),
        }
    }
}

// This centralizes all different errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::PrismaError(error) if error.is_prisma_error::<UniqueKeyViolation>() => {
                StatusCode::CONFLICT
            }
            AppError::PrismaError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        };

        status.into_response()
    }
}



#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        Config {
            database_url,
            jwt_secret,
        }
    }
}