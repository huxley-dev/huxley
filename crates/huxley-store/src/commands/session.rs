use chrono::{Date, DateTime, Utc};
use std::net::IpAddr;
use uuid::Uuid;

use crate::common::Field;

pub struct CreateSession {
    pub user_id: Uuid,
    pub idp_id: Uuid,
    pub token_hash: Vec<u8>,
    pub aal: i16,
    pub auth_method: String,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub last_seen_at: DateTime<Utc>,
    pub idle_expires_at: DateTime<Utc>,
    pub absolute_expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

pub struct UpdateSession {
    pub ip: Field<String>,
    pub user_agent: Field<String>,
    pub last_seen_at: Field<DateTime<Utc>>,
    pub idle_expires_at: Field<DateTime<Utc>>,
    pub absolute_expires_at: Field<DateTime<Utc>>,
    pub revoked_at: Field<DateTime<Utc>>,
}
