-- ────────────────────────────────────────────────────────────────────────────
-- 035: Sessions
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS sessions (
    session_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    idp_id UUID NOT NULL REFERENCES identity_providers(idp_id) ON DELETE SET NULL,
    token_hash BYTEA NOT ULL UNIQUE,
    aal SMALLINT NOT NULL DEFAULT 1,
    auth_method TEXT NOT NULL,
    ip INET,
    user_agent TEXT,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    idle_expires_at TIMESTAMPTZ NOT NULL,
    absolute_expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('sessions');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_idle_expires_at ON sessions(idle_expires_at);
