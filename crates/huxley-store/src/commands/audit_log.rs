use std::net::IpAddr;
use uuid::Uuid;

pub struct CreateAuditLog {
    pub user_id: Uuid,
    pub event: String,
    pub target: Option<String>,
    pub metadata: serde_json::Value,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
}
