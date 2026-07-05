use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::login_attempt::CreateLoginAttempt,
    models::login_attempt::LoginAttemptModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait LoginAttemptsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateLoginAttempt) -> HuxleyStoreResult<LoginAttemptModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<LoginAttemptModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<LoginAttemptModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<LoginAttemptModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgLoginAttemptsRepository;

#[async_trait]
impl LoginAttemptsRepository for PgLoginAttemptsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateLoginAttempt) -> HuxleyStoreResult<LoginAttemptModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<LoginAttemptModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<LoginAttemptModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<LoginAttemptModel>> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
