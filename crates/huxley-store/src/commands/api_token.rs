use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateApiToken {
    pub user_id: Uuid,
    pub name: String,
    pub prefix: String,
    pub token_hash: Vec<u8>,
    pub scopes: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Default)]
pub struct UpdateApiToken {
    pub name: Field<String>,
    pub prefix: Field<String>,
    pub scopes: Field<Vec<String>>,
    pub last_used_at: Field<DateTime<Utc>>,
    pub expires_at: Field<DateTime<Utc>>,
    pub revoked_at: Field<DateTime<Utc>>,
}
