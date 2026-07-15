use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::session::{CreateSession, UpdateSession},
    common::{Page, PageQuery, PageSort},
    models::session::SessionModel,
};

#[async_trait]
pub trait SessionsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateSession,
    ) -> HuxleyStoreResult<SessionModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<SessionModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<SessionModel>>;
    async fn list_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<SessionModel>>;
    async fn list_by_idp_id(
        &self,
        conn: &mut PgConnection,
        idp_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<SessionModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateSession,
    ) -> HuxleyStoreResult<Option<SessionModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgSessionsRepository;

#[async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateSession,
    ) -> HuxleyStoreResult<SessionModel> {
        let result = sqlx::query_as!(
            SessionModel,
            r#"
                INSERT INTO sessions (user_id, idp_id, token_hash, aal, auth_method, ip, user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at)
                VALUES ($1, $2, $3, $4, $5, CAST($6 AS TEXT)::inet, $7, $8, $9, $10, $11)
                RETURNING session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
            "#,
            input.user_id,
            input.idp_id,
            input.token_hash,
            input.aal,
            input.auth_method,
            input.ip,
            input.user_agent,
            input.last_seen_at,
            input.idle_expires_at,
            input.absolute_expires_at,
            input.revoked_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<SessionModel>> {
        let result = sqlx::query_as!(
            SessionModel,
            r#"
                SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                FROM sessions
                WHERE session_id = $1
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
    ) -> HuxleyStoreResult<Page<SessionModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id >= $2)
                        ORDER BY session_id ASC
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
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id <= $2)
                        ORDER BY session_id DESC
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
        let items: Vec<SessionModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.session_id)
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
    ) -> HuxleyStoreResult<Page<SessionModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id >= $2) AND (user_id = $3)
                        ORDER BY session_id ASC
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
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id <= $2) AND (user_id = $3)
                        ORDER BY session_id DESC
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
        let items: Vec<SessionModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.session_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_idp_id(
        &self,
        conn: &mut PgConnection,
        idp_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<SessionModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id >= $2) AND (idp_id = $3)
                        ORDER BY session_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    idp_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    SessionModel,
                    r#"
                        SELECT session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
                        FROM sessions
                        WHERE ($2::uuid IS NULL OR session_id <= $2) AND (idp_id = $3)
                        ORDER BY session_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    idp_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<SessionModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.session_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateSession,
    ) -> HuxleyStoreResult<Option<SessionModel>> {
        let (set_ip, ip) = input.ip.into_parts();
        let (set_user_agent, user_agent) = input.user_agent.into_parts();
        let (set_last_seen_at, last_seen_at) = input.last_seen_at.into_parts();
        let (set_idle_expires_at, idle_expires_at) = input.idle_expires_at.into_parts();
        let (set_absolute_expires_at, absolute_expires_at) = input.absolute_expires_at.into_parts();
        let (set_revoked_at, revoked_at) = input.revoked_at.into_parts();

        let result = sqlx::query_as!(
            SessionModel,
            r#"
                UPDATE sessions
                SET ip = CASE WHEN $2 THEN CAST($3 AS TEXT)::inet ELSE ip END,
                    user_agent = CASE WHEN $4 THEN $5::text ELSE user_agent END,
                    last_seen_at = CASE WHEN $6 THEN $7::timestamptz ELSE last_seen_at END,
                    idle_expires_at = CASE WHEN $8 THEN $9::timestamptz ELSE idle_expires_at END,
                    absolute_expires_at = CASE WHEN $10 THEN $11::timestamptz ELSE absolute_expires_at END,
                    revoked_at = CASE WHEN $12 THEN $13::timestamptz ELSE revoked_at END
                WHERE session_id = $1
                RETURNING session_id, user_id, idp_id, token_hash, aal, auth_method, ip::text AS "ip?", user_agent, last_seen_at, idle_expires_at, absolute_expires_at, revoked_at, created_at, updated_at
            "#,
            id,
            set_ip, ip,
            set_user_agent, user_agent,
            set_last_seen_at, last_seen_at,
            set_idle_expires_at, idle_expires_at,
            set_absolute_expires_at, absolute_expires_at,
            set_revoked_at, revoked_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM SESSIONS
                WHERE session_id = $1
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
