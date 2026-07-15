use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use huxley_config::HuxleyConfig;
use huxley_store::repos;

pub struct HuxleyState {
    pub config: Arc<HuxleyConfig>,

    pub db_pool: PgPool,
    pub cache_pool: Pool,

    pub api_tokens_repo: Arc<dyn repos::api_tokens::ApiTokensRepository>,
    pub app_role_perms_repo: Arc<dyn repos::app_role_perms::AppRolePermsRepository>,
    pub app_roles_repo: Arc<dyn repos::app_roles::AppRolesRepository>,
    pub app_settings_repo: Arc<dyn repos::app_settings::AppSettingsRepository>,
    pub audit_logs_repo: Arc<dyn repos::audit_logs::AuditLogsRepository>,
    pub auth_flows_repo: Arc<dyn repos::auth_flows::AuthFlowsRepository>,
    pub federated_identities_repo:
        Arc<dyn repos::federated_identities::FederatedIdentitiesRepository>,
    pub folders_repo: Arc<dyn repos::folders::FoldersRepository>,
    pub identity_providers_repo: Arc<dyn repos::identity_providers::IdentityProvidersRepository>,
    pub login_attempts_repo: Arc<dyn repos::login_attempts::LoginAttemptsRepository>,
    pub org_role_perms_repo: Arc<dyn repos::org_role_perms::OrgRolePermsRepository>,
    pub org_roles_repo: Arc<dyn repos::org_roles::OrgRolesRepository>,
    pub org_users_repo: Arc<dyn repos::org_users::OrgUsersRepository>,
    pub organizations_repo: Arc<dyn repos::organizations::OrganizationsRepository>,
    pub projects_repo: Arc<dyn repos::projects::ProjectsRepository>,
    pub recovery_codes_repo: Arc<dyn repos::recovery_codes::RecoveryCodesRepository>,
    pub sessions_repo: Arc<dyn repos::sessions::SessionsRepository>,
    pub tags_repo: Arc<dyn repos::tags::TagsRepository>,
    pub totp_credentials_repo: Arc<dyn repos::totp_credentials::TotpCredentialsRepository>,
    pub users_repo: Arc<dyn repos::users::UsersRepository>,
    pub verification_tokens_repo: Arc<dyn repos::verification_tokens::VerificationTokensRepository>,
    pub webauthn_credentials_repo:
        Arc<dyn repos::webauthn_credentials::WebAuthnCredentialsRepository>,
}

impl HuxleyState {
    pub fn new(config: Arc<HuxleyConfig>, db_pool: PgPool, cache_pool: Pool) -> Self {
        Self {
            config,
            db_pool,
            cache_pool,
            api_tokens_repo: Arc::new(repos::api_tokens::PgApiTokensRepository),
            app_role_perms_repo: Arc::new(repos::app_role_perms::PgAppRolePermsRepository),
            app_roles_repo: Arc::new(repos::app_roles::PgAppRolesRepository),
            app_settings_repo: Arc::new(repos::app_settings::PgAppSettingsRepository),
            audit_logs_repo: Arc::new(repos::audit_logs::PgAuditLogsRepository),
            auth_flows_repo: Arc::new(repos::auth_flows::PgAuthFlowsRepository),
            federated_identities_repo: Arc::new(
                repos::federated_identities::PgFederatedIdentitiesRepository,
            ),
            folders_repo: Arc::new(repos::folders::PgFoldersRepository),
            identity_providers_repo: Arc::new(
                repos::identity_providers::PgIdentityProvidersRepository,
            ),
            login_attempts_repo: Arc::new(repos::login_attempts::PgLoginAttemptsRepository),
            org_role_perms_repo: Arc::new(repos::org_role_perms::PgOrgRolePermsRepository),
            org_roles_repo: Arc::new(repos::org_roles::PgOrgRolesRepository),
            org_users_repo: Arc::new(repos::org_users::PgOrgUsersRepository),
            organizations_repo: Arc::new(repos::organizations::PgOrganizationsRepository),
            projects_repo: Arc::new(repos::projects::PgProjectsRepository),
            recovery_codes_repo: Arc::new(repos::recovery_codes::PgRecoveryCodesRepository),
            sessions_repo: Arc::new(repos::sessions::PgSessionsRepository),
            tags_repo: Arc::new(repos::tags::PgTagsRepository),
            totp_credentials_repo: Arc::new(repos::totp_credentials::PgTotpCredentialsRepository),
            users_repo: Arc::new(repos::users::PgUsersRepository),
            verification_tokens_repo: Arc::new(
                repos::verification_tokens::PgVerificationTokensRepository,
            ),
            webauthn_credentials_repo: Arc::new(
                repos::webauthn_credentials::PgWebAuthnCredentialsRepository,
            ),
        }
    }
}
