use uuid::Uuid;

use crate::common::Field;

pub struct CreateVariable {
    pub org_id: Uuid,
    pub var_type: String,
    pub name: String,
    pub value: Option<String>,
    pub inheritable: bool,
}

pub struct UpdateVariable {
    pub name: Field<String>,
    pub value: Field<String>,
    pub inheritable: Field<bool>,
}
