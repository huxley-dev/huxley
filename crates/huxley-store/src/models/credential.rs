use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CredentialModel {
    pub cred_id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub value: Option<Vec<u8>>,
    pub inheritable: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
