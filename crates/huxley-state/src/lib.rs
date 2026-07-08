use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use huxley_config::HuxleyConfig;
use huxley_store::repos::*;

pub struct HuxleyState {
    pub config: Arc<HuxleyConfig>,

    pub db_pool: PgPool,
    pub cache_pool: Pool,

    pub api_tokens_repo: Arc<dyn repos::ApiTokensRepository>,
    pub app_role_perms_repo: Arc<dyn repos::AppRolePermsRepository>,
    pub app_roles_repo: Arc<dyn repos::AppRolesRepository>,
    pub app_settings_repo: Arc<dyn repos::AppSettingsRepository>,
    pub audit_logs_repo: Arc<dyn repos::AuditLogsRepository>,
    pub auth_flows_repo: Arc<dyn repos::AuthFlowsRepository>,
    pub federated_identities_repo: Arc<dyn repos::FederatedIdentitiesRepository>,
    pub identity_providers_repo: Arc<dyn repos::IdentityProvidersRepository>,
    pub login_attempts_repo: Arc<dyn repos::LoginAttemptsRepository>,
    pub org_role_perms_repo: Arc<dyn repos::OrgRolePermsRepository>,
    pub org_roles_repo: Arc<dyn repos::OrgRolesRepository>,
    pub org_users_repo: Arc<dyn repos::OrgUsersRepository>,
    pub organizations_repo: Arc<dyn repos::OrganizationsRepository>,
    pub recovery_codes_repo: Arc<dyn repos::RecoveryCodesRepository>,
    pub sessions_repo: Arc<dyn repos::SessionsRepository>,
    pub tags_repo: Arc<dyn repos::TagsRepository>,
    pub totp_credentials_repo: Arc<dyn repos::TotpCredentialsRepository>,
    pub users_repo: Arc<dyn repos::UserRepository>,
    pub verification_tokens_repo: Arc<dyn repos::VerificationTokensRepository>,
    pub webauthn_credentials_repo: Arc<dyn repos::WebAuthnCredentialsRepository>,
    pub workflow_projects_repo: Arc<dyn repos::WorkflowProjectsRepository>,
}

impl HuxleyState {
    pub fn new(
        config: Arc<HuxleyConfig>,
        db_pool: PgPool,
        cache_pool: Pool,
    ) -> Self {
        Self {
            config: config,
            db_pool: db_pool,
            cache_pool: cache_pool,
            api_tokens_repo: Arc::new(repos::PgApiTokensRepository),
            app_role_perms_repo: Arc::new(repos::PgAppRolePermsRepository),
            app_roles_repo: Arc::new(repos::PgAppRolesRepository),
            app_settings_repo: Arc::new(repos::PgAppSettingsRepository),
            audit_logs_repo: Arc::new(repos::PgAuditLogsRepository),
            auth_flows_repo: Arc::new(repos::PgAuthFlowsRepository),
            federated_identities_repo: Arc::new(repos::PgFederatedIdentitiesRepository),
            identity_providers_repo: Arc::new(repos::PgIdentityProvidersRepository),
            login_attempts_repo: Arc::new(repos::PgLoginAttemptsRepository),
            org_role_perms_repo: Arc::new(repos::PgOrgRolePermsRepository),
            org_roles_repo: Arc::new(repos::PgOrgRolesRepository),
            org_users_repo: Arc::new(repos::PgOrgUsersRepository),
            organizations_repo: Arc::new(repos::PgOrganizationsRepository),
            recovery_codes_repo: Arc::new(repos::PgRecoveryCodesRepository),
            sessions_repo: Arc::new(repos::PgSessionsRepository),
            tags_repo: Arc::new(repos::PgTagsRepository),
            totp_credentials_repo: Arc::new(repos::PgTotpCredentialsRepository),
            users_repo: Arc::new(repos::PgUserRepository),
            verification_tokens_repo: Arc::new(repos::PgVerificationTokensRepository),
            webauthn_credentials_repo: Arc::new(repos::PgWebAuthnCredentialsRepository),
            workflow_projects_repo: Arc::new(repos::PgWorkflowProjectsRepo),
        }
    }
}
