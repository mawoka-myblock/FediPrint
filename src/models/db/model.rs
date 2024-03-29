use crate::models::db::ModelLicense;
use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, FromRow, PgPool};
use std::collections::HashSet;
use uuid::Uuid;

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
        sqlx::query!(
            r#"UPDATE file SET file_for_model_id = $1 WHERE id = ANY($2);"#,
            &ret_data.id,
            &self.files
        )
        .execute(&pool)
        .await?;
        sqlx::query!(
            r#"UPDATE file SET image_for_model_id = $1 WHERE id = ANY($2);"#,
            &ret_data.id,
            &self.images
        )
        .execute(&pool)
        .await?;
        Ok(ret_data)
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
            WHERE published = true ORDER BY created_at DESC OFFSET $1 LIMIT $2
            "#,
            offset, limit
        ).fetch_all(&pool).await
    }
}

fn remove_duplicates_from_list_of_models(models: &mut Vec<FullModelWithRelationsIds>) {
    for model in models {
        remove_duplicates(model)
    }
}
fn remove_duplicates(model: &mut FullModelWithRelationsIds) {
    if let Some(files) = &model.files {
        if let Some(unique_files) = remove_duplicates_from_vec(files.clone()) {
            model.files = Some(unique_files);
        }
    }
    if let Some(images) = &model.images {
        if let Some(unique_images) = remove_duplicates_from_vec(images.clone()) {
            model.images = Some(unique_images);
        }
    }
}
// Helper function to remove duplicates from a vector
fn remove_duplicates_from_vec<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Option<Vec<T>> {
    let mut set: HashSet<_> = HashSet::new();
    let mut unique_vec = Vec::new();
    for item in vec {
        if set.insert(item.clone()) {
            unique_vec.push(item);
        }
    }
    if unique_vec.is_empty() {
        None
    } else {
        Some(unique_vec)
    }
}

#[derive(Serialize, Debug, PartialEq, FromRow)]
pub struct FullModelWithRelationsIds {
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
    pub files: Option<Vec<Uuid>>,
    pub images: Option<Vec<Uuid>>,
}

impl FullModelWithRelationsIds {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullModelWithRelationsIds, Error> {
        let model_query = sqlx::query_as!(
            FullModelWithRelationsIds,
            r#"
        SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,array_agg(f.id) AS files,array_agg(i.id) AS images
        FROM
            model AS m
        LEFT JOIN
            file AS f ON m.id = f.file_for_model_id
        LEFT JOIN
            file AS i ON m.id = i.image_for_model_id
        WHERE
            m.id = $1
        GROUP BY
            m.id;
        "#,
            id
        );

        let mut model_with_relations: FullModelWithRelationsIds =
            model_query.fetch_one(&pool).await?;
        remove_duplicates(&mut model_with_relations);
        Ok(model_with_relations)
    }
    pub async fn get_newest_published_models_paginated(
        limit: &i64,
        offset: &i64,
        pool: PgPool,
    ) -> Result<Vec<FullModelWithRelationsIds>, Error> {
        let mut res = sqlx::query_as!(FullModelWithRelationsIds, r#"SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,array_agg(f.id) AS files,array_agg(i.id) AS images
        FROM
            model AS m
        LEFT JOIN
            file AS f ON m.id = f.file_for_model_id
        LEFT JOIN
            file AS i ON m.id = i.image_for_model_id
        WHERE
            m.published = true
        GROUP BY
            m.id
        ORDER BY created_at DESC OFFSET $1 LIMIT $2;
            "#,
            offset, limit
        ).fetch_all(&pool).await?;
        remove_duplicates_from_list_of_models(&mut res);
        Ok(res)
    }
}
