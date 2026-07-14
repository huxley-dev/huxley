-- ────────────────────────────────────────────────────────────────────────────
-- 061: Folders
-- ────────────────────────────────────────────────────────────────────────────

SET lock_timeout = 5000;

-- ─── Tables ─────────────────────────────────────────────────────────────────
CREATE TABLE IF NOT EXISTS folders (
    folder_id UUID PRIMARY KEY DEFAULT uuidv7(),
    project_id UUID NOT NULL REFERENCES projects(project_id) ON DELETE CASCADE,
    parent_id UUID REFERENCES folders(folder_id) ON DELETE SET NULL,
    name TEXT COLLATE "case_insensitive" NOT NULL,
    slug TEXT COLLATE "case_insensitive" NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    UNIQUE (project_id, parent_id, name),
    UNIQUE (project_id, parent_id, slug)
);

-- ─── Triggers ───────────────────────────────────────────────────────────────
SELECT trigger_updated_at('folders');

-- ─── Indexes ────────────────────────────────────────────────────────────────
CREATE INDEX idx_folders_parent_id ON folders (parent_id);
CREATE INDEX idx_folders_name_id ON folders (name);
CREATE INDEX idx_folders_slug_id ON folders (slug);
