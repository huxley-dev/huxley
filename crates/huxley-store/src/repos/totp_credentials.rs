use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::totp_credential::{CreateTotpCredential, UpdateTotpCredential},
    models::totp_credential::TotpCredentialModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait TotpCredentialsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateTotpCredential) -> HuxleyStoreResult<TotpCredentialModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<TotpCredentialModel>>;
    async fn find_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Option<TotpCredentialModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<TotpCredentialModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateTotpCredential) -> HuxleyStoreResult<TotpCredentialModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgTotpCredentialsRepository;

#[async_trait]
impl TotpCredentialsRepository for PgTotpCredentialsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateTotpCredential) -> HuxleyStoreResult<TotpCredentialModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<TotpCredentialModel>> {

    }

    async fn find_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Option<TotpCredentialModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<TotpCredentialModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateTotpCredential) -> HuxleyStoreResult<TotpCredentialModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
