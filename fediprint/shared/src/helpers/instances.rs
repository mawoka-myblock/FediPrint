use anyhow::{bail, Result};
use sqlx::PgPool;

use crate::db::instances::CreateInstance;
use crate::db::instances::FullInstance;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WellKnownNodeInfoResponse {
    pub links: Vec<Link>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub rel: String,
    pub href: String,
}

// Based on following JSON Schema: http://nodeinfo.diaspora.software/ns/schema/2.0
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub version: String,
    pub software: Software,
    pub protocols: Vec<String>,
    pub services: Services,
    pub openregistrations: bool,
    pub usage: Usage,
    pub metadata: std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Software {
    pub name: String,
    pub version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Services {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub users: Users,
    pub local_posts: Option<i32>,
    pub local_comments: Option<i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Users {
    pub total: Option<i32>,
    pub active_halfyear: Option<i32>,
    pub active_month: Option<i32>,
}

async fn get_node_info_url(url: String) -> Result<Option<String>> {
    let nodeinfo_wellknown = reqwest::get(url)
        .await?
        .json::<WellKnownNodeInfoResponse>()
        .await?;
    for link in nodeinfo_wellknown.links {
        if link.rel != "http://nodeinfo.diaspora.software/ns/schema/2.0" {
            continue;
        };
        return Ok(Some(link.href));
    }
    Ok(None)
}

pub async fn get_instance_by_base_url(base_url: &str, pool: PgPool) -> Result<FullInstance> {
    // BetterErrorHandling
    let instance_db_res = FullInstance::get_by_base_url(base_url, pool.clone()).await;
    if instance_db_res.is_ok() {
        return Ok(instance_db_res.unwrap());
    }
    let nodeinfo_url = get_node_info_url(format!("{}/.well-known/nodeinfo", base_url)).await?;
    if nodeinfo_url.is_none() {
        bail!("NodeInfo Url could not be found")
    }
    let node_info = reqwest::get(nodeinfo_url.unwrap())
        .await?
        .json::<NodeInfo>()
        .await?;
    if node_info.version != "2.0" {
        bail!("Manifest version wrong")
    }
    let instance = CreateInstance {
        base_url: base_url.to_string(),
        instance_name: node_info.metadata.get("nodeName").cloned(),
        user_count: node_info.usage.users.total,
        software: node_info.software.name,
        software_version: Some(node_info.software.version),
    };
    return Ok(instance.create_and_return_full(pool.clone()).await?);
}
