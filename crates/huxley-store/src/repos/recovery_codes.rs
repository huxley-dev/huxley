use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::recovery_code::{CreateRecoveryCode, UpdateRecoveryCode},
    common::{Page, PageQuery, PageSort},
    models::recovery_code::RecoveryCodePublicModel,
};

#[async_trait]
pub trait RecoveryCodesRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateRecoveryCode,
    ) -> HuxleyStoreResult<RecoveryCodePublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<RecoveryCodePublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<RecoveryCodePublicModel>>;
    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<RecoveryCodePublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateRecoveryCode,
    ) -> HuxleyStoreResult<Option<RecoveryCodePublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgRecoveryCodesRepository;

#[async_trait]
impl RecoveryCodesRepository for PgRecoveryCodesRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateRecoveryCode,
    ) -> HuxleyStoreResult<RecoveryCodePublicModel> {
        let result = sqlx::query_as!(
            RecoveryCodePublicModel,
            r#"
                INSERT INTO recovery_codes (user_id, code_hash, used_at)
                VALUES ($1, $2, $3)
                RETURNING rec_code_id, user_id, used_at, created_at, updated_at
            "#,
            input.user_id,
            input.code_hash,
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
    ) -> HuxleyStoreResult<Option<RecoveryCodePublicModel>> {
        let result = sqlx::query_as!(
            RecoveryCodePublicModel,
            r#"
                SELECT rec_code_id, user_id, used_at, created_at, updated_at
                FROM recovery_codes
                WHERE rec_code_id = $1
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
    ) -> HuxleyStoreResult<Page<RecoveryCodePublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    RecoveryCodePublicModel,
                    r#"
                        SELECT rec_code_id, user_id, used_at, created_at, updated_at
                        FROM recovery_codes
                        WHERE ($2::uuid IS NULL OR rec_code_id >= $2)
                        ORDER BY rec_code_id ASC
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
                    RecoveryCodePublicModel,
                    r#"
                        SELECT rec_code_id, user_id, used_at, created_at, updated_at
                        FROM recovery_codes
                        WHERE ($2::uuid IS NULL OR rec_code_id <= $2)
                        ORDER BY rec_code_id DESC
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
        let items: Vec<RecoveryCodePublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.rec_code_id)
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
    ) -> HuxleyStoreResult<Page<RecoveryCodePublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    RecoveryCodePublicModel,
                    r#"
                        SELECT rec_code_id, user_id, used_at, created_at, updated_at
                        FROM recovery_codes
                        WHERE ($2::uuid IS NULL OR rec_code_id >= $2) AND (user_id = $3)
                        ORDER BY rec_code_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    user_id,
                )
                .fetch_all(conn)
                .await?
            }
            PageSort::Desc => {
                sqlx::query_as!(
                    RecoveryCodePublicModel,
                    r#"
                        SELECT rec_code_id, user_id, used_at, created_at, updated_at
                        FROM recovery_codes
                        WHERE ($2::uuid IS NULL OR rec_code_id <= $2) AND (user_id = $3)
                        ORDER BY rec_code_id DESC
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
        let items: Vec<RecoveryCodePublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.rec_code_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateRecoveryCode,
    ) -> HuxleyStoreResult<Option<RecoveryCodePublicModel>> {
        let (set_used_at, used_at) = input.used_at.into_parts();

        let result = sqlx::query_as!(
            RecoveryCodePublicModel,
            r#"
                UPDATE recovery_codes
                SET used_at = CASE WHEN $2 THEN $3::timestamptz ELSE used_at END
                WHERE rec_code_id = $1
                RETURNING rec_code_id, user_id, used_at, created_at, updated_at
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
                DELETE FROM recovery_codes
                WHERE rec_code_id = $1
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
