use crate::models::db::model::FullModel;
use crate::models::db::note::FullNote;
use crate::models::db::EventAudience;
use chrono::{DateTime, Utc};
use meilisearch_sdk::{errors::Error, Index, SearchResults};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq)]
pub enum RecordType {
    Model,
    Note,
}

#[derive(Deserialize, Serialize, PartialEq)]
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

pub async fn index_model(model: &FullModel, profile_id: &Uuid, index: &Index) -> Result<(), Error> {
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
        profile_id: profile_id.clone(),
        created_at: model.created_at,
        updated_at: model.updated_at,
        record_type: RecordType::Note,
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
        profile_id: profile_id.clone(),
        record_type: RecordType::Note,
    }
    .index(index)
    .await?;
    Ok(())
}

pub async fn search(
    query: &str,
    page: i64,
    page_size: i64,
    index: &Index,
) -> Result<SearchResults<MsModel>, Error> {
    Ok(index
        .search()
        .with_query(query)
        .with_hits_per_page(page_size as usize)
        .with_page(page as usize)
        .execute()
        .await?)
}
