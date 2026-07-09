use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateRecoveryCode {
    pub user_id: Uuid,
    pub code_hash: Vec<u8>,
    pub used_at: Option<DateTime<Utc>>,
}

pub struct UpdateRecoveryCode {
    pub used_at: Field<DateTime<Utc>>,
}
