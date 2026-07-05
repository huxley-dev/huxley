use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct IdentityProviderModel {
    pub idp_id: Uuid,
    pub kind: String,
    pub name: String,
    pub slug: String,
    pub enabled: bool,
    pub config: serde_json::Value,
    pub secret_enc: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
