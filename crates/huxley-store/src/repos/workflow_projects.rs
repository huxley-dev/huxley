use async_trait::async_trait;
use sqlx::{PgConnection, postgres::PgConnectOptions};
use uuid::Uuid;

use crate::{
  commands::workflow_project::{CreateWorkflowProject, UpdateWorkflowProject},
  models::workflow_project::WorkflowProjectModel,
  common::{Page, PageQuery, PageSort},
  HuxleyStoreResult
};

#[async_trait]
pub trait WorkflowProjectsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WorkflowProjectModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>>;
    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>>;
    async fn update(&self, connect: &mut PgConnection, id: Uuid, input: UpdateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel>;
    async fn delete(&self, connect: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgWorkflowProjectsRepository;

#[async_trait]
impl WorkflowProjectsRepository for PgWorkflowProjectsRepository {
    async fn create(&self, conn: &mut PgConnectOptions, input: CreateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel> {
        let result = sqlx::query_as!(
            WorkflowProjectModel,
            r#"
                INSERT INTO workflow_projects (project_type, org_id, user_id, name, slug, description)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
            "#,
            input.project_type,
            input.org_id,
            input.user_id,
            input.name,
            input.slug,
            input.description,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WorkflowProjectModel>> {
        let result = sqlx::query_as!(
            WorkflowProjectModel,
            r#"
                SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                FROM workflow_projects
                WHERE wf_project_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id >= $2)
                        ORDER BY wf_project_id ASC
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
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id <= $2)
                        ORDER BY wf_project_id DESC
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
        let items: Vec<WorkflowProjectModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.wf_project_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id >= $2) AND (user_id = $3)
                        ORDER BY wf_project_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    user_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id <= $2) AND (user_id = $3)
                        ORDER BY wf_project_id DESC
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

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<WorkflowProjectModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.wf_project_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<WorkflowProjectModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id >= $2) AND (org_id = $3)
                        ORDER BY wf_project_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    user_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    WorkflowProjectModel,
                    r#"
                        SELECT wf_project_id, project_type, org_id, user_id, name, slug, description, created_at, updated_at
                        FROM workflow_projects
                        WHERE ($2::bigint IS NULL OR wf_project_id <= $2) AND (org_id = $3)
                        ORDER BY wf_project_id DESC
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

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<WorkflowProjectModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.wf_project_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel> {
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            WorkflowProjectModel,
            r#"
                UPDATE workflow_projects
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    slug = CASE WHEN $4 THEN $5::text ELSE slug END,
                    description = CASE WHEN $6 THEN $7::text ELSE description END,
                WHERE wf_project_id = $1
            "#,
            id,
            set_name, name,
            set_slug, slug,
            set_description, description,
        )
        .execute(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM workflow_projects
                WHERE wf_project_id = $1
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
