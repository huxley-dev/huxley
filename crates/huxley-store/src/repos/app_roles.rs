use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::app_role::{CreateAppRole, UpdateAppRole},
    models::app_role::AppRoleModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AppRolesRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppRole) -> HuxleyStoreResult<AppRoleModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppRoleModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppRoleModel>>;
    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<AppRoleModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppRole) -> HuxleyStoreResult<AppRoleModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAppRolesRepository;

#[async_trait]
impl AppRolesRepository for PgAppRolesRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppRole) -> HuxleyStoreResult<AppRoleModel> {
        let app_role = sqlx::query_as!(
            AppRoleModel,
            r#"
                INSERT INTO app_roles (name, description, is_active, metadata)
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

        Ok(app_role)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppRoleModel>> {
        let app_role = sqlx::query_as!(
            AppRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_roles
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(app_role)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppRoleModel>> {
        let app_roles = sqlx::query_as!(
            AppRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_roles
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(app_roles)
    }

    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<AppRoleModel>> {
        let app_roles = sqlx::query_as!(
            AppRoleModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_roles
                WHERE is_active = $1
            "#,
            is_active,
        )
        .fetch_all(conn)
        .await?;

        Ok(app_roles)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppRole) -> HuxleyStoreResult<AppRoleModel> {
        let app_role = sqlx::query_as!(
            AppRoleModel,
            r#"
                UPDATE app_roles
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

        Ok(app_role)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_roles
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
