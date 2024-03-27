use crate::models::db::profile::FullProfile;
use anyhow::bail;
use reqwest::header::CONTENT_TYPE;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

const PROFILE_QUERY: &str = r#"{\"query\":\"query UserProfileSocial($id: ID) {\\n\\tuser(id: $id) {\\n\\t\\tid\\n\\t\\tpublicUsername\\n\\t\\tavatarFilePath\\n\\t\\thandle\\n\\t\\thandle\\n\\t\\tpublicUsername\\n\\t\\temail\\n\\t\\tmakesCount\\n\\t\\tdateCreated\\n\\t\\tbio\\n\\t\\tsocialLinks {\\n\\t\\t\\tid\\n\\t\\t\\tsocialType\\n\\t\\t\\turl\\n\\t\\t}\\n\\t\\tprinters {\\n\\t\\t\\tid\\n\\t\\t\\tname\\n\\t\\t}\\n\\t}\\n}\\n\",\"operationName\":\"UserProfileSocial\",\"variables\":{\"id\":\"@_USERNAME_\"}}"#;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub public_username: String,
    pub avatar_file_path: String,
    pub handle: String,
    pub email: String,
    pub makes_count: i64,
    pub date_created: String,
    pub bio: String,
    pub social_links: Vec<SocialLink>,
    pub printers: Vec<Printer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialLink {
    pub id: String,
    pub social_type: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Printer {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub enum CheckPrintablesProfile {
    UserNotFound,
    LinkNotFound,
    IsOk,
}

pub async fn get_printables_profile(printables_username: &str) -> anyhow::Result<Option<User>> {
    let gql_query = PROFILE_QUERY.replace("_USERNAME_", printables_username);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.printables.com/graphql/")
        .header(CONTENT_TYPE, "application/json")
        .body(gql_query)
        .send()
        .await?
        .json::<Value>()
        .await?;
    let data = match res.get("data") {
        Some(d) => d.get("user").unwrap(),
        None => return Ok(None),
    };
    Ok(Some(serde_json::from_value::<User>(data.clone())?))
}

pub async fn check_printables_profile(
    printables_username: &str,
    profile_id: &Uuid,
    base_url: &str,
) -> anyhow::Result<CheckPrintablesProfile> {
    let user: User = get_printables_profile(printables_username)?;
    let wanted_url = format!("{}/links/printables/{}", base_url, profile_id);
    for l in user.social_links {
        if wanted_url == l.url {
            return Ok(CheckPrintablesProfile::IsOk);
        }
    }
    Ok(CheckPrintablesProfile::LinkNotFound)
}
