use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::identity_provider::{CreateIdentityProvider, UpdateIdentityProvider},
    common::{Page, PageQuery, PageSort},
    models::identity_provider::IdentityProviderModel,
};

#[async_trait]
pub trait IdentityProvidersRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateIdentityProvider,
    ) -> HuxleyStoreResult<IdentityProviderModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<IdentityProviderModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<IdentityProviderModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateIdentityProvider,
    ) -> HuxleyStoreResult<Option<IdentityProviderModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgIdentityProvidersRepository;

#[async_trait]
impl IdentityProvidersRepository for PgIdentityProvidersRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateIdentityProvider,
    ) -> HuxleyStoreResult<IdentityProviderModel> {
        let result = sqlx::query_as!(
            IdentityProviderModel,
            r#"
                INSERT INTO identity_providers (kind, name, slug, enabled, config, secret_enc)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING idp_id, kind, name, slug, enabled, config, secret_enc, created_at, updated_at
            "#,
            input.kind,
            input.name,
            input.slug,
            input.enabled,
            input.config,
            &input.secret_enc,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<IdentityProviderModel>> {
        let result = sqlx::query_as!(
            IdentityProviderModel,
            r#"
                SELECT idp_id, kind, name, slug, enabled, config, created_at, updated_at
                FROM identity_providers
                WHERE idp_id = $1
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
    ) -> HuxleyStoreResult<Page<IdentityProviderModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    IdentityProviderModel,
                    r#"
                        SELECT idp_id, kind, name, slug, enabled, config, created_at, updated_at
                        FROM identity_providers
                        WHERE ($2::bigint IS NULL OR idp_id >= $2)
                        ORDER BY idp_id ASC
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
                    IdentityProviderModel,
                    r#"
                        SELECT idp_id, kind, name, slug, enabled, config, created_at, updated_at
                        FROM identity_provider
                        WHERE ($2::bigint IS NULL OR idp_id <= $2)
                        ORDER BY idp_id DESC
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
        let items: Vec<IdentityProviderModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.idp_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateIdentityProvider,
    ) -> HuxleyStoreResult<IdentityProviderModel> {
        let (set_kind, kind) = input.kind.into_parts();
        let (set_name, name) = input.name.into_parts();
        let (set_slug, slug) = input.slug.into_parts();
        let (set_enabled, enabled) = input.enabled.into_parts();
        let (set_config, config) = input.config.into_parts();
        let (set_secret_enc, secret_enc) = input.secret_enc.into_parts();

        let result = sqlx::query_as!(
            IdentityProviderModel,
            r#"
                UPDATE identity_providers
                SET kind = CASE WHEN $2 THEN $3::text ELSE kind END,
                    name = CASE WHEN $4 THEN $5::text ELSE name END,
                    slug = CASE WHEN $6 THEN $7::text ELSE slug END,
                    enabled = CASE WHEN $8 THEN $9::boolean ELSE enabled END,
                    config = CASE WHEN $10 THEN $11::jsonb ELSE config END,
                    secret_enc = CASE WHEN $12 THEN $13::bytea ELSE secret_enc END
                WHERE idp_id = $1
            "#,
            id,
            set_kind,
            kind,
            set_name,
            name,
            set_slug,
            slug,
            set_enabled,
            enabled,
            set_config,
            config,
            set_secret_enc,
            secret_enc,
        )
        .execute(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from identity_providers
                WHERE idp_id = $1
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
