use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use huxley_store::{
    commands::app_setting::UpdateAppSetting, common::to_field,
    models::app_setting::AppSettingPublicModel,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields = nullable, export_to = "appsetting.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct UpdateAppSettingRequest {
    #[ts(optional, type = "string | null")]
    #[serde(default, with = "::serde_with::rust::double_option")]
    pub value: Option<Option<String>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "appsetting.model.ts")]
#[serde(rename_all = "camelCase")]
pub struct AppSettingResponse {
    pub app_set_id: Uuid,
    pub name: String,
    pub value: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl From<UpdateAppSettingRequest> for UpdateAppSetting {
    fn from(req: UpdateAppSettingRequest) -> Self {
        UpdateAppSetting {
            value: to_field(req.value),
        }
    }
}

impl From<AppSettingPublicModel> for AppSettingResponse {
    fn from(setting: AppSettingPublicModel) -> Self {
        AppSettingResponse {
            app_set_id: setting.app_set_id,
            name: setting.name,
            value: setting.value,
            created_at: setting.created_at,
            updated_at: setting.updated_at,
        }
    }
}
