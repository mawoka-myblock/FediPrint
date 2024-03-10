use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Queryable, Selectable, Insertable, Associations, Identifiable};
use serde_derive::Serialize;
use uuid::Uuid;
use crate::schema::Profile;

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
    pub public_key: String
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
    pub registered_at: NaiveDate
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
#[diesel(table_name = Profile)]
pub struct FullProfile<'a> {
    pub id: &'a Uuid,
    pub username: &'a str,
    pub server: &'a str,
    pub server_id: &'a str,
    pub display_name: &'a str,
    pub summary: &'a str,
    pub inbox: &'a str,
    pub outbox: &'a str,
    pub public_key: &'a str,
    pub registered_at: &'a NaiveDate,
    pub updated_at: &'a NaiveDateTime
}