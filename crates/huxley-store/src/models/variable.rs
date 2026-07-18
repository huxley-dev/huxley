use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct VariableRowModel {
    pub var_id: Uuid,
    pub org_id: Uuid,
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
    pub inheritable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct VariablePublicModel {
    pub var_id: Uuid,
    pub org_id: Uuid,
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
    pub inheritable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
