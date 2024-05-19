use crate::models::db::ModifiedScale;
use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use uuid::Uuid;

#[derive(Serialize, Debug, PartialEq)]
pub struct CreatePrinter {
    pub name: String,
    pub manufacturer: String,
    pub profile_id: Uuid,
    pub public: bool,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale,
}

impl CreatePrinter {
    pub async fn create(self, pool: PgPool) -> Result<FullPrinter, Error> {
        sqlx::query_as!(FullPrinter,r#"INSERT INTO printer (name, manufacturer, profile_id,public, slicer_config, slicer_config_public, description, modified_scale)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, created_at, updated_at, name, manufacturer, profile_id, public, slicer_config, slicer_config_public, description, modified_scale AS "modified_scale!: ModifiedScale""#,
            self.name, self.manufacturer, self.profile_id, self.public, self.slicer_config, self.slicer_config_public, self.description, self.modified_scale as _
        ).fetch_one(&pool).await
    }

    pub async fn update_by_id_and_profile_id(
        self,
        id: &Uuid,
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<FullPrinter, Error> {
        sqlx::query_as!(FullPrinter,r#"UPDATE printer SET name = $3, manufacturer= $4, profile_id = $5, public= $6, slicer_config = $7, slicer_config_public = $8, description = $9, modified_scale = $10
                WHERE profile_id = $1 AND id = $2
                RETURNING id, created_at, updated_at, name, manufacturer, profile_id, public, slicer_config, slicer_config_public, description, modified_scale AS "modified_scale!: ModifiedScale""#,
            profile_id, id,self.name, self.manufacturer, self.profile_id, self.public, self.slicer_config, self.slicer_config_public, self.description, self.modified_scale as _
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FullPrinter {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub manufacturer: String,
    pub profile_id: Uuid,
    pub public: bool,
    pub slicer_config: Option<String>,
    pub slicer_config_public: bool,
    pub description: Option<String>,
    pub modified_scale: ModifiedScale,
}

impl FullPrinter {
    pub async fn get_all_printer_by_profile(
        profile_id: &Uuid,
        pool: PgPool,
    ) -> Result<Vec<FullPrinter>, Error> {
        sqlx::query_as!(FullPrinter,r#"SELECT id, created_at, updated_at, name, manufacturer, profile_id, public, slicer_config, slicer_config_public, description, modified_scale AS "modified_scale!: ModifiedScale" FROM printer
            WHERE profile_id = $1"#,
            profile_id
        ).fetch_all(&pool).await
    }
}
