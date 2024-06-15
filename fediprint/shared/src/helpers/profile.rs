use chrono::DateTime;
use sqlx::PgPool;
use tracing::debug;
use uuid::Uuid;

use crate::{
    db::profile::{ExtendedCreateProfile, FullProfile},
    models::activitypub::Profile,
};

impl FullProfile {
    pub async fn get_from_activitypub(
        url: &str,
        instance_id: Uuid,
        pool: PgPool,
    ) -> anyhow::Result<FullProfile> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
        );
        let ap_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let ap_profile_response = ap_client.get(url).send().await?.json::<Profile>().await?;
        debug!("{:?}", ap_profile_response);
        Ok(ExtendedCreateProfile {
            id: Uuid::now_v7(),
            username: ap_profile_response.preferred_username.clone(),
            server_id: ap_profile_response.id,
            display_name: ap_profile_response.name,
            summary: "".to_string(),
            inbox: ap_profile_response.inbox,
            outbox: ap_profile_response.outbox,
            public_key: ap_profile_response.public_key.public_key_pem,
            registered_at: DateTime::from(chrono::DateTime::parse_from_rfc3339(
                &ap_profile_response.published,
            )?),
            instance: instance_id,
        }
        .create(pool.clone())
        .await?)
    }
}
