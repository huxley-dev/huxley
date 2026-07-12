use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiResult, HuxleyApiError};
use huxley_store::{
    commands::tag::{CreateTag, UpdateTag},
    models::tag::TagModel,
    common::{Field, to_field},
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "tag.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateTagRequest {
    pub tag_type: String,
    pub name: String,
    pub bg_color: String,
    pub text_color: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "tag.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagRequest {
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub bg_color: Option<Option<String>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub text_color: Option<Option<String>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "tag.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct TagResponse {
    tag_id: Uuid,
    tag_type: String,
    name: String,
    bg_color: String,
    text_color: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateTagRequest> for CreateTag {
    type Error = HuxleyApiError;

    fn try_from(req: CreateTagRequest) -> HuxleyApiResponse<Self> {
        Ok(
            CreateTag {
                tag_type: req.tag_type,
                name: req.name,
                bg_color: req.name,
                text_color: req.name,
            }
        )
    }
}

impl From<UpdateTagRequest> for UpdateTag {
    fn from(req: UpdateTagRequest) -> Self {
        UpdateTag {
            name: to_field(req.name),
            bg_color: to_field(req.bg_color),
            text_color: to_field(req.text_color),
        }
    }
}

impl From<TagModel> for TagResponse {
    fn from (tag: TagModel) -> Self {
        TagResponse {
            tag_id: tag.tag_id,
            tag_type: tag.tag_type,
            name: tag.name,
            bg_color: tag.bg_color,
            text_color: tag.text_color,
            created_at: tag.created_at,
            updated_at: tag.updated_at,
        }
    }
}
