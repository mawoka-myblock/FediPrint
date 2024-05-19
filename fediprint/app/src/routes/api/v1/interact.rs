use crate::helpers::auth::UserState;
use crate::helpers::AppResult;
use shared::db::note::{CreateNote, UserFacingNote};
use shared::db::EventAudience;
use crate::AppState;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{debug_handler, Extension, Json};
use serde_derive::Deserialize;
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
    // let mut mentions_vec: Vec<profile::UniqueWhereParam> = vec![];
    // for mention in input.mentions {
    //     mentions_vec.push(profile::server_id::equals(mention));
    // }

    let unfinished_note = CreateNote {
        server_id: None,
        content: input.content,
        hashtags: input.hashtags,
        audience: input.audience,
        comment_of_model_id: None,
        in_reply_to_note_id: None,
        actor_id: claims.profile_id,
        in_reply_to_comment_id: None,
    }
    .create(state.pool.clone())
    .await?;
    let s_id = format!(
        "{}/api/v1/notes/{}/{}",
        state.env.public_url, claims.username, &unfinished_note.id
    );
    let note =
        UserFacingNote::set_server_id(&unfinished_note.id, &s_id, state.pool.clone()).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&note).unwrap()))
        .unwrap())
}
