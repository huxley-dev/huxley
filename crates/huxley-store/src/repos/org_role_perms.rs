use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::org_role_perm::CreateOrgRolePerm,
    models::org_role_perm::OrgRolePermModel,
    common::{Page, PageQuery, PageSort},
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrgRolePermsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgRolePerm) -> HuxleyStoreResult<OrgRolePermModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgRolePermModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>>;
    async fn list_by_org_role_id(&self, conn: &mut PgConnection, org_role_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>>;
    async fn list_by_permission(&self, conn: &mut PgConnection, permission: &str, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrgRolePermsRepository;

#[async_trait]
impl OrgRolePermsRepository for PgOrgRolePermsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrgRolePerm) -> HuxleyStoreResult<OrgRolePermModel> {
        let org_role_perm = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                INSERT INTO org_role_perms (org_role_id, permission)
                VALUES ($1, $2)
                RETURNING org_role_perm_id, org_role_id, permission, created_at, updated_at
            "#,
            input.org_role_id,
            input.permission,
        )
        .fetch_one(conn)
        .await?;

        Ok(org_role_perm)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrgRolePermModel>> {
        let result = sqlx::query_as!(
            OrgRolePermModel,
            r#"
                SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                FROM org_role_perms
                WHERE org_role_perm_id = $1
            "#,
            id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id >= $2)
                        ORDER BY org_role_perm_id ASC
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
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id <= $2)
                        ORDER BY org_role_perm_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<OrgRolePermModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_role_perm_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }


    async fn list_by_org_role_id(&self, conn: &mut PgConnection, org_role_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id >= $2) AND (org_role_id = $3)
                        ORDER BY org_role_perm_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_role_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id <= $2) AND (org_role_id = $3)
                        ORDER BY org_role_perm_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_role_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<OrgRolePermModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_role_perm_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_permission(&self, conn: &mut PgConnection, permission: &str, page: PageQuery) -> HuxleyStoreResult<Page<OrgRolePermModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id >= $2) AND (permission = $3)
                        ORDER BY org_role_perm_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    permission,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgRolePermModel,
                    r#"
                        SELECT org_role_perm_id, org_role_id, permission, created_at, updated_at
                        FROM org_role_perms
                        WHERE ($2::bigint IS NULL OR org_role_perm_id <= $2) AND (permission = $3)
                        ORDER BY org_role_perm_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    permission,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<OrgRolePermModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_role_perm_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM org_role_perms
                WHERE org_role_perm_id = $1
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
