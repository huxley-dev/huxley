pub struct CreateAppPerm {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}

pub struct UpdateAppPerm {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub metadata: serde_json::Value,
}
