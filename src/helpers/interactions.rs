use crate::models::activitypub::Profile;
use crate::models::data::Webfinger;
use anyhow::Context;
use crate::prisma;
use crate::prisma::PrismaClient;

pub async fn create_remote_profile(username: String, domain: String, db: &PrismaClient) -> anyhow::Result<bool> {
    let webfinger_response = reqwest::get(format!(
        "https://{domain}/.well-known/webfinger?resource=acct:{username}@{domain}"
    ))
        .await?
        .json::<Webfinger>()
        .await?;
    let mut server_id = None;
    for link in webfinger_response.links {
        if link.rel != "self" {
            continue;
        }
        server_id = Some(link.href.with_context(|| "server_id is None")?);
    }
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("application/activity+json").unwrap(),
    );
    let ap_client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let ap_profile_response = ap_client
        .get(server_id.unwrap())
        .send()
        .await?
        .json::<Profile>()
        .await?;
    println!("{:?}", ap_profile_response);
    let profile = db.profile().create(
        ap_profile_response.preferred_username.clone(), domain, ap_profile_response.id, ap_profile_response.name, ap_profile_response.public_key.public_key_pem,
        vec![
            prisma::profile::registered_at::set(chrono::DateTime::parse_from_rfc3339(&*ap_profile_response.published)?),
            // prisma::profile::summary::set(ap_profile_response.summary)
        ],
    ).exec().await?;
    Ok(true)
}
