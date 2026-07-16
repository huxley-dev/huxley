use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiError, HuxleyApiResult};
use huxley_store::{
    commands::folder::{CreateFolder, UpdateFolder},
    common::to_field,
    models::folder::FolderModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "folder.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "folder.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderRequest {
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub parent_id: Option<Option<Uuid>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub slug: Option<Option<String>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub description: Option<Option<String>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "folder.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct FolderResponse {
    folder_id: Uuid,
    project_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    slug: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateFolderRequest> for CreateFolder {
    type Error = HuxleyApiError;

    fn try_from(req: CreateFolderRequest) -> HuxleyApiResult<Self> {
        Ok(CreateFolder {
            parent_id: req.parent_id,
            project_id: req.project_id,
            name: req.name,
            slug: req.slug,
            description: req.description,
        })
    }
}

impl From<UpdateFolderRequest> for UpdateFolder {
    fn from(req: UpdateFolderRequest) -> Self {
        UpdateFolder {
            parent_id: to_field(req.parent_id),
            name: to_field(req.name),
            slug: to_field(req.slug),
            description: to_field(req.description),
        }
    }
}

impl From<FolderModel> for FolderResponse {
    fn from(folder: FolderModel) -> Self {
        FolderResponse {
            folder_id: folder.folder_id,
            project_id: folder.project_id,
            parent_id: folder.parent_id,
            name: folder.name,
            slug: folder.slug,
            description: folder.description,
            created_at: folder.created_at,
            updated_at: folder.updated_at,
        }
    }
}
