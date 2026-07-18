use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    HuxleyStoreResult,
    commands::app_setting::UpdateAppSetting,
    common::{Page, PageQuery, PageSort},
    models::app_setting::AppSettingPublicModel,
};

#[async_trait]
pub trait AppSettingsRepository: Send + Sync {
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>>;
    async fn find_by_name(
        &self,
        conn: &mut PgConnection,
        name: &str,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>>;
    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppSettingPublicModel>>;
    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAppSetting,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>>;
}

pub struct PgAppSettingsRepository;

#[async_trait]
impl AppSettingsRepository for PgAppSettingsRepository {
    async fn find_by_id(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>> {
        let result = sqlx::query_as!(
            AppSettingPublicModel,
            r#"
                SELECT app_set_id, name, value, created_at, updated_at
                FROM app_settings
                WHERE app_set_id = $1
            "#,
            id
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn find_by_name(
        &self,
        conn: &mut PgConnection,
        name: &str,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>> {
        let result = sqlx::query_as!(
            AppSettingPublicModel,
            r#"
                SELECT app_set_id, name, value, created_at, updated_at
                FROM app_settings
                WHERE name = $1
            "#,
            name
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }

    async fn list(
        &self,
        conn: &mut PgConnection,
        page: PageQuery,
    ) -> HuxleyStoreResult<Page<AppSettingPublicModel>> {
        let resolved_limit = page.resolved_limit();

        let result = match page.resolved_sort() {
            PageSort::Asc => {
                sqlx::query_as!(
                    AppSettingPublicModel,
                    r#"
                        SELECT app_set_id, name, value, created_at, updated_at
                        FROM app_settings
                        WHERE ($2::uuid IS NULL OR app_set_id >= $2)
                        ORDER BY app_set_id ASC
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
                    AppSettingPublicModel,
                    r#"
                        SELECT app_set_id, name, value, created_at, updated_at
                        FROM app_settings
                        WHERE ($2::uuid IS NULL OR app_set_id <= $2)
                        ORDER BY app_set_id DESC
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
        let items: Vec<AppSettingPublicModel> =
            result.into_iter().take(resolved_limit as usize).collect();
        let next_cursor = if has_more {
            items.last().map(|i| i.app_set_id)
        } else {
            None
        };

        Ok(Page { items, next_cursor })
    }

    async fn update(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
        input: UpdateAppSetting,
    ) -> HuxleyStoreResult<Option<AppSettingPublicModel>> {
        let (set_value, value) = input.value.into_parts();

        let result = sqlx::query_as!(
            AppSettingPublicModel,
            r#"
                UPDATE app_settings
                SET value = CASE WHEN $2 THEN $3::text ELSE value END
                WHERE app_set_id = $1
                RETURNING app_set_id, name, value, created_at, updated_at
            "#,
            id,
            set_value,
            value,
        )
        .fetch_optional(conn)
        .await?;

        Ok(result)
    }
}
