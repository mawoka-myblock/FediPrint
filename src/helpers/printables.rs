use crate::models::db::file::{CreateFile, FullFile};
use crate::models::db::model::FullModel;
use crate::models::db::profile::FullProfile;
use crate::models::db::ModelLicense;
use crate::AppState;
use anyhow::bail;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use reqwest::header::CONTENT_TYPE;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;
use std::io;
use std::sync::Arc;
use tokio_util::io::StreamReader;
use tracing::debug;
use uuid::Uuid;

const PROFILE_QUERY: &str = r#"{"query":"query UserProfileSocial($id: ID) {\tuser(id: $id) { id publicUsername avatarFilePath handle handle publicUsername email makesCount dateCreated bio socialLinks { id socialType url } printers { id name }\t}}","variables":{"id":"@_USERNAME_"}}"#;

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

#[derive(Debug, Clone, PartialEq)]
pub enum CheckPrintablesProfile {
    UserNotFound,
    LinkNotFound,
    IsOk,
}

pub async fn get_printables_profile(
    printables_username: &str,
) -> Result<Option<User>, reqwest::Error> {
    let gql_query = PROFILE_QUERY.replace("_USERNAME_", printables_username);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.printables.com/graphql/")
        // .post("https://eopumybgejld0p5.m.pipedream.net")
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
    Ok(serde_json::from_value::<User>(data.clone()).ok())
}

pub async fn check_printables_profile(
    printables_username: &str,
    profile_id: &Uuid,
    base_url: &str,
) -> Result<CheckPrintablesProfile, reqwest::Error> {
    let user: User = match get_printables_profile(printables_username).await? {
        Some(d) => d,
        None => return Ok(CheckPrintablesProfile::UserNotFound),
    };
    let wanted_url = format!("{}/links/printables/{}", base_url, profile_id);
    for l in user.social_links {
        debug!("Link: {}", &l.url);
        if wanted_url == l.url {
            return Ok(CheckPrintablesProfile::IsOk);
        }
    }
    Ok(CheckPrintablesProfile::LinkNotFound)
}

