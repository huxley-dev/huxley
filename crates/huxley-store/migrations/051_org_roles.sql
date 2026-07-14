-- ────────────────────────────────────────────────────────────────────────────
-- 051: Org Roles
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS org_roles (
    org_role_id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('org_roles');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_org_roles_name ON org_roles (name);
