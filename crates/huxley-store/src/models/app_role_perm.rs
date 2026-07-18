use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AppRolePermRowModel {
    pub app_role_perm_id: Uuid,
    pub app_role_id: Uuid,
    pub permission: String,
    pub built_in: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AppRolePermPublicModel {
    pub app_role_perm_id: Uuid,
    pub app_role_id: Uuid,
    pub permission: String,
    pub built_in: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
