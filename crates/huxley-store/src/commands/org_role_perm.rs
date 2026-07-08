use uuid::Uuid;

pub struct CreateOrgRolePerm {
    pub org_role_id: Uuid,
    pub permision: String,
}
