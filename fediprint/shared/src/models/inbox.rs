use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InboxEvent {
    #[serde(rename = "@context")]
    pub context: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub to: Option<Vec<String>>,
    pub actor: String,
    pub object: Value,
}
