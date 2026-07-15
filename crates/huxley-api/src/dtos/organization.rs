use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiError, HuxleyApiResult};
use huxley_store::{
    commands::{CreateOrganization, UpdateOrganization},
    common::to_field,
    models::OrganizationModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "organization.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateOrganizationRequest {
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub status: String,
    pub settings: serde_json::Value,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "organization.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrganizationRequest {
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
    pub status: Option<Option<String>>,
    #[ts(optional, as = "Option<serde_json::Value>")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub settings: Option<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "organization.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct OrganizationResponse {
    org_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    slug: String,
    status: String,
    settings: serde_json::Value,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateOrganizationRequest> for CreateOrganization {
    type Error = HuxleyApiError;

    fn try_from(req: CreateOrganizationRequest) -> HuxleyApiResult<Self> {
        Ok(CreateOrganization {
            parent_id: req.parent_id,
            name: req.name,
            slug: req.slug,
            status: req.status,
            settings: req.settings,
        })
    }
}

impl From<UpdateOrganizationRequest> for UpdateOrganization {
    fn from(req: UpdateOrganizationRequest) -> Self {
        UpdateOrganization {
            parent_id: to_field(req.parent_id),
            name: to_field(req.name),
            slug: to_field(req.slug),
            status: to_field(req.status),
            settings: to_field(req.settings),
        }
    }
}

impl From<OrganizationModel> for OrganizationResponse {
    fn from(org: OrganizationModel) -> Self {
        OrganizationResponse {
            org_id: org.org_id,
            parent_id: org.parent_id,
            name: org.name,
            slug: org.slug,
            status: org.status,
            settings: org.settings,
            created_at: org.created_at,
            updated_at: org.updated_at,
        }
    }
}
