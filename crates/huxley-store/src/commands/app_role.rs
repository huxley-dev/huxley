pub struct CreateAppRole {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}

pub struct UpdateAppRole {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub metadata: Option<serde_json::Value>,
}
