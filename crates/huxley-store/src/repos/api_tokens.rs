use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::api_token::{CreateApiToken, UpdateApiToken},
    common::{Page, PageQuery, PageSort},
    models::api_token::ApiTokenModel,
};

#[async_trait]
pub trait ApiTokensRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateApiToken,
    ) -> HuxleyStoreResult<ApiTokenModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<ApiTokenModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<ApiTokenModel>>;
    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<ApiTokenModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateApiToken,
    ) -> HuxleyStoreResult<Option<ApiTokenModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgApiTokensRepository;

#[async_trait]
impl ApiTokensRepository for PgApiTokensRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateApiToken,
    ) -> HuxleyStoreResult<ApiTokenModel> {
        let result = sqlx::query_as!(
            ApiTokenModel,
            r#"
                INSERT INTO api_tokens (user_id, name, prefix, token_hash, scopes, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING api_token_id, user_id, name, prefix, token_hash, scopes, last_used_at, expires_at, revoked_at, created_at, updated_at
            "#,
            input.user_id,
            input.name,
            input.prefix,
            input.token_hash,
            &input.scopes,
            input.expires_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<ApiTokenModel>> {
        let result = sqlx::query_as!(
            ApiTokenModel,
            r#"
                SELECT api_token_id, user_id, name, prefix, token_hash, scopes, last_used_at, expires_at, revoked_at, created_at, updated_at
                FROM api_tokens
                WHERE api_token_id = $1
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
    ) -> HuxleyStoreResult<Page<ApiTokenModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    ApiTokenModel,
                    r#"
                        SELECT api_token_id, user_id, name, prefix, scopes, token_hash, last_used_at, expires_at, revoked_at, created_at, updated_at
                        FROM api_tokens
                        WHERE ($2::uuid IS NULL OR api_token_id >= $2)
                        ORDER BY api_token_id ASC
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
                    ApiTokenModel,
                    r#"
                        SELECT api_token_id, user_id, name, prefix, scopes, token_hash, last_used_at, expires_at, revoked_at, created_at, updated_at
                        FROM api_tokens
                        WHERE ($2::uuid IS NULL OR api_token_id <= $2)
                        ORDER BY api_token_id DESC
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
        let items: Vec<ApiTokenModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.api_token_id)
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
    ) -> HuxleyStoreResult<Page<ApiTokenModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    ApiTokenModel,
                    r#"
                        SELECT api_token_id, user_id, name, prefix, scopes, token_hash, last_used_at, expires_at, revoked_at, created_at, updated_at
                        FROM api_tokens
                        WHERE ($2::uuid IS NULL OR api_token_id >= $2) AND (user_id = $3)
                        ORDER BY api_token_id ASC
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
                    ApiTokenModel,
                    r#"
                        SELECT api_token_id, user_id, name, prefix, scopes, token_hash, last_used_at, expires_at, revoked_at, created_at, updated_at
                        FROM api_tokens
                        WHERE ($2::uuid IS NULL OR api_token_id <= $2) AND (user_id = $3)
                        ORDER BY api_token_id DESC
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
        let items: Vec<ApiTokenModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.api_token_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateApiToken,
    ) -> HuxleyStoreResult<Option<ApiTokenModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_prefix, prefix) = input.prefix.into_parts();
        let (set_scopes, scopes) = input.scopes.into_parts();
        let (set_last_used_at, last_used_at) = input.last_used_at.into_parts();
        let (set_expires_at, expires_at) = input.expires_at.into_parts();
        let (set_revoked_at, revoked_at) = input.revoked_at.into_parts();

        let result = sqlx::query_as!(
            ApiTokenModel,
            r#"
                UPDATE api_tokens
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    prefix = CASE WHEN $4 THEN $5::text ELSE prefix END,
                    scopes = CASE WHEN $6 THEN $7::text[] ELSE scopes END,
                    last_used_at = CASE WHEN $8 THEN $9::timestamptz ELSE last_used_at END,
                    expires_at = CASE WHEN $10 THEN $11::timestamptz ELSE expires_at END,
                    revoked_at = CASE WHEN $12 THEN $13::timestamptz ELSE revoked_at END
                WHERE api_token_id = $1
                RETURNING api_token_id, user_id, name, prefix, token_hash, scopes, last_used_at, expires_at, revoked_at, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_prefix,
            prefix,
            set_scopes,
            scopes.as_ref().map(|v| &v[..]),
            set_last_used_at,
            last_used_at,
            set_expires_at,
            expires_at,
            set_revoked_at,
            revoked_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from api_tokens
                WHERE api_token_id = $1
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
