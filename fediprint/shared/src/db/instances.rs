use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use uuid::{uuid, Uuid};

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct CreateInstance {
    pub base_url: String,
    pub instance_name: Option<String>,
    pub user_count: Option<i32>,
    pub software: String,
    pub software_version: Option<String>,
}

impl CreateInstance {
    pub async fn create_and_return_full(self, pool: PgPool) -> Result<FullInstance, Error> {
        sqlx::query_as!(FullInstance,
            r#"INSERT INTO instances (base_url, instance_name, user_count, software, software_version)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, created_at, updated_at, base_url, instance_name, user_count, software, software_version
            "#, self.base_url, self.instance_name, self.user_count, self.software, self.software_version
        ).fetch_one(&pool).await
    }
    pub async fn create_local(self, pool: PgPool) -> Result<(), Error> {
        sqlx::query!(r#"INSERT INTO instances (id,base_url, instance_name, user_count, software, software_version)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
                SET base_url = $2,
                    instance_name = $3,
                    software = $5,
                    software_version = $6
            "#, uuid!("00000000-0000-0000-0000-000000000000"), self.base_url, self.instance_name, self.user_count, self.software, self.software_version
        ).execute(&pool).await?;
        Ok(())
    }
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct FullInstance {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub base_url: String,
    pub instance_name: Option<String>,
    pub user_count: Option<i32>,
    pub software: String,
    pub software_version: Option<String>,
}

impl FullInstance {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullInstance, Error> {
        sqlx::query_as!(FullInstance,
        r#"SELECT id, created_at, updated_at, base_url, instance_name, user_count, software, software_version
        FROM instances WHERE id = $1"#, id
    ).fetch_one(&pool).await
    }

    pub async fn get_by_base_url(base_url: &str, pool: PgPool) -> Result<FullInstance, Error> {
        sqlx::query_as!(FullInstance,
        r#"SELECT id, created_at, updated_at, base_url, instance_name, user_count, software, software_version
        FROM instances WHERE base_url = $1"#, base_url)
        .fetch_one(&pool).await
    }
}
