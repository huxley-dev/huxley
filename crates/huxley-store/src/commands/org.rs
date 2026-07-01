pub struct CreateOrg {
    pub parent_id: Option<String>,
    pub name: String,
    pub slug: Option<String>,
    pub is_active: bool,
    pub mappings: serde_json::Value,
    pub metadata: serde_json::Value,
}

pub struct UpdateOrg {
    pub name: String,
    pub slug: Option<String>,
    pub is_active: bool,
    pub mappings: serde_json::Value,
    pub metadata: serde_json::Value,
}
