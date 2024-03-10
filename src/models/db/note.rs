use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::db::EventAudience;
use crate::schema::Note;

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = Note)]
pub struct CreateNote {
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
    pub audience: EventAudience,
    pub in_reply_to_comment_id: Option<Uuid>,
    pub in_reply_to_note_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub comment_of_model_id: Option<Uuid>,
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = Note)]
pub struct FullNote {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
    pub audience: EventAudience,
    pub in_reply_to_comment_id: Option<Uuid>,
    pub in_reply_to_note_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub comment_of_model_id: Option<Uuid>,
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable, Deserialize)]
#[diesel(table_name = Note)]
pub struct UserFacingNote {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Option<Vec<String>>,
    pub audience: EventAudience,
    pub actor_id: Uuid,
}