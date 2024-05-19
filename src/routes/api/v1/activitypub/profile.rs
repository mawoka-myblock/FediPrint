use crate::helpers::{ensure_ap_header, AppResult};
use database::::activitypub::Profile;
use database::::activitypub::{
    AlsoKnownAs, Claim, Context, Endpoints, FingerprintKey, IdentityKey, OrderedCollection,
    PeopleDataPage, PublicKey,
};
use database::::db::profile::{FullProfile, FullProfileWithFollower, FullProfileWithFollowing};
use crate::AppState;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use serde_derive::Deserialize;
use std::sync::Arc;

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

    let user = FullProfile::get_by_username_and_server(
        &username,
        &state.env.base_domain,
        state.pool.clone(),
    )
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

    let user = FullProfile::get_by_username_and_server(
        &username,
        &state.env.base_domain,
        state.pool.clone(),
    )
    .await?;
    let count: i64 =
        FullProfileWithFollowing::count_following(&user.id, state.pool.clone()).await?;

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

    let data = FullProfileWithFollower::get_by_id(&user.id, state.pool.clone()).await?;

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/followers?page={}", user.server_id, page.unwrap()),
        next: format!("{}/followers?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: data
            .followers
            .iter()
            .map(|d| d.server_id.to_string())
            .collect(),
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

    let user = FullProfile::get_by_username_and_server(
        &username,
        &state.env.base_domain,
        state.pool.clone(),
    )
    .await?;
    let count: i64 =
        FullProfileWithFollowing::count_following(&user.id, state.pool.clone()).await?;

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
    let data = FullProfileWithFollowing::get_by_id(&user.id, state.pool.clone()).await?;

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/following?page={}", user.server_id, page.unwrap()),
        next: format!("{}/following?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: data
            .following
            .iter()
            .map(|follower| follower.server_id.to_string())
            .collect(),
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
