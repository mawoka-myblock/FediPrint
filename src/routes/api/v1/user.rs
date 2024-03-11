use crate::helpers::{ensure_ap_header, AppResult, internal_app_error};
use crate::models::activitypub::{
    AlsoKnownAs, Claim, Context, Endpoints, FingerprintKey, FocalPoint, IdentityKey,
    NoteBoxItemFirst, NoteBoxItemObject, NoteBoxItemReplies, NoteBoxItemRoot, OrderedCollection,
    OrderedItem, OutboxContext, OutboxDataPage, PeopleDataPage, Profile, PublicKey,
};
use crate::{AppState};
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_derive::Deserialize;
use serde_json::json;
use std::fmt::format;
use std::sync::Arc;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use crate::models::db::profile::FullProfile;

#[debug_handler]
pub async fn get_user_profile(
    Path(username): Path<String>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    use crate::schema::Profile::dsl::{Profile, server,username as db_username};

    let user = Profile.filter(db_username.eq(username))
        .filter(server.eq(state.env.base_domain))
        .select(FullProfile::as_select())
        .first(&mut conn)
        .await?;
    let data = Profile {
        context: (
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string(),
            Context {
                curve25519key: Some("toot:Curve25519Key".to_string()),
                // device: "toot:Device".to_string(),
                ed25519key: Some("toot:Ed25519Key".to_string()),
                ed25519signature: Some("toot:Ed25519Signature".to_string()),
                encrypted_message: Some("toot:EncryptedMessage".to_string()),
                hashtag: Some("as:Hashtag".to_string()),
                property_value: Some("schema:PropertyValue".to_string()),
                also_known_as: AlsoKnownAs {
                    id: "as:alsoKnownAs".to_string(),
                    type_field: "@id".to_string(),
                },
                cipher_text: Some("toot:cipherText".to_string()),
                claim: Some(Claim {
                    id: "toot:claim".to_string(),
                    type_field: "@id".to_string(),
                }),
                // device_id: "toot:deviceId".to_string(),
                // devices: Devices{
                //     id: "toot:devices".to_string(),
                //     type_field: "@id".to_string()
                // },
                // discoverable: "toot:discoverable".to_string(),
                // featured: Featured {
                //     id: "toot:featured".to_string(),
                //     type_field: "@id".to_string()
                // },
                // featured_tags: FeaturedTags {
                //     id: "toot:featuredTags".to_string(),
                //     type_field: "@id".to_string()
                // },
                fingerprint_key: Some(FingerprintKey {
                    id: "toot:fingerprintKey".to_string(),
                    type_field: "@id".to_string(),
                }),
                // focal_point: FocalPoint {
                //     container: "@list".to_string(),
                //     id: "toot:focalPoint".to_string(),
                // },
                identity_key: Some(IdentityKey {
                    id: "toot:identityKey".to_string(),
                    type_field: "@id".to_string(),
                }),
                // indexable: "toot:indexable".to_string(),
                // manually_approves_followers: "as:manuallyApprovesFollowers".to_string(),
                // memorial: "toot:memorial".to_string(),
                message_franking: Some("toot:messageFranking".to_string()),
                message_type: Some("toot:messageType".to_string()),
                // moved_to: MovedTo {
                //     id: "as:movedTo".to_string(),
                //     type_field: "@id".to_string()
                // },
                public_key_base64: Some("toot:publicKeyBase64".to_string()),
                schema: Some("http://schema.org#".to_string()),
                // suspended: "toot:suspended".to_string(),
                toot: "http://joinmastodon.org/ns#".to_string(),
                value: Some("schema:value".to_string()),
            },
        ),
        endpoints: Endpoints {
            shared_inbox: format!("{}/api/v1/inbox", state.env.public_url),
        },
        followers: format!(
            "{}/api/v1/user/{}/followers",
            state.env.public_url, user.username
        ),
        following: format!(
            "{}/api/v1/user/{}/following",
            state.env.public_url, user.username
        ),
        id: format!("{}/api/v1/user/{}", state.env.public_url, user.username),
        inbox: format!(
            "{}/api/v1/user/{}/inbox",
            state.env.public_url, user.username
        ),
        name: user.username.clone(),
        outbox: format!(
            "{}/api/v1/user/{}/outbox",
            state.env.public_url, user.username
        ),
        preferred_username: user.display_name,
        public_key: PublicKey {
            id: format!(
                "{}/api/v1/user/{}#main-key",
                state.env.public_url, user.username
            ),
            owner: format!("{}/api/v1/user/{}", state.env.public_url, user.username),
            public_key_pem: user.public_key,
        },
        published: user.registered_at.to_rfc3339(),
        type_field: "Person".to_string(),
        url: format!("{}/@{}", state.env.public_url, user.username),
    };
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct GetFollowersQuery {
    pub page: Option<usize>,
}

//noinspection Annotator
#[debug_handler]
pub async fn get_followers(
    Path(username): Path<String>,
    headers: HeaderMap,
    query: Query<GetFollowersQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let page = query.page;

    // TODO get the count right
    let count: i64 = 12;

    // TODO get 404 handling
    if page.is_none() {
        let return_data = OrderedCollection {
            context: "https://www.w3.org/ns/activitystreams".to_string(),
            first: format!(
                "{}/api/v1/user/{}/followers?page=1",
                state.env.public_url, &username
            ),
            id: format!(
                "{}/api/v1/user/{}/followers",
                state.env.public_url, &username
            ),
            total_items: count,
            type_field: "OrderedCollection".to_string(),
            last: None,
        };
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/activity+json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&return_data).unwrap()))
            .unwrap());
    }
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    use crate::schema::Profile::dsl::{Profile, server,username as db_username};

    let user = Profile.filter(db_username.eq(username))
        .filter(server.eq(state.env.base_domain))
        .select(FullProfile::as_select())
        .first(&mut conn)
        .await?;

