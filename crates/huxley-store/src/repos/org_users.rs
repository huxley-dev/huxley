use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org_user::{CreateOrgUser, UpdateOrgUser},
    models::org_user::OrgUserModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgUserRepository: Send + Sync {
    fn create(&self, conn: &mut PgConnection, input: CreateOrgUser) -> impl Future<Output = HuxleyStoreResult<OrgUserModel>> + Send + '_;
    fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<Option<OrgUserModel>>> + Send + '_;
    fn list(&self, conn: &mut PgConnection) -> impl Future<Output = HuxleyStoreResult<Vec<OrgUserModel>>> + Send + '_;
    fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> impl Future<Output = HuxleyStoreResult<Vec<OrgUserModel>>> + Send + '_;
    fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> impl Future<Output = HuxleyStoreResult<Vec<OrgUserModel>>> + Send + '_;
    fn list_by_org_role_id(&self, conn: &mut PgConnection, org_role_id: Uuid) -> impl Future<Output = HuxleyStoreResult<Vec<OrgUserModel>>> + Send + '_;
    fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgUser) -> impl Future<Output = HuxleyStoreResult<OrgUserModel>> + Send + '_;
    fn delete(&self, conn: &mut PgConnection, id: Uuid) -> impl Future<Output = HuxleyStoreResult<bool>> + Send + '_;
}

pub struct PgOrgUserRepository;

#[async_trait]
impl OrgUserRepository for PgOrgUserRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgUser) -> HuxleyStoreResult<OrgUserModel> {
        let org_user = sqlx::query_as!(
            OrgUserModel,
            r#"
                INSERT INTO org_users (org_id, user_id, org_role_id, metadata)
                VALUES ($1, $2, $3, $4)
                RETURNING id, org_id, user_id, org_role_id, metadata, created_at, updated_at
            "#,
            input.org_id,
            input.user_id,
            input.org_role_id,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(org_user)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgUserModel>> {
        let org_user = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT id, org_id, user_id, org_role_id, metadata, created_at, updated_at
                FROM org_users
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(org_user)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgUserModel>> {
        let org_users = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM org_users
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(org_users)
    }


    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<OrgUserModel>> {
        let org_users = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM org_users
                WHERE org_id = $1
            "#,
            org_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_users)
    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<OrgUserModel>> {
        let org_users = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM org_usersD
                WHERE user_id = $1
            "#,
            user_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_users)
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

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgUser) -> HuxleyStoreResult<OrgUserModel> {
        let org_user = sqlx::query_as!(
            OrgUserModel,
            r#"
                UPDATE org_users
                SET metadata = $2,
                WHERE id = $1
            "#,
            id,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(org_user)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM org_users
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
