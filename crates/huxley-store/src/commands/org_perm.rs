pub struct CreateOrgPerm {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}

pub struct UpdateOrgPerm {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}
