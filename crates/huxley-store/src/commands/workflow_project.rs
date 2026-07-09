use uuid::Uuid;

use crate::common::Field;

pub struct CreateWorkflowProject {
    pub project_type: String,
    pub org_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct UpdateWorkflowProject {
    pub name: Field<String>,
    pub slug: Field<String>,
    pub description: Field<String>,
}
