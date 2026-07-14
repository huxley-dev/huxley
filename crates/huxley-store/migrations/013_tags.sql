-- ────────────────────────────────────────────────────────────────────────────
-- 013: Tags
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS tags (
    tag_id UUID PRIMARY KEY DEFAULT uuidv7(),
    tag_type TEXT COLLATE "case_insensitive" NOT NULL
        CHECK (tag_type IN ('workflow')),
    name TEXT COLLATE "case_insensitive" NOT NULL,
    bg_color TEXT NOT NULL,
    text_color TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ,

    UNIQUE (tag_type, name)
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('tags');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_tags_tag_type ON tags (tag_type);
CREATE INDEX idx_tags_name ON tags (name);
