use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::user::{CreateUser, UpdateUser},
    models::user::UserModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait UsersRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateUser) -> HuxleyStoreResult<UserModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<UserModel>>;
    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> HuxleyStoreResult<Option<UserModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<UserModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateUser) -> HuxleyStoreResult<UserModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgUsersRepository;

#[async_trait]
impl UsersRepository for PgUsersRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateUser) -> HuxleyStoreResult<UserModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<UserModel>> {

    }

    async fn find_by_email(&self, conn: &mut PgConnection, email: &str) -> HuxleyStoreResult<Option<UserModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<UserModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateUser) -> HuxleyStoreResult<UserModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
