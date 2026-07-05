-- ────────────────────────────────────────────────────────────────────────────
-- 060: Workflow Projects
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS workflow_projects (
    wf_project_id UUID PRIMARY KEY DEFAULT uuidv7(),
    project_type COLLATE "case_insensitive" TEXT NOT NULL
        CHECK (project_type IN ('org', 'user')),
    org_id UUID REFERENCES org(id) NULL,
    user_id UUID REFERENCES user(id) NULL,
    name TEXT COLLATE "case_insensitive" NOT NULL,
    slug TEXT COLLATE "case_insensitive" NOT NULL,
    description TEXT NULL,
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

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('workflow_projects');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_workflow_projects_org_id ON workflow_projects (org_id) WHERE org_id IS NOT NULL;
CREATE INDEX idx_workflow_projects_user_id ON workflow_projects (user_id) WHERE user_id IS NOT NULL;
CREATE INDEX idx_workflow_projects_name ON workflow_projects (name);
CREATE INDEX idx_workflow_projects_slug ON workflow_projects (slug);
