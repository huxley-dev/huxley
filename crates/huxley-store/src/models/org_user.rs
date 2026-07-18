use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrgUserRowModel {
    pub org_user_id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub org_role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct OrgUserPublicModel {
    pub org_user_id: Uuid,
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub org_role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
