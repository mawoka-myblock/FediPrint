use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::models::db::ModelLicense;

#[derive(Serialize, Debug, PartialEq)]
pub struct CreateModel {
    pub server: String,
    pub server_id: Option<String>,
    pub profile_id: Uuid,
    pub published: bool,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    pub license: ModelLicense
}

impl CreateModel {
    pub async fn create(self, pool: PgPool) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"INSERT INTO model (server, server_id, profile_id, published, title, summary, description, tags, license)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at"#,
            self.server, self.server_id, self.profile_id, self.published, self.title, self.summary, self.description, &self.tags, self.license as _
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FullModel {
    pub id: Uuid,
    pub server: String,
    pub server_id: Option<String>,
    pub profile_id: Uuid,
    pub published: bool,
    pub title: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<String>,
    pub license: ModelLicense,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FullModel {
    pub async fn update_server_id_and_return(
        id: &Uuid,
        server_id: &str,
        pool: PgPool,
    ) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"UPDATE model SET server_id = $1 WHERE id = $2
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at"#,
            server_id, id
        ).fetch_one(&pool).await
    }

    pub async fn get_models_of_profile(
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<Vec<FullModel>, Error> {
        sqlx::query_as!(FullModel, r#"SELECT id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at FROM model
            WHERE profile_id = $1"#,
            profile_id
        ).fetch_all(&pool).await
    }

    pub async fn change_visibility_with_id_and_profile_id(
        published: &bool,
        id: &Uuid,
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"UPDATE model SET published = $1 WHERE id = $2 AND profile_id = $3
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at"#,
            published, id, profile_id
        ).fetch_one(&pool).await
    }
    pub async fn get_newest_published_models_paginated(
        limit: &i64,
        offset: &i64,
        pool: PgPool,
    ) -> Result<Vec<FullModel>, Error> {
        sqlx::query_as!(FullModel, r#"SELECT id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at FROM model
            ORDER BY created_at DESC OFFSET $1 LIMIT $2
            "#,
            offset, limit
        ).fetch_all(&pool).await
    }
}
