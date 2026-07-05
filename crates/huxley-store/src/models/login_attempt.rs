use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct LoginAttemptModel {
    pub login_attempt_id: Uuid,
    pub user_id: Uuid,
    pub email: Option<String>,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub successful: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
