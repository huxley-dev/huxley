use uuid::Uuid;

use crate::common::Field;

pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub status: String,
    pub preferences: serde_json::Value,
    pub app_role_id: Uuid,
}

pub struct UpdateUser {
    pub name: Field<String>,
    pub email: Field<String>,
    pub email_verified: Field<bool>,
    pub password_hash: Field<String>,
    pub status: Field<String>,
    pub preferences: Field<serde_json::Value>,
    pub app_role_id: Field<Uuid>,
}
