use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::net::IpAddr;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct AuditLogModel {
    pub aud_log_id: Uuid,
    pub user_id: Uuid,
    pub event: String,
    pub target: Option<String>,
    pub metadata: serde_json::Value,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
