-- ────────────────────────────────────────────────────────────────────────────
-- 011: App Roles
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS app_roles (
    app_role_id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    description TEXT,
    built_in BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('app_roles');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_app_roles_name ON app_roles (name);
