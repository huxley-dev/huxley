use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org_perm::{CreateOrgPerm, UpdateOrgPerm},
    models::org_perm::OrgPermModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgPermsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgPerm) -> HuxleyStoreResult<OrgPermModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgPermModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgPermModel>>;
    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<OrgPermModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgPerm) -> HuxleyStoreResult<OrgPermModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrgPermsRepository;

#[async_trait]
impl OrgPermsRepository for PgOrgPermsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgPerm) -> HuxleyStoreResult<OrgPermModel> {
        let org_perm = sqlx::query_as!(
            OrgPermModel,
            r#"
                INSERT INTO org_perms (name, description, is_active, metadata)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, is_active, metadata, created_at, updated_at
            "#,
            input.name,
            input.description,
            input.is_active,
            input.metadata,
        )
        .fetch_one(connect)
        .await?;

        Ok(org_perm)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgPermModel>> {
        let org_perm = sqlx::query_as!(
            OrgPermModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_perms
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(org_perm)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgPermModel>> {
        let org_perms = sqlx::query_as!(
            OrgPermModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_perms
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(org_perms)
    }

    async fn list_by_active(&self, conn: &mut PgConnection, is_active: bool) -> HuxleyStoreResult<Vec<OrgPermModel>> {
        let org_perms = sqlx::query_as!(
            OrgPermModel,
            r#"
                SELECT id, name, description, is_active, metadata, created_at, updated_at
                FROM org_perms
                WHERE is_active = $1
            "#,
            is_active,
        )
        .fetch_all(conn)
        .await?;

        Ok(org_perms)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrgPerm) -> HuxleyStoreResult<OrgPermModel> {
        let org_perm = sqlx::query_as!(
            OrgPermModel,
            r#"
                UPDATE org_perms
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

        Ok(org_perm)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM org_perms
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
