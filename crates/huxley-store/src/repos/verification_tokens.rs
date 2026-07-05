use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::verification_token::{CreateVerificationToken, UpdateVerificationToken},
    models::verification_token::VerificationTokenModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait VerificationTokensRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateVerificationToken) -> HuxleyStoreResult<VerificationTokenModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<VerificationTokenModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<VerificationTokenModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<VerificationTokenModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: VerificationTokenModel) -> HuxleyStoreResult<VerificationTokenModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgVerificationTokensRepository;

#[async_trait]
impl VerificationTokensRepository for PgVerificationTokensRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateVerificationToken) -> HuxleyStoreResult<VerificationTokenModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<VerificationTokenModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<VerificationTokenModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<VerificationTokenModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateVerificationToken) -> HuxleyStoreResult<VerificationTokenModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
