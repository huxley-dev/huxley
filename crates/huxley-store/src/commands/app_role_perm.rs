use uuid::Uuid;

pub struct CreateAppRolePerm {
    pub app_role_id: Uuid,
    pub app_perm_id: Uuid,
    pub metadata: serde_json::Value,
}

pub struct UpdateAppRolePerm {
    pub metadata: Option<serde_json::Value>,
}
