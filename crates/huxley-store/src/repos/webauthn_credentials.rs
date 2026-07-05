use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::webauthn_credential::{CreateWebAuthnCredential, UpdateWebAuthnCredential},
    models::webauthn_credential::WebAuthnCredentialModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait WebAuthnCredentialsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateWebAuthnCredential) -> HuxleyStoreResult<WebAuthnCredentialModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WebAuthnCredentialModel>>;
    async fn find_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Option<WebAuthnCredentialModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<WebAuthnCredentialModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateWebAuthnCredential) -> HuxleyStoreResult<WebAuthnCredentialModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgWebAuthnCredentialsRepository;

#[async_trait]
impl WebAuthnCredentialsRepository for PgWebAuthnCredentialsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateWebAuthnCredential) -> HuxleyStoreResult<WebAuthnCredentialModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WebAuthnCredentialModel>> {

    }

    async fn find_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Option<WebAuthnCredentialModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<WebAuthnCredentialModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateWebAuthnCredential) -> HuxleyStoreResult<WebAuthnCredentialModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
