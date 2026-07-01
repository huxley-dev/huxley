use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org_role::{CreateOrgRole, UpdateOrgRole},
    models::org_role::OrgRoleModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgRoleRepository: Send + Sync {
    fn create(&self, conn: &mut PgConnection, input: CreateOrgRole) -> impl Future<Output = HuxleyStoreResult<OrgRoleModel>> + Send + '_;
    fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<Option<OrgRoleModel>>> + Send + '_;
    fn list(&self, conn: &mut PgConnection) -> impl Future<Output = HuxleyStoreResult<Vec<OrgRoleModel>>> + Send + '_;
    fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> impl Future<Output = HuxleyStoreResult<Vec<OrgRoleModel>>> + Send + '_;
    fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgRole) -> impl Future<Output = HuxleyStoreResult<OrgRoleModel>> + Send + '_;
    fn delete(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<bool>> + Send + '_;
}

pub struct PgOrgRoleRepository;

#[async_trait]
impl OrgRoleRepository for PgOrgRoleRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgRole) -> HuxleyStoreResult<OrgRoleModel> {
        let org_role = sqlx::query_as!(
            OrgRoleModel,
            r#"
                INSERT INTO org_roles (name, description, is_active, metadata)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, is_active, metadata, created_at, updated_at
            "#,
            input.name,
            input.description,
            input.is_active,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(org_role)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgRoleModel>> {
        let org_role = sqlx::query_as!(
            OrgRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_roles
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(org_role)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgRoleModel>> {
        let org_roles = sqlx::query_as!(
            OrgRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_roles
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(org_roles)
    }

    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<OrgRoleModel>> {
        let org_roles = sqlx::query_as!(
            OrgRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_roles
                WHERE is_active = $1
            "#,
            is_active,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_roles)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgRole) -> HuxleyStoreResult<OrgRoleModel> {
        let org_role = sqlx::query_as!(
            OrgRoleModel,
            r#"
                UPDATE org_roles
                SET name = $2,
                    description = $3,
                    is_active = $4,
                    metadata = $5,
                WHERE id = $1
            "#,
            id,
            input.name,
            input.description,
            input.is_active,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(org_role)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM org_roles
                WHERE id = $1
            "#,
            id
        )
        .execute(conn)
        .await?;

        if result.rows_affected() == 0 {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}
