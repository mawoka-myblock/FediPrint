use crate::helpers::auth::UserState;
use crate::helpers::sign::sign_post_request_with_body;
use shared::helpers::config::Config;
use shared::models::activitypub::FollowRequest;

use shared::db::profile::FullProfile;
use uuid::Uuid;

pub async fn follow_user(
    to_follow: &FullProfile,
    claims: &UserState,
    env: &Config,
) -> anyhow::Result<()> {
    let data = FollowRequest {
        context: "https://www.w3.org/ns/activitystreams".to_string(),
        id: format!("{}/{}", env.public_url, Uuid::new_v4()),
        type_field: "Follow".to_string(),
        actor: claims.server_id.clone(),
        object: to_follow.server_id.clone().to_string(),
    };
    let json_data = serde_json::to_string(&data).unwrap();
    let data_signature = sign_post_request_with_body(
        &to_follow.inbox,
        json_data.as_ref(),
        claims.private_key.clone(),
        format!("{}#main-key", &to_follow.server_id),
    )?;
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
    );
    headers.insert(
        "Digest",
        reqwest::header::HeaderValue::from_str(&data_signature.1).unwrap(),
    );
    headers.insert(
        "Signature",
        reqwest::header::HeaderValue::from_str(&data_signature.1).unwrap(),
    );
    let ap_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    ap_client
        .post(to_follow.inbox.clone())
        .body(json_data)
        .send()
        .await?;
    Ok(())
}
