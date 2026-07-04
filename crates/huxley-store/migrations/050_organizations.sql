-- ────────────────────────────────────────────────────────────────────────────
-- 050: Organizations
-- ────────────────────────────────────────────────────────────────────────────

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
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('organizations');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_orgs_parent_id ON orgs (parent_id);
CREATE INDEX idx_orgs_name ON orgs (name);
CREATE INDEX idx_orgs_slug ON orgs (slug);
CREATE INDEX idx_orgs_status ON orgs (status);
