use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org::{CreateOrg, UpdateOrg},
    models::org::OrgModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrg) -> HuxleyStoreResult<OrgModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgModel>>;
    async fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> HuxleyStoreResult<Option<OrgModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrg) -> HuxleyStoreResult<OrgModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrgsRepository;

#[async_trait]
impl OrgsRepository for PgOrgsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrg) -> HuxleyStoreResult<OrgModel> {
        let org = sqlx::query_as!(
            OrgModel,
            r#"
                INSERT INTO orgs (parent_id, name, slug, is_active, mappings, metadata)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, parent_id, name, slug, is_active, mappings, metadata, created_at, updated_at
            "#,
            input.parent_id,
            input.name,
            input.slug,
            input.is_active,
            input.mappings,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(org)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgModel>> {
        let org = sqlx::query_as!(
            OrgModel,
            r#"
                SELECT id, parent_id, name, slug, is_active, mappings, metadata, created_at, updated_at
                FROM orgs
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(org)
    }

    async fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> HuxleyStoreResult<Option<OrgModel>> {
        let org = sqlx::query_as!(
            OrgModel,
            r#"
                SELECT id, parent_id, name, slug, is_active, mappings, metadata, created_at, updated_at
                FROM orgs
                WHERE name = $1
            "#,
            name
        )
        .fetch_optional(conn)
        .await?;

        Ok(org)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrgModel>> {
        let orgs = sqlx::query_as!(
            OrgModel,
            r#"
                SELECT id, parent_id, slug, is_active, mappings, metadata, created_at, updated_at
                FROM org
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(orgs)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrg) -> HuxleyStoreResult<OrgModel> {
        let org = sqlx::query_as!(
            OrgModel,
            r#"
                UPDATE orgs
                SET name = $2,
                    slug = $3,
                    is_active = $4,
                    mappings = $5,
                    metadata = $6,
                    updated_at = now(),
                WHERE id = $1
            "#,
            input.id,
            input.name,
            input.is_active,
            input.mappings,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(org)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from orgs
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
