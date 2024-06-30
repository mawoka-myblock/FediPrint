use crate::{
    db::ModelLicense, helpers::media::handle_media, models::activitypub::note::NoteResponse,
    AppState,
};
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tracing::trace;
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq, Deserialize)]
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
    pub cost: i16,
    pub currency: stripe::Currency,
}

impl CreateModel {
    pub async fn create(self, pool: PgPool) -> Result<FullModel, Error> {
        let ret_data = sqlx::query_as!(FullModel, r#"INSERT INTO model (server, server_id, profile_id, published, title, summary, description, tags, license, cost, currency)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency"#,
            self.server, self.server_id, self.profile_id, self.published, self.title, self.summary, self.description, &self.tags, self.license as _, self.cost, self.currency.to_string()
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
    pub printables_url: Option<String>,
    pub cost: Option<i16>,
    pub currency: Option<String>,
}

impl FullModel {
    pub async fn update_server_id_and_return(
        id: &Uuid,
        server_id: &str,
        pool: PgPool,
    ) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"UPDATE model SET server_id = $1 WHERE id = $2
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency"#,
            server_id, id
        ).fetch_one(&pool).await
    }

    pub async fn change_visibility_with_id_and_profile_id(
        published: &bool,
        id: &Uuid,
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"UPDATE model SET published = $1 WHERE id = $2 AND profile_id = $3
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency"#,
            published, id, profile_id
        ).fetch_one(&pool).await
    }
    pub async fn get_newest_published_models_paginated(
        limit: &i64,
        offset: &i64,
        pool: PgPool,
    ) -> Result<Vec<FullModel>, Error> {
        sqlx::query_as!(FullModel, r#"SELECT id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency FROM model
            WHERE published = true ORDER BY created_at DESC OFFSET $1 LIMIT $2
            "#,
            offset, limit
        ).fetch_all(&pool).await
    }
    pub async fn create(self, pool: PgPool) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"INSERT INTO model (id, server, server_id, profile_id, published, title, summary, description, tags, license, created_at, updated_at, printables_url, cost, currency)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency"#,
            self.id, self.server, self.server_id, self.profile_id, self.published, self.title, self.summary, self.description, &self.tags, self.license as _, self.created_at, self.updated_at, self.printables_url, self.cost, self.currency
        ).fetch_one(&pool).await
    }
    pub async fn get_by_id_and_public_and_paid(
        id: &Uuid,
        pool: PgPool,
    ) -> Result<FullModel, Error> {
        sqlx::query_as!(FullModel, r#"SELECT id, server, server_id, profile_id, published, title, summary, description, tags, license AS "license!: ModelLicense", created_at, updated_at, printables_url, cost, currency FROM model
            WHERE id = $1 AND cost IS NOT NULL AND currency IS NOT NULL
            "#,id
        ).fetch_one(&pool).await
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
    pub cost: Option<i16>,
    pub currency: Option<String>,
}

impl FullModelWithRelationsIds {
    pub async fn get_by_id(
        id: &Uuid,
        include_files_if_paid: bool,
        pool: PgPool,
    ) -> Result<FullModelWithRelationsIds, Error> {
        let model_query = sqlx::query_as!(
            FullModelWithRelationsIds,
            r#"
        SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,m.cost,m.currency,
        CASE WHEN (m.cost = 0 OR m.cost IS NULL OR $2) THEN array_agg(f.id) ELSE '{}'::uuid[] END AS files,
        array_agg(i.id) AS images
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
            id,
            include_files_if_paid
        );

        let mut model_with_relations: FullModelWithRelationsIds =
            model_query.fetch_one(&pool).await?;
        remove_duplicates(&mut model_with_relations);
        Ok(model_with_relations)
    }
    pub async fn get_newest_published_models_paginated(
        limit: &i64,
        offset: &i64,
        include_files_if_paid: bool,
        pool: PgPool,
    ) -> Result<Vec<FullModelWithRelationsIds>, Error> {
        let mut res = sqlx::query_as!(FullModelWithRelationsIds, r#"SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,m.cost,m.currency,
        CASE WHEN (m.cost = 0 OR m.cost IS NULL OR $3) THEN array_agg(f.id) ELSE '{}'::uuid[] END AS files,
        array_agg(i.id) AS images
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
            offset, limit, include_files_if_paid
        ).fetch_all(&pool).await?;
        remove_duplicates_from_list_of_models(&mut res);
        Ok(res)
    }

