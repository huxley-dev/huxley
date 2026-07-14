-- ────────────────────────────────────────────────────────────────────────────
-- 032: Recovery Codes
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS recovery_codes (
    rec_code_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    code_hash BYTEA NOT NULL,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('recovery_codes');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_recovery_codes_user_id ON recovery_codes (user_id);
