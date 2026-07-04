use uuid::Uuid;

pub struct CreateOrgRolePerm {
    pub org_role_id: Uuid,
    pub org_perm_id: Uuid,
    pub metadata: serde_json::Value,
}

pub struct UpdateOrgRolePerm {
    pub metadata: Option<serde_json::Value>,
}
