use deadpool_redis::Pool;
use sqlx::PgPool;
use std::sync::Arc;

use huxley_config::HuxleyConfig;
use huxley_state::repos::*;

pub struct HuxleyState {
    pub config: Arc<HuxleyConfig>,

    pub db_pool: PgPool,
    pub cache_pool: Pool,

    pub app_perms_repo: Arc<dyn repos::AppPermsRepository>,
    pub app_role_perms_repo: Arc<dyn repos::AppRolePermsRepository>,
    pub app_roles_repo: Arc<dyn repos::AppRolesRepository>,
    pub org_perms_repo: Arc<dyn repos::OrgPermsRepository>,
    pub org_role_perms_repo: Arc<dyn repos::OrgRolePermsRepository>,
    pub org_roles_repo: Arc<dyn repos::OrgRolesRepository>,
    pub org_users_repo: Arc<dyn repos::OrgUsersRepository>,
    pub orgs_repo: Arc<dyn repos::OrgsRepository>,
    pub tags_repo: Arc<dyn repos::TagsRepository>,
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
            app_perms_repo: Arc::new(repos::PgAppPermsRepository),
            app_role_perms_repo: Arc::new(repos::PgAppRolePermsRepository),
            app_roles_repo: Arc::new(repos::PgAppRolesRepository),
            org_perms_repo: Arc::new(repos::PgOrgPermsRepository),
            org_role_perms_repo: Arc::new(repos::PgOrgRolePermsRepository),
            org_roles_repo: Arc::new(repos::PgOrgRolesRepository),
            org_users_repo: Arc::new(repos::PgOrgUsersRepository),
            orgs_repo: Arc::new(repos::PgOrgsRepository),
            tags_repo: Arc::new(repos::PgTagsRepository)
        }
    }
}
