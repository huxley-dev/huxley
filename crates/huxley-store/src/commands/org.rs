pub struct CreateOrg {
    pub parent_id: Option<String>,
    pub name: String,
    pub slug: String,
    pub is_active: bool,
    pub mappings: serde_json::Value,
    pub metadata: serde_json::Value,
}

pub struct UpdateOrg {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub is_active: Option<bool>,
    pub mappings: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
}
