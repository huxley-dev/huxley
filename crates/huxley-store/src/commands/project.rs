use uuid::Uuid;

use crate::common::Field;

pub struct CreateProject {
    pub project_type: String,
    pub org_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct UpdateProject {
    pub name: Field<String>,
    pub slug: Field<String>,
    pub description: Field<String>,
}
