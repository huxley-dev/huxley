use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::recovery_code::{CreateRecoveryCode, UpdateRecoveryCode},
    models::recovery_code::RecoveryCodeModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait RecoveryCodesRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateRecoveryCode) -> HuxleyStoreResult<RecoveryCodeModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<RecoveryCodeModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<RecoveryCodeModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<RecoveryCodeModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateRecoveryCode) -> HuxleyStoreResult<RecoveryCodeModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgRecoveryCodesRepository;

#[async_trait]
impl RecoveryCodesRepository for PgRecoveryCodesRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateRecoveryCode) -> HuxleyStoreResult<RecoveryCodeModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<RecoveryCodeModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<RecoveryCodeModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<RecoveryCodeModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateRecoveryCode) -> HuxleyStoreResult<RecoveryCodeModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
