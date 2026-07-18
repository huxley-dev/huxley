use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct FederatedIdentityRowModel {
    pub fedid_id: Uuid,
    pub user_id: Uuid,
    pub idp_id: Uuid,
    pub subject: String,
    pub email_at_idp: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct FederatedIdentityPublicModel {
    pub fedid_id: Uuid,
    pub user_id: Uuid,
    pub idp_id: String,
    pub subject: String,
    pub email_at_idp: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
