-- ────────────────────────────────────────────────────────────────────────────
-- 030: TOTP Credentials
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS totp_credentials (
    totp_cred_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    secret_enc BYTEA NOT NULL,
    confirmed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    UNIQUE (user_id),
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('totp_credentials');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_totp_credentials_user_id ON totp_credentials (user_id);
