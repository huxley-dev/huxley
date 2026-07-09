use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateWebAuthnCredential {
    pub user_id: Uuid,
    pub name: Option<String>,
    pub credential: Vec<u8>,
    pub public_key: Vec<u8>,
    pub sign_count: i64,
    pub aaguid: Option<Uuid>,
    pub transports: Vec<String>,
    pub last_used_at: Option<DateTime<Utc>>,
}

pub struct UpdateWebAuthnCredential {
    pub name: Field<String>,
    pub last_used_at: Field<DateTime<Utc>>,
}
