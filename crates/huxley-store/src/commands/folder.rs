use uuid::Uuid;

use crate::common::Field;

pub struct CreateFolder {
    pub project_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

pub struct UpdateFolder {
    pub parent_id: Field<Uuid>,
    pub name: Field<String>,
    pub slug: Field<String>,
    pub description: Field<String>,
}
