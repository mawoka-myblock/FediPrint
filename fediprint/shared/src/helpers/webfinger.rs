use anyhow::{bail, Result};

use serde_derive::Deserialize;
use serde_derive::Serialize;
use tracing::error;
use tracing::trace;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebFingerResponse {
    pub subject: String,
    pub aliases: Vec<String>,
    pub links: Vec<WebFingerLink>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebFingerLink {
    pub rel: String,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub href: Option<String>,
    pub template: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructuredWebFingerResponse {
    pub profile_page: Option<String>,
    pub ap_page: Option<String>,
    pub subscrive_url: Option<String>,
    pub avatar: Option<String>,
}

impl WebFingerResponse {
    pub fn to_structured_response(self) -> StructuredWebFingerResponse {
        let mut response = StructuredWebFingerResponse::default();

        for link in self.links {
            match (
                &*link.rel,
                &**link.type_field.as_ref().unwrap_or(&"".to_string()),
                link.href.as_ref(),
            ) {
                ("http://webfinger.net/rel/profile-page", _, Some(href)) => {
                    response.profile_page = Some(href.clone());
                }
                ("self", "application/activity+json", Some(href)) => {
                    response.ap_page = Some(href.clone());
                }
                ("http://ostatus.org/schema/1.0/subscribe", _, Some(template)) => {
                    response.subscrive_url = Some(template.clone());
                }
                ("http://webfinger.net/rel/avatar", "image/png", Some(href)) => {
                    response.avatar = Some(href.clone());
                }
                _ => {}
            }
        }
        response
    }
}

pub async fn get_webfinger_details(
    base_url: &str,
    handle: &str,
) -> Result<Option<WebFingerResponse>> {
    let webfinger_url = format!("{base_url}/.well-known/webfinger?resource=acct:{handle}");
    trace!("WebFinger URL: {}", &webfinger_url);
    let resp = reqwest::get(webfinger_url).await?;
    if resp.status() == 400 || resp.status() == 404 {
        return Ok(None);
    };
    if resp.status() == 200 {
        return Ok(Some(resp.json::<WebFingerResponse>().await?));
    }
    error!("Webfinger Respone was {}", resp.status());
    bail!("Response not 200");
}