/*    let user = match state
        .db
        .profile()
        .find_first(vec![
            prisma::profile::username::equals(username),
            prisma::profile::server::equals("local".to_string()),
        ])
        .include(prisma::profile::include!({
            followers: select
            {
                server_id
            }
        }))
        .exec()
        .await?
    {
        Some(d) => d,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap());
        }
    };*/
    // Data { id: "ff928bab-96ea-485b-9d40-667e79a19dcc", username: "Mawoka", server: "local", display_name: "Mawoka", summary: "", public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3OV1S2zNjE0OPICeA9pC\nm3pi5x6u9NLYyY51OSutpLpCFLEA50HjXKvqCaXVNtRXzmQmMg5lsrimm+/nJT3a\nrKuLhecXo6HrOV6GQ2+4n/kRRk75Uymk80upeAH5uI6CFBGB+1114JZp5MonuHQx\nt1um+DR3gtFkp8TLiJp5xk/L4/OQMBDfJROCKRw3OFFmEiWM9JlMxHOhekXkl9uc\nljVoese7xVaw+lD0R7sxdqLBHgjDDgf3A6dAQ/fTG+7DGUbMZvubvpQu7taCpevi\nLTtAi94R8RcLcg6/yAACXe2+gn2fGTeT2MncJgNuwTnZjNWmEfRNvX0cZ32qUc99\nfQIDAQAB\n-----END PUBLIC KEY-----\n", registered_at: 2024-02-18T00:00:00+00:00, followers: [Data { server_id: "http://localhost:3000/api/v1/user/Mawoka" }] }

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/followers?page={}", user.server_id, page.unwrap()),
        next: format!("{}/followers?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: /*user
            .followers
            .iter()
            .map(|server| server.server_id.to_string())
            .collect(),*/ vec!["PLACEHOLDER".to_string()],
        part_of: format!("{}/followers", user.server_id),
        total_items: count,
        type_field: "OrderedCollectionPage".to_string(),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}

