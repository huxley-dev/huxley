use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::auth_flow::{CreateAuthFlow, UpdateAuthFlow},
    common::{Page, PageQuery, PageSort},
    models::auth_flow::AuthFlowPublicModel,
};

#[async_trait]
pub trait AuthFlowsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAuthFlow,
    ) -> HuxleyStoreResult<AuthFlowPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AuthFlowPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AuthFlowPublicModel>>;
    async fn list_by_idp_id(
        &self,
        conn: &mut PgConnection,
        idp_id: Uuid,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AuthFlowPublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAuthFlow,
    ) -> HuxleyStoreResult<Option<AuthFlowPublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAuthFlowsRepository;

#[async_trait]
impl AuthFlowsRepository for PgAuthFlowsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateAuthFlow,
    ) -> HuxleyStoreResult<AuthFlowPublicModel> {
        let result = sqlx::query_as!(
            AuthFlowPublicModel,
            r#"
                INSERT INTO auth_flows (idp_id, state, pkce_verifier, nonce, relate_state, redirect_to, expires_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
            "#,
            input.idp_id,
            input.state,
            input.pkce_verifier,
            input.nonce,
            input.relate_state,
            input.redirect_to,
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
    ) -> HuxleyStoreResult<Option<AuthFlowPublicModel>> {
        let result = sqlx::query_as!(
            AuthFlowPublicModel,
            r#"
                SELECT auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
                FROM auth_flows
                WHERE auth_flow_id = $1
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
    ) -> HuxleyStoreResult<Page<AuthFlowPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AuthFlowPublicModel,
                    r#"
                        SELECT auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
                        FROM auth_flows
                        WHERE ($2::uuid IS NULL OR auth_flow_id >= $2)
                        ORDER BY auth_flow_id ASC
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
                    AuthFlowPublicModel,
                    r#"
                        SELECT auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
                        FROM auth_flows
                        WHERE ($2::uuid IS NULL OR auth_flow_id <= $2)
                        ORDER BY auth_flow_id DESC
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
        let items: Vec<AuthFlowPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.auth_flow_id)
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
    ) -> HuxleyStoreResult<Page<AuthFlowPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AuthFlowPublicModel,
                    r#"
                        SELECT auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
                        FROM auth_flows
                        WHERE ($2::uuid IS NULL OR auth_flow_id >= $2) AND (idp_id = $3)
                        ORDER BY auth_flow_id ASC
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
                    AuthFlowPublicModel,
                    r#"
                        SELECT auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
                        FROM auth_flows
                        WHERE ($2::uuid IS NULL OR auth_flow_id <= $2) AND (idp_id = $3)
                        ORDER BY auth_flow_id DESC
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
        let items: Vec<AuthFlowPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.auth_flow_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAuthFlow,
    ) -> HuxleyStoreResult<Option<AuthFlowPublicModel>> {
        let (set_idp_id, idp_id) = input.idp_id.into_parts();
        let (set_state, state) = input.state.into_parts();
        let (set_pkce_verifier, pkce_verifier) = input.pkce_verifier.into_parts();
        let (set_nonce, nonce) = input.nonce.into_parts();
        let (set_relate_state, relate_state) = input.relate_state.into_parts();
        let (set_redirect_to, redirect_to) = input.redirect_to.into_parts();
        let (set_expires_at, expires_at) = input.expires_at.into_parts();

        let result = sqlx::query_as!(
            AuthFlowPublicModel,
            r#"
                UPDATE auth_flows
                SET idp_id = CASE WHEN $2 THEN $3::uuid ELSE idp_id END,
                    state = CASE WHEN $4 THEN $5::text ELSE state END,
                    pkce_verifier = CASE WHEN $6 THEN $7::text ELSE pkce_verifier END,
                    nonce = CASE WHEN $8 THEN $9::text ELSE nonce END,
                    relate_state = CASE WHEN $10 THEN $11::text ELSE relate_state END,
                    redirect_to = CASE WHEN $12 THEN $13::text ELSE redirect_to END,
                    expires_at = CASE WHEN $14 THEN $15::timestamptz ELSE expires_at END
                WHERE auth_flow_id = $1
                RETURNING auth_flow_id, idp_id, state, relate_state, redirect_to, expires_at, created_at, updated_at
            "#,
            id,
            set_idp_id,
            idp_id,
            set_state,
            state,
            set_pkce_verifier,
            pkce_verifier,
            set_nonce,
            nonce,
            set_relate_state,
            relate_state,
            set_redirect_to,
            redirect_to,
            set_expires_at,
            expires_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from auth_flows
                WHERE auth_flow_id = $1
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
