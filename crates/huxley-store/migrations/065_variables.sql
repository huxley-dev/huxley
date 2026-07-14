-- ────────────────────────────────────────────────────────────────────────────
-- 065: Variables
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS variables (
    var_id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES organizations(org_id),
    var_type TEXT NOT NULL
        CHECK (var_type IN ('general')),
    name TEXT NOT NULL,
    value TEXT NULL,
    inheritable BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    update_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('variables');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_variables_name ON variables (name);
CREATE INDEX idx_variables_inheritable ON variables (inheritable);
