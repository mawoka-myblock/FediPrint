use crate::types::{FullJob, JobResponseFailure};
use askama::Template;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use shared::{
    db::{account::FullAccount, profile::FullProfile},
    helpers::config::{Config, SmtpData},
};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "email/register.html")]
struct RegisterTemplate<'a> {
    username: &'a str,
    verify_link: &'a str,
}

async fn send_email<'a>(
    subject: &str,
    to: &str,
    body: &str,
    smtp: &SmtpData,
) -> Result<(), JobResponseFailure> {
    let email = Message::builder()
        .from(
            format!("FediPrint <{}>", smtp.email)
                .parse()
                .map_err(|_| JobResponseFailure::never_try("Invalid from email"))?,
        )
        .to(to
            .parse()
            .map_err(|_| JobResponseFailure::never_try("Invalid to email"))?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())
        .map_err(|e| JobResponseFailure::never_try(&format!("Invalid email: {}", e)))?;
    let creds = Credentials::new(smtp.email.clone(), smtp.password.clone());
    let mailer = SmtpTransport::relay(&smtp.server)
        .map_err(|e| {
            JobResponseFailure::try_in_30(&format!("Failed to contact smtp server: {:?}", e))
        })?
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(JobResponseFailure::try_in_30(&format!(
            "Failed to send email: {}",
            e
        ))),
    }
}

pub async fn send_register_email(
    job: FullJob,
    cfg: &Config,
    pool: PgPool,
) -> Result<String, JobResponseFailure> {
    let user = FullAccount::get_by_id(
        &Uuid::parse_str(&job.input_data.unwrap()).unwrap(),
        pool.clone(),
    )
    .await
    .unwrap();

    let profile = FullProfile::get_by_id(&user.profile_id, pool)
        .await
        .unwrap();
    let link = format!("{}/verify?token={}", cfg.public_url, "PLACEHOLDER");
    let template = RegisterTemplate {
        username: &profile.username,
        verify_link: &link,
    };
    let html = template.render().map_err(|e| {
        JobResponseFailure::try_in_30(&format!("Failed to render email template: {e}"))
    })?;
    send_email("Verify your email", &user.email, &html, &cfg.smtp).await?;
    Ok("".into())
}
