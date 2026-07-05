use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct TotpCredentialModel {
    pub totp_cred_id: Uuid,
    pub user_id: Uuid,
    pub secret_enc: Vec<u8>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
