pub mod auth;
pub mod interactions;
pub mod middleware;
pub mod sign;

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
use s3::error::S3Error;

pub type AppJsonResult<T> = AppResult<Json<T>>;

pub enum AppError {
    PrismaError(QueryError),
    ToStrError(ToStrError),
    S3Error(S3Error),
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

impl From<S3Error> for AppError {
    fn from(error: S3Error) -> Self {
        AppError::S3Error(error)
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
            AppError::S3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
        // TODO allow "application/ld+json; profile="https://www.w3.org/ns/activitystreams"" as well
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
    pub s3_base_url: String,
    pub s3_region: String,
    pub s3_username: String,
    pub s3_password: String,
    pub s3_bucket_name: String,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");
        let base_domain = std::env::var("BASE_DOMAIN").expect("BASE_DOMAIN must be set");
        let s3_base_url = std::env::var("S3_BASE_URL").expect("S3_BASE_URL must be set");
        let s3_region = std::env::var("S3_REGION").expect("S3_REGION must be set");
        let s3_username = std::env::var("S3_USERNAME").expect("S3_USERNAME must be set");
        let s3_password = std::env::var("S3_PASSWORD").expect("S3_PASSWORD must be set");
        let s3_bucket_name = std::env::var("S3_BUCKET_NAME").unwrap_or("fediprint".to_string());
        Config {
            database_url,
            jwt_secret,
            public_url,
            base_domain,
            s3_base_url,
            s3_region,
            s3_username,
            s3_password,
            s3_bucket_name,
        }
    }
}
