use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::common::Field;

pub struct CreateAuthFlow {
    pub idp_id: Uuid,
    pub state: String,
    pub pkce_verifier: Option<String>,
    pub nonce: Option<String>,
    pub relate_state: Option<String>,
    pub redirect_to: Option<String>,
    pub expires_at: DateTime<Utc>,
}

pub struct UpdateAuthFlow {
    pub idp_id: Field<Uuid>,
    pub state: Field<String>,
    pub pkce_verifier: Field<String>,
    pub nonce: Field<String>,
    pub relate_state: Field<String>,
    pub redirect_to: Field<String>,
    pub expires_at: Field<DateTime<Utc>>,
}
