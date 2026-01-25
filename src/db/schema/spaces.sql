CREATE TABLE IF NOT EXISTS spaces (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    archived INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    archived_at TEXT
);

