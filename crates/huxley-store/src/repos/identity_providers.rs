use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::identity_provider::{CreateIdentityProvider, UpdateIdentityProvider},
    models::identity_provider::IdentityProviderModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait IdentityProvidersRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateIdentityProvider) -> HuxleyStoreResult<IdentityProviderModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<IdentityProviderModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<IdentityProviderModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateIdentityProvider) -> HuxleyStoreResult<IdentityProviderModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgIdentityProvidersRepository;

#[async_trait]
impl IdentityProvidersRepository for PgIdentityProvidersRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateIdentityProvider) -> HuxleyStoreResult<IdentityProviderModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<IdentityProviderModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<IdentityProviderModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateIdentityProvider) -> HuxleyStoreResult<IdentityProviderModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
