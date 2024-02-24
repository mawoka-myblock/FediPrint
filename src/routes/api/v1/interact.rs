use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use crate::prisma::{note, profile, EventAudience};
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde::Serialize;
use serde_derive::Deserialize;
use std::any::Any;
use std::sync::Arc;

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
    let note = match state
        .db
        .note()
        .create(
            input.content,
            input.audience,
            profile::id::equals(claims.profile_id.to_string()),
            vec![
                note::hashtags::set(input.hashtags),
                note::mentions::connect(mentions_vec),
            ],
        )
        .exec()
        .await
    {
        Ok(d) => d,
        Err(e) => {
            println!("{:?}", e);
            return Ok(Response::builder()
                .status(StatusCode::PRECONDITION_FAILED)
                .body(Body::from(""))
                .unwrap());
        }
    };
    let server_id = format!(
        "{}/api/v1/notes/{}/{}",
        state.env.public_url, claims.username, &note.id
    );

    state
        .db
        .note()
        .update(
            note::id::equals(note.id),
            vec![note::server_id::set(Some(server_id))],
        )
        .exec()
        .await
        .unwrap();
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(""))
        .unwrap())
}
