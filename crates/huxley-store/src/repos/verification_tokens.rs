use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::verification_token::{CreateVerificationToken, UpdateVerificationToken},
    common::{Page, PageQuery, PageSort},
    models::verification_token::VerificationTokenModel,
};

#[async_trait]
pub trait VerificationTokensRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateVerificationToken,
    ) -> HuxleyStoreResult<VerificationTokenModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<VerificationTokenModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<VerificationTokenModel>>;
    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<VerificationTokenModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateVerificationToken,
    ) -> HuxleyStoreResult<Option<VerificationTokenModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgVerificationTokensRepository;

#[async_trait]
impl VerificationTokensRepository for PgVerificationTokensRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateVerificationToken,
    ) -> HuxleyStoreResult<VerificationTokenModel> {
        let result = sqlx::query_as!(
            VerificationTokenModel,
            r#"
                INSERT INTO verification_tokens (user_id, purpose, token_hash, used_at)
                VALUES ($1, $2, $3, $4)
                RETURNING ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
            "#,
            input.user_id,
            input.purpose,
            input.token_hash,
            input.used_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<VerificationTokenModel>> {
        let result = sqlx::query_as!(
            VerificationTokenModel,
            r#"
                SELECT ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
                FROM verification_tokens
                WHERE ver_token_id = $1
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
    ) -> HuxleyStoreResult<Page<VerificationTokenModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    VerificationTokenModel,
                    r#"
                        SELECT ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
                        FROM verification_tokens
                        WHERE ($2::uuid IS NULL OR ver_token_id >= $2)
                        ORDER BY ver_token_id ASC
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
                    VerificationTokenModel,
                    r#"
                        SELECT ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
                        FROM verification_tokens
                        WHERE ($2::uuid IS NULL OR ver_token_id <= $2)
                        ORDER BY ver_token_id DESC
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
        let items: Vec<VerificationTokenModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.ver_token_id)
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
    ) -> HuxleyStoreResult<Page<VerificationTokenModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    VerificationTokenModel,
                    r#"
                        SELECT ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
                        FROM verification_tokens
                        WHERE ($2::uuid IS NULL OR ver_token_id >= $2) AND (user_id = $3)
                        ORDER BY ver_token_id ASC
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
                    VerificationTokenModel,
                    r#"
                        SELECT ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
                        FROM verification_tokens
                        WHERE ($2::uuid IS NULL OR ver_token_id <= $2) AND (user_id = $3)
                        ORDER BY ver_token_id DESC
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
        let items: Vec<VerificationTokenModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.ver_token_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateVerificationToken,
    ) -> HuxleyStoreResult<Option<VerificationTokenModel>> {
        let (set_used_at, used_at) = input.used_at.into_parts();

        let result = sqlx::query_as!(
            VerificationTokenModel,
            r#"
                UPDATE verification_tokens
                SET used_at = CASE WHEN $2 THEN $3::timestamptz ELSE used_at END
                WHERE ver_token_id = $1
                RETURNING ver_token_id, user_id, purpose, token_hash, used_at, created_at, updated_at
            "#,
            id,
            set_used_at,
            used_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM verification_tokens
                WHERE ver_token_id = $1
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
