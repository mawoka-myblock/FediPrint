use crate::helpers::activitypub::inbox_activities::{
    handle_accept, handle_add, handle_announce, handle_create, handle_delete, handle_follow,
    handle_like, handle_reject, handle_remove, handle_undo, handle_update,
};
use crate::helpers::{ensure_ap_header, AppResult};
use crate::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::Deserialize;
use serde_json::json;
use shared::db::profile::FullProfile;
use shared::models::activitypub::{
    FocalPoint, NoteBoxItemFirst, NoteBoxItemObject, NoteBoxItemReplies, NoteJoinedModel,
    OrderedCollection, OrderedItem, OutboxContext, OutboxDataPage, Tag,
};
use shared::models::inbox::InboxEvent;
use std::sync::Arc;
use tracing::error;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetBox {
    pub page: Option<bool>,
    pub min_id: Option<Uuid>,
}

pub async fn get_outbox(
    Path(username): Path<String>,
    headers: HeaderMap,
    query: Query<GetBox>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };

    // TODO get the count right
    let count: i64 = 12;

    let user = FullProfile::get_by_username_and_server(
        &username,
        &state.env.base_domain,
        state.pool.clone(),
    )
    .await?;

    if query.page.is_none() {
        let return_data = OrderedCollection {
            context: "https://www.w3.org/ns/activitystreams".to_string(),
            first: format!(
                "{}/api/v1/user/{}/outbox?page=true",
                state.env.public_url, &user.username
            ),
            id: format!(
                "{}/api/v1/user/{}/outbox",
                state.env.public_url, &user.username
            ),
            total_items: count,
            type_field: "OrderedCollection".to_string(),
            last: Some(format!(
                "{}/api/v1/user/{}/outbox?min_id=0",
                state.env.public_url, &user.username
            )),
        };
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/activity+json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&return_data).unwrap()))
            .unwrap());
    }
    let data = NoteJoinedModel::get_by_profile_id(&user.id, state.pool.clone()).await?;
    let mut ordered_items: Vec<OrderedItem> = vec![];
    for item in data {
        let to = vec!["https://www.w3.org/ns/activitystreams#Public".to_string()];
        let cc = vec![format!(
            "{}/api/v1/user/{}/followers",
            state.env.public_url, &user.username
        )];
        ordered_items.push(OrderedItem {
            type_field: "Create".to_string(),
            id: format!(
                "{}/api/v1/user/{}/statuses/{}/activity",
                state.env.public_url,
                &user.username,
                &item.note_id.or(item.model_id).unwrap()
            ),
            actor: user.server_id.to_string(),
            published: item.created_at.to_string(),
            to: to.clone(),
            cc: cc.clone(),
            object: json!(NoteBoxItemObject {
                id: format!(
                    "{}/api/v1/user/{}/statuses/{}",
                    state.env.public_url,
                    &user.username,
                    &item.note_id.or(item.model_id).unwrap()
                ),
                type_field: "Note".to_string(),
                to,
                cc,
                content: item.content,
                summary: item.summary,
                tag: Tag::from_strs(item.hashtags, &state.env.public_url),
                replies: NoteBoxItemReplies {
                    id: format!(
                        "{}/api/v1/user/{}/statuses/{}/replies",
                        state.env.public_url,
                        &user.username,
                        &item.note_id.or(item.model_id).unwrap()
                    ),
                    type_field: "Collection".to_string(),
                    first: NoteBoxItemFirst {
                        type_field: "CollectionPage".to_string(),
                        next: format!(
                            "{}/api/v1/user/{}/statuses/{}/replies?page=true",
                            state.env.public_url,
                            &user.username,
                            &item.note_id.or(item.model_id).unwrap()
                        ),
                        part_of: format!(
                            "{}/api/v1/user/{}/statuses/{}/replies",
                            state.env.public_url,
                            &user.username,
                            &item.note_id.or(item.model_id).unwrap()
                        ),
                        items: item.first_reply_server_id.map_or(Vec::new(), |s| vec![s])
                    }
                },
                attachment: vec![],
                attributed_to: user.server_id.to_string(),
                updated: None,
                published: item.created_at.to_string(),
                url: format!(
                    "{}/@{}/statuses/{}",
                    state.env.public_url,
                    &user.username,
                    &item.note_id.or(item.model_id).unwrap()
                ),
                // in_reply_to: matchitem.in_reply_to_comment_id
                in_reply_to: serde_json::Value::String("PLACEHOLDER".to_string())
            }),
        })
    }

    let data = OutboxDataPage {
        context: (
            "https://www.w3.org/ns/activitystreams".to_string(),
            OutboxContext {
                ostatus: "http://ostatus.org#".to_string(),
                conversation: "ostatus:conversation".to_string(),
                toot: "http://joinmastodon.org/ns#".to_string(),
                hashtag: "as:Hashtag".to_string(),
                blurhash: "toot:blurhash".to_string(),
                focal_point: FocalPoint {
                    container: "@list".to_string(),
                    id: "toot:focalPoint".to_string(),
                },
            },
        ),
        id: format!(
            "{}/api/v1/user/{}/outbox?page=true",
            state.env.public_url, &user.username
        ),
        type_field: "OrderedCollectionPage".to_string(),
        next: "TO_BE_IMPLEMENTED".to_string(), // TODO
        // Example: https://mastodon.online/users/Mawoka/outbox?max_id=111828830388463327&page=true
        prev: "TO_BE_IMPLEMENTED".to_string(), // TODO
        // Example: https://mastodon.online/users/Mawoka/outbox?min_id=111953759725965499&page=true
        part_of: format!(
            "{}/api/v1/user/{}/outbox",
            state.env.public_url, &user.username
        ),
        ordered_items,
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}

pub async fn post_inbox(
    Path(username): Path<String>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(event): Json<InboxEvent>,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let event_type = event.event_type.as_str();
    let _ = match event_type {
        "Announce" => handle_announce(event).await,
        "Create" => handle_create(event).await,
        "Update" => handle_update(event).await,
        "Delete" => handle_delete(event).await,
        "Follow" => handle_follow(event).await,
        "Accept" => handle_accept(event).await,
        "Reject" => handle_reject(event).await,
        "Remove" => handle_remove(event).await,
        "Like" => handle_like(event).await,
        "Undo" => handle_undo(event).await,
        "Add" => handle_add(event).await,
        _ => {
            error!("Unknown event: {}", event_type);
            Ok(())
        }
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(Body::from(""))
        .unwrap())
}
