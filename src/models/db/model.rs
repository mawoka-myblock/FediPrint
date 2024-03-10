use chrono::NaiveDateTime;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde_derive::Serialize;
use uuid::Uuid;
use crate::schema::Model;


#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = Model)]
pub struct CreateModel {
    pub server: String,
    pub server_id: Option<String>,
    pub profile_id: Uuid,
    pub published: bool,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = Model)]
pub struct FullModel {
    pub id: Uuid,
    pub server: String,
    pub server_id: Option<String>,
    pub profile_id: Uuid,
    pub published: bool,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub tags: Option<Vec<String>>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}