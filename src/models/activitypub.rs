use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

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
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
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
    // pub content_map: ContentMap,
    pub attachment: Vec<Value>,
    pub tag: Vec<Value>,
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
    pub items: Vec<Value>,
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
