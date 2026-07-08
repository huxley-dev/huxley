use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::login_attempt::CreateLoginAttempt,
    models::login_attempt::LoginAttemptModel,
    common::{Page, PageQuery, PageSort},
    HuxleyStoreResult,
};

#[async_trait]
pub trait LoginAttemptsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateLoginAttempt) -> HuxleyStoreResult<LoginAttemptModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<LoginAttemptModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<LoginAttemptModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<LoginAttemptModel>>;
    async fn delete_older_than(&self, conn: &mut PgConnection, id: Uuid, date: DateTime<Utc>) -> HuxleyStoreResult<bool>;
}

pub struct PgLoginAttemptsRepository;

#[async_trait]
impl LoginAttemptsRepository for PgLoginAttemptsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateLoginAttempt) -> HuxleyStoreResult<LoginAttemptModel> {
        let result = sqlx::query_as!(
            LoginAttemptModel,
            r#"
                INSERT INTO login_attempts (user_id, email, ip, user_agent, successful)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
            "#,
            input.user_id,
            input.email,
            input.ip,
            input.user_agent,
            input.successful,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<LoginAttemptModel>> {
        let result = sqlx::query_as!(
            LoginAttemptModel,
            r#"
                SELECT login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
                FROM login_attempts
                WHERE login_attempt_id = $1
            "#,
            id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<LoginAttemptModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    LoginAttemptModel,
                    r#"
                        SELECT login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
                        FROM login_attempts
                        WHERE ($2::bigint IS NULL OR login_attempt_id >= $2)
                        ORDER BY login_attempt_id ASC
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
                    LoginAttemptModel,
                    r#"
                        SELECT login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
                        FROM login_attempts
                        WHERE ($2::bigint IS NULL OR login_attempt_id <= $2)
                        ORDER BY login_attempt_id DESC
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
        let items: Vec<LoginAttemptModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.login_attempt_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<LoginAttemptModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    LoginAttemptModel,
                    r#"
                        SELECT login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
                        FROM login_attempts
                        WHERE ($2::bigint IS NULL OR login_attempt_id >= $2) AND (user_id = $3)
                        ORDER BY login_attempt_id ASC
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
                    LoginAttemptModel,
                    r#"
                        SELECT login_attempt_id, user_id, email, ip, user_agent, successful, created_at, updated_at
                        FROM login_attempts
                        WHERE ($2::bigint IS NULL OR login_attempt_id <= $2) AND (user_id = $3)
                        ORDER BY login_attempt_id DESC
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
        let items: Vec<LoginAttemptModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.login_attempt_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn delete_older_than(&self, conn: &mut PgConnection, id: Uuid, date: DateTime<Utc>) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from login_attempts
                WHERE created_at < $1
            "#,
            date
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
