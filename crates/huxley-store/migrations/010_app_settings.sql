-- ────────────────────────────────────────────────────────────────────────────
-- 010: App Settings
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS app_settings (
    app_set_id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT COLLATE "case_insensitive" NOT NULL,
    value TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('app_settings');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_app_settings_name ON app_settings (name);
