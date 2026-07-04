use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::tag::{CreateTag, UpdateTag},
    models::tag::TagModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait TagsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateTag) -> HuxleyStoreResult<TagModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<TagModel>>;
    async fn find_by_type_and_name(&self, conn: &mut PgConnection, tag_type: i16, name: &str) -> HuxleyStoreResult<Option<TagModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<TagModel>>;
    async fn list_by_type(&self, conn: &mut PgConnection, tag_type: i16) -> HuxleyStoreResult<Vec<TagModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateTag) -> HuxleyStoreResult<TagModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgTagsRepository;

#[async_trait]
impl TagsRepository for PgTagsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateTag) -> HuxleyStoreResult<TagModel> {
        let tag = sqlx::query_as!(
            TagModel,
            r#"
                INSERT INTO tags (tag_type, name, bg_color, text_color, metadata)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, tag_type, name, bg_color, text_color, metadata, created_at, updated_at
            "#,
            input.tag_type,
            input.name,
            input.bg_color,
            input.text_color,
            input.metadata,
        )
        .fetch_one(conn)
        .await?;

        Ok(tag)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<TagModel>> {
        let tag = sqlx::query_as!(
            TagModel,
            r#"
                SELECT id, tag_type, name, bg_color, text_color, metadata, created_at, updated_at
                FROM tags
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(tag)
    }

    async fn find_by_type_and_name(&self, conn: &mut PgConnection, tag_type: i16, name: &str) -> HuxleyStoreResult<Option<TagModel>> {
        let tag = sqlx::query_as!(
            TagModel,
            r#"
                SELECT id, tag_type, name, bg_color, text_color, metadata, created_at, updated_at
                FROM tags
                WHERE tag_type = $1 AND name = $2
            "#,
            tag_type,
            name,
        )
        .fetch_one(conn)
        .await?;

        Ok(tag)
    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<TagModel>> {
        let tags = sqlx::query_as!(
            TagModel,
            r#"
                SELECT id, tag_type, name, bg_color, text_color, metadata, created_at, updated_at
                FROM tags
            "#
        )
        .fetch_all(conn)
        .await?;

        Ok(tags)
    }

    async fn list_by_type(&self, conn: &mut PgConnection, tag_type: i16) -> HuxleyStoreResult<Vec<TagModel>> {
        let tags = sqlx::query_as!(
            TagModel,
            r#"
                SELECT id, tag_type, name, bg_color, text_color, metadata, created_at, updated_at
                FROM tags
                WHERE tag_type = $1
            "#,
            tag_type,
        )
        .fetch_all(conn)
        .await?;

        Ok(tags)
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateTag) -> HuxleyStoreResult<TagModel> {
        let tag = sqlx::query_as!(
            TagModel,
            r#"
                UPDATE tags
                SET name = $2,
                    bg_color = $3,
                    text_color = $4
                    metadata = $5,
                WHERE id = $1
            "#,
            id,
            input.name,
            input.bg_color,
            input.text_color,
            input.metadata,
        )
        .execute(conn)
        .await?;

        Ok(tag)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM tags
                WHERE id = $1
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
