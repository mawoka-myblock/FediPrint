use crate::db::file::FullFile;
use crate::db::profile::UsernameAndServerId;
use crate::db::ModelLicense;
use chrono::{DateTime, Utc};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::{json, Value};
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[serde(rename = "@context")]
    pub context: (String, String, Context),
    // pub attachment: Vec<Attachment>,
    // pub devices: String,
    // pub discoverable: bool,
    pub endpoints: Endpoints,
    // pub featured: String,
    // pub featured_tags: String,
    pub followers: String,
    pub following: String,
    // pub icon: Icon,
    pub id: String,
    pub inbox: String,
    // pub indexable: bool,
    // pub manually_approves_followers: bool,
    // pub memorial: bool,
    pub name: String,
    pub outbox: String,
    pub preferred_username: String,
    pub public_key: PublicKey,
    pub published: String,
    // creation of account (Date)
    // pub summary: String,
    // pub tag: Vec<Tag>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    #[serde(rename = "Curve25519Key")]
    pub curve25519key: Option<String>,
    // #[serde(rename = "Device")]
    // pub device: String,
    #[serde(rename = "Ed25519Key")]
    pub ed25519key: Option<String>,
    #[serde(rename = "Ed25519Signature")]
    pub ed25519signature: Option<String>,
    #[serde(rename = "EncryptedMessage")]
    pub encrypted_message: Option<String>,
    #[serde(rename = "Hashtag")]
    pub hashtag: Option<String>,
    #[serde(rename = "PropertyValue")]
    pub property_value: Option<String>,
    pub also_known_as: AlsoKnownAs,
    pub cipher_text: Option<String>,
    pub claim: Option<Claim>,
    // pub device_id: String,
    // pub devices: Devices,
    // pub discoverable: String,
    // pub featured: Featured,
    // pub featured_tags: FeaturedTags,
    pub fingerprint_key: Option<FingerprintKey>,
    // pub focal_point: FocalPoint,
    pub identity_key: Option<IdentityKey>,
    // pub indexable: String,
    // pub manually_approves_followers: String,
    // pub memorial: String,
    pub message_franking: Option<String>,
    pub message_type: Option<String>,
    // pub moved_to: MovedTo,
    pub public_key_base64: Option<String>,
    pub schema: Option<String>,
    // pub suspended: String,
    pub toot: String,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlsoKnownAs {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Featured {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeaturedTags {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FingerprintKey {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FocalPoint {
    #[serde(rename = "@container")]
    pub container: String,
    #[serde(rename = "@id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentityKey {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovedTo {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub blurhash: Option<String>,
    pub height: Option<i64>,
    pub width: Option<i64>,
    pub media_type: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoints {
    pub shared_inbox: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icon {
    pub media_type: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    pub id: String,
    pub owner: String,
    pub public_key_pem: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub href: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

impl Tag {
    pub fn from_str(tag: &str, base_url: &str) -> Tag {
        Tag {
            name: format!("#{}", tag),
            href: format!("{}/api/v1/tags/{}", base_url, tag),
            type_field: String::from("Hashtag"),
        }
    }
    pub fn from_strs(tags: Vec<String>, base_url: &str) -> Vec<Tag> {
        tags.iter().map(|h| Tag::from_str(h, base_url)).collect()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedCollection {
    #[serde(rename = "@context")]
    pub context: String,
    pub first: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<String>,
    pub total_items: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeopleDataPage {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    pub next: String,
    pub ordered_items: Vec<String>,
    pub part_of: String,
    pub total_items: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FollowRequest {
    #[serde(rename = "@context")]
    pub context: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub actor: String,
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutboxDataPage {
    #[serde(rename = "@context")]
    pub context: (String, OutboxContext),
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub next: String,
    pub prev: String,
    pub part_of: String,
    pub ordered_items: Vec<OrderedItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderedItem {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub actor: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub object: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBoxItemRoot {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub actor: String,
    pub published: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub object: NoteBoxItemObject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBoxItemObject {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    // pub summary: Value,
    pub in_reply_to: Value,
    pub published: String,
    pub url: String,
    pub attributed_to: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    // pub sensitive: bool,
    // pub atom_uri: String,
    // pub in_reply_to_atom_uri: Value,
    // pub conversation: String,
    pub content: String,
    pub summary: Option<String>,
    // pub content_map: ContentMap,
    pub attachment: Vec<Attachment>,
    pub tag: Vec<Tag>,
    pub updated: Option<String>,
    pub replies: NoteBoxItemReplies,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBoxItemReplies {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub first: NoteBoxItemFirst,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteBoxItemFirst {
    #[serde(rename = "type")]
    pub type_field: String,
    pub next: String,
    pub part_of: String,
    pub items: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutboxContext {
    pub ostatus: String,
    // pub atom_uri: String,
    // pub in_reply_to_atom_uri: String,
    pub conversation: String,
    // pub sensitive: String,
    pub toot: String,
    // pub voters_count: String,
    #[serde(rename = "Hashtag")]
    pub hashtag: String,
    pub blurhash: String,
    pub focal_point: FocalPoint,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoteJoinedModel {
    pub profile_server_id: String,
    pub profile_id: Uuid,
    pub note_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
    pub hashtags: Vec<String>,
    pub content: String,
    pub summary: Option<String>,
    pub server_id: Option<String>,
    pub license: Option<ModelLicense>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub first_reply_server_id: Option<String>,
    pub title: Option<String>
}

impl NoteJoinedModel {
    pub async fn get_by_profile_id(id: &Uuid, pool: PgPool) -> Result<Vec<NoteJoinedModel>, Error> {
        // Type overrides necessary, as sqlx wants everything to be Option<> in Rust which just isn't
        // true in this case. Let's see when I'll have to fix this query.
        // https://github.com/launchbadge/sqlx/issues/1266
        sqlx::query_as!(
            NoteJoinedModel,
            r#"
SELECT p.server_id  AS "profile_server_id!: String",
       p.id         AS "profile_id!: Uuid",
       n.id         AS note_id,
       NULL         AS model_id,
       n.hashtags   AS "hashtags!: Vec<String>",
       n.content    AS "content!: String",
       NULL         AS summary,
       n.server_id  AS server_id,
       NULL         AS "license!: Option<ModelLicense>",
       n.created_at AS "created_at!: DateTime<Utc>",
       n.updated_at AS "updated_at!: DateTime<Utc>",
       r.server_id  AS "first_reply_server_id!: Option<String>",
       NULL         AS "title"
FROM profile AS p
         LEFT JOIN note AS n ON p.id = n.actor_id
         LEFT JOIN note AS r ON n.id = r.in_reply_to_note_id
WHERE p.id = $1
  AND n.id IS NOT NULL
  AND n.audience = 'PUBLIC'

UNION ALL
SELECT p.server_id   AS "profile_server_id!: String",
       p.id          AS "profile_id!: Uuid",
       NULL          AS note_id,
       m.id          AS model_id,
       m.tags        AS "hashtags!: Vec<String>",
       m.description AS "content!: String",
       m.summary     AS summary,
       m.server_id   AS server_id,
       m.license     AS "license!: Option<ModelLicense>",
       m.created_at  AS "created_at!: DateTime<Utc>",
       m.updated_at  AS "updated_at!: DateTime<Utc>",
       r.server_id   AS "first_reply_server_id!: Option<String>",
       m.title          AS "title"
FROM profile AS p
         LEFT JOIN model AS m ON p.id = m.profile_id
         LEFT JOIN note AS r ON m.id = r.in_reply_to_model_id
         LEFT JOIN file AS f ON f.image_for_model_id = m.id
WHERE p.id = $1
  AND m.id IS NOT NULL
ORDER BY "created_at!: DateTime<Utc>"
LIMIT 15;
       "#,
            id
        )
        .fetch_all(&pool)
        .await
    }
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<NoteJoinedModel, Error> {
        // Type overrides necessary, as sqlx wants everything to be Option<> in Rust which just isn't
        // true in this case. Let's see when I'll have to fix this query.
        // https://github.com/launchbadge/sqlx/issues/1266
        sqlx::query_as!(
            NoteJoinedModel,
            r#"
SELECT p.server_id  AS "profile_server_id!: String",
       p.id         AS "profile_id!: Uuid",
       n.id         AS note_id,
       NULL         AS model_id,
       n.hashtags   AS "hashtags!: Vec<String>",
       n.content    AS "content!: String",
       NULL         AS summary,
       n.server_id  AS server_id,
       NULL         AS "license!: Option<ModelLicense>",
       n.created_at AS "created_at!: DateTime<Utc>",
       n.updated_at AS "updated_at!: DateTime<Utc>",
       r.server_id  AS "first_reply_server_id!: Option<String>",
       NULL         AS "title"
FROM profile AS p
         LEFT JOIN note AS n ON p.id = n.actor_id
         LEFT JOIN note AS r ON n.id = r.in_reply_to_note_id
WHERE n.id = $1

UNION ALL
SELECT p.server_id   AS "profile_server_id!: String",
       p.id          AS "profile_id!: Uuid",
       NULL          AS note_id,
       m.id          AS model_id,
       m.tags        AS "hashtags!: Vec<String>",
       m.description AS "content!: String",
       m.summary     AS summary,
       m.server_id   AS server_id,
       m.license     AS "license!: Option<ModelLicense>",
       m.created_at  AS "created_at!: DateTime<Utc>",
       m.updated_at  AS "updated_at!: DateTime<Utc>",
       r.server_id   AS "first_reply_server_id!: Option<String>",
       m.title          AS "title"
FROM profile AS p
         LEFT JOIN model AS m ON p.id = m.profile_id
         LEFT JOIN note AS r ON m.id = r.in_reply_to_model_id
         LEFT JOIN file AS f ON f.image_for_model_id = m.id
WHERE m.id = $1
ORDER BY "created_at!: DateTime<Utc>"
       "#,
            id
        )
        .fetch_one(&pool)
        .await
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityPubModel {
    #[serde(rename = "@context")]
    pub context: (String, Value),
    pub attachment: Vec<Attachment>,
    pub attributed_to: String,
    pub cc: Vec<String>,
    pub content: String,
    pub id: String,
    pub published: DateTime<Utc>,
    pub replies: Replies,
    pub sensitive: bool,
    pub summary: Option<String>,
    pub tag: Vec<Tag>,
    pub to: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub url: String,
    pub license: Option<ModelLicense>,
    pub name: Option<String>
}

impl ActivityPubModel {
    pub async fn get_by_id(
        id: &Uuid,
        pool: PgPool,
        public_url: String,
    ) -> Result<ActivityPubModel, Error> {
        let status = NoteJoinedModel::get_by_id(id, pool.clone()).await?;
        let id = status.note_id.unwrap_or(status.model_id.unwrap());
        let is_model = status.model_id.is_some();
        let mut attachments: Vec<Attachment> = vec![];
        if is_model {
            let files = FullFile::get_many_files_by_model(&id, pool.clone()).await?;
            attachments = files
                .iter()
                .map(|v| Attachment {
                    blurhash: v.thumbhash.clone(),
                    height: None,
                    width: None,
                    media_type: v.mime_type.clone(),
                    url: format!("{public_url}/api/v1/storage/download/{}", v.id),
                    name: v.description.clone().unwrap_or_default(),
                    type_field: "Document".to_string(),
                })
                .collect();
        }

        let user_data = UsernameAndServerId::get_by_id(&status.profile_id, pool.clone()).await?;
        let model_context: Value = json!({
            "Hashtag": "as:Hashtag",
            //"atomUri": "ostatus:atomUri",
            //"blurhash": "toot:blurhash",
            //"conversation": "ostatus:conversation",
            "inReplyToAtomUri": "ostatus:inReplyToAtomUri",
            "ostatus": "http://ostatus.org#",
            "sensitive": "as:sensitive",
            "toot": "http://joinmastodon.org/ns#",
            "3dModel": "https://3dmodel.mawoka.eu"
        });
        Ok(ActivityPubModel {
            context: (
                "https://www.w3.org/ns/activitystreams".to_string(),
                model_context,
            ),
            attachment: attachments,
            attributed_to: user_data.server_id,
            cc: vec![format!(
                "{}/api/v1/user/{}/followers",
                public_url, user_data.username
            )],
            content: status.content,
            id: status.server_id.unwrap(),
            published: status.created_at,
            replies: Replies {
                first: First {
                    items: vec![],
                    next: format!("{}/api/v1/model/{}/replies&page=true", public_url, &id),
                    part_of: format!("{}/api/v1/model/{}/replies", public_url, &id),
                    type_field: "CollectionPage".to_string(),
                },
                id: format!("{}/api/v1/model/{}/replies", public_url, &id),
                type_field: "Collection".to_string(),
            },
            sensitive: false,
            summary: status.summary,
            tag: status
                .hashtags
                .iter()
                .map(|t| Tag {
                    href: format!("{}/api/v1/tags/{}", public_url, t),
                    name: format!("#{}", t),
                    type_field: "Hashtag".to_string(),
                })
                .collect(),
            to: vec!["https://www.w3.org/ns/activitystreams#Public".to_string()],
            type_field: "Note".to_string(),
            url: format!("{}/api/v1/model/{}", public_url, &id),
            license: status.license,
            name: status.title
        })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Replies {
    pub first: First,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct First {
    pub items: Vec<Value>,
    pub next: String,
    pub part_of: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub mod note {
    use super::{Replies, Tag};
    use serde_derive::Deserialize;
    use serde_derive::Serialize;
    use serde_json::Value;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NoteResponse {
        #[serde(rename = "@context")]
        pub context: (String, std::collections::HashMap<String, Value>),
        pub atom_uri: Option<String>,
        pub attachment: Vec<Value>,
        pub attributed_to: String,
        pub cc: Vec<String>,
        pub content: String,
        pub content_map: Option<std::collections::HashMap<String, String>>,
        pub conversation: Option<String>,
        pub id: String,
        pub in_reply_to: Option<Value>,
        pub in_reply_to_atom_uri: Option<Value>,
        pub published: String,
        pub replies: Replies,
        pub sensitive: Option<bool>,
        pub summary: Option<String>,
        pub tag: Vec<Tag>,
        pub to: Vec<String>,
        #[serde(rename = "type")]
        pub type_field: String,
        pub updated: Option<String>,
        pub url: String,
        pub license: Option<String>,
        pub name: Option<String>, // that's the title
    }
}
