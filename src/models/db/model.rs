use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;
use crate::models::db::file::FullFile;
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
    pub license: ModelLicense,
    pub files: Vec<Uuid>,
    pub images: Vec<Uuid>,
}

impl CreateModel {
    pub async fn create(self, pool: PgPool) -> Result<FullModel, Error> {
        let ret_data = sqlx::query_as!(FullModel, r#"INSERT INTO model (server, server_id, profile_id, published, title, summary, description, tags, license)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at"#,
            self.server, self.server_id, self.profile_id, self.published, self.title, self.summary, self.description, &self.tags, self.license as _
        ).fetch_one(&pool).await?;
        sqlx::query!(r#"UPDATE file SET file_for_model_id = $1 WHERE id = ANY($2);"#, &ret_data.id, &self.files).execute(&pool).await?;
        sqlx::query!(r#"UPDATE file SET image_for_model_id = $1 WHERE id = ANY($2);"#, &ret_data.id, &self.images).execute(&pool).await?;
        return Ok(ret_data);
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

#[derive(Serialize, Debug, PartialEq)]
pub struct FullModelWithRelations {
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
    pub files: Vec<FullFile>,
    pub images: Vec<FullFile>,
}

impl FullModelWithRelations {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullModelWithRelations, Error> {
        sqlx::query_as!(FullModelWithRelations,
            r#"SELECT (f.id,f.created_at,f.updated_at,f.mime_type,f.size,f.file_name,f.description,f.alt_text,f.thumbhash,f.preview_file_id,f.to_be_deleted_at,f.profile_id,f.file_for_model_id,f.image_for_model_id) AS "images!: Vec<FullFile>",
            (i.id,i.created_at,i.updated_at,i.mime_type,i.size,i.file_name,i.description,i.alt_text,i.thumbhash,i.preview_file_id,i.to_be_deleted_at,i.profile_id,i.file_for_model_id,i.image_for_model_id) AS "files!: Vec<FullFile>",
            m.id,server,server_id,published,title,summary,m.description,tags,license AS "license!: ModelLicense",m.created_at,m.updated_at, m.profile_id
                FROM file f
                JOIN model m ON f.file_for_model_id = m.id
                LEFT JOIN file i ON i.image_for_model_id = m.id
                WHERE m.id = $1"#,
            id
        ).fetch_one(&pool).await
    }
}