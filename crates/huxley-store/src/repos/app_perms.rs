use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::app_perm::{CreateAppPerm, UpdateAppPerm},
    models::app_perm::AppPermModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AppPermsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppPerm) -> HuxleyStoreResult<AppPermModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppPermModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppPermModel>>;
    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<AppPermModel>>;
    async fn update(&self, id: Uuid, conn: &mut PgConnection, input: UpdateAppPerm) -> HuxleyStoreResult<AppPermModel>;
    async fn delete(&self, id: Uuid, conn: &mut PgConnection) -> HuxleyStoreResult<bool>;
}

pub struct PgAppPermsRepository;

#[async_trait]
impl AppPermsRepository for PgAppPermsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppPerm) -> HuxleyStoreResult<AppPermModel> {
        let app_perm = sqlx::query_as!(
            AppPermModel,
            r#"
                INSERT INTO app_perms (name, description, is_active, metadata)
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

        Ok(app_perm)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppPermModel>> {
        let app_perm = sqlx::query_as!(
            AppPermModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_perms
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(app_perm)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppPermModel>> {
        let app_perms = sqlx::query_as!(
            AppPermModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_perms
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(app_perms)
    }

    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<AppPermModel>> {
        let app_perms = sqlx::query_as!(
            AppPermStoreModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM app_perms
                WHERE is_active = $1
            "#,
            is_active,
        )
        .fetch_all(conn)
        .await?;

        Ok(app_perms)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppPerm) -> HuxleyStoreResult<AppPermModel> {
        let app_perm = sqlx::query_as!(
            AppPermModel,
            r#"
                UPDATE app_perms
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
            input.is_active,
        )
        .execute(conn)
        .await?;

        Ok(app_perm)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_perms
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
