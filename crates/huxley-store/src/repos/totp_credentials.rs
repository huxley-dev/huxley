use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::totp_credential::{CreateTotpCredential, UpdateTotpCredential},
    common::{Page, PageQuery, PageSort},
    models::totp_credential::TotpCredentialPublicModel,
};

#[async_trait]
pub trait TotpCredentialsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateTotpCredential,
    ) -> HuxleyStoreResult<TotpCredentialPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>>;
    async fn find_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TotpCredentialPublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateTotpCredential,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgTotpCredentialsRepository;

#[async_trait]
impl TotpCredentialsRepository for PgTotpCredentialsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateTotpCredential,
    ) -> HuxleyStoreResult<TotpCredentialPublicModel> {
        let result = sqlx::query_as!(
            TotpCredentialPublicModel,
            r#"
                INSERT INTO totp_credentials (user_id, secret_enc, confirmed_at)
                VALUES ($1, $2, $3)
                RETURNING totp_cred_id, user_id, confirmed_at, created_at, updated_at
            "#,
            input.user_id,
            input.secret_enc,
            input.confirmed_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>> {
        let result = sqlx::query_as!(
            TotpCredentialPublicModel,
            r#"
                SELECT totp_cred_id, user_id, confirmed_at, created_at, updated_at
                FROM totp_credentials
                WHERE totp_cred_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>> {
        let result = sqlx::query_as!(
            TotpCredentialPublicModel,
            r#"
                SELECT totp_cred_id, user_id, confirmed_at, created_at, updated_at
                FROM totp_credentials
                WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<TotpCredentialPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    TotpCredentialPublicModel,
                    r#"
                        SELECT totp_cred_id, user_id, confirmed_at, created_at, updated_at
                        FROM totp_credentials
                        WHERE ($2::uuid IS NULL OR totp_cred_id >= $2)
                        ORDER BY totp_cred_id ASC
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
                    TotpCredentialPublicModel,
                    r#"
                        SELECT totp_cred_id, user_id, confirmed_at, created_at, updated_at
                        FROM totp_credentials
                        WHERE ($2::uuid IS NULL OR totp_cred_id <= $2)
                        ORDER BY totp_cred_id DESC
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
        let items: Vec<TotpCredentialPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.totp_cred_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateTotpCredential,
    ) -> HuxleyStoreResult<Option<TotpCredentialPublicModel>> {
        let (set_confirmed_at, confirmed_at) = input.confirmed_at.into_parts();

        let result = sqlx::query_as!(
            TotpCredentialPublicModel,
            r#"
                UPDATE totp_credentials
                SET confirmed_at = CASE WHEN $2 THEN $3::timestamptz ELSE confirmed_at END
                WHERE totp_cred_id = $1
                RETURNING totp_cred_id, user_id, confirmed_at, created_at, updated_at
            "#,
            id,
            set_confirmed_at,
            confirmed_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM totp_credentials
                WHERE totp_cred_id = $1
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
