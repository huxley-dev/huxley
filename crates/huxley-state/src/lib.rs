use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use huxley_config::HuxleyConfig;
use huxley_store::repos::*;

pub struct HuxleyState {
    pub config: Arc<HuxleyConfig>,

    pub db_pool: PgPool,
    pub cache_pool: Pool,

    pub api_tokens_repo: Arc<dyn api_tokens::ApiTokensRepository>,
    pub app_role_perms_repo: Arc<dyn app_role_perms::AppRolePermsRepository>,
    pub app_roles_repo: Arc<dyn app_roles::AppRolesRepository>,
    pub app_settings_repo: Arc<dyn app_settings::AppSettingsRepository>,
    pub audit_logs_repo: Arc<dyn audit_logs::AuditLogsRepository>,
    pub auth_flows_repo: Arc<dyn auth_flows::AuthFlowsRepository>,
    pub credentials_repo: ArC<dyn credentials::CredentialsRepository>,
    pub federated_identities_repo: Arc<dyn federated_identities::FederatedIdentitiesRepository>,
    pub folders_repo: Arc<dyn folders::FoldersRepository>,
    pub identity_providers_repo: Arc<dyn identity_providers::IdentityProvidersRepository>,
    pub login_attempts_repo: Arc<dyn login_attempts::LoginAttemptsRepository>,
    pub org_role_perms_repo: Arc<dyn org_role_perms::OrgRolePermsRepository>,
    pub org_roles_repo: Arc<dyn org_roles::OrgRolesRepository>,
    pub org_users_repo: Arc<dyn org_users::OrgUsersRepository>,
    pub organizations_repo: Arc<dyn organizations::OrganizationsRepository>,
    pub projects_repo: Arc<dyn projects::ProjectsRepository>,
    pub recovery_codes_repo: Arc<dyn recovery_codes::RecoveryCodesRepository>,
    pub sessions_repo: Arc<dyn sessions::SessionsRepository>,
    pub tags_repo: Arc<dyn tags::TagsRepository>,
    pub totp_credentials_repo: Arc<dyn totp_credentials::TotpCredentialsRepository>,
    pub users_repo: Arc<dyn users::UsersRepository>,
    pub variables_repo: Arc<dyn variables::VariablesRepository>,
    pub verification_tokens_repo: Arc<dyn verification_tokens::VerificationTokensRepository>,
    pub webauthn_credentials_repo: Arc<dyn webauthn_credentials::WebAuthnCredentialsRepository>,
}

impl HuxleyState {
    pub fn new(config: Arc<HuxleyConfig>, db_pool: PgPool, cache_pool: Pool) -> Self {
        Self {
            config,
            db_pool,
            cache_pool,
            api_tokens_repo: Arc::new(api_tokens::PgApiTokensRepository),
            app_role_perms_repo: Arc::new(app_role_perms::PgAppRolePermsRepository),
            app_roles_repo: Arc::new(app_roles::PgAppRolesRepository),
            app_settings_repo: Arc::new(app_settings::PgAppSettingsRepository),
            audit_logs_repo: Arc::new(audit_logs::PgAuditLogsRepository),
            auth_flows_repo: Arc::new(auth_flows::PgAuthFlowsRepository),
            credentials_repo: Arc::new(credentials::PgCredentialsRepository),
            federated_identities_repo: Arc::new(
                federated_identities::PgFederatedIdentitiesRepository,
            ),
            folders_repo: Arc::new(folders::PgFoldersRepository),
            identity_providers_repo: Arc::new(identity_providers::PgIdentityProvidersRepository),
            login_attempts_repo: Arc::new(login_attempts::PgLoginAttemptsRepository),
            org_role_perms_repo: Arc::new(org_role_perms::PgOrgRolePermsRepository),
            org_roles_repo: Arc::new(org_roles::PgOrgRolesRepository),
            org_users_repo: Arc::new(org_users::PgOrgUsersRepository),
            organizations_repo: Arc::new(organizations::PgOrganizationsRepository),
            projects_repo: Arc::new(projects::PgProjectsRepository),
            recovery_codes_repo: Arc::new(recovery_codes::PgRecoveryCodesRepository),
            sessions_repo: Arc::new(sessions::PgSessionsRepository),
            tags_repo: Arc::new(tags::PgTagsRepository),
            totp_credentials_repo: Arc::new(totp_credentials::PgTotpCredentialsRepository),
            users_repo: Arc::new(users::PgUsersRepository),
            variables_repo: Arc::new(variables::PgVariablesRepository),
            verification_tokens_repo: Arc::new(verification_tokens::PgVerificationTokensRepository),
            webauthn_credentials_repo: Arc::new(
                webauthn_credentials::PgWebAuthnCredentialsRepository,
            ),
        }
    }
}
