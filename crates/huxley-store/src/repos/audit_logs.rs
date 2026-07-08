use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::audit_log::CreateAuditLog,
    models::audit_log::AuditLogModel,
    common::{Page, PageQuery, PageSort},
    HuxleyStoreResult,
};

#[async_trait]
pub trait AuditLogsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuditLog) -> HuxleyStoreResult<AuditLogModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuditLogModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<AuditLogModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<AuditLogModel>>;
    async fn delete_older_than(&self, conn: &mut PgConnection, date: DateTime<Utc>) -> HuxleyStoreResult<bool>;
}

pub struct PgAuditLogsRepository;

#[async_trait]
impl AuditLogsRepository for PgAuditLogsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuditLog) -> HuxleyStoreResult<AuditLogModel> {
        let result = sqlx::query_as!(
            AuditLogModel,
            r#"
                INSERT INTO audit_logs (user_id, event, target, metadata, ip, user_agent)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING aud_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
            "#,
            input.user_id,
            input.event,
            input.target,
            input.metadata,
            input.ip,
            input.user_agent
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuditLogModel>> {
        let result = sqlx::query_as!(
            AuditLogModel,
            r#"
                SELECT aud_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
                FROM audit_logs
                WHERE aud_log_id = $1
            "#,
            id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<AuditLogModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AuditLogModel,
                    r#"
                        SELECT aud_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
                        FROM audit_logs
                        WHERE ($2::bigint IS NULL OR aud_log_id >= $2)
                        ORDER BY aud_log_id ASC
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
                    AuditLogModel,
                    r#"
                        SELECT aug_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
                        FROM audit_logs
                        WHERE ($2::bigint IS NULL OR aud_log_id <= $2)
                        ORDER BY aud_log_id DESC
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
        let items: Vec<AuditLogModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.aud_log_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<AuditLogModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AuditLogModel,
                    r#"
                        SELECT aud_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
                        FROM audit_logs
                        WHERE ($2::bigint IS NULL OR aud_log_id >= $2) AND (user_id = $3)
                        ORDER BY aud_log_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    AuditLogModel,
                    r#"
                        SELECT aug_log_id, user_id, event, target, metadata, ip, user_agent, created_at, updated_at
                        FROM audit_logs
                        WHERE ($2::bigint IS NULL OR aud_log_id <= $2) AND (user_id = $3)
                        ORDER BY aud_log_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i64 > resolved_limit;
        let items: Vec<AuditLogModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.aud_log_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn delete_older_than(&self, conn: &mut PgConnection, date: DateTime<Utc>) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from api_tokens
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
