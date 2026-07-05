use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct VerificationTokenModel {
    pub ver_token_id: Uuid,
    pub user_id: Uuid,
    pub purpose: String,
    pub token_hash: Vec<u8>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
