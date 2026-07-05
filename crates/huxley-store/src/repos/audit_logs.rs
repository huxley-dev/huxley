use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    commands::audit_log::CreateAuditLog,
    models::audit_log::AuditLogModel,
    HuxleyStoreResult,
};

#[async_trait]
pub trait AuditLogsRepository: Send + Sync {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuditLog) -> HuxleyStoreResult<AuditLogModel>;
    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuditLogModel>>;
    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AuditLogModel>>;
    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<AuditLogModel>>;
    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool>;
}

pub struct PgAuditLogsRepository;

#[async_trait]
impl AuditLogsRepository for PgAuditLogsRepository {
    async fn create(&self, conn: &mut PgConnection, input: CreateAuditLog) -> HuxleyStoreResult<AuditLogModel> {

    }

    async fn find_by_id(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<Option<AuditLogModel>> {

    }

    async fn list(&self, conn: &mut PgConnection) -> HuxleyStoreResult<Vec<AuditLogModel>> {

    }

    async fn list_by_user_id(&self, conn: &mut PgConnection, user_id: Uuid) -> HuxleyStoreResult<Vec<AuditLogModel>> {

    }

    async fn delete(&self, conn: &mut PgConnection, id: Uuid) -> HuxleyStoreResult<bool> {

    }
}
