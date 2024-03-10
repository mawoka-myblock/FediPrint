use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateImageMetadata {
    pub id: Uuid,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
}
