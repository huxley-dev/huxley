use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::org_role::{CreateOrgRole, UpdateOrgRole},
    common::{Page, PageQuery, PageSort},
    models::org_role::OrgRolePublicModel,
};

#[async_trait]
pub trait OrgRolesRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrgRole,
    ) -> HuxleyStoreResult<OrgRolePublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<OrgRolePublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<OrgRolePublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrgRole,
    ) -> HuxleyStoreResult<Option<OrgRolePublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrgRolesRepository;

#[async_trait]
impl OrgRolesRepository for PgOrgRolesRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateOrgRole,
    ) -> HuxleyStoreResult<OrgRolePublicModel> {
        let result = sqlx::query_as!(
            OrgRolePublicModel,
            r#"
                INSERT INTO org_roles (name, description)
                VALUES ($1, $2)
                RETURNING org_role_id, name, description, created_at, updated_at
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
    ) -> HuxleyStoreResult<Option<OrgRolePublicModel>> {
        let result = sqlx::query_as!(
            OrgRolePublicModel,
            r#"
                SELECT org_role_id, name, description, created_at, updated_at
                FROM org_roles
                WHERE org_role_id = $1
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
    ) -> HuxleyStoreResult<Page<OrgRolePublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    OrgRolePublicModel,
                    r#"
                        SELECT org_role_id, name, description, created_at, updated_at
                        FROM org_roles
                        WHERE ($2::uuid IS NULL OR org_role_id >= $2)
                        ORDER BY org_role_id ASC
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
                    OrgRolePublicModel,
                    r#"
                        SELECT org_role_id, name, description, created_at, updated_at
                        FROM org_roles
                        WHERE ($2::uuid IS NULL OR org_role_id <= $2)
                        ORDER BY org_role_id DESC
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
        let items: Vec<OrgRolePublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.org_role_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateOrgRole,
    ) -> HuxleyStoreResult<Option<OrgRolePublicModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            OrgRolePublicModel,
            r#"
                UPDATE org_roles
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    description = CASE WHEN $4 THEN $5::text ELSE description END
                WHERE org_role_id = $1
                RETURNING org_role_id, name, description, created_at, updated_at
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
                DELETE FROM org_roles
                WHERE org_role_id = $1
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
