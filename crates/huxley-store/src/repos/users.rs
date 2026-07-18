use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::user::{CreateUser, UpdateUser},
    common::{Page, PageQuery, PageSort},
    models::user::UserPublicModel,
};

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateUser,
    ) -> HuxleyStoreResult<UserPublicModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<UserPublicModel>>;
    async fn find_by_email(
        &self,
        conn: &mut PgConnection,
        email: &str,
    ) -> HuxleyStoreResult<Option<UserPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<UserPublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateUser,
    ) -> HuxleyStoreResult<Option<UserPublicModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgUsersRepository;

#[async_trait]
impl UsersRepository for PgUsersRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateUser,
    ) -> HuxleyStoreResult<UserPublicModel> {
        let result = sqlx::query_as!(
            UserPublicModel,
            r#"
                INSERT INTO users (name, email, email_verified, password_hash, status, preferences, app_role_id)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
            "#,
            input.name,
            input.email,
            input.email_verified,
            input.password_hash,
            input.status,
            input.preferences,
            input.app_role_id,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<UserPublicModel>> {
        let result = sqlx::query_as!(
            UserPublicModel,
            r#"
                SELECT user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
                FROM users
                WHERE user_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_email(
        &self,
        conn: &mut PgConnection,
        email: &str,
    ) -> HuxleyStoreResult<Option<UserPublicModel>> {
        let result = sqlx::query_as!(
            UserPublicModel,
            r#"
                SELECT user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
                FROM users
                WHERE email = $1
            "#,
            email
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<UserPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    UserPublicModel,
                    r#"
                        SELECT user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
                        FROM users
                        WHERE ($2::uuid IS NULL OR user_id >= $2)
                        ORDER BY user_id ASC
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
                    UserPublicModel,
                    r#"
                        SELECT user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
                        FROM users
                        WHERE ($2::uuid IS NULL OR user_id <= $2)
                        ORDER BY user_id DESC
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
        let items: Vec<UserPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateUser,
    ) -> HuxleyStoreResult<Option<UserPublicModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_email, email) = input.email.into_parts();
        let (set_email_verified, email_verified) = input.email_verified.into_parts();
        let (set_password_hash, password_hash) = input.password_hash.into_parts();
        let (set_status, status) = input.status.into_parts();
        let (set_preferences, preferences) = input.preferences.into_parts();
        let (set_app_role_id, app_role_id) = input.app_role_id.into_parts();

        let result = sqlx::query_as!(
            UserPublicModel,
            r#"
                UPDATE users
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    email = CASE WHEN $4 THEN $5::text ELSE email END,
                    email_verified = CASE WHEN $6 THEN $7::boolean ELSE email_verified END,
                    password_hash = CASE WHEN $8 THEN $9::text ELSE password_hash END,
                    status = CASE WHEN $10 THEN $11::text ELSE status END,
                    preferences = CASE WHEN $12 THEN $13::jsonb ELSE preferences END,
                    app_role_id = CASE WHEN $14 THEN $15::uuid ELSE app_role_id END
                WHERE user_id = $1
                RETURNING user_id, name, email, email_verified, status, preferences, app_role_id, created_at, updated_at
            "#,
            id,
            set_name,
            name,
            set_email,
            email,
            set_email_verified,
            email_verified,
            set_password_hash,
            password_hash,
            set_status,
            status,
            set_preferences,
            preferences,
            set_app_role_id,
            app_role_id,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {
        let result = sqlx::query!(
            r#"
                DELETE FROM users
                WHERE user_id = $1
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
