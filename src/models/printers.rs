use database::::db::ModifiedScale;
use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Clone)]
pub struct CreatePrinter {
    pub name: String,
    pub manufacturer: String,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale,
    pub public: bool,
}

#[derive(Deserialize, Clone)]
pub struct UpdatePrinter {
    pub id: Uuid,
    pub name: String,
    pub manufacturer: String,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale,
    pub public: bool,
}
