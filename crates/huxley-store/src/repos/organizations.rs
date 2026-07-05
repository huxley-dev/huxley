use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::organization::{CreateOrganization, UpdateOrganization},
    models::organization::OrganizationModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait OrganizationsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrganization) -> HuxleyStoreResult<OrganizationModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrganizationModel>>;
    async fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> HuxleyStoreResult<Option<OrganizationModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrganizationModel>>;
    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrganization) -> HuxleyStoreResult<OrganizationModel>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgOrganizationsRepository;

#[async_trait]
impl OrganizationsRepository for PgOrganizationsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateOrganization) -> HuxleyStoreResult<OrganizationModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<OrganizationModel>> {

    }

    async fn find_by_name(&self, conn: &mut PgConnection, name: &str) -> HuxleyStoreResult<Option<OrganizationModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<OrganizationModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateOrganization) -> HuxleyStoreResult<OrganizationModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
