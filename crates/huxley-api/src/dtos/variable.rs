use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiError, HuxleyApiResult};
use huxley_store::{
    commands::variable::{CreateVariable, UpdateVariable},
    common::to_field,
    models::variable::VariableModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "variable.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateVariableRequest {
    pub org_id: Uuid,
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
    pub inheritable: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "variable.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateVariableRequest {
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub name: Option<Option<String>>,
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<String>>,
    #[ts(optional, type = "boolean | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub inheritable: Option<Option<bool>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "variable.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct VariableResponse {
    var_id: Uuid,
    org_id: Uuid,
    var_type: String,
    name: String,
    value: Option<String>,
    inhertiable: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateVariableRequest> for CreateVariable {
    type Error = HuxleyApiError;

    fn try_from(req: CreateVariableRequest) -> HuxleyApiResult<Self> {
        Ok(CreateVariable {
            org_id: req.org_id,
            var_type: req.var_type,
            name: req.name,
            value: req.value,
            inheritable: req.inheritable,
        })
    }
}

impl From<UpdateVariableRequest> for UpdateVariable {
    fn from(req: UpdateVariableRequest) -> Self {
        UpdateVariable {
            name: to_field(req.name),
            value: to_field(req.value),
            inheritable: to_field(req.inheritable),
        }
    }
}

impl From<VariableModel> for VariableResponse {
    fn from(var: VariableModel) -> Self {
        VariableResponse {
            var_id: var.var_id,
            org_id: var.org_id,
            var_type: var.var_type,
            name: var.name,
            value: var.value,
            inhertiable: var.inheritable,
            created_at: var.created_at,
            updated_at: var.updated_at,
        }
    }
}
