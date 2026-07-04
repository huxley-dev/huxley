use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::app_role_perm::{CreateAppRolePerm, UpdateAppRolePerm},
    models::app_role_perm::AppRolePermModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AppRolePermsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppRolePerm) -> HuxleyStoreResult<AppRolePermModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppRolePermModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppRolePermModel>>;
    async fn list_by_app_role_id(&self, conn: &mut PgConnection, app_role_id: Uuid) -> HuxleyStoreResult<Vec<AppRolePermModel>>;
    async fn list_by_app_perm_id(&self, conn: &mut PgConnection, app_perm_id: Uuid) -> HuxleyStoreResult<Vec<AppRolePermModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppRolePerm) -> HuxleyStoreResult<AppRolePermModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAppRolePermsRepository;

#[async_trait]
impl AppRolePermsRepository for PgAppRolePermsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppRolePerm) -> HuxleyStoreResult<AppRolePermModel> {
        let app_role_perm = sqlx::query_as!(
            AppRolePermModel,
            r#"
                INSERT INTO app_role_perms (app_role_id, app_perm_id, metadata)
                VALUES ($1, $2, $3)
                RETURNING id, app_role_id, app_perm_id, metadata, created_at, updated_at
            "#,
            input.app_role_id,
            input.app_perm_id,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(app_role_perm)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppRolePermModel>> {
        let app_role_perm = sqlx::query_as!(
            AppRolePermModel,
            r#"
                SELECT id, app_role_id, app_perm_id, metadata, created_at, updated_at
                FROM app_role_perms
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(app_role_perm)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppRolePermModel>> {
        let app_role_perms = sqlx::query_as!(
            AppRolePermModel,
            r#"
                SELECT id, org_id, user_id, org_user_id, metadata, created_at, updated_at
                FROM app_role_perms
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(app_role_perms)
    }


    async fn list_by_app_role_id(&self, conn: &mut PgConnection, app_role_id: Uuid) -> HuxleyStoreResult<Vec<AppRolePermModel>> {
        let app_role_perms = sqlx::query_as!(
            AppRolePermModel,
            r#"
                SELECT id, app_role_id, app_perm_id, metadata, created_at, updated_at
                FROM app_role_perms
                WHERE app_role_id = $1
            "#,
            app_role_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(app_role_perms)
    }

    async fn list_by_app_perm_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<AppRolePermModel>> {
        let app_role_perms = sqlx::query_as!(
            AppRolePermModel,
            r#"
                SELECT id, app_role_id, app_perm_id, metadata, created_at, updated_at
                FROM app_role_perms
                WHERE app_perm_id = $1
            "#,
            app_perm_id,
        )
        .fetch_all(conn)
        .await?;

        Ok(app_role_perms)
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

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppRolePerm) -> HuxleyStoreResult<AppRolePermModel> {
        let app_role_perm = sqlx::query_as!(
            AppRolePermModel,
            r#"
                UPDATE app_role_perms
                SET metadata = $1,
                WHERE id = $1
            "#,
            id,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(app_role_perm)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_role_perms
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
