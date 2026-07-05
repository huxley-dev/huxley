-- ────────────────────────────────────────────────────────────────────────────
-- 039: Login Attempts
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS login_attempts (
    login_attempt_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE SET NULL,
    email TEXT COLLATE "case_insensitive",
    ip INET,
    user_agent TEXT,
    successful BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('login_attempts');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX ON login_attempts (email, created_at DESC);
CREATE INDEX ON login_attempts (ip, created_at DESC);
