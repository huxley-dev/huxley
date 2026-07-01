use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org_role_perm::{CreateOrgRolePerm, UpdateOrgRolePerm},
    models::org_role_perm::OrgRolePermModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgRolePermRepository: Send + Sync {
    fn create(&self, conn: &mut PgConnection, input: CreateOrgRolePerm) -> impl Future<Output = HuxleyStoreResult<OrgRolePermModel>> + Send + '_;
    fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<Option<OrgRolePermModel>>> + Send + '_;
    fn list(&self, conn: &mut PgConnection) -> impl Future<Output = HuxleyStoreResult<Vec<OrgRolePermModel>>> + Send + '_;
    fn list_by_org_role_id(&self, conn: &mut PgConnection, org_role_id: Uuid) -> impl Future<Output = HuxleyStoreResult<Vec<OrgRolePermModel>>> + Send + '_;
    fn list_by_org_perm_id(&self, conn: &mut PgConnection, org_perm_id: Uuid) -> impl Future<Output = HuxleyStoreResult<Vec<OrgRolePermModel>>> + Send + '_;
    fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgRolePerm) -> impl Future<Output = HuxleyStoreResult<OrgRolePermModel>> + Send + '_;
    fn delete(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<bool>> + Send + '_;
}

pub struct PgOrgRolePermRepository;

#[async_trait]
impl OrgRolePermRepository for PgOrgRolePermRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgRolePerm) -> HuxleyStoreResult<OrgRolePermModel> {
        let org_role_perm = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                INSERT INTO org_role_perms (org_role_id, org_perm_id, metadata)
                VALUES ($1, $2, $3)
                RETURNING id, org_role_id, org_perm_id, metadata, created_at, updated_at
            "#,
            input.org_role_id,
            input.org_perm_id,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(org_role_perm)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgRolePermModel>> {
        let org_role_perm = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                SELECT id, org_role_id, org_perm_id, metadata, created_at, updated_at
                FROM org_role_perms
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(org_role_perm)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgRolePermModel>> {
        let org_role_perms = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM org_role_perms
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(org_role_perms)
    }


    async fn list_by_org_role_id(&self, conn: &mut PgConnection, org_role_id: Uuid) -> HuxleyStoreResult<Vec<OrgRolePermModel>> {
        let org_role_perms = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                SELECT id, org_role_id, org_perm_id, metadata, created_at, updated_at
                FROM org_role_perms
                WHERE org_role_id = $1
            "#,
            org_role_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_role_perms)
    }

    async fn list_by_org_perm_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<OrgRolePermModel>> {
        let org_role_perms = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                SELECT id, org_role_id, org_perm_id, metadata, created_at, updated_at
                FROM org_role_perms
                WHERE org_perm_id = $1
            "#,
            org_perm_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_role_perms)
    }

    async fn list_by_org_role_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<OrgUserModel>> {
        let org_users = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM org_users
                WHERE org_role_id = $1
            "#,
            org_role_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_users)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgRolePerm) -> HuxleyStoreResult<OrgRolePermModel> {
        let org_role_perm = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                UPDATE org_role_perms
                SET metadata = $1,
                WHERE id = $1
            "#,
            id,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(org_role_perm)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM org_role_perms
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
