use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct SessionModel {
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub idp_id: Uuid,
    pub token_hash: Vec<u8>,
    pub aal: i16,
    pub auth_method: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub last_seen_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub absolute_expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
