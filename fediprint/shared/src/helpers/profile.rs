#[doc(inline)]
use chrono::DateTime;
use reqwest::Error;
use serde_derive::Deserialize;
use serde_derive::Serialize;
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
        //! Fetches the Profile by URL and saves them in the db.
        //! This does not check if the user already exists!
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
        let follower_count = OrderedCollection::get_with_ap_headers(&ap_profile_response.followers)
            .await?
            .total_items;
        let following_count =
            OrderedCollection::get_with_ap_headers(&ap_profile_response.following)
                .await?
                .total_items;
        let message_count = OrderedCollection::get_with_ap_headers(&ap_profile_response.outbox)
            .await?
            .total_items;
        debug!("{:?}", ap_profile_response);
        Ok(ExtendedCreateProfile {
            id: Uuid::now_v7(),
            username: ap_profile_response.preferred_username.clone(),
            server_id: ap_profile_response.id,
            display_name: ap_profile_response.name,
            summary: "".to_string(),
            inbox: ap_profile_response.inbox,
            outbox: ap_profile_response.outbox,
            follower_count,
            following_count,
            message_count,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "@context")]
    pub context: String,
    pub first: String,
    pub id: String,
    pub total_items: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl OrderedCollection {
    async fn get_with_ap_headers(url: &str) -> Result<OrderedCollection, Error> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Accept",
            reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
        );
        let ap_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        ap_client
            .get(url)
            .send()
            .await?
            .json::<OrderedCollection>()
            .await
    }
}
