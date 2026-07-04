-- ────────────────────────────────────────────────────────────────────────────
-- 053: Org Users
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS org_users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_role_id UUID NOT NULL REFERENCES org_roles(org_role_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    UNIQUE (org_role_id, user_id),
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('org_users');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_org_users_user_id ON org_users (user_id);
