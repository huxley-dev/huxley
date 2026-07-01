use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrgModel {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: Option<String>,
    pub is_active: bool,
    pub mappings: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
