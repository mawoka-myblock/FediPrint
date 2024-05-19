use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct CreateAccount<'a> {
    pub password: &'a str,
    pub email: &'a str,
    pub profile_id: &'a Uuid,
    pub private_key: &'a str,
}

impl CreateAccount<'_> {
    pub async fn create(self, pool: PgPool) -> Result<FullAccount, Error> {
        sqlx::query_as!(FullAccount,
            "INSERT INTO account (password, email, profile_id, private_key) VALUES ($1, $2, $3, $4) RETURNING *",
            self.password, self.email, self.profile_id, self.private_key
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FullAccount {
    pub id: Uuid,
    pub registered_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub password: String,
    pub email: String,
    pub verified: Option<String>,
    pub profile_id: Uuid,
    pub private_key: String,
}

impl FullAccount {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullAccount, Error> {
        sqlx::query_as!(FullAccount,
            r#"SELECT id, registered_at, updated_at, password, email, verified, profile_id, private_key FROM account where id = $1"#,
            id).fetch_one(&pool).await
    }
    pub async fn get_by_email(email: &str, pool: PgPool) -> Result<FullAccount, Error> {
        sqlx::query_as!(FullAccount,
            "SELECT id, registered_at, updated_at, password, email, verified, profile_id, private_key FROM account where email = $1",
            email).fetch_one(&pool).await
    }
}
