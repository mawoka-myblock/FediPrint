use chrono::{DateTime, Utc};
use meilisearch_sdk::{errors::Error, Index, SearchResult, SearchResults};
use serde_derive::{Deserialize, Serialize};
use serde_json::{Map, Value};
use shared::db::model::{FullModel, FullModelWithRelationsIds};
use shared::db::note::FullNote;
use shared::db::EventAudience;
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub enum RecordType {
    Model,
    Note,
}

#[derive(Deserialize, Serialize, PartialEq, Clone)]
pub struct MsModel {
    pub id: Uuid,
    pub title: Option<String>,
    pub content: String,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
    pub profile_id: Uuid,
    pub record_type: RecordType,
    pub image_ids: Vec<Uuid>,
}

impl MsModel {
    async fn index(self, index: &Index) -> Result<(), Error> {
        index
            .add_or_replace(
                &[MsModel {
                    id: self.id,
                    title: self.title,
                    content: self.content,
                    summary: self.summary,
                    created_at: self.created_at,
                    updated_at: self.updated_at,
                    tags: self.tags,
                    profile_id: self.profile_id,
                    record_type: RecordType::Model,
                    image_ids: self.image_ids,
                }],
                Some("id"),
            )
            .await?;
        Ok(())
    }
    async fn delete_if_existing(id: &Uuid, index: &Index) -> Result<(), Error> {
        let str_id = id.to_string();
        if index.get_document::<MsModel>(&str_id).await.is_ok() {
            index.delete_document(&str_id).await?;
        }
        Ok(())
    }
}

pub async fn index_model(
    model: &FullModelWithRelationsIds,
    profile_id: &Uuid,
    index: &Index,
) -> Result<(), Error> {
    if !model.published {
        MsModel::delete_if_existing(&model.id, index).await?;
        return Ok(());
    }
    MsModel {
        id: model.id,
        title: Some(model.title.clone()),
        content: model.description.clone(),
        summary: Some(model.summary.clone()),
        tags: model.tags.clone(),
        profile_id: *profile_id,
        created_at: model.created_at,
        updated_at: model.updated_at,
        record_type: RecordType::Note,
        image_ids: model.images.clone().unwrap_or(vec![]),
    }
    .index(index)
    .await?;

    Ok(())
}

pub async fn index_note(note: &FullNote, profile_id: &Uuid, index: &Index) -> Result<(), Error> {
    if note.audience != EventAudience::Public {
        MsModel::delete_if_existing(&note.id, index).await?;
        return Ok(());
    }
    MsModel {
        id: note.id,
        title: None,
        content: note.content.clone(),
        summary: None,
        tags: note.hashtags.clone(),
        updated_at: note.updated_at,
        created_at: note.created_at,
        profile_id: *profile_id,
        record_type: RecordType::Note,
        image_ids: vec![],
    }
    .index(index)
    .await?;
    Ok(())
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SafeSearchResult {
    pub result: MsModel,
    pub formatted_result: Option<Map<String, Value>>,
    pub ranking_score: Option<f64>,
}
impl SafeSearchResult {
    pub fn from_ms(d: &SearchResult<MsModel>) -> SafeSearchResult {
        SafeSearchResult {
            result: d.result.clone(),
            formatted_result: d.formatted_result.clone(),
            ranking_score: d.ranking_score,
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct SafeSearchResults {
    pub hits: Vec<SafeSearchResult>,
    pub offset: Option<usize>,
    pub limit: Option<usize>,
    pub estimated_total_hits: Option<usize>,
    pub page: Option<usize>,
    pub hits_per_page: Option<usize>,
    pub total_hits: Option<usize>,
    pub total_pages: Option<usize>,
    pub processing_time_ms: usize,
}

impl SafeSearchResults {
    pub fn from_ms(d: SearchResults<MsModel>) -> SafeSearchResults {
        SafeSearchResults {
            hits: d.hits.iter().map(SafeSearchResult::from_ms).collect(),
            offset: d.offset,
            limit: d.limit,
            estimated_total_hits: d.estimated_total_hits,
            page: d.page,
            hits_per_page: d.hits_per_page,
            total_hits: d.total_hits,
            total_pages: d.total_pages,
            processing_time_ms: d.processing_time_ms,
        }
    }
}

pub async fn search(
    query: &str,
    page: i64,
    page_size: i64,
    index: &Index,
) -> Result<SafeSearchResults, Error> {
    let resp: SearchResults<MsModel> = index
        .search()
        .with_query(query)
        .with_hits_per_page(page_size as usize)
        .with_page(page as usize)
        .execute()
        .await?;
    Ok(SafeSearchResults::from_ms(resp))
}
