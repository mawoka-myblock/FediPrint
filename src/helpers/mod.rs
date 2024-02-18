pub mod auth;
pub mod middleware;

use std::sync::Arc;

use crate::prisma::*;
use axum::body::Body;
use axum::http::header::ToStrError;
use axum::http::HeaderMap;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use prisma_client_rust::{
    prisma_errors::query_engine::{RecordNotFound, UniqueKeyViolation},
    QueryError,
};

pub type AppJsonResult<T> = AppResult<Json<T>>;

pub enum AppError {
    PrismaError(QueryError),
    ToStrError(ToStrError),
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

impl From<ToStrError> for AppError {
    fn from(error: ToStrError) -> Self {
        AppError::ToStrError(error)
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
            AppError::ToStrError(_) => StatusCode::BAD_REQUEST,
        };

        status.into_response()
    }
}

pub fn ensure_ap_header(headers: &HeaderMap) -> Result<(), Response> {
    let accept_h = match headers.get("accept") {
        Some(d) => d.to_str().map_err(|_| {
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to read header"))
                .unwrap()
        })?,
        None => {
            return Err(Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .body(Body::from(""))
                .unwrap());
        }
    };
    if !accept_h.contains("application/activity+json") {
        return Err(Response::builder()
            .status(StatusCode::NOT_ACCEPTABLE)
            .body(Body::from(""))
            .unwrap());
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub public_url: String,
    pub base_domain: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");
        let base_domain = std::env::var("BASE_DOMAIN").expect("BASE_DOMAIN must be set");
        Config {
            database_url,
            jwt_secret,
            public_url,
            base_domain,
        }
    }
}
