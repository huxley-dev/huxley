use async_trait::async_trait;
use sqlx::{PgConnection};
use uuid::Uuid;

use crate::{
  commands::folder::{CreateFolder, UpdateFolder},
  models::folder::FolderModel,
  common::{Page, PageQuery, PageSort},
  HuxleyStoreResult
};

#[async_trait]
pub trait FoldersRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateFolder) -> HuxleyStoreResult<FolderModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FolderModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>>;
    async fn list_by_project_id(&self, conn: &mut PgConnection, project_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>>;
    async fn list_by_parent_id(&self, conn: &mut PgConnection, parent_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>>;
    async fn update(&self, connect: &mut PgConnection, id: Uuid, input: UpdateFolder) -> HuxleyStoreResult<FolderModel>;
    async fn delete(&self, connect: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgFoldersRepository;

#[async_trait]
impl FoldersRepository for PgFoldersRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateFolder) -> HuxleyStoreResult<FolderModel> {
        let result = sqlx::query_as!(
            FolderModel,
            r#"
                INSERT INTO folders (project_id, parent_id, name, slug, description)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
            "#,
            input.project_id,
            input.parent_id,
            input.name,
            input.slug,
            input.description,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FolderModel>> {
        let result = sqlx::query_as!(
            FolderModel,
            r#"
                SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                FROM folders
                WHERE folder_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::bigint IS NULL OR folder_id >= $2)
                        ORDER BY folder_id ASC
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
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM projects
                        WHERE ($2::bigint IS NULL OR folder_id <= $2)
                        ORDER BY folder_id DESC
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
        let items: Vec<FolderModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_project_id(&self, conn: &mut PgConnection, project_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::bigint IS NULL OR folder_id >= $2) AND (project_id = $3)
                        ORDER BY folder_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    project_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::bigint IS NULL OR folder_id <= $2) AND (project_id = $3)
                        ORDER BY folder_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    project_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<FolderModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_parent_id(&self, conn: &mut PgConnection, parent_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FolderModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::bigint IS NULL OR folder_id >= $2) AND (parent_id = $3)
                        ORDER BY folder_id ASC
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
                    FolderModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::bigint IS NULL OR folder_id <= $2) AND (parent_id = $3)
                        ORDER BY parent_id DESC
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

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<FolderModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateFolder) -> HuxleyStoreResult<FolderModel> {
        let (set_parent_id, parent_id) = input.parent_id.into_parts();
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            FolderModel,
            r#"
                UPDATE folders
                SET parent_id = CASE WHEN $2 THEN $3::uuid ELSE parent_id END,
                    name = CASE WHEN $4 THEN $5::text ELSE name END,
                    slug = CASE WHEN $6 THEN $7::text ELSE slug END,
                    description = CASE WHEN $8 THEN $9::text ELSE description END,
                WHERE folder_id = $1
            "#,
            id,
            set_parent_id, parent_id,
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
                DELETE FROM folders
                WHERE folder_id = $1
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
