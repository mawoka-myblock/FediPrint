use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde_derive::Serialize;
use uuid::Uuid;
use crate::schema::File;


#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = File)]
pub struct CreateFile {
    pub id: Uuid,
    pub mime_type: String,
    pub size: i64,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
    pub preview_file_id: Option<Uuid>,
    pub profile_id: Uuid,
    pub file_for_model_id: Option<Uuid>,
    pub image_for_model_id: Option<Uuid>
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = File)]
pub struct FullFile {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub mime_type: String,
    pub size: i64,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
    pub preview_file_id: Option<Uuid>,
    pub to_be_deleted_at: Option<NaiveDateTime>,
    pub profile_id: Uuid,
    pub file_for_model_id: Option<Uuid>,
    pub image_for_model_id: Option<Uuid>
}