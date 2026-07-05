use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::session::{CreateSession, UpdateSession},
    models::session::SessionModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait SessionsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateSession) -> HuxleyStoreResult<SessionModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<SessionModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<SessionModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<SessionModel>>;
    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<SessionModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateSession) -> HuxleyStoreResult<SessionModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgSessionsRepository;

#[async_trait]
impl SessionsRepository for PgSessionsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateSession) -> HuxleyStoreResult<SessionModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<SessionModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<SessionModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<SessionModel>> {

    }

    async fn list_by_idp_id(&self, conn: &mut PgConnection, idp_id: Uuid) -> HuxleyStoreResult<Vec<SessionModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateSession) -> HuxleyStoreResult<SessionModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
