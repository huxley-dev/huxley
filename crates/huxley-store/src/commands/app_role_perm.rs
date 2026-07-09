use uuid::Uuid;

use crate::common::Field;

pub struct CreateAppRolePerm {
    pub app_role_id: Uuid,
    pub permission: String,
}
