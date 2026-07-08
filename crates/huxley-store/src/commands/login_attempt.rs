use uuid::Uuid;
use std::net::IpAddr;

pub struct CreateLoginAttempt {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub ip: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub successfull: bool,
}
