-- ────────────────────────────────────────────────────────────────────────────
-- 050: Organizations
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS organizations (
    org_id UUID PRIMARY KEY DEFAULT uuidv7(),
    parent_id UUID REFERENCES organizations(org_id),
    name TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    slug TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    status TEXT COLLATE "case_insensitive" NOT NULL
        CHECK (status IN ('active', 'disabled', 'deleted')),
    settings JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('organizations');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_orgs_parent_id ON organizations (parent_id);
CREATE INDEX idx_orgs_name ON organizations (name);
CREATE INDEX idx_orgs_slug ON organizations (slug);
CREATE INDEX idx_orgs_status ON organizations (status);
