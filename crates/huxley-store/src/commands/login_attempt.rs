use uuid::Uuid;

pub struct CreateLoginAttempt {
    pub user_id: Uuid,
    pub email: Option<String>,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub successful: bool,
}
