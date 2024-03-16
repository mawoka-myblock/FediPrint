use serde_derive::Deserialize;
use uuid::Uuid;
use crate::models::db::ModelLicense;

#[derive(Deserialize)]
pub struct CreateModel {
    pub title: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    pub images: Vec<Uuid>,
    pub files: Vec<Uuid>,
    pub license: ModelLicense
}
