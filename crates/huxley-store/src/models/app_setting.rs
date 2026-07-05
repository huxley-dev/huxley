use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AppSettingModel {
    pub app_set_id: Uuid,
    pub name: String,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
