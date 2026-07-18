use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::folder::{CreateFolder, UpdateFolder},
    common::{Page, PageQuery, PageSort},
    models::folder::FolderPublicModel,
};

#[async_trait]
pub trait FoldersRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateFolder,
    ) -> HuxleyStoreResult<FolderPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<FolderPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<FolderPublicModel>>;
    async fn list_by_project_id(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<FolderPublicModel>>;
    async fn list_by_parent_id(
        &self,
        conn: &mut PgConnection,
        parent_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<FolderPublicModel>>;
    async fn update(
        &self,
        connect: &mut PgConnection,
        id: Uuid,
        input: UpdateFolder,
    ) -> HuxleyStoreResult<Option<FolderPublicModel>>;
    async fn delete(&self, connect: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgFoldersRepository;

#[async_trait]
impl FoldersRepository for PgFoldersRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateFolder,
    ) -> HuxleyStoreResult<FolderPublicModel> {
        let result = sqlx::query_as!(
            FolderPublicModel,
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

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<FolderPublicModel>> {
        let result = sqlx::query_as!(
            FolderPublicModel,
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

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<FolderPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id >= $2)
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
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id <= $2)
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

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<FolderPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_project_id(
        &self,
        conn: &mut PgConnection,
        project_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<FolderPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id >= $2) AND (project_id = $3)
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
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id <= $2) AND (project_id = $3)
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

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<FolderPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
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
    ) -> HuxleyStoreResult<Page<FolderPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id >= $2) AND (parent_id = $3)
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
                    FolderPublicModel,
                    r#"
                        SELECT folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
                        FROM folders
                        WHERE ($2::uuid IS NULL OR folder_id <= $2) AND (parent_id = $3)
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

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<FolderPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.folder_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateFolder,
    ) -> HuxleyStoreResult<Option<FolderPublicModel>> {
        let (set_parent_id, parent_id) = input.parent_id.into_parts();
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_description, description) = input.description.into_parts();

        let result = sqlx::query_as!(
            FolderPublicModel,
            r#"
                UPDATE folders
                SET parent_id = CASE WHEN $2 THEN $3::uuid ELSE parent_id END,
                    name = CASE WHEN $4 THEN $5::text ELSE name END,
                    slug = CASE WHEN $6 THEN $7::text ELSE slug END,
                    description = CASE WHEN $8 THEN $9::text ELSE description END
                WHERE folder_id = $1
                RETURNING folder_id, project_id, parent_id, name, slug, description, created_at, updated_at
            "#,
            id,
            set_parent_id,
            parent_id,
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
