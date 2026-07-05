use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateFederatedIdentity {
    pub user_id: Uuid,
    pub idp_id: Uuid,
    pub subject: String,
    pub email_at_idp: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
}

pub struct UpdateFederatedIdentity {
    pub user_id: Field<Uuid>,
    pub idp_id: Field<Uuid>,
    pub subject: Field<String>,
    pub email_at_idp: Field<String>,
    pub last_login_at: Field<DateTime<Utc>>,
}
