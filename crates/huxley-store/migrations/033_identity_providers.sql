-- ────────────────────────────────────────────────────────────────────────────
-- 033: Identity Providers
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS identity_providers (
    idp_id UUID PRIMARY KEY DEFAULT uuidv7(),
    kind TEXT COLLATE "case_insensitive" NOT NULL
        CHECK (kind IN ('oidc', 'saml')),
    name TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    slug TEXT COLLATE "case_insensitive" NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT true,
    config JSONB NOT NULL DEFAULT '{}'::jsonb,
    secret_enc BYTEA,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('identity_providers');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_identity_providers_name ON identity_providers (name);
CREATE INDEX idx_identity_providers_slug ON identity_providers (slug);
CREATE INDEX idx_identity_providers_enabled ON identity_providers (enabled);
