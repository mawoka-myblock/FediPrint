pub mod auth;
pub mod interactions;
pub mod middleware;
pub mod printables;
pub mod search;
pub mod sign;

use axum::body::Body;
use axum::http::header::ToStrError;
use axum::http::HeaderMap;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use meilisearch_sdk::errors::Error as ms_error;
use reqwest::Error;
use s3::error::S3Error;
use sqlx::Error as SqlxError;
use std::borrow::Cow;
use std::str::FromStr;
use tracing::debug;

pub type AppJsonResult<T> = AppResult<Json<T>>;

#[derive(Debug)]
pub enum AppError {
    SqlxError(SqlxError),
    ToStrError(ToStrError),
    S3Error(S3Error),
    MeiliSearchError(ms_error),
    NotFound,
    InternalServerError,
}

pub fn internal_app_error<E>(_: E) -> AppError
where
    E: std::error::Error,
{
    AppError::InternalServerError
}

pub type AppResult<T> = Result<T, AppError>;

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        debug!("{:?}", &error);
        AppError::SqlxError(error)
    }
}

impl From<ToStrError> for AppError {
    fn from(error: ToStrError) -> Self {
        AppError::ToStrError(error)
    }
}

impl From<S3Error> for AppError {
    fn from(error: S3Error) -> Self {
        debug!("{:?}", &error);
        AppError::S3Error(error)
    }
}

impl From<ms_error> for AppError {
    fn from(error: ms_error) -> Self {
        AppError::MeiliSearchError(error)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(_: Error) -> Self {
        AppError::InternalServerError
    }
}

// This centralizes all different errors from our app in one place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let unique_error = Cow::from("23505");
        let status = match self {
            AppError::SqlxError(error) => match error {
                SqlxError::RowNotFound => StatusCode::NOT_FOUND,
                SqlxError::Database(d) => {
                    let e = d.code();
                    if e == Some(unique_error) {
                        StatusCode::CONFLICT
                    } else {
                        StatusCode::INTERNAL_SERVER_ERROR
                    }
                }
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            AppError::MeiliSearchError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::ToStrError(_) => StatusCode::BAD_REQUEST,
            AppError::S3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}

pub fn ensure_ap_header(headers: &HeaderMap) -> Result<(), Response> {
    let is_ap_h = is_ap_header(headers)?;
    if is_ap_h {
        return Ok(());
    };
    Err(Response::builder()
        .status(StatusCode::NOT_ACCEPTABLE)
        .body(Body::from(""))
        .unwrap())
}

pub fn is_ap_header(headers: &HeaderMap) -> Result<bool, Response> {
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
                .body(Body::from("Accept header empty"))
                .unwrap());
        }
    };
    Ok(accept_h.contains("application/activity+json"))
    // TODO allow "application/ld+json; profile="https://www.w3.org/ns/activitystreams"" as well
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
    pub meilisearch_url: String,
    pub meilisearch_key: String,
    pub registration_disabled: bool,
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
        let registration_disabled =
            bool::from_str(&std::env::var("REGISTRATION_DISABLED").unwrap_or("false".to_string()))
                .expect("REGISTRATION_DISABLED no valid boolean");
        let meilisearch_url =
            std::env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
        let meilisearch_key =
            std::env::var("MEILISEARCH_KEY").expect("MEILISEARCH_KEY must be set");
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
            meilisearch_url,
            meilisearch_key,
            registration_disabled,
        }
    }
}
