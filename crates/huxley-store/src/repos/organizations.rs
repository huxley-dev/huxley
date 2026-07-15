use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::organization::{CreateOrganization, UpdateOrganization},
    common::{Page, PageQuery, PageSort},
    models::organization::OrganizationModel,
};

#[async_trait]
pub trait OrganizationsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrganization,
    ) -> HuxleyStoreResult<OrganizationModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<OrganizationModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrganizationModel>>;
    async fn list_by_parent_id(
        &self,
        conn: &mut PgConnection,
        parent_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrganizationModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganization,
    ) -> HuxleyStoreResult<Option<OrganizationModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrganizationsRepository;

#[async_trait]
impl OrganizationsRepository for PgOrganizationsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrganization,
    ) -> HuxleyStoreResult<OrganizationModel> {
        let result = sqlx::query_as!(
            OrganizationModel,
            r#"
                INSERT INTO organizations (parent_id, name, slug, status, settings)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING org_id, parent_id, name, slug, status, settings, created_at, updated_at
            "#,
            input.parent_id,
            input.name,
            input.slug,
            input.status,
            input.settings,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<OrganizationModel>> {
        let result = sqlx::query_as!(
            OrganizationModel,
            r#"
                SELECT org_id, parent_id, name, slug, status, settings, created_at, updated_at
                FROM organizations
                WHERE org_id = $1
            "#,
            id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrganizationModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrganizationModel,
                    r#"
                        SELECT org_id, parent_id, name, slug, status, settings, created_at, updated_at
                        FROM organizations
                        WHERE ($2::uuid IS NULL OR org_id >= $2)
                        ORDER BY org_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    OrganizationModel,
                    r#"
                        SELECT org_id, parent_id, name, slug, status, settings, created_at, updated_at
                        FROM organizations
                        WHERE ($2::uuid IS NULL OR org_id <= $2)
                        ORDER BY org_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<OrganizationModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_parent_id(
        &self,
        conn: &mut PgConnection,
        parent_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrganizationModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrganizationModel,
                    r#"
                        SELECT org_id, parent_id, name, slug, status, settings, created_at, updated_at
                        FROM organizations
                        WHERE ($2::uuid IS NULL OR org_id >= $2) AND (parent_id = $3)
                        ORDER BY org_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    parent_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    OrganizationModel,
                    r#"
                        SELECT org_id, parent_id, name, slug, status, settings, created_at, updated_at
                        FROM organizations
                        WHERE ($2::uuid IS NULL OR org_id <= $2) AND (parent_id = $3)
                        ORDER BY org_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    parent_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<OrganizationModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrganization,
    ) -> HuxleyStoreResult<Option<OrganizationModel>> {
        let (set_parent_id, parent_id) = input.parent_id.into_parts();
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_status, status) = input.status.into_parts();
        let (set_settings, settings) = input.settings.into_parts();

        let result = sqlx::query_as!(
            OrganizationModel,
            r#"
                UPDATE organizations
                SET parent_id = CASE WHEN $2 THEN $3::uuid ELSE parent_id END,
                    name = CASE WHEN $4 THEN $5::text ELSE name END,
                    slug = CASE WHEN $6 THEN $7::text ELSE slug END,
                    status = CASE WHEN $8 THEN $9::text ELSE status END,
                    settings = CASE WHEN $10 THEN $11::jsonb ELSE settings END
                WHERE org_id = $1
                RETURNING org_id, parent_id, name, slug, status, settings, created_at, updated_at
            "#,
            id,
            set_parent_id,
            parent_id,
            set_name,
            name,
            set_slug,
            slug,
            set_status,
            status,
            set_settings,
            settings,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from organizations
                WHERE org_id = $1
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
