use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateVerificationToken {
    pub user_id: Uuid,
    pub purpose: String,
    pub token_hash: Vec<u8>,
    pub used_at: Option<DateTime<Utc>>,
}

pub struct UpdateVerificationToken {
    pub used_at: Field<DateTime<Utc>>,
}
