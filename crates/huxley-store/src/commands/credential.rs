use uuid::Uuid;

use crate::common::Field;

pub struct CreateCredential {
    pub org_id: Uuid,
    pub name: String,
    pub value: Option<Vec<u8>>,
    pub inheritable: bool,
}

pub struct UpdateCredential {
    pub name: Field<String>,
    pub value: Field<Vec<u8>>,
    pub inheritable: Field<bool>,
}
