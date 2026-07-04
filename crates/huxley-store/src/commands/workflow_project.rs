use uuid::Uuid;

pub struct CreateWorkflowProject {
    pub project_type: i16,
    pub org_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub metadata: serde_json::Value,
}

pub struct UpdateWorkflowProject {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
