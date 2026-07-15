use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::org_user::CreateOrgUser,
    common::{Page, PageQuery, PageSort},
    models::org_user::OrgUserModel,
};

#[async_trait]
pub trait OrgUsersRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrgUser,
    ) -> HuxleyStoreResult<OrgUserModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<OrgUserModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>>;
    async fn list_by_org_id(
        &self,
        conn: &mut PgConnection,
        org_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>>;
    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>>;
    async fn list_by_org_role_id(
        &self,
        conn: &mut PgConnection,
        org_role_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrgUsersRepository;

#[async_trait]
impl OrgUsersRepository for PgOrgUsersRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrgUser,
    ) -> HuxleyStoreResult<OrgUserModel> {
        let result = sqlx::query_as!(
            OrgUserModel,
            r#"
                INSERT INTO org_users (org_id, user_id, org_role_id)
                VALUES ($1, $2, $3)
                RETURNING org_user_id, org_id, user_id, org_role_id, created_at, updated_at
            "#,
            input.org_id,
            input.user_id,
            input.org_role_id,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<OrgUserModel>> {
        let result = sqlx::query_as!(
            OrgUserModel,
            r#"
                SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                FROM org_users
                WHERE org_user_id = $1
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
    ) -> HuxleyStoreResult<Page<OrgUserModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id >= $2)
                        ORDER BY org_user_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id <= $2)
                        ORDER BY org_user_id DESC
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
        let items: Vec<OrgUserModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_org_id(
        &self,
        conn: &mut PgConnection,
        org_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id >= $2) AND (org_id = $3)
                        ORDER BY org_user_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_id,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id <= $2) AND (org_id = $3)
                        ORDER BY org_user_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<OrgUserModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id >= $2) AND (user_id = $3)
                        ORDER BY org_user_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    user_id,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id <= $2) AND (user_id = $3)
                        ORDER BY org_user_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    user_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<OrgUserModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_org_role_id(
        &self,
        conn: &mut PgConnection,
        org_role_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgUserModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id >= $2) AND (org_role_id = $3)
                        ORDER BY org_user_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_role_id,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    OrgUserModel,
                    r#"
                        SELECT org_user_id, org_id, user_id, org_role_id, created_at, updated_at
                        FROM org_users
                        WHERE ($2::uuid IS NULL OR org_user_id <= $2) AND (org_role_id = $3)
                        ORDER BY org_user_id DESC
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

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<OrgUserModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from org_users
                WHERE org_user_id = $1
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
