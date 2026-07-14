-- ────────────────────────────────────────────────────────────────────────────
-- 034: Federated Identities
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS federated_identities (
    fedid_id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    idp_id UUID NOT NULL REFERENCES identity_providers(idp_id) ON DELETE CASCADE,
    subject TEXT NOT NULL,
    email_at_idp TEXT COLLATE "case_insensitive",
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    UNIQUE (user_id, idp_id)
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('federated_identities');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_identity_providers_idp_id ON federated_identities (idp_id);
