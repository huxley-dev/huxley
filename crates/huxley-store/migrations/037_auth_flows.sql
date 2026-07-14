-- ────────────────────────────────────────────────────────────────────────────
-- 037: Auth Flows
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS auth_flows (
    auth_flow_id UUID PRIMARY KEY DEFAULT uuidv7(),
    idp_id UUID NOT NULL REFERENCES identity_providers(idp_id) ON DELETE CASCADE,
    state TEXT NOT NULL UNIQUE,
    pkce_verifier TEXT,
    nonce TEXT,
    relate_state TEXT,
    redirect_to TEXT,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('auth_flows');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX ON auth_flows (expires_at);
