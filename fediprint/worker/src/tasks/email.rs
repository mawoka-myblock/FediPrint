use crate::types::FullJob;
use askama::Template;
use shared::helpers::config::Config;
use sqlx::PgPool;

#[derive(Template)]
#[template(path = "email/register.html")]
struct RegisterTemplate<'a> {
    username: &'a str,
    verify_link: &'a str,
}

pub async fn send_register_email(
    job: FullJob,
    cfg: &Config,
    pool: PgPool,
) -> Result<String, String> {
    Ok("".to_string())
}
