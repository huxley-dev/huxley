use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct ApiTokenModel {
    pub api_token_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub prefix: String,
    pub token_hash: Vec<u8>,
    pub scopes: Vec<String>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
