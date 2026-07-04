-- ────────────────────────────────────────────────────────────────────────────
-- 001: Initial schema
-- ────────────────────────────────────────────────────────────────────────────

CREATE EXTENSION IF NOT EXISTS "citext";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ─── Tags ───────────────────────────────────────────────────────────────────
CREATE TABLE "tags" IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    tag_type SMALLINT NOT NULL,
    name TEXT NOT NULL,
    bg_color TEXT NOT NULL,
    text_color TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (tag_type, name),
);

CREATE INDEX idx_tags_tag_type ON tags (tag_type);
CREATE INDEX idx_tags_name ON tags (name);

-- ─── Orgs ───────────────────────────────────────────────────────────────────
CREATE TABLE orgs IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    parent_id UUID REFERENCES orgs (id) NULL,
    name TEXT NOT NULL UNIQUE,
    slug TEXT NULL UNIQUE,
    is_active BOOLEAN NOT NULL DEFAULT true,
    mappings JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_orgs_parent_id ON orgs(parent_id);
CREATE INDEX idx_orgs_name ON orgs (name);
CREATE INDEX idx_orgs_slug ON orgs (slug);
CREATE INDEX idx_orgs_is_active ON orgs (is_active);

-- ─── Users ──────────────────────────────────────────────────────────────────
CREATE TABLE users IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    password TEXT NULL,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status in ('active', 'disabled', 'locked')),
    preferences JSONB NOT NULL DEFAULT '{}'::jsonb,
    app_role_id UUID REFERENCES app_roles(id),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_name ON users (name);
CREATE INDEX idx_users_is_active ON users (name);
CREATE INDEX idx_users_role ON users (role_id);

-- ─── Org Users ──────────────────────────────────────────────────────────────
CREATE TABLE org_users IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID REFERENCES orgs(id) NOT NULL,
    user_id UUID REFERENCES user(id) NOT NULL,
    org_role_id UUID REFERENCES org_roles(id) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (org_id, user_id),
);

CREATE INDEX idx_org_users_user_id ON org_users (user_id);

-- ─── App Roles ──────────────────────────────────────────────────────────────
CREATE TABLE app_roles IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL,
    description TEXT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_app_roles_name ON app_roles (name);
CREATE INDEX idx_app_roles_is_active ON app_roles (is_active);

-- ─── App Perms ──────────────────────────────────────────────────────────────
CREATE TABLE app_perms IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_app_perms_name ON app_perms (name);
CREATE INDEX idx_app_perms_is_active ON app_perms (is_active);

-- ─── App Role Perms ─────────────────────────────────────────────────────────
CREATE TABLE app_role_perms IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    app_role_id UUID REFERENCES app_roles(id) NOT NULL,
    app_perm_id UUID REFERENCES app_perms(id) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (app_role_id, app_perm_id)
);

CREATE INDEX idx_app_role_perms_app_perm_id ON app_role_perms (app_perm_id);

-- ─── Org Roles ──────────────────────────────────────────────────────────────
CREATE TABLE org_roles IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_org_roles_name ON org_roles (name);
CREATE INDEX idx_org_roles_is_active ON org_roles (is_active);

-- ─── Org Perms ──────────────────────────────────────────────────────────────
CREATE TABLE org_perms IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    description TEXT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    metadata JSONB NUL NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_org_perms_name ON org_perms (name);
CREATE INDEX idx_org_perms_is_active ON org_perms (is_active);

-- ─── Org Role Perms ─────────────────────────────────────────────────────────
CREATE TABLE org_role_perms IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_role_id UUID REFERENCES org_roles(id) NOT NULL,
    org_perm_id UUID REFERENCES org_perms(id) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (org_role_id, org_perm_id)
);

CREATE INDEX idx_org_role_perms_org_perm_id ON org_role_perms (org_perm_id);

-- ─── Auth Providers ─────────────────────────────────────────────────────────
CREATE TABLE auth_providers IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL UNIQUE,
    auth_type SMALLINT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_auth_providers_name ON auth_providers (name);
CREATE INDEX idx_auth_providers_is_active ON auth_providers (is_active);

-- ─── User Auth Providers ────────────────────────────────────────────────────
CREATE TABLE user_auth_providers IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID REFERENCES users(id) NOT NULL,
    auth_provider_id UUID REFERENCES auth_providers(id) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (user_id, auth_provider_id),
);

CREATE INDEX idx_user_auth_providers_auth_provider_id ON user_auth_providers (auth_provider_id);

