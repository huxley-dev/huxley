use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiError, HuxleyApiResult};
use huxley_store::{
    commands::{CreateProject, UpdateProject},
    common::to_field,
    models::ProjectModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "project.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    pub project_type: String,
    pub org_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "project.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateProjectRequest {
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
#[ts(export, export_to = "project.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    project_id: Uuid,
    project_type: String,
    org_id: Option<Uuid>,
    user_id: Option<Uuid>,
    name: String,
    slug: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateProjectRequest> for CreateProject {
    type Error = HuxleyApiError;

    fn try_from(req: CreateProjectRequest) -> HuxleyApiResult<Self> {
        Ok(CreateProject {
            project_type: req.project_type,
            org_id: req.org_id,
            user_id: req.user_id,
            name: req.name,
            slug: req.slug,
            description: req.description,
        })
    }
}

impl From<UpdateProjectRequest> for UpdateProject {
    fn from(req: UpdateProjectRequest) -> Self {
        UpdateProject {
            name: to_field(req.name),
            slug: to_field(req.slug),
            description: to_field(req.description),
        }
    }
}

impl From<ProjectModel> for ProjectResponse {
    fn from(project: ProjectModel) -> Self {
        ProjectResponse {
            project_id: project.project_id,
            project_type: project.project_type,
            org_id: project.org_id,
            user_id: project.user_id,
            name: project.name,
            slug: project.slug,
            description: project.description,
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}
