use dotenvy::dotenv;
use shared::helpers::config::Config;
use std::sync::Arc;
use std::time::Duration;

use crate::helpers::middleware::auth_middleware;
use crate::routes::api::v1;
use awscreds::Credentials;
use axum::extract::DefaultBodyLimit;
use axum::http::Method;
use axum::{
    middleware,
    routing::{delete, get, head, patch, post, put},
    Router,
};
use s3::{Bucket, BucketConfiguration, Region};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
#[cfg(test)]
use uuid::{uuid, Uuid};

pub mod helpers;
pub mod routes;

pub struct AppState {
    env: Config,
    s3: Bucket,
    pool: PgPool,
    ms: meilisearch_sdk::Index,
}

#[cfg(test)]
pub static TEST_ACCOUNT_UUID: Uuid = uuid!("018e7b20-51e5-79c2-878e-02d01f941165");
#[cfg(test)]
pub static TEST_PROFILE_UUID: Uuid = uuid!("018e7b20-51bd-703a-96c6-9c70cc723c67");

pub async fn get_state(pool: Option<PgPool>) -> Arc<AppState> {
    dotenv().ok();
    let config = Config::init();

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
    let sqlx_pool = match pool {
        Some(d) => d,
        None => PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.database_url)
            .await
            .expect("can't connect to database"),
    };
    sqlx::migrate!("../../migrations")
        .run(&sqlx_pool)
        .await
        .unwrap();

    let client =
        meilisearch_sdk::Client::new(&config.meilisearch_url, Some(&config.meilisearch_key));
    // Make sure Meilisearch is more or less working as the client init doesn't do any checks
    assert_eq!(client.health().await.unwrap().status, "available");
    client.create_index("fedi_print", Some("id")).await.unwrap();
    client
        .index("fedi_print")
        .set_filterable_attributes(&["created_at", "tags", "record_type", "profile_id"])
        .await
        .unwrap();
    client
        .index("fedi_print")
        .set_sortable_attributes(&["created_at", "updated_at"])
        .await
        .unwrap();

    Arc::new(AppState {
        env: config,
        s3: bucket,
        pool: sqlx_pool,
        ms: client.index("fedi_print"),
    })
}

pub async fn get_server() -> Router {
    let state = get_state(None).await;

    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    // build our application with a single route
    Router::new()
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
        .route(
            "/.well-known/nodeinfo",
            get(routes::well_known::nodeinfo::handler),
        )
        .route(
            "/api/v1/user/:username",
            get(v1::activitypub::profile::get_user_profile),
        )
        .route(
            "/api/v1/user/:username/followers",
            get(v1::activitypub::profile::get_followers),
        )
        .route(
            "/api/v1/user/:username/following",
            get(v1::activitypub::profile::get_following),
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
        .route(
            "/api/v1/user/:username/outbox",
            get(v1::activitypub::boxes::get_outbox),
        )
        .route(
            "/api/v1/user/:username/inbox",
            get(v1::activitypub::boxes::post_inbox),
        )
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
            post(v1::storage::upload_file)
                .layer(DefaultBodyLimit::max(52_428_800))
                .route_layer(middleware::from_fn_with_state(
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
            "/api/v1/storage/delete",
            delete(v1::storage::delete_file).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route("/api/v1/storage/download/:id", get(v1::storage::get_file))
        .route(
            "/api/v1/storage/download/:id",
            head(v1::storage::get_file_head),
        )
        .route(
            "/api/v1/model/create",
            post(v1::model::create_model).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/model/list",
            get(v1::model::list_own_models).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/model/visibility",
            patch(v1::model::change_model_visibility).route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            )),
        )
        .route(
            "/api/v1/model/public/newest",
            get(v1::model::get_newest_models),
        )
        .route("/api/v1/model/:id", get(v1::model::get_model))
        .route("/api/v1/search/model", get(v1::model::search_models))
        .route(
            "/api/v1/links/printables",
            post(v1::links::printables::link_to_printables).route_layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware),
            ),
        )
        .route(
            "/api/v1/links/printables/import",
            post(v1::links::printables::import_all_from_printables).route_layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware),
            ),
        )
        .route(
            "/api/v1/links/printables/import/single",
            post(v1::links::printables::import_one_from_printables).route_layer(
                middleware::from_fn_with_state(state.clone(), auth_middleware),
            ),
        )
        .route("/api/v1/statuses/:id", get(v1::statuses::get_status))
        .route("/api/v1/nodeinfo/2.0", get(v1::nodeinfo::get_nodeinfo))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

#[tokio::main]
async fn main() {
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

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, get_server().await).await.unwrap();
}
