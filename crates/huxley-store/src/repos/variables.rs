use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::variable::{CreateVariable, UpdateVariable},
    common::{Page, PageQuery, PageSort},
    models::variable::VariableModel,
};

#[async_trait]
pub trait VariablesRepository: Send + Sync {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateVariable,
    ) -> HuxleyStoreResult<VariableModel>;
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<VariableModel>>;
    async fn find_by_org_id_and_name(
        &self,
        conn: &mut PgConnection,
        org_id: Uuid,
        name: &str,
    ) -> HuxleyStoreResult<Option<VariableModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<VariableModel>>;
    async fn list_by_org_id(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<VariableModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateVariable,
    ) -> HuxleyStoreResult<Option<VariableModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgVariablesRepository;

#[async_trait]
impl VariablesRepository for PgVariablesRepository {
    async fn create(
        &self,
        conn: &mut PgConnection,
        input: CreateVariable,
    ) -> HuxleyStoreResult<VariableModel> {
        let result = sqlx::query_as!(
            VariableModel,
            r#"
                INSERT INTO variables (org_id, var_type, name, value, inheritable)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
            "#,
            input.org_id,
            input.var_type,
            input.name,
            input.value,
            input.inheritable,
        )
        .fetch_one(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<VariableModel>> {
        let result = sqlx::query_as!(
            VariableModel,
            r#"
                SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                FROM variables
                WHERE var_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_org_id_and_name(
        &self,
        conn: &mut PgConnection,
        org_id: Uuid,
        name: &str,
    ) -> HuxleyStoreResult<Option<VariableModel>> {
        let result = sqlx::query_as!(
            VariableModel,
            r#"
                SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                FROM variables
                WHERE org_id = $1 AND name = $2
            "#,
            org_id,
            name,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<VariableModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    VariableModel,
                    r#"
                        SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                        FROM variables
                        WHERE ($2::uuid IS NULL OR var_id >= $2)
                        ORDER BY var_id ASC
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
                    VariableModel,
                    r#"
                        SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                        FROM variables
                        WHERE ($2::uuid IS NULL OR var_id <= $2)
                        ORDER BY var_id DESC
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
        let items: Vec<VariableModel> = result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.user_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn list_by_org_id(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
        org_id: Uuid,
    ) -> HuxleyStoreResult<Page<VariableModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    VariableModel,
                    r#"
                        SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                        FROM variables
                        WHERE ($2::uuid IS NULL OR var_id >= $2) AND (org_id = $3)
                        ORDER BY var_id ASC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_id,
                )
                .fetch_all(conn)
                .await?
            },
            PageSort::Desc => {
                sqlx::query_as!(
                    VariableModel,
                    r#"
                        SELECT var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
                        FROM variables
                        WHERE ($2::uuid IS NULL OR var_id <= $2) AND (org_id = $3)
                        ORDER BY var_id DESC
                        LIMIT $1 + 1
                    "#,
                    resolved_limit,
                    page.next_cursor,
                    org_id,
                )
                .fetch_all(conn)
                .await?
            }
        };

        let has_more = result.len() as i32 > resolved_limit;
        let items: Vec<VariableModel> = result.into_iter().take(resolved_limit as usize).collect();
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
        input: UpdateVariable,
    ) -> HuxleyStoreResult<Option<VariableModel>> {
        let (set_name, name) = input.name.into_parts();
        let (set_value, value) = input.value.into_parts();
        let (set_inheritable, inheritable) = input.inheritable.into_parts();

        let result = sqlx::query_as!(
            VariableModel,
            r#"
                UPDATE variables
                SET name = CASE WHEN $2 THEN $3::text ELSE name END,
                    value = CASE WHEN $4 THEN $5::text ELSE value END,
                    inheritable = CASE WHEN $6 THEN $7::boolean ELSE inheritable END,
                WHERE var_id = $1
                RETURNING var_id, org_id, var_type, name, value, inheritable, created_at, updated_at
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
                DELETE FROM variables
                WHERE var_id = $1
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
