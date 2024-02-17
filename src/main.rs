use dotenv::dotenv;
use helpers::Config;
use std::sync::Arc;

use crate::prisma::*;
use crate::routes::api::v1;
use axum::{extract::Extension, routing::post, Router};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

pub mod helpers;
pub mod models;
pub mod prisma;
pub mod routes;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Arc::new(Config::init());

    let prisma_client = Arc::new(PrismaClient::_builder().build().await.unwrap());

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .route("/api/v1/auth/create", post(v1::auth::create_user))
        .route("/api/v1/auth/login", post(v1::auth::login))
        .layer(Extension(prisma_client))
        .layer(Extension(config))
        .layer(cors);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
