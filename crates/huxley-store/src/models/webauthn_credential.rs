use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct WebAuthnCredentialRowModel {
    pub wauthn_cred_id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub credential: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub aaguid: Option<Uuid>,
    pub transports: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct WebAuthnCredentialPublicModel {
    pub wauthn_cred_id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
