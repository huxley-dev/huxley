use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::app_role_perm::CreateAppRolePerm,
    common::{Page, PageQuery, PageSort},
    models::app_role_perm::AppRolePermPublicModel,
};

#[async_trait]
pub trait AppRolePermsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAppRolePerm,
    ) -> HuxleyStoreResult<AppRolePermPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppRolePermPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppRolePermPublicModel>>;
    async fn list_by_app_role_id(
        &self,
        conn: &mut PgConnection,
        app_role_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppRolePermPublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAppRolePermsRepository;

#[async_trait]
impl AppRolePermsRepository for PgAppRolePermsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAppRolePerm,
    ) -> HuxleyStoreResult<AppRolePermPublicModel> {
        let result = sqlx::query_as!(
            AppRolePermPublicModel,
            r#"
                INSERT INTO app_role_perms (app_role_id, permission)
                VALUES ($1, $2)
                RETURNING app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
            "#,
            input.app_role_id,
            input.permission,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppRolePermPublicModel>> {
        let result = sqlx::query_as!(
            AppRolePermPublicModel,
            r#"
                SELECT app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
                FROM app_role_perms
                WHERE app_role_perm_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppRolePermPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AppRolePermPublicModel,
                    r#"
                        SELECT app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
                        FROM app_role_perms
                        WHERE ($2::uuid IS NULL OR app_role_perm_id >= $2)
                        ORDER BY app_role_perm_id ASC
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
                    AppRolePermPublicModel,
                    r#"
                        SELECT app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
                        FROM app_role_perms
                        WHERE ($2::uuid IS NULL OR app_role_perm_id <= $2)
                        ORDER BY app_role_perm_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                )
                .fetch_all(conn)
                .await?
            },
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<AppRolePermPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.app_role_perm_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_app_role_id(
        &self,
        conn: &mut PgConnection,
        app_role_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppRolePermPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AppRolePermPublicModel,
                    r#"
                        SELECT app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
                        FROM app_role_perms
                        WHERE ($2::uuid IS NULL OR app_role_perm_id >= $2) AND (app_role_id = $3)
                        ORDER BY app_role_perm_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    app_role_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    AppRolePermPublicModel,
                    r#"
                        SELECT app_role_perm_id, app_role_id, permission, built_in, created_at, updated_at
                        FROM app_role_perms
                        WHERE ($2::uuid IS NULL OR app_role_perm_id <= $2) AND (app_role_id = $3)
                        ORDER BY app_role_perm_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    app_role_id
                )
                .fetch_all(conn)
                .await?
            },
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<AppRolePermPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.app_role_perm_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_role_perms
                WHERE app_role_perm_id = $1
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
