use crate::{db::EventAudience, models::activitypub::note::NoteResponse};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use tracing::trace;
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

impl FullNote {
    pub async fn create(self, pool: PgPool) -> Result<FullNote, Error> {
        sqlx::query_as!(FullNote, r#"INSERT INTO note (id, created_at, updated_at, server_id, content, hashtags, audience, in_reply_to_comment_id, in_reply_to_note_id, actor_id,
                comment_of_model_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id, created_at, updated_at, server_id, content, hashtags, audience AS "audience!: EventAudience", in_reply_to_comment_id, in_reply_to_note_id, actor_id, comment_of_model_id"#,
                self.id, self.created_at, self.updated_at, self.server_id, self.content, &self.hashtags, self.audience as _, self.in_reply_to_comment_id, self.in_reply_to_note_id, self.actor_id, self.comment_of_model_id
        ).fetch_one(&pool).await
    }

    pub async fn get_by_server_id(server_id: &str, pool: PgPool) -> Result<FullNote, Error> {
        sqlx::query_as!(FullNote, r#"SELECT id, created_at, updated_at, server_id, content, hashtags, audience AS "audience!: EventAudience", in_reply_to_comment_id, in_reply_to_note_id, actor_id, comment_of_model_id
                FROM note WHERE server_id = $1 LIMIT 1"#,
                Some(server_id)
        ).fetch_one(&pool).await
    }

    pub async fn create_from_note_response(
        d: NoteResponse,
        profile_id: Uuid,
        pool: PgPool,
    ) -> Result<FullNote, Error> {
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339(&d.published).unwrap().into();
        let note = FullNote {
            id: Uuid::now_v7(),
            created_at: date,
            updated_at: date,
            server_id: Some(d.id),
            content: d.content,
            hashtags: d.tag.into_iter().map(|v| v.name).collect(),
            audience: EventAudience::Public,
            in_reply_to_comment_id: None,
            in_reply_to_note_id: None,
            actor_id: profile_id,
            comment_of_model_id: None,
        };
        trace!("note: {:?}", &note);
        note.create(pool).await
    }

    pub async fn create_or_get_from_note_response(
        d: NoteResponse,
        profile_id: Uuid,
        pool: PgPool,
    ) -> Result<FullNote, Error> {
        if let Ok(d) = FullNote::get_by_server_id(&d.id, pool.clone()).await {
            return Ok(d);
        }
        FullNote::create_from_note_response(d, profile_id, pool).await
    }
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

pub struct BoxNoteAttachment {
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
    pub mentions: Option<Vec<String>>,
    pub comment_r_server_id: Option<String>,
    pub comment_n_server_id: Option<String>,
}

impl BoxNoteAttachment {
    pub async fn get_by_profile_id(
        id: &Uuid,
        pool: PgPool,
    ) -> Result<Vec<BoxNoteAttachment>, Error> {
        sqlx::query_as!(
            BoxNoteAttachment,
            r#"
SELECT n.id,
       n.created_at,
       n.updated_at,
       n.server_id,
       n.content,
       n.hashtags,
       n.audience AS "audience!: EventAudience",
       n.in_reply_to_comment_id,
       n.in_reply_to_note_id,
       n.actor_id,
       n.comment_of_model_id,
       array_agg(m.server_id) AS mentions,
       comment_r.server_id AS comment_r_server_id,
       comment_n.server_id AS comment_n_server_id
FROM note AS n
         JOIN _mentions _m ON _m.note_id = id
         JOIN profile m ON m.id = _m.profile_id
         JOIN note comment_r ON comment_r.id = n.in_reply_to_comment_id
         JOIN note comment_n ON comment_n.id = n.in_reply_to_note_id
WHERE n.actor_id = $1
GROUP BY n.id, comment_r.server_id, comment_n.server_id;
        "#,
            id
        )
        .fetch_all(&pool)
        .await
    }
}
