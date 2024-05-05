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

#[derive(Serialize, Debug, PartialEq, sqlx::FromRow, Clone)]
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
    pub linked_printables_profile: Option<String>,
}

impl FullProfile {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile,
            r#"SELECT id, username, server, server_id, display_name, summary, inbox, outbox, public_key, registered_at, updated_at, linked_printables_profile
            FROM profile WHERE id = $1"#,
            id).fetch_one(&pool).await
    }
    pub async fn get_by_username_and_server(
        username: &str,
        server: &str,
        pool: PgPool,
    ) -> Result<FullProfile, Error> {
        sqlx::query_as!(FullProfile, r#"SELECT id, username, server, server_id, display_name, summary, inbox, outbox, public_key, registered_at, updated_at, linked_printables_profile
        FROM profile WHERE username = $1 and server = $2"#,
            username, server).fetch_one(&pool).await
    }

    pub async fn link_printables_profile(
        self,
        printables_profile: &str,
        pool: PgPool,
    ) -> Result<FullProfile, Error> {
        sqlx::query!(
            r#"UPDATE profile SET linked_printables_profile = $1 WHERE id = $2;"#,
            printables_profile,
            self.id
        )
            .execute(&pool)
            .await?;
        let mut profile = self.clone();
        profile.linked_printables_profile = Some(printables_profile.to_string());
        Ok(profile)
    }
}

#[derive(Serialize, Debug, PartialEq, sqlx::FromRow)]
pub struct BarebonesProfile {
    pub id: Uuid,
    pub username: String,
    pub server: String,
    pub server_id: String,
    pub display_name: String,
    pub summary: String,
}

#[derive(Serialize, Debug, PartialEq, sqlx::FromRow)]
pub struct FullProfileWithFollower {
    pub profile: FullProfile,
    pub followers: Vec<BarebonesProfile>,
}

impl FullProfileWithFollower {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullProfileWithFollower, Error> {
        let profile = FullProfile::get_by_id(id, pool.clone()).await?;
        let followers = sqlx::query_as!(
            BarebonesProfile,
            r#"SELECT p.id, p.username, p.server, p.server_id, p.display_name, p.summary
                FROM followers f
                JOIN profile p ON p.id = f.follower_id
                WHERE f.profile_id = $1;"#,
            id
        )
            .fetch_all(&pool)
            .await?;
        Ok(FullProfileWithFollower { profile, followers })
    }

    pub async fn count_followers(id: &Uuid, pool: PgPool) -> Result<i64, Error> {
        let c: Option<i64> = sqlx::query_scalar!(
            r#"SELECT COUNT(p.id)
                FROM followers f
                JOIN profile p ON p.id = f.follower_id
                WHERE f.profile_id = $1;"#,
            id
        )
            .fetch_one(&pool)
            .await?;
        Ok(c.unwrap())
    }
}

#[derive(Serialize, Debug, PartialEq, sqlx::FromRow)]
pub struct FullProfileWithFollowing {
    pub profile: FullProfile,
    pub following: Vec<BarebonesProfile>,
}

impl FullProfileWithFollowing {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<FullProfileWithFollowing, Error> {
        let profile = FullProfile::get_by_id(id, pool.clone()).await?;
        let following = sqlx::query_as!(
            BarebonesProfile,
            r#"SELECT p.id, p.username, p.server, p.server_id, p.display_name, p.summary
                FROM followers f
                JOIN profile p ON p.id = f.follower_id
                WHERE f.follower_id = $1;"#,
            id
        )
            .fetch_all(&pool)
            .await?;
        Ok(FullProfileWithFollowing { profile, following })
    }

    pub async fn count_following(id: &Uuid, pool: PgPool) -> Result<i64, Error> {
        let c: Option<i64> = sqlx::query_scalar!(
            r#"SELECT COUNT(p.id)
                FROM followers f
                JOIN profile p ON p.id = f.follower_id
                WHERE f.follower_id = $1;"#,
            id
        )
            .fetch_one(&pool)
            .await?;
        Ok(c.unwrap())
    }
}

pub struct UsernameAndServerId {
    pub username: String,
    pub server_id: String,
}

impl UsernameAndServerId {
    pub async fn get_by_id(id: &Uuid, pool: PgPool) -> Result<UsernameAndServerId, Error> {
        sqlx::query_as!(UsernameAndServerId,r#"select server_id, username from profile where id= $1"#, id).fetch_one(&pool).await
    }
}