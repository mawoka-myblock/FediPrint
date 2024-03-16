use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct CreateFile {
    pub id: Uuid,
    pub mime_type: String,
    pub size: i64,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
    pub preview_file_id: Option<Uuid>,
    pub profile_id: Uuid,
    pub file_for_model_id: Option<Uuid>,
    pub image_for_model_id: Option<Uuid>,
}

impl CreateFile {
    pub async fn create(self, pool: PgPool) -> Result<FullFile, Error> {
        sqlx::query_as!(FullFile, r#"INSERT INTO file (id, mime_type, size, file_name, description, alt_text, thumbhash, preview_file_id, profile_id,
                        file_for_model_id, image_for_model_id)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                        RETURNING id, created_at, updated_at, mime_type, size, file_name, description, alt_text, thumbhash, preview_file_id, to_be_deleted_at, profile_id, file_for_model_id, image_for_model_id"#,
            self.id, self.mime_type, self.size, self.file_name, self.description, self.alt_text, self.thumbhash, self.preview_file_id, self.profile_id,
            self.file_for_model_id, self.image_for_model_id
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct UpdateFile {
    pub id: Uuid,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
}

impl UpdateFile {
    pub async fn update_by_profile_and_return(
        self,
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<FullFile, Error> {
        sqlx::query_as!(FullFile, r#"UPDATE file SET file_name = $3, description = $4, alt_text = $5, thumbhash = $6
                            WHERE id = $1 AND profile_id = $2
                            RETURNING id, created_at, updated_at, mime_type, size, file_name, description, alt_text, thumbhash, preview_file_id, to_be_deleted_at, profile_id, file_for_model_id, image_for_model_id"#,
            self.id, profile_id, self.file_name, self.description, self.alt_text, self.thumbhash
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq, FromRow, sqlx::Decode)]
pub struct FullFile {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub mime_type: String,
    pub size: i64,
    pub file_name: Option<String>,
    pub description: Option<String>,
    pub alt_text: Option<String>,
    pub thumbhash: Option<String>,
    pub preview_file_id: Option<Uuid>,
    pub to_be_deleted_at: Option<DateTime<Utc>>,
    pub profile_id: Uuid,
    pub file_for_model_id: Option<Uuid>,
    pub image_for_model_id: Option<Uuid>,
}

impl FullFile {
    pub async fn get_newest_files_by_profile_paginated(
        profile_id: &Uuid,
        limit: &i64,
        offset: &i64,
        pool: PgPool,
    ) -> Result<Vec<FullFile>, Error> {
        sqlx::query_as!(FullFile, r#"SELECT id, created_at, updated_at, mime_type, size, file_name, description, alt_text, thumbhash, preview_file_id, to_be_deleted_at, profile_id, file_for_model_id, image_for_model_id FROM file
        WHERE profile_id = $1
        ORDER BY created_at DESC OFFSET $2 LIMIT $3;"#,
            profile_id, offset, limit
        ).fetch_all(&pool).await
    }

    pub async fn get_by_id_and_profile_id(id: &Uuid, profile_id: &Uuid, pool: PgPool) -> Result<FullFile, Error> {
        sqlx::query_as!(FullFile, r#"SELECT id, created_at, updated_at, mime_type, size, file_name, description, alt_text, thumbhash, preview_file_id, to_be_deleted_at, profile_id, file_for_model_id, image_for_model_id FROM file
        WHERE id = $1 AND profile_id = $2;"#,
            id, profile_id
        ).fetch_one(&pool).await
    }

    pub async fn delete(self, pool: PgPool) -> Result<(), Error> {
        let _ = sqlx::query!(r#"DELETE from file WHERE id = $1"#,
        self.id).execute(&pool).await?;
        Ok(())
    }

    pub async fn delete_by_id_and_profile_id(id: &Uuid, profile_id: &Uuid, pool: PgPool) -> Result<(), Error> {
        let _ = sqlx::query!(r#"DELETE from file WHERE id = $1 AND profile_id = $2"#,
        id, profile_id).execute(&pool).await?;
        Ok(())
    }
}
