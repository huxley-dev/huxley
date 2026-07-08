use crate::common::Field;

pub struct CreateOrganization {
    pub parent_id: Option<String>,
    pub name: String,
    pub slug: String,
    pub status: String,
    pub settings: serde_json::Value,
}

pub struct UpdateOrganization {
    pub parent_id: Field<Uuid>,
    pub name: Field<String>,
    pub slug: Field<String>,
    pub state: Field<String>,
    pub settings: Field<serde_json::Value>,
}
