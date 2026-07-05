use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AuthFlowModel {
    pub auth_flow_id: Uuid,
    pub idp_id: Uuid,
    pub state: String,
    pub pkce_verifier: Option<String>,
    pub nonce: Option<String>,
    pub relate_state: Option<String>,
    pub redirect_to: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
