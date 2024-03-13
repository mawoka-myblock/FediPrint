use crate::models::db::EventAudience;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, FromRow)]
pub struct CreateNote {
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Vec<String>,
    pub audience: EventAudience,
    pub in_reply_to_comment_id: Option<Uuid>,
    pub in_reply_to_note_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub comment_of_model_id: Option<Uuid>,
}

impl CreateNote {
    pub async fn create(self, pool: PgPool) -> Result<FullNote, Error> {
        sqlx::query_as!(FullNote,
            r#"INSERT INTO note (server_id, content, hashtags, audience, in_reply_to_comment_id, in_reply_to_note_id, actor_id,
                comment_of_model_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, created_at, updated_at, server_id, content, hashtags, audience AS "audience!: EventAudience", in_reply_to_comment_id, in_reply_to_note_id, actor_id, comment_of_model_id"#,
            self.server_id, self.content, &self.hashtags, self.audience as _, self.in_reply_to_comment_id, self.in_reply_to_note_id, self.actor_id, self.comment_of_model_id
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]

pub struct FullNote {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Vec<String>,
    pub audience: EventAudience,
    pub in_reply_to_comment_id: Option<Uuid>,
    pub in_reply_to_note_id: Option<Uuid>,
    pub actor_id: Uuid,
    pub comment_of_model_id: Option<Uuid>,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct UserFacingNote {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub server_id: Option<String>,
    pub content: String,
    pub hashtags: Vec<String>,
    pub audience: EventAudience,
    pub actor_id: Uuid,
}

impl UserFacingNote {
    pub async fn set_server_id(
        id: &Uuid,
        server_id: &str,
        pool: PgPool,
    ) -> Result<UserFacingNote, Error> {
        sqlx::query_as!(
            UserFacingNote,
            r#"UPDATE note SET server_id = $1 WHERE id = $2
                RETURNING id, created_at, updated_at, server_id, content, hashtags, audience AS "audience!: EventAudience", actor_id"#,
            server_id, id
        ).fetch_one(&pool).await
    }
}
