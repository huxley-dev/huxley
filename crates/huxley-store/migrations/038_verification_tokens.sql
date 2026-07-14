-- ────────────────────────────────────────────────────────────────────────────
-- 038: Verification Tokens
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS verification_tokens (
    ver_token_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    purpose TEXT COLLATE "case_insensitive" NOT NULL
        CHECK (purpose IN ('email_verify', 'password_reset')),
    token_hash BYTEA NOT NULL UNIQUE,
    used_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('verification_tokens');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX ON verification_tokens (user_id);
