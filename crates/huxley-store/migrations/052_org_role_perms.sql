-- ────────────────────────────────────────────────────────────────────────────
-- 052: Org Role Perms
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS org_role_perms (
    org_role_perm_id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_role_id UUID NOT NULL REFERENCES org_roles(org_role_id) ON DELETE CASCADE,
    permission TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('org_role_perms');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_org_perms_name ON org_role_perms (permission);