const MODELS_QUERY: &str = r#"{"query":"query UserModels($userId: ID!, $ordering: String, $limit: Int!) { userModels(userId: $userId, ordering: $ordering, limit: $limit) { items { id name slug datePublished firstPublish description summary images { id filePath rotation fileSize name } stls { id name folder note created fileSize filePreviewPath } otherFiles { id name folder note created fileSize } category { id path { id name } } tags { id name } license { name } modified image { id filePath rotation fileSize name } nsfw premium user { handle } } }}","variables":{"userId":"_USERID_","ordering":"-first_publish","limit":100}}"#;
const USER_ID_QUERY: &str = r#"{"query":"query UserProfileSocial($id: ID) { user(id: $id) { id }}","variables":{"id":"@_USERNAME_"}}"#;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootModelResponse {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub user_models: UserModels,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserModels {
    pub items: Vec<PrintablesModel>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintablesModel {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub date_published: DateTime<Utc>,
    pub first_publish: DateTime<Utc>,
    pub description: String,
    pub summary: String,
    pub images: Vec<Image>,
    pub stls: Vec<Stl>,
    pub other_files: Vec<OtherFile>,
    pub category: Category,
    pub tags: Vec<Tag>,
    pub license: License,
    pub modified: String,
    pub image: Image,
    pub nsfw: bool,
    pub premium: bool,
    pub user: SmallUser,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stl {
    pub id: String,
    pub name: String,
    pub folder: String,
    pub note: String,
    pub created: String,
    pub file_size: i64,
    pub file_preview_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherFile {
    pub id: String,
    pub name: String,
    pub folder: String,
    pub note: String,
    pub created: String,
    pub file_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: String,
    pub path: Vec<Path>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub id: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub id: String,
    pub file_path: String,
    pub rotation: i64,
    pub file_size: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmallUser {
    pub handle: String,
}

async fn get_user_id(username: &str) -> Option<String> {
    let client = reqwest::Client::new();
    let gql_query = USER_ID_QUERY.replace("_USERNAME_", username);
    let res = client
        .post("https://api.printables.com/graphql/")
        .header(CONTENT_TYPE, "application/json")
        .body(gql_query)
        .send()
        .await
        .ok();
    res.as_ref()?;
    let json: Option<Value> = res.unwrap().json().await.ok();
    json.as_ref()?;
    let f_json = json.unwrap();
    let temp_d = f_json.get("data").unwrap().get("user");
    let temp_d2: Option<&Value> = match temp_d {
        Some(d) => d.get("id"),
        None => return None,
    };
    let f = temp_d2?;
    f.as_str().map(|d| d.to_string())
}

async fn save_file_to_s3(
    url: &str,
    filename: &str,
    model_id: Uuid,
    profile_id: &Uuid,
    preview_file_id: Option<Uuid>,
    state: Arc<AppState>,
) -> FullFile {
    debug!("Download url: {}", &url);
    let id = Uuid::now_v7();
    let client = reqwest::Client::new();
    // Make separate HEAD request top get content type as the stream on the "main" request consumes the body
    let content_type_res = client.head(url).send().await.unwrap();
    let content_type = content_type_res
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap();
    let resp = client.get(url).send().await.unwrap();
    let body_with_io_error = resp
        .bytes_stream()
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);
    state
        .s3
        .put_object_stream_with_content_type(&mut body_reader, filename, content_type)
        .await
        .unwrap();
    // 🗿 TIL that there's a Unicode Character for Moai.
    let metadata = state.s3.head_object(filename).await.unwrap().0;
    let mut file = CreateFile {
        id,
        mime_type: content_type.to_string(),
        size: metadata.content_length.unwrap(),
        profile_id: *profile_id,
        file_name: Some(filename.to_string()),
        preview_file_id,
        thumbhash: None,
        description: None,
        alt_text: None,
        file_for_model_id: None,
        image_for_model_id: None,
    };
    if preview_file_id.is_none() {
        if content_type.contains("image") {
            file.image_for_model_id = Some(model_id)
        } else {
            file.file_for_model_id = Some(model_id)
        }
    }
    file.create(state.pool.clone()).await.unwrap()
}

const DOWNLOAD_LINK_QUERY: &str = r#"{"query":"mutation GetDownloadLink($id: ID!, $printId: ID!, $fileType: DownloadFileTypeEnum!, $source: DownloadSourceEnum!) { getDownloadLink(id: $id, printId: $printId, fileType: $fileType, source: $source) { ok output { link } }}","variables":{"fileType":"stl","id":"_FILE_ID_","printId":"_MODEL_ID_","source":"model_viewer"}}"#;

async fn get_stl_download_link(file_id: &str, model_id: &str) -> Option<String> {
    let gql_query = DOWNLOAD_LINK_QUERY
        .replace("_FILE_ID_", file_id)
        .replace("_MODEL_ID_", model_id);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.printables.com/graphql/")
        .header(CONTENT_TYPE, "application/json")
        .body(gql_query)
        .send()
        .await
        .ok();
    res.as_ref()?;
    let json: Value = match res.unwrap().json().await.ok() {
        None => return None,
        Some(d) => d,
    };
    let output = json["data"]["getDownloadLink"]["output"]
        .as_object()
        .unwrap();
    let link = output.get("link")?;
    link.as_str().map(|d| d.to_string())
}

async fn create_single_model(
    model: PrintablesModel,
    profile_id: &Uuid,
    username: &str,
    state: Arc<AppState>,
) -> anyhow::Result<FullModel> {
    let id = Uuid::now_v7();
    let s_id = format!(
        "{}/api/v1/models/{}/{}",
        state.env.public_url, username, &id
    );
    let d = FullModel {
        id,
        profile_id: *profile_id,
        created_at: model.first_publish,
        description: model.description,
        license: ModelLicense::Bsd, // TODO
        server_id: Some(s_id),
        summary: model.summary,
        tags: model.tags.iter().map(|t| t.name.clone()).collect(),
        server: state.env.base_domain.clone(),
        title: model.name,
        updated_at: model.date_published,
        published: false,
        printables_url: Some(format!(
            "https://www.printables.com/model/{}-{}",
            model.id, model.slug
        )),
    }.create(state.pool.clone()).await?;
    for image in model.images {
        save_file_to_s3(
            &format!("https://media.printables.com/{}", image.file_path),
            &image.name,
            d.id,
            profile_id,
            None,
            state.clone(),
        )
            .await;
    }
    for file in model.stls {
        let download_link = match get_stl_download_link(&file.id, &model.id).await {
            Some(d) => d,
            None => continue,
        };
        save_file_to_s3(
            &download_link,
            &file.name,
            d.id,
            &profile_id,
            None,
            state.clone(),
        )
            .await;
    }
    Ok(d)
}

pub async fn import_all_models(profile: FullProfile, state: Arc<AppState>) -> anyhow::Result<()> {
    let printables_user_id = match get_user_id(&profile.linked_printables_profile.unwrap()).await {
        Some(d) => d,
        None => bail!("No profile found"),
    };
    debug!("User_id: {}", &printables_user_id);
    let gql_query = MODELS_QUERY.replace("_USERID_", &printables_user_id);
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.printables.com/graphql/")
        // .post("https://eopumybgejld0p5.m.pipedream.net")
        .header(CONTENT_TYPE, "application/json")
        .body(gql_query)
        .send()
        .await?
        .json::<RootModelResponse>()
        .await?;
    // .text().await?;
    let models = res.data.user_models.items;
    // debug!("Models: {:?}", &models);
    for model in models {
        create_single_model(model, &profile.id, &profile.username, state.clone()).await?;
    }
    Ok(())
}
