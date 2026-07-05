use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::app_setting::{CreateAppSetting, UpdateAppSetting},
    models::app_setting::AppSettingModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AppSettingsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppSetting) -> HuxleyStoreResult<AppSettingModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppSettingModel>>;
    async fn find_by_name(&self, conn: &mut PgConnection, name: String) -> HuxleyStoreResult<Option<AppSettingModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppSettingModel>>;
    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<AppSettingModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppSetting) -> HuxleyStoreResult<AppSettingModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAppSettingsRepository;

#[async_trait]
impl AppSettingsRepository for PgAppSettingsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAppSetting) -> HuxleyStoreResult<AppSettingModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AppSettingModel>> {

    }

    async fn find_by_name(&self, conn: &mut PgConnection, name: String) -> HuxleyStoreResult<Option<AppSettingModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AppSettingModel>> {

    }

    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<AppSettingModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAppSetting) -> HuxleyStoreResult<AppSettingModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
