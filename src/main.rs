use dotenv::dotenv;
use helpers::Config;
use std::sync::Arc;

use crate::prisma::*;
use crate::routes::api::v1;
use axum::{routing::{post, get}, Router, middleware};
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use crate::helpers::middleware::auth_middleware;

pub mod helpers;
pub mod models;
pub mod prisma;
pub mod routes;


pub struct AppState {
    env: Config,
    db: PrismaClient
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    let prisma_client = PrismaClient::_builder().build().await.unwrap();
    let state = Arc::new(AppState {
        db: prisma_client,
        env: config
    });

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
        .route("/api/v1/auth/me", get(v1::auth::get_me_handler).route_layer(middleware::from_fn_with_state(state.clone(), auth_middleware)))
        .with_state(state)
        .layer(cors);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
