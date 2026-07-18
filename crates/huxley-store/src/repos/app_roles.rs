use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::app_role::{CreateAppRole, UpdateAppRole},
    common::{Page, PageQuery, PageSort},
    models::app_role::AppRolePublicModel,
};

#[async_trait]
pub trait AppRolesRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAppRole,
    ) -> HuxleyStoreResult<AppRolePublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppRolePublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppRolePublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAppRole,
    ) -> HuxleyStoreResult<Option<AppRolePublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAppRolesRepository;

#[async_trait]
impl AppRolesRepository for PgAppRolesRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAppRole,
    ) -> HuxleyStoreResult<AppRolePublicModel> {
        let result = sqlx::query_as!(
            AppRolePublicModel,
            r#"
                INSERT INTO app_roles (name, description)
                VALUES ($1, $2)
                RETURNING app_role_id, name, description, built_in, created_at, updated_at
            "#,
            input.name,
            input.description,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppRolePublicModel>> {
        let result = sqlx::query_as!(
            AppRolePublicModel,
            r#"
                SELECT app_role_id, name, description, built_in, created_at, updated_at
                FROM app_roles
                WHERE app_role_id = $1
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
    ) -> HuxleyStoreResult<Page<AppRolePublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AppRolePublicModel,
                    r#"
                        SELECT app_role_id, name, description, built_in, created_at, updated_at
                        FROM app_roles
                        WHERE ($2::uuid IS NULL OR app_role_id >= $2)
                        ORDER BY app_role_id ASC
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
                    AppRolePublicModel,
                    r#"
                        SELECT app_role_id, name, description, built_in, created_at, updated_at
                        FROM app_roles
                        WHERE ($2::uuid IS NULL OR app_role_id <= $2)
                        ORDER BY app_role_id DESC
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
        let items: Vec<AppRolePublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.app_role_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAppRole,
    ) -> HuxleyStoreResult<Option<AppRolePublicModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            AppRolePublicModel,
            r#"
                UPDATE app_roles
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    description = CASE WHEN $4 THEN $5::text ELSE description END
                WHERE app_role_id = $1 AND built_in = FALSE
                RETURNING app_role_id, name, description, built_in, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_description,
            description,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM app_roles
                WHERE app_role_id = $1
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
