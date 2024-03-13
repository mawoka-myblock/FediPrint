use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
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

impl CreateProfile {
    pub async fn create(self, pool: PgPool) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile,
            "INSERT INTO profile (username, server, server_id, display_name, inbox, outbox, public_key) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            self.username, self.server, self.server_id, self.display_name, self.inbox, self.outbox, self.public_key
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]
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
    pub registered_at: DateTime<Utc>,
}

impl ExtendedCreateProfile {
    pub async fn create(self, pool: PgPool) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile,
            "INSERT INTO profile (username, server, server_id, display_name, inbox, outbox, public_key, registered_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *",
            self.username, self.server, self.server_id, self.display_name, self.inbox, self.outbox, self.public_key, self.registered_at
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq, sqlx::FromRow)]
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
    pub registered_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FullProfile {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile,
            r#"SELECT id, username, server, server_id, display_name, summary, inbox, outbox, public_key, registered_at, updated_at
            FROM profile WHERE id = $1"#,
            id).fetch_one(&pool).await
    }
    pub async fn get_by_username_and_server(
        username: &str,
        server: &str,
        pool: PgPool,
    ) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile, r#"SELECT id, username, server, server_id, display_name, summary, inbox, outbox, public_key, registered_at, updated_at
        FROM profile WHERE username = $1 and server = $2"#,
            username, server).fetch_one(&pool).await
    }
}
