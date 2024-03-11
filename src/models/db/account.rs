use diesel::{Queryable, Selectable, Insertable, Identifiable};
use serde_derive::Serialize;
use uuid::Uuid;
use crate::schema::Account;
use chrono::NaiveDateTime;



#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq)]
#[diesel(table_name = Account)]
pub struct CreateAccount<'a> {
    pub password: &'a str,
    pub email: &'a str,
    pub profile_id: &'a Uuid,
    pub private_key: &'a str,
}

#[derive(Serialize, Insertable, Queryable, Selectable, Debug, PartialEq, Identifiable)]
// #[diesel(belongs_to(FullProfile, foreign_key = profile_id))]
#[diesel(table_name = Account)]
pub struct FullAccount {
    pub id: Uuid,
    pub registered_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub password: String,
    pub email: String,
    pub verified: Option<String>,
    pub profile_id: Uuid,
    pub private_key: String,
}

