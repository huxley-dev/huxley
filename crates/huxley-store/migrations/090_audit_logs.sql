-- ────────────────────────────────────────────────────────────────────────────
-- 090: Audit Logs
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS audit_logs (
    aud_log_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE SET NULL,
    event TEXT NOT NULL,
    target TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    ip INET,
    user_agent TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('audit_logs');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX ON audit_logs (user_id, created_at DESC);
