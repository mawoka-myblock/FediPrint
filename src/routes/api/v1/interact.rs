use std::sync::Arc;
use axum::{debug_handler, Extension, Json};
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_derive::Deserialize;
use crate::AppState;
use crate::helpers::AppResult;
use crate::helpers::auth::UserState;
use crate::prisma::{EventAudience, note, profile};

#[derive(Deserialize)]
pub struct PostNoteInput {
    pub content: String,
    pub audience: EventAudience,
    pub hashtags: Vec<String>,
    pub mentions: Vec<String>,
    pub in_reply_to: Option<String>,
}

#[debug_handler]
pub async fn post_note(
    Extension(claims): Extension<UserState>,
    State(state): State<Arc<AppState>>,
    Json(input): Json<PostNoteInput>,
) -> AppResult<impl IntoResponse> {
    let mut mentions_vec: Vec<profile::UniqueWhereParam> = vec![];
    for mention in input.mentions {
        mentions_vec.push(profile::server_id::equals(mention));
    }


    let note = state.db.note().create(
        input.content,
        input.audience,
        profile::id::equals(claims.sub.to_string()),
        vec![
            note::hashtags::set(input.hashtags),
            note::mentions::connect(mentions_vec),
        ],
    ).exec().await.unwrap();
    let server_id = format!("{}/api/v1/notes/{}/{}", state.env.public_url, claims.username, &note.id);

    state.db.note().update(note::id::equals(note.id), vec![note::server_id::set(Some(server_id))]).exec().await.unwrap();
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from(""))
        .unwrap())
}