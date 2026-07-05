use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::api_token::{CreateApiToken, UpdateApiToken},
    models::api_token::ApiTokenModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait ApiTokensRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateApiToken) -> HuxleyStoreResult<ApiTokenModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<ApiTokenModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<ApiTokenModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<ApiTokenModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateApiToken) -> HuxleyStoreResult<ApiTokenModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgApiTokensRepository;

#[async_trait]
impl ApiTokensRepository for PgApiTokensRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateApiToken) -> HuxleyStoreResult<ApiTokenModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<ApiTokenModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<ApiTokenModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<ApiTokenModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateApiToken) -> HuxleyStoreResult<ApiTokenModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
