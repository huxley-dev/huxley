use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::tag::{CreateTag, UpdateTag},
    common::{Page, PageQuery, PageSort},
    models::tag::TagModel,
};

#[async_trait]
pub trait TagsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateTag,
    ) -> HuxleyStoreResult<TagModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<TagModel>>;
    async fn find_by_type_and_name(
        &self,
        conn: &mut PgConnection,
        tag_type: &str,
        name: &str,
    ) -> HuxleyStoreResult<Option<TagModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TagModel>>;
    async fn list_by_type(
        &self,
        conn: &mut PgConnection,
        tag_type: &str,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TagModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateTag,
    ) -> HuxleyStoreResult<Option<TagModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgTagsRepository;

#[async_trait]
impl TagsRepository for PgTagsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateTag,
    ) -> HuxleyStoreResult<TagModel> {
        let result = sqlx::query_as!(
            TagModel,
            r#"
                INSERT INTO tags (tag_type, name, bg_color, text_color)
                VALUES ($1, $2, $3, $4)
                RETURNING tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
            "#,
            input.tag_type,
            input.name,
            input.bg_color,
            input.text_color,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<TagModel>> {
        let result = sqlx::query_as!(
            TagModel,
            r#"
                SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                FROM tags
                WHERE tag_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_type_and_name(
        &self,
        conn: &mut PgConnection,
        tag_type: &str,
        name: &str,
    ) -> HuxleyStoreResult<Option<TagModel>> {
        let result = sqlx::query_as!(
            TagModel,
            r#"
                SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                FROM tags
                WHERE tag_type = $1 AND name = $2
            "#,
            tag_type,
            name,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TagModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    TagModel,
                    r#"
                        SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                        FROM tags
                        WHERE ($2::uuid IS NULL OR tag_id >= $2)
                        ORDER BY tag_id ASC
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
                    TagModel,
                    r#"
                        SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                        FROM tags
                        WHERE ($2::uuid IS NULL OR tag_id <= $2)
                        ORDER BY tag_id DESC
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
        let items: Vec<TagModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.tag_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_type(
        &self,
        conn: &mut PgConnection,
        tag_type: &str,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TagModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    TagModel,
                    r#"
                        SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                        FROM tags
                        WHERE ($2::uuid IS NULL OR tag_id >= $2) AND (tag_type = $3)
                        ORDER BY tag_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    tag_type,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    TagModel,
                    r#"
                        SELECT tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
                        FROM tags
                        WHERE ($2::uuid IS NULL OR tag_id <= $2) AND (tag_type = $3)
                        ORDER BY tag_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    tag_type,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<TagModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.tag_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateTag,
    ) -> HuxleyStoreResult<Option<TagModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_text_color, text_color) = input.text_color.into_parts();
        let (set_bg_color, bg_color) = input.bg_color.into_parts();

        let result = sqlx::query_as!(
            TagModel,
            r#"
                UPDATE tags
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    text_color = CASE WHEN $4 THEN $5::text ELSE text_color END,
                    bg_color = CASE WHEN $6 THEN $7::text ELSE bg_color END
                WHERE tag_id = $1
                RETURNING tag_id, tag_type, name, bg_color, text_color, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_text_color,
            text_color,
            set_bg_color,
            bg_color,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM tags
                WHERE tag_id = $1
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
