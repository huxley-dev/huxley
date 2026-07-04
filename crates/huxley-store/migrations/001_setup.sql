-- ────────────────────────────────────────────────────────────────────────────
-- 001: Setup
-- ────────────────────────────────────────────────────────────────────────────

-- ─── Extensions ─────────────────────────────────────────────────────────────
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ─── Triggers ───────────────────────────────────────────────────────────────
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
    BEGIN
        NEW.updated_at = now();
        return NEW;
    END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION trigger_updated_at(tablename regclass)
RETURNS VOID AS $$
    BEGIN
        EXECUTE format('
            CREATE TRIGGER set_updated_at
            BEFORE UPDATE
            ON %s
            FOR EACH ROW
            WHEN (OLD IS DISTINCT FROM NEW)
            EXECUTE FUNCTION set_updated_at();',
            tablename
        );
    END;
$$ LANGUAGE plpgsql;

-- ─── Collations ─────────────────────────────────────────────────────────────
CREATE COLLATION case_insensitive (PROVIDER = icu, LOCALE = 'und-u-ks-level2', DETERMINISTIC = false);
