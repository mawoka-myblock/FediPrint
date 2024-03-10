use crate::helpers::auth::UserState;
use crate::helpers::{AppResult, internal_app_error};
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
use diesel_async::RunQueryDsl;
use crate::models::db::EventAudience;
use crate::models::db::note::{CreateNote, FullNote, UserFacingNote};

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
    use crate::schema::Note::dsl::*;
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    let unfinished_note = diesel::insert_into(Note::table)
        .values(&CreateNote{
            server_id: None,
            content: input.content,
            hashtags: Some(input.hashtags),
            audience: input.audience,
            comment_of_model_id: None,
            in_reply_to_note_id: None,
            actor_id: claims.profile_id,
            in_reply_to_comment_id: None
        }).returning(FullNote::as_returning()).get_result(&mut conn).await?;
    let s_id = format!(
        "{}/api/v1/notes/{}/{}",
        state.env.public_url, claims.username, &unfinished_note.id
    );
    let note = diesel::update(Note.find(unfinished_note.id)).set(server_id.eq(s_id)).returning(UserFacingNote::as_returning()).get_result(&mut conn).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&note).unwrap()))
        .unwrap())
}
