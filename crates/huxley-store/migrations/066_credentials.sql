-- ────────────────────────────────────────────────────────────────────────────
-- 066: Credentials
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS credentials (
    cred_id UUID PRIMARY KEY DEFAULT uuidv7(),
    org_id UUID NOT NULL REFERENCES organizations(org_id),
    name TEXT NOT NULL,
    value BYTEA NULL,
    inheritable BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    update_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('credentials');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_credential_name ON credentials (name);
CREATE INDEX idx_credential_inheritable ON credentials (inheritable);
