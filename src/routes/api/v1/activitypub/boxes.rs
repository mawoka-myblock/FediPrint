use crate::helpers::{ensure_ap_header, AppResult};
use crate::models::activitypub::{
    FocalPoint, NoteBoxItemFirst, NoteBoxItemObject, NoteBoxItemReplies, OrderedCollection,
    OrderedItem, OutboxContext, OutboxDataPage, Tag,
};
use crate::models::db::note::BoxNote;
use crate::models::db::profile::FullProfile;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_derive::Deserialize;
use serde_json::json;
use std::sync::Arc;
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
    let data = BoxNote::get_by_profile_id(&user.id, state.pool.clone()).await?;
    let mut ordered_items: Vec<OrderedItem> = vec![];
    for item in data {
        let to = vec![
            "https://www.w3.org/ns/activitystreams#Public".to_string(), // TODO Implement check for Audience
        ];
        let cc = vec![format!(
            "{}/api/v1/user/{}/followers",
            state.env.public_url, &user.username
        )];
        ordered_items.push(OrderedItem {
            type_field: "Create".to_string(),
            id: format!(
                "{}/api/v1/user/{}/statuses/{}/activity",
                state.env.public_url, &user.username, &item.id
            ),
            actor: user.server_id.to_string(),
            published: item.created_at.to_string(),
            to: to.clone(),
            cc: cc.clone(),
            object: json!(NoteBoxItemObject {
                id: format!(
                    "{}/api/v1/user/{}/statuses/{}",
                    state.env.public_url, &user.username, &item.id
                ),
                type_field: "Note".to_string(),
                to,
                cc,
                content: item.content,
                tag: Tag::from_strs(item.hashtags, &state.env.public_url),
                replies: NoteBoxItemReplies {
                    id: format!(
                        "{}/api/v1/user/{}/statuses/{}/replies",
                        state.env.public_url, &user.username, &item.id
                    ),
                    type_field: "Collection".to_string(),
                    first: NoteBoxItemFirst {
                        type_field: "CollectionPage".to_string(),
                        next: "TO_BE_IMPLEMENTED".to_string(), // TODO
                        // Example: https://mastodon.online/users/Mawoka/statuses/111952053623777585/replies?only_other_accounts=true&page=true
                        part_of: format!(
                            "{}/api/v1/user/{}/statuses/{}/replies",
                            state.env.public_url, &user.username, &item.id
                        ),
                        items: vec![]
                    }
                },
                attachment: vec![],
                attributed_to: user.server_id.to_string(),
                updated: None,
                published: item.created_at.to_string(),
                url: format!(
                    "{}/@{}/statuses/{}",
                    state.env.public_url, &user.username, &item.id
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