    pub async fn get_models_of_profile(
        profile_id: &Uuid,
        limit: &i64,
        offset: &i64,
        include_files_if_paid: bool,
        pool: PgPool,
    ) -> Result<Vec<FullModelWithRelationsIds>, Error> {
        sqlx::query_as!(FullModelWithRelationsIds, r#"SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,m.cost,m.currency,
        CASE WHEN (m.cost = 0 OR m.cost IS NULL OR $4) THEN array_agg(f.id) ELSE '{}'::uuid[] END AS files,
        array_agg(i.id) AS images
        FROM
            model AS m
        LEFT JOIN
            file AS f ON m.id = f.file_for_model_id
        LEFT JOIN
            file AS i ON m.id = i.image_for_model_id
        WHERE
            m.profile_id = $3
        GROUP BY
            m.id
        ORDER BY created_at DESC OFFSET $1 LIMIT $2;
            "#,
            offset, limit, profile_id, include_files_if_paid
        ).fetch_all(&pool).await
    }
    pub async fn change_visibility_with_id_and_profile_id(
        published: &bool,
        id: &Uuid,
        profile_id: &Uuid,
        include_files_if_paid: bool,
        pool: PgPool,
    ) -> Result<FullModelWithRelationsIds, Error> {
        sqlx::query_as!(FullModelWithRelationsIds, r#"WITH updated_model AS (
            UPDATE model
            SET published = $1
            WHERE id = $2 AND profile_id = $3
            RETURNING id, server, server_id, profile_id, published, title, summary, description, tags, license, created_at, updated_at, cost, currency
        )
        SELECT
            m.id,
            m.server,
            m.server_id,
            m.profile_id,
            m.published,
            m.title,
            m.summary,
            m.description,
            m.tags,
            m.license AS "license!: ModelLicense",
            m.created_at,
            m.updated_at,
            m.currency,
            m.cost,
            CASE WHEN (m.cost = 0 OR m.cost IS NULL OR $4) THEN array_agg(f.id) ELSE '{}'::uuid[] END AS files,
            array_agg(DISTINCT i.id) AS images
        FROM
            updated_model AS m
        LEFT JOIN
            file AS f ON m.id = f.file_for_model_id
        LEFT JOIN
            file AS i ON m.id = i.image_for_model_id
        GROUP BY
            m.id,
            m.server,
            m.server_id,
            m.profile_id,
            m.published,
            m.title,
            m.summary,
            m.description,
            m.tags,
            m.license,
            m.created_at,
            m.currency,
            m.cost,
            m.updated_at;"#r,
            published, id, profile_id, include_files_if_paid
        ).fetch_one(&pool).await
    }

    pub async fn get_by_server_id(
        server_id: &str,
        include_files_if_paid: bool,
        pool: PgPool,
    ) -> Result<FullModelWithRelationsIds, Error> {
        trace!(server_id = %server_id);
        let mut model = sqlx::query_as!(
            FullModelWithRelationsIds,
            r#"
        SELECT m.id,m.server,m.server_id,m.profile_id,m.published,m.title,m.summary,m.description,m.tags,m.license AS "license!: ModelLicense",m.created_at,m.updated_at,m.cost,m.currency,
        CASE WHEN (m.cost = 0 OR m.cost IS NULL OR $2) THEN array_agg(f.id) ELSE '{}'::uuid[] END AS files,
        array_agg(i.id) AS images
        FROM
            model AS m
        LEFT JOIN
            file AS f ON m.id = f.file_for_model_id
        LEFT JOIN
            file AS i ON m.id = i.image_for_model_id
        WHERE
            m.server_id = $1
        GROUP BY
            m.id;
        "#,
            server_id, include_files_if_paid
        ).fetch_one(&pool).await?;
        remove_duplicates(&mut model);
        Ok(model)
    }

    pub async fn create_from_note_response(
        d: NoteResponse,
        server: String,
        profile_id: Uuid,
        pool: PgPool,
    ) -> Result<FullModelWithRelationsIds, Error> {
        // TODO get rid of unwrap
        let date: DateTime<Utc> = DateTime::parse_from_rfc3339(&d.published).unwrap().into();
        let id = Uuid::now_v7();
        trace!("{:?}", &d);
        let model = FullModel {
            description: d.content,
            server_id: Some(d.id),
            created_at: date,
            summary: d.summary.unwrap(),
            id,
            server,
            license: ModelLicense::from_str(&d.license.unwrap()).unwrap(),
            profile_id,
            printables_url: None,
            published: true,
            tags: d.tag.into_iter().map(|v| v.name).collect(),
            title: d.name.unwrap(),
            updated_at: date,
            cost: None,
            currency: None,
        };
        model.create(pool.clone()).await?;
        FullModelWithRelationsIds::get_by_id(&id, false, pool).await
    }
    pub async fn create_or_get_from_note_response(
        d: NoteResponse,
        server: String,
        profile_id: Uuid,
        state: Arc<AppState>,
    ) -> anyhow::Result<FullModelWithRelationsIds> {
        trace!(
            "db res: {:?}",
            FullModelWithRelationsIds::get_by_server_id(&d.id, false, state.pool.clone()).await
        );
        // if let Ok(d) = FullModelWithRelationsIds::get_by_server_id(&d.id, pool.clone()).await {
        //     return Ok(d);
        // }
        let unfinished_model = FullModelWithRelationsIds::create_from_note_response(
            d.clone(),
            server,
            profile_id,
            state.pool.clone(),
        )
        .await?;
        handle_media(
            d.attachment.iter().filter_map(|v| v.as_str()).collect(),
            unfinished_model.id,
            profile_id,
            state.clone(),
        )
        .await?;
        Ok(
            FullModelWithRelationsIds::get_by_id(&unfinished_model.id, false, state.pool.clone())
                .await?,
        )
    }
}
