use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use crate::{HuxleyApiError, HuxleyApiResult};
use huxley_store::{
    commands::credential::{CreateCredential, UpdateCredential},
    common::to_field,
    models::credential::CredentialModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "credential.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CreateCredentialRequest {
    pub org_id: Uuid,
    pub name: String,
    pub value: Option<String>,
    pub inheritable: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "credential.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateCredentialRequest {
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
#[ts(export, export_to = "credential.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct CredentialResponse {
    cred_id: Uuid,
    org_id: Uuid,
    name: String,
    value: Option<String>,
    inhertiable: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl TryFrom<CreateCredentialRequest> for CreateCredential {
    type Error = HuxleyApiError;

    fn try_from(req: CreateCredentialRequest) -> HuxleyApiResult<Self> {
        Ok(CreateCredential {
            org_id: req.org_id,
            name: req.name,
            value: req.value,
            inheritable: req.inheritable,
        })
    }
}

impl From<UpdateCredentialRequest> for UpdateCredential {
    fn from(req: UpdateCredentialRequest) -> Self {
        UpdateCredential {
            name: to_field(req.name),
            value: to_field(req.value),
            inheritable: to_field(req.inheritable),
        }
    }
}

impl From<CredentialModel> for CredentialResponse {
    fn from(cred: CredentialModel) -> Self {
        CredentialResponse {
            cred_id: cred.vred_id,
            org_id: cred.org_id,
            name: cred.name,
            value: cred.value,
            inhertiable: cred.inheritable,
            created_at: cred.created_at,
            updated_at: cred.updated_at,
        }
    }
}
