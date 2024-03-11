use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use serde_derive::Serialize;
use uuid::Uuid;
use crate::models::db::ModifiedScale;
use crate::schema::Printer;
#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, AsChangeset)]
#[diesel(table_name = Printer)]
pub struct CreatePrinter {
    pub name: String,
    pub manufacturer: String,
    pub profile_id: Uuid,
    pub public: bool,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = Printer)]
pub struct FullPrinter {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub manufacturer: String,
    pub profile_id: Uuid,
    pub public: bool,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale
}