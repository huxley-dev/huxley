use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;
pub struct CreateTotpCredential {
    pub user_id: Uuid,
    pub secret_enc: Vec<u8>,
    pub confirmed_at: Option<DateTime<Utc>>,
}

pub struct UpdateTotpCredential {
    pub confirmed_at: Field<DateTime<Utc>>,
}
