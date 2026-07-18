use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::project::{CreateProject, UpdateProject},
    common::{Page, PageQuery, PageSort},
    models::project::ProjectModel,
};

#[async_trait]
pub trait ProjectsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateProject,
    ) -> HuxleyStoreResult<ProjectModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<ProjectModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<ProjectModel>>;
    async fn list_by_org_id(
        &self,
        conn: &mut PgConnection,
        org_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<ProjectModel>>;
    async fn update(
        &self,
        connect: &mut PgConnection,
        id: Uuid,
        input: UpdateProject,
    ) -> HuxleyStoreResult<Option<ProjectModel>>;
    async fn delete(&self, connect: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgProjectsRepository;

#[async_trait]
impl ProjectsRepository for PgProjectsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateProject,
    ) -> HuxleyStoreResult<ProjectModel> {
        let result = sqlx::query_as!(
            ProjectModel,
            r#"
                INSERT INTO projects (project_type, org_id, name, slug, description)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING project_id, project_type, org_id, name, slug, description, created_at, updated_at
            "#,
            input.project_type,
            input.org_id,
            input.name,
            input.slug,
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
    ) -> HuxleyStoreResult<Option<ProjectModel>> {
        let result = sqlx::query_as!(
            ProjectModel,
            r#"
                SELECT project_id, project_type, org_id, name, slug, description, created_at, updated_at
                FROM projects
                WHERE project_id = $1
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
    ) -> HuxleyStoreResult<Page<ProjectModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    ProjectModel,
                    r#"
                        SELECT project_id, project_type, org_id, name, slug, description, created_at, updated_at
                        FROM projects
                        WHERE ($2::uuid IS NULL OR project_id >= $2)
                        ORDER BY project_id ASC
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
                    ProjectModel,
                    r#"
                        SELECT project_id, project_type, org_id, name, slug, description, created_at, updated_at
                        FROM projects
                        WHERE ($2::uuid IS NULL OR project_id <= $2)
                        ORDER BY project_id DESC
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
        let items: Vec<ProjectModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.project_id)
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
    ) -> HuxleyStoreResult<Page<ProjectModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    ProjectModel,
                    r#"
                        SELECT project_id, project_type, org_id, name, slug, description, created_at, updated_at
                        FROM projects
                        WHERE ($2::uuid IS NULL OR project_id >= $2) AND (org_id = $3)
                        ORDER BY project_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    ProjectModel,
                    r#"
                        SELECT project_id, project_type, org_id, name, slug, description, created_at, updated_at
                        FROM projects
                        WHERE ($2::uuid IS NULL OR project_id <= $2) AND (org_id = $3)
                        ORDER BY project_id DESC
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
        let items: Vec<ProjectModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.project_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateProject,
    ) -> HuxleyStoreResult<Option<ProjectModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            ProjectModel,
            r#"
                UPDATE projects
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    slug = CASE WHEN $4 THEN $5::text ELSE slug END,
                    description = CASE WHEN $6 THEN $7::text ELSE description END
                WHERE project_id = $1
                RETURNING project_id, project_type, org_id, name, slug, description, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_slug,
            slug,
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
                DELETE FROM projects
                WHERE project_id = $1
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
