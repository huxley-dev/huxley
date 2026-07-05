use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::federated_identity::{CreateFederatedIdentity, UpdateFederatedIdentity},
    models::federated_identity::FederatedIdentityModel,
    common::{Page, PageQuery, PageSort},
    HuxleyStoreResult,
};

#[async_trait]
pub trait FederatedIdentitiesRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FederatedIdentityModel>>;
    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>>;
    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgFederatedIdentitiesRepository;

#[async_trait]
impl FederatedIdentitiesRepository for PgFederatedIdentitiesRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel> {
        let result = sqlx::query_as!(
            FederatedIdentityModel,
            r#"
                INSERT INTO federated_identities (user_id, idp_id, subject, email_at_idp, last_login_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
            "#,
            input.user_id,
            input.idp_id,
            input.subject,
            input.email_at_idp,
            input.last_login_at,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FederatedIdentityModel>> {
        let result = sqlx::query_as!(
            FederatedIdentityModel,
            r#"
                SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                FROM federated_identities
                WHERE fedid_id = $1
            "#,
            id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(&self, conn: &mut PgConnection, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id >= $2)
                        ORDER BY fedid_id ASC
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
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id <= $2)
                        ORDER BY fedid_id DESC
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
        let items: Vec<FederatedIdentityModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.fedid_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id >= $2) AND (user_id = $3)
                        ORDER BY fedid_id ASC
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
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id <= $2) AND (user_id = $3)
                        ORDER BY fedid_id DESC
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
        let items: Vec<FederatedIdentityModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.fedid_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid, page: PageQuery) -> HuxleyStoreResult<Page<FederatedIdentityModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id >= $2) AND (fedid_id = $3)
                        ORDER BY fedid_id ASC
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
                    FederatedIdentityModel,
                    r#"
                        SELECT fedid_id, user_id, idp_id, subject, email_at_idp, last_login_at, created_at, updated_at
                        FROM federated_identities
                        WHERE ($2::bigint IS NULL OR fedid_id <= $2) AND (fedid_id = $3)
                        ORDER BY fedid_id DESC
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
        let items: Vec<FederatedIdentityModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.fedid_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel> {
        let (set_user_id, user_id) = input.user_id.into_parts();
        let (set_idp_id, idp_id) = input.idp_id.into_parts();
        let (set_subject, subject) = input.subject.into_parts();
        let (set_email_at_idp, email_at_idp) = input.email_at_idp.into_parts();
        let (set_last_login_at, last_login_at) = input.last_login_at.into_parts();

        let result = sqlx::query_as!(
            FederatedIdentityModel,
            r#"
                UPDATE federated_identities
                SET user_id = CASE WHEN $2 THEN $3::text ELSE user_id END,
                    idp_id = CASE WHEN $4 THEN $5::text ELSE idp_id END,
                    subject = CASE WHEN $6 THEN $7::text[] ELSE subject END,
                    email_at_idp = CASE WHEN $8 THEN $9::timestamptz ELSE email_at_idp END,
                    last_login_at = CASE WHEN $10 THEN $11::timestamptz ELSE last_login_at END,
                WHERE fedid_id = $1
            "#,
            id,
            set_user_id, user_id,
            set_idp_id, idp_id,
            set_subject, subject,
            set_email_at_idp, email_at_idp,
            set_last_login_at, last_login_at,
        )
        .execute(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE from federated_identities
                WHERE fedid_id = $1
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
