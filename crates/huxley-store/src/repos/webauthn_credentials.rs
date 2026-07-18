use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::webauthn_credential::{CreateWebAuthnCredential, UpdateWebAuthnCredential},
    common::{Page, PageQuery, PageSort},
    models::webauthn_credential::WebAuthnCredentialPublicModel,
};

#[async_trait]
pub trait WebAuthnCredentialsRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateWebAuthnCredential,
    ) -> HuxleyStoreResult<WebAuthnCredentialPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>>;
    async fn find_by_user_id(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        paeg: PageQuery,
    ) -> HuxleyStoreResult<Page<WebAuthnCredentialPublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateWebAuthnCredential,
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgWebAuthnCredentialsRepository;

#[async_trait]
impl WebAuthnCredentialsRepository for PgWebAuthnCredentialsRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateWebAuthnCredential,
    ) -> HuxleyStoreResult<WebAuthnCredentialPublicModel> {
        let result = sqlx::query_as!(
            WebAuthnCredentialPublicModel,
            r#"
                INSERT INTO webauthn_credentials (user_id, name, credential, public_key, sign_count, aaguid, transports, last_used_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
            "#,
            input.user_id,
            input.name,
            input.credential,
            input.public_key,
            input.sign_count,
            input.aaguid,
            &input.transports,
            input.last_used_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>> {
        let result = sqlx::query_as!(
            WebAuthnCredentialPublicModel,
            r#"
                SELECT wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
                FROM webauthn_credentials
                WHERE wauthn_cred_id = $1
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
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>> {
        let result = sqlx::query_as!(
            WebAuthnCredentialPublicModel,
            r#"
                SELECT wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
                FROM webauthn_credentials
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
    ) -> HuxleyStoreResult<Page<WebAuthnCredentialPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    WebAuthnCredentialPublicModel,
                    r#"
                        SELECT wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
                        FROM webauthn_credentials
                        WHERE ($2::uuid IS NULL OR wauthn_cred_id >= $2)
                        ORDER BY wauthn_cred_id ASC
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
                    WebAuthnCredentialPublicModel,
                    r#"
                        SELECT wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
                        FROM webauthn_credentials
                        WHERE ($2::uuid IS NULL OR wauthn_cred_id <= $2)
                        ORDER BY wauthn_cred_id DESC
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
        let items: Vec<WebAuthnCredentialPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.wauthn_cred_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateWebAuthnCredential,
    ) -> HuxleyStoreResult<Option<WebAuthnCredentialPublicModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_last_used_at, last_used_at) = input.last_used_at.into_parts();

        let result = sqlx::query_as!(
            WebAuthnCredentialPublicModel,
            r#"
                UPDATE webauthn_credentials
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    last_used_at = CASE WHEN $4 THEN $5::timestamptz ELSE last_used_at END
                WHERE wauthn_cred_id = $1
                RETURNING wauthn_cred_id, user_id, name, last_used_at, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_last_used_at,
            last_used_at,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM webauthn_credentials
                WHERE wauthn_cred_id = $1
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
