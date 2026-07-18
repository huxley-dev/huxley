use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrgRolePermRowModel {
    pub org_role_perm_id: Uuid,
    pub org_role_id: Uuid,
    pub permission: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrgRolePermPublicModel {
    pub org_role_perm_id: Uuid,
    pub org_role_id: Uuid,
    pub permission: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
