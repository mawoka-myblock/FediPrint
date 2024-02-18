use crate::helpers::{ensure_ap_header, AppResult};
use crate::models::activitypub::{
    AlsoKnownAs, Claim, Context, Devices, Endpoints, Featured, FeaturedTags, FingerprintKey,
    FocalPoint, IdentityKey, MovedTo, PeopleData, PeopleDataPage, Profile, PublicKey,
};
use crate::prisma::profile;
use crate::routes::well_known::webfinger::WebfingerQuery;
use crate::{prisma, AppState};
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

    let user = match state
        .db
        .profile()
        .find_first(vec![
            prisma::profile::username::equals(username),
            prisma::profile::server::equals("local".to_string()),
        ])
        .exec()
        .await?
    {
        Some(d) => d,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from(""))
                .unwrap())
        }
    };
    let data = Profile {
        context: (
            "https://www.w3.org/ns/activitystreams".to_string(),
            "https://w3id.org/security/v1".to_string(),
            Context {
                curve25519key: "toot:Curve25519Key".to_string(),
                // device: "toot:Device".to_string(),
                ed25519key: "toot:Ed25519Key".to_string(),
                ed25519signature: "toot:Ed25519Signature".to_string(),
                encrypted_message: "toot:EncryptedMessage".to_string(),
                hashtag: "as:Hashtag".to_string(),
                property_value: "schema:PropertyValue".to_string(),
                also_known_as: AlsoKnownAs {
                    id: "as:alsoKnownAs".to_string(),
                    type_field: "@id".to_string(),
                },
                cipher_text: "toot:cipherText".to_string(),
                claim: Claim {
                    id: "toot:claim".to_string(),
                    type_field: "@id".to_string(),
                },
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
                fingerprint_key: FingerprintKey {
                    id: "toot:fingerprintKey".to_string(),
                    type_field: "@id".to_string(),
                },
                // focal_point: FocalPoint {
                //     container: "@list".to_string(),
                //     id: "toot:focalPoint".to_string(),
                // },
                identity_key: IdentityKey {
                    id: "toot:identityKey".to_string(),
                    type_field: "@id".to_string(),
                },
                // indexable: "toot:indexable".to_string(),
                // manually_approves_followers: "as:manuallyApprovesFollowers".to_string(),
                // memorial: "toot:memorial".to_string(),
                message_franking: "toot:messageFranking".to_string(),
                message_type: "toot:messageType".to_string(),
                // moved_to: MovedTo {
                //     id: "as:movedTo".to_string(),
                //     type_field: "@id".to_string()
                // },
                public_key_base64: "toot:publicKeyBase64".to_string(),
                schema: "http://schema.org#".to_string(),
                // suspended: "toot:suspended".to_string(),
                toot: "http://joinmastodon.org/ns#".to_string(),
                value: "schema:value".to_string(),
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
        let return_data = PeopleData {
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
        };
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/activity+json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&return_data).unwrap()))
            .unwrap());
    }

    let user = match state
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
                .unwrap())
        }
    }; // Data { id: "ff928bab-96ea-485b-9d40-667e79a19dcc", username: "Mawoka", server: "local", display_name: "Mawoka", summary: "", public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3OV1S2zNjE0OPICeA9pC\nm3pi5x6u9NLYyY51OSutpLpCFLEA50HjXKvqCaXVNtRXzmQmMg5lsrimm+/nJT3a\nrKuLhecXo6HrOV6GQ2+4n/kRRk75Uymk80upeAH5uI6CFBGB+1114JZp5MonuHQx\nt1um+DR3gtFkp8TLiJp5xk/L4/OQMBDfJROCKRw3OFFmEiWM9JlMxHOhekXkl9uc\nljVoese7xVaw+lD0R7sxdqLBHgjDDgf3A6dAQ/fTG+7DGUbMZvubvpQu7taCpevi\nLTtAi94R8RcLcg6/yAACXe2+gn2fGTeT2MncJgNuwTnZjNWmEfRNvX0cZ32qUc99\nfQIDAQAB\n-----END PUBLIC KEY-----\n", registered_at: 2024-02-18T00:00:00+00:00, followers: [Data { server_id: "http://localhost:3000/api/v1/user/Mawoka" }] }

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/followers?page={}", user.server_id, page.unwrap()),
        next: format!("{}/followers?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: user
            .followers
            .iter()
            .map(|server| server.server_id.to_string())
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

    // TODO get the count right
    let count: i64 = 12;

    // TODO get 404 handling
    if page.is_none() {
        let return_data = PeopleData {
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
        };
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/activity+json; charset=utf-8")
            .body(Body::from(serde_json::to_string(&return_data).unwrap()))
            .unwrap());
    }

    let user = match state
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
                .unwrap())
        }
    }; // Data { id: "ff928bab-96ea-485b-9d40-667e79a19dcc", username: "Mawoka", server: "local", display_name: "Mawoka", summary: "", public_key: "-----BEGIN PUBLIC KEY-----\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEA3OV1S2zNjE0OPICeA9pC\nm3pi5x6u9NLYyY51OSutpLpCFLEA50HjXKvqCaXVNtRXzmQmMg5lsrimm+/nJT3a\nrKuLhecXo6HrOV6GQ2+4n/kRRk75Uymk80upeAH5uI6CFBGB+1114JZp5MonuHQx\nt1um+DR3gtFkp8TLiJp5xk/L4/OQMBDfJROCKRw3OFFmEiWM9JlMxHOhekXkl9uc\nljVoese7xVaw+lD0R7sxdqLBHgjDDgf3A6dAQ/fTG+7DGUbMZvubvpQu7taCpevi\nLTtAi94R8RcLcg6/yAACXe2+gn2fGTeT2MncJgNuwTnZjNWmEfRNvX0cZ32qUc99\nfQIDAQAB\n-----END PUBLIC KEY-----\n", registered_at: 2024-02-18T00:00:00+00:00, following: [Data { server_id: "http://localhost:3000/api/v1/user/Mawoka" }] }

    let data = PeopleDataPage {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/following?page={}", user.server_id, page.unwrap()),
        next: format!("{}/following?page={}", user.server_id, page.unwrap() + 1),
        ordered_items: user
            .following
            .iter()
            .map(|server| server.server_id.to_string())
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
