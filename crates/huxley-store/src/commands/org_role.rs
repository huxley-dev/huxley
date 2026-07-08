use crate::common::Field;

pub struct CreateOrgRole {
    pub name: String,
    pub description: Option<String>,
}

pub struct UpdateOrgRole {
    pub name: Field<String>,
    pub description: Field<String>,
}