#[debug_handler]
pub async fn get_following(
    Path(username): Path<String>,
    headers: HeaderMap,
    query: Query<GetFollowersQuery>,
    State(state): State<Arc<AppState>>,
) -> AppResult<impl IntoResponse> {
    match ensure_ap_header(&headers) {
        Ok(_) => (),
        Err(e) => return Ok(e),
    };
    let page = query.page;

    // TODO get the count right
    let count: i64 = 12;

    // TODO get 404 handling
    if page.is_none() {
        let return_data = OrderedCollection {
            context: "https://www.w3.org/ns/activitystreams".to_string(),
            first: format!(
                "{}/api/v1/user/{}/following?page=1",
                state.env.public_url, &username
            ),
            id: format!(
                "{}/api/v1/user/{}/following",
                state.env.public_url, &username
            ),
            total_items: count,
            type_field: "OrderedCollection".to_string(),
            last: None,
        };
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/activity+json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&return_data).unwrap()))
            .unwrap());
    }
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    use crate::schema::Profile::dsl::{Profile, server,username as db_username};

    let user = Profile.filter(db_username.eq(username))
        .filter(server.eq(state.env.base_domain))
        .select(FullProfile::as_select())
        .first(&mut conn)
        .await?;
/*    let user = match state
        .db
        .profile()
        .find_first(vec![
            prisma::profile::username::equals(username),
            prisma::profile::server::equals("local".to_string()),
        ])
        .include(prisma::profile::include!({
            following: select
            {
                server_id
            }
        }))
        .exec()
        .await?
    {
        Some(d) => d,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap());
        }
    }; */
    // Data { id: "ff928bab-96ea-485b-9d40-667e79a19dcc", username: "Mawoka", server: "local", display_name: "Mawoka", summary: "", public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3OV1S2zNjE0OPICeA9pC\nm3pi5x6u9NLYyY51OSutpLpCFLEA50HjXKvqCaXVNtRXzmQmMg5lsrimm+/nJT3a\nrKuLhecXo6HrOV6GQ2+4n/kRRk75Uymk80upeAH5uI6CFBGB+1114JZp5MonuHQx\nt1um+DR3gtFkp8TLiJp5xk/L4/OQMBDfJROCKRw3OFFmEiWM9JlMxHOhekXkl9uc\nljVoese7xVaw+lD0R7sxdqLBHgjDDgf3A6dAQ/fTG+7DGUbMZvubvpQu7taCpevi\nLTtAi94R8RcLcg6/yAACXe2+gn2fGTeT2MncJgNuwTnZjNWmEfRNvX0cZ32qUc99\nfQIDAQAB\n-----END PUBLIC KEY-----\n", registered_at: 2024-02-18T00:00:00+00:00, following: [Data { server_id: "http://localhost:3000/api/v1/user/Mawoka" }] }

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/following?page={}", user.server_id, page.unwrap()),
        next: format!("{}/following?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: /*user
            .following
            .iter()
            .map(|server| server.server_id.to_string())
            .collect(),*/vec!["PLACEHOLDER".to_string()],
        part_of: format!("{}/following", user.server_id),
        total_items: count,
        type_field: "OrderedCollectionPage".to_string(),
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/activity+json; charset=utf-8")
        .body(Body::from(serde_json::to_string(&data).unwrap()))
        .unwrap())
}

#[derive(Deserialize)]
pub struct GetBox {
    pub page: Option<bool>,
    pub min_id: Option<i64>,
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
    let mut conn = state.db.get().await.map_err(internal_app_error)?;
    use crate::schema::Profile::dsl::{Profile, server,username as db_username};

    let user = Profile.filter(db_username.eq(username))
        .filter(server.eq(state.env.base_domain))
        .select(FullProfile::as_select())
        .first(&mut conn)
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
/*    let data = state
        .db
        .note()
        .find_many(vec![note::actor_id::equals(user.id)])
        .include(prisma::note::include!({
            mentions: select
            {
                server_id
            }
            in_reply_to_comment: select
            {
                server_id
            }
            in_reply_to_note: select
            {
                server_id
            }
        }))
        .exec()
        .await?;*/
    // [Data { id: 1, created_at: 2024-02-24T10:22:48.100+00:00, server_id: Some("http://localhost:3000/api/v1/notes/string/1"), content: "<string>", hashtags: ["<string>", "<string>"], audience: Public, in_reply_to_comment_id: None, in_reply_to_note_id: None, actor_id: "36e820ce-e913-4402-ae7b-86d3cb1552cb", mentions: [], in_reply_to_comment: None, in_reply_to_note: None }]
    let mut ordered_items: Vec<OrderedItem> = vec![];
    /*println!("{:?}", data);
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
                tag: vec![], // TODO
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
    }*/

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
