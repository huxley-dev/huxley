use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct UserModel {
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: Option<String>,
    pub status: String,
    pub preferences: serde_json::Value,
    pub app_role_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
