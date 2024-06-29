use percent_encoding::percent_decode_str;
use std::str::FromStr;
use url::Url;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub public_url: String,
    pub base_domain: String,
    pub s3_base_url: String,
    pub s3_region: String,
    pub s3_username: String,
    pub s3_password: String,
    pub s3_bucket_name: String,
    pub meilisearch_url: String,
    pub meilisearch_key: String,
    pub registration_disabled: bool,
    pub smtp: SmtpData,
    pub stripe: Option<StripeData>,
}

#[derive(Debug, Clone)]
pub struct SmtpData {
    pub server: String,
    pub username: String,
    pub password: String,
    pub port: i32,
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct StripeData {
    pub key: String,
    pub webhook_key: String,
    pub platform_fee_percent: i32,
}

impl Config {
    pub fn init() -> Config {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let public_url = std::env::var("PUBLIC_URL").expect("PUBLIC_URL must be set");
        let base_domain = std::env::var("BASE_DOMAIN").expect("BASE_DOMAIN must be set");
        let s3_base_url = std::env::var("S3_BASE_URL").expect("S3_BASE_URL must be set");
        let s3_region = std::env::var("S3_REGION").expect("S3_REGION must be set");
        let s3_username = std::env::var("S3_USERNAME").expect("S3_USERNAME must be set");
        let s3_password = std::env::var("S3_PASSWORD").expect("S3_PASSWORD must be set");
        let s3_bucket_name = std::env::var("S3_BUCKET_NAME").unwrap_or("fediprint".to_string());
        let registration_disabled =
            bool::from_str(&std::env::var("REGISTRATION_DISABLED").unwrap_or("false".to_string()))
                .expect("REGISTRATION_DISABLED no valid boolean");
        let meilisearch_url =
            std::env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
        let meilisearch_key =
            std::env::var("MEILISEARCH_KEY").expect("MEILISEARCH_KEY must be set");
        let smtp_uri =
            Url::parse(&std::env::var("SMTP_URI").expect("SMTP_URI must be set")).unwrap();
        let smtp = SmtpData {
            server: smtp_uri.host().unwrap().to_string(),
            port: i32::from(smtp_uri.port().unwrap()),
            username: percent_decode_str(smtp_uri.username())
                .decode_utf8_lossy()
                .to_string(),
            password: percent_decode_str(smtp_uri.password().unwrap())
                .decode_utf8_lossy()
                .to_string(),
            email: smtp_uri.path()[1..].to_string(),
        };
        let stripe_key = std::env::var("STRIPE__KEY").ok();
        let stripe_webhook_key = std::env::var("STRIPE__WEBHOOK_KEY").ok();
        let stripe_platform_fee_percent: Option<i32> =
            std::env::var("STRIPE__PLATFORM_FEE_PERCENT").ok().map(|d| {
                i32::from_str(&d).expect("STRIPE__PLATFORM_FEE_PERCENT not a valid number")
            });
        let mut stripe: Option<StripeData> = None;
        if stripe_key.is_some() {
            stripe = Some(StripeData {
                key: stripe_key.expect("STRIPE__KEY must be set when Stripe is enabled"),
                webhook_key: stripe_webhook_key
                    .expect("STRIPE__WEBHOOK_KEY must be set when Stripe is enabled"),
                platform_fee_percent: stripe_platform_fee_percent
                    .expect("STRIPE__PLATFORM_FEE_PERCENT must be set when Stripe is enabled, 0 is recommended"),
            })
        };
        Config {
            database_url,
            jwt_secret,
            public_url,
            base_domain,
            s3_base_url,
            s3_region,
            s3_username,
            s3_password,
            s3_bucket_name,
            meilisearch_url,
            meilisearch_key,
            registration_disabled,
            smtp,
            stripe,
        }
    }
}
