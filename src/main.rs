use dotenv::dotenv;
use helpers::Config;
use std::sync::Arc;

use crate::helpers::middleware::auth_middleware;
use crate::prisma::*;
use crate::routes::api::v1;
use awscreds::Credentials;
use axum::http::Method;
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use s3::{Bucket, BucketConfiguration, Region};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod helpers;
pub mod models;
pub mod prisma;
pub mod routes;

pub struct AppState {
    env: Config,
    db: PrismaClient,
    s3: Bucket,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let config = Config::init();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "fedi_print=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let prisma_client = PrismaClient::_builder().build().await.unwrap();
    let s3_region = Region::Custom {
        region: config.s3_region.clone(),
        endpoint: config.s3_base_url.clone(),
    };
    let s3_creds = Credentials::new(
        Some(&config.s3_username),
        Some(&config.s3_password),
        None,
        None,
        None,
    )
    .expect("S3 credentials invalid");
    let mut bucket = Bucket::new(&config.s3_bucket_name, s3_region.clone(), s3_creds.clone())
        .expect("S3 Bucket initialization failed");
    bucket.set_path_style();

    if !bucket.exists().await.unwrap() {
        bucket = Bucket::create_with_path_style(
            &config.s3_bucket_name,
            s3_region,
            s3_creds,
            BucketConfiguration::default(),
        )
        .await
        .unwrap()
        .bucket;
        bucket.set_path_style();
    }

    let state = Arc::new(AppState {
        db: prisma_client,
        env: config,
        s3: bucket,
    });

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a single route
    let app = Router::new()
        // .route("/", get(|| async { "Hello, World!" }))
        .layer(cors)
        .route("/api/v1/auth/create", post(v1::auth::create_user))
        .route("/api/v1/auth/login", post(v1::auth::login))
        .route(
            "/api/v1/auth/me",
            get(v1::auth::get_me_handler).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/.well-known/webfinger",
            get(routes::well_known::webfinger::handler),
        )
        .route("/api/v1/user/:username", get(v1::user::get_user_profile))
        .route(
            "/api/v1/user/:username/followers",
            get(v1::user::get_followers),
        )
        .route(
            "/api/v1/user/:username/following",
            get(v1::user::get_following),
        )
        .route(
            "/api/v1/manage/follow",
            post(v1::manage::follow_user_route).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/manage/interact/note",
            post(v1::interact::post_note).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route("/api/v1/user/:username/outbox", get(v1::user::get_outbox))
        .route(
            "/api/v1/printers/create",
            post(v1::printers::create_printer).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/printers/list",
            get(v1::printers::get_all_printers).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/printers/update",
            put(v1::printers::update_printer).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/storage/upload",
            post(v1::storage::upload_file).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/storage/edit",
            put(v1::storage::edit_file_metadata).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/storage/list",
            get(v1::storage::list_own_files).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/model/create",
            post(v1::model::create_model).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http());
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
