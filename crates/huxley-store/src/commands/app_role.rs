pub struct CreateAppRole {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}

pub struct UpdateAppRole {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}
