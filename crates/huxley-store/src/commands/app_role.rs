use crate::common::Field;

pub struct CreateAppRole {
    pub name: String,
    pub description: Option<String>,
}

pub struct UpdateAppRole {
    pub name: Field<String>,
    pub description: Field<String>,
}
