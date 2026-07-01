use uuid::Uuid;

pub struct CreateOrgUser {
    pub org_id: Uuid,
    pub user_id: Uuid,
    pub org_role_id: Uuid,
    pub metadata: serde_json::Value,
}

pub struct UpdateOrgUser {
    pub metadata: serde_json::Value,
}
