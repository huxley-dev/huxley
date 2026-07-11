use uuid::Uuid;

pub struct CreateAppRolePerm {
    pub app_role_id: Uuid,
    pub permission: String,
}
