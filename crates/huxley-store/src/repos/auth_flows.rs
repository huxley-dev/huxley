use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::auth_flow::{CreateAuthFlow, UpdateAuthFlow},
    models::auth_flow::AuthFlowModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AuthFlowsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuthFlow) -> HuxleyStoreResult<AuthFlowModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuthFlowModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AuthFlowModel>>;
    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<AuthFlowModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAuthFlow) -> HuxleyStoreResult<AuthFlowModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAuthFlowsRepository;

#[async_trait]
impl AuthFlowsRepository for PgAuthFlowsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuthFlow) -> HuxleyStoreResult<AuthFlowModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuthFlowModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AuthFlowModel>> {

    }

    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<AuthFlowModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateAuthFlow) -> HuxleyStoreResult<AuthFlowModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
