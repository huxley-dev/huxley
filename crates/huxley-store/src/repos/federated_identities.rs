use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::federated_identity::{CreateFederatedIdentity, UpdateFederatedIdentity},
    models::federated_identity::FederatedIdentityModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait FederatedIdentitiesRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FederatedIdentityModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<FederatedIdentityModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<FederatedIdentityModel>>;
    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<FederatedIdentityModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgFederatedIdentitiesRepository;

#[async_trait]
impl FederatedIdentitiesRepository for PgFederatedIdentitiesRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<FederatedIdentityModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<FederatedIdentityModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<FederatedIdentityModel>> {

    }

    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<FederatedIdentityModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateFederatedIdentity) -> HuxleyStoreResult<FederatedIdentityModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
