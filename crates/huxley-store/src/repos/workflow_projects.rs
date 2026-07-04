use async_trait::async_trait;
use sqlx::{PgConnection, postgres::PgConnectOptions};
use uuid::Uuid;

use crate::{
  commands::workflow_project::{CreateWorkflowProject, UpdateWorkflowProject},
  models::workflow_project::WorkflowProjectModel,
  HuxleyStoreResult
};

#[async_trait]
pub trait WorkflowProjectsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WorkflowProjectModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<WorkflowProjectModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<WorkflowProjectModel>>;
    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<WorkflowProjectModel>>;
    async fn update(&self, connect: &mut PgConnection, id: Uuid, input: UpdateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel>;
    async fn delete(&self, connect: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgWorkflowProjectsRepository;

#[async_trait]
impl WorkflowProjectsRepository for PgWorkflowProjectsRepository {
    async fn create(&self, conn: &mut PgConnectOptions, input: CreateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<WorkflowProjectModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<WorkflowProjectModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<WorkflowProjectModel>> {

    }

    async fn list_by_org_id(&self, conn: &mut PgConnection, org_id: Uuid) -> HuxleyStoreResult<Vec<WorkflowProjectModel>> {

    }

    async fn update(&self, conn: &mut PgConnection, id: Uuid, input: UpdateWorkflowProject) -> HuxleyStoreResult<WorkflowProjectModel> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
