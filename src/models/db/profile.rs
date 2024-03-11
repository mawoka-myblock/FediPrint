use crate::schema::Profile;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde_derive::Serialize;
use uuid::Uuid;

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = Profile)]
pub struct CreateProfile {
    pub id: Uuid,
    pub username: String,
    pub server: String,
    pub server_id: String,
    pub display_name: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub public_key: String,
}
#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = Profile)]
pub struct ExtendedCreateProfile {
    pub id: Uuid,
    pub username: String,
    pub server: String,
    pub server_id: String,
    pub display_name: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub public_key: String,
    pub registered_at: NaiveDate,
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = Profile)]
pub struct FullProfile {
    pub id: Uuid,
    pub username: String,
    pub server: String,
    pub server_id: String,
    pub display_name: String,
    pub summary: String,
    pub inbox: String,
    pub outbox: String,
    pub public_key: String,
    pub registered_at: NaiveDate,
    pub updated_at: NaiveDateTime,
}
