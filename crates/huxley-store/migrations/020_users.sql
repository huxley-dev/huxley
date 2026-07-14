-- ────────────────────────────────────────────────────────────────────────────
-- 020: Users
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS users (
    user_id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT COLLATE "case_insensitive" NOT NULL,
    email TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    email_verified BOOLEAN NOT NULL DEFAULT false,
    password_hash TEXT,
    status TEXT COLLATE "case_insensitive" NOT NULL DEFAULT 'active'
        CHECK (status in ('active', 'disabled', 'locked')),
    preferences JSONB NOT NULL DEFAULT '{}'::jsonb,
    app_role_id UUID NOT NULL REFERENCES app_roles(app_role_id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('users');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_name ON users (name);
CREATE INDEX idx_users_status on users (status);
CREATE INDEX idx_users_app_role ON users (app_role_id);
