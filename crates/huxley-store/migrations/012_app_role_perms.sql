-- ────────────────────────────────────────────────────────────────────────────
-- 012: App Role Perms
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS app_role_perms (
    app_role_perm_id UUID PRIMARY KEY DEFAULT uuidv7(),
    app_role_id UUID NOT NULL REFERENCES app_roles(app_role_id) ON DELETE CASCADE,
    permission TEXT NOT NULL,
    built_in BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('app_role_perms');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_app_role_perms_name ON app_role_perms (name);
