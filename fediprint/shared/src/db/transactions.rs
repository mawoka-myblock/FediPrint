use anyhow::bail;
use chrono::{DateTime, Utc};
use serde_derive::Serialize;
use sqlx::{Error, PgPool};
use tracing::debug;
use uuid::{uuid, Uuid};

#[derive(Serialize, Debug, PartialEq)]
pub struct CreateTransaction {
    pub model_id: Uuid,
    pub buyer_profile: Uuid,
    pub buyer_account: Uuid,
    pub seller_profile: Uuid,
    pub stripe_id: String,
}

impl CreateTransaction {
    pub async fn create(self, pool: PgPool) -> Result<FullTransaction, Error> {
        sqlx::query_as!(FullTransaction,
            r#"INSERT INTO transactions (model_id, buyer_profile, buyer_account, seller_profile, stripe_id)
            VALUES ($1, $2, $3, $4, $5) RETURNING id, created_at, model_id, buyer_profile, buyer_account, seller_profile, stripe_id, payment_success, completed
            "#, Some(self.model_id), self.buyer_profile, self.buyer_account, Some(self.seller_profile), self.stripe_id
        ).fetch_one(&pool).await
    }
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FullTransaction {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub model_id: Option<Uuid>,
    pub buyer_profile: Uuid,
    pub buyer_account: Uuid,
    pub seller_profile: Option<Uuid>,
    pub stripe_id: String,
    pub payment_success: Option<bool>,
    pub completed: bool,
}

impl FullTransaction {
    pub async fn mark_completed_true(stripe_id: &str, pool: PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"UPDATE transactions SET completed = true WHERE stripe_id = $1"#,
            stripe_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
    pub async fn mark_payment_success_true(stripe_id: &str, pool: PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"UPDATE transactions SET payment_success = true WHERE stripe_id = $1"#,
            stripe_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
    pub async fn mark_payment_success_false(stripe_id: &str, pool: PgPool) -> Result<(), Error> {
        sqlx::query!(
            r#"UPDATE transactions SET payment_success = false WHERE stripe_id = $1"#,
            stripe_id
        )
        .execute(&pool)
        .await?;
        Ok(())
    }
}