-- ─── Org Variables ──────────────────────────────────────────────────────────
CREATE TABLE org_variables IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID REFERENCES orgs(id) NOT NULL,
    var_type SMALLINT NOT NULL,
    name TEXT NOT NULL,
    value TEXT NULL,
    is_inheritable BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (org_id, name),
);

CREATE INDEX idx_org_variables_name ON org_variables (name);
CREATE INDEX idx_org_variables_is_inheritable ON org_variables (is_heritable);

-- ─── Org Credentials  ───────────────────────────────────────────────────────
CREATE TABLE org_credentials IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID REFERENCES orgs(id) NOT NULL,
    name TEXT NOT NULL,
    value BYTEA NOT NULL,
    is_inheritable BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (org_id, name),
);

CREATE INDEX idx_org_credentials_name on org_credentials (name);
CREATE INDEX idx_org_credentials_is_heritable ON org_variables (is_inheritable);

-- ─── Workflow Projects ──────────────────────────────────────────────────────
CREATE TABLE workflow_projects IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    project_type SMALLINT NOT NULL,
    org_id UUID REFERENCES org(id) NULL,
    user_id UUID REFERENCES user(id) NULL,
    name TEXT NOT NULL,
    slug TEXT NOT NULL,
    description TEXT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT check_owner_type CHECK (
        (org_id IS NOT NULL AND user_id IS NULL) OR
        (org_id IS NULL AND user_id IS NOT NULL)
    ),

    CONSTRAINT unique_project_name_per_org UNIQUE (name, org_id),
    CONSTRAINT unique_project_slug_per_org UNIQUE (slug, org_id),
    CONSTRAINT unique_project_name_per_user UNIQUE (name, user_id),
    CONSTRAINT unique_project_slug_per_user UNIQUE (slug, user_id),
);

CREATE INDEX idx_workflow_projects_org_id ON workflow_projects (org_id) WHERE org_id IS NOT NULL;
CREATE INDEX idx_workflow_projects_user_id ON workflow_projects (user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_workflow_projects_name ON workflow_projects (name);
CREATE INDEX idx_workflow_projects_slug ON workflow_projects (slug);

-- ─── Workflows ──────────────────────────────────────────────────────────────
CREATE TABLE workflows IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    rev_id UUID NOT NULL DEFAULT uuidv7(),
    org_id UUID REFERENCES orgs(id) NOT NULL,
    org_vars JSONB NOT NULL DEFAULT '{}'::jsonb,
    triggers JSONB NOT NULL DEFAULT '{}'::jsonb,
    nodes JSONB NOT NULL DEFAULT '{}'::jsonb,
    edges JSONB NOT NULL DEFAULT '{}'::jsonb,
    status SMALLINT NOT NULL,
    workflow_project_id UUID REFERENCES workflow_folders(id),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_workflows_rev_id ON workflows (rev_id);
CREATE INDEX idx_workflows_org_id ON workflows (org_id);
CREATE INDEX idx_workflows_status ON workflows (status);

-- ─── Workflow Tags ──────────────────────────────────────────────────────────
CREATE TABLE workflow_tags IF NOT EXISTS (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    workflow_id UUID REFERENCES workflows(id) NOT NULL,
    tag_id UUID REFERENCES tags(id) NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT Now(),

    UNIQUE (workflow_id, tag_id)
);

CREATE INDEX idx_workflow_tags_tag_id ON workflow_tags (tag_id);

-- ─── Workflow Revs ──────────────────────────────────────────────────────────
CREATE TABLE workflow_revs IF NOT EXISTS (
    id UUID PRIMARY KEY,
    workflow_id UUID REFERENCES workflows(id) NOT NULL,
    org_id UUID REFERENCES orgs(id) NOT NULL,
    org_vars JSONB NOT NULL DEFAULT '{}'::jsonb,
    triggers JSONB NOT NULL DEFAULT '{}'::jsonb,
    nodes JSONB NOT NULL DEFAULT '{}'::jsonb,
    edges JSONB NOT NULL DEFAULT '{}'::jsonb,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_workflow_revs_workflow_id ON workflow_revs (workflow_id);
CREATE INDEX idx_workflow_revs_org_id ON workflow_revs (org_id);

-- ─── Workflow Execs ─────────────────────────────────────────────────────────

-- ─── Git Providers ──────────────────────────────────────────────────────────
CREATE TABLE git_provider_configs IF NOT EXISTS (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    token BYTEA NULL,
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    is_active BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
);

CREATE INDEX idx_git_provider_configs_name ON git_provider_configs (name);
CREATE INDEX idx_git_provider_configs_is_active ON git_provider_configs (is_active);
