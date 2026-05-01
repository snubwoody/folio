-- migrate:up
PRAGMA foreign_keys = OFF;
CREATE TABLE categories_new(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (unixepoch('now')),
    deleted_at INT NULL,
    is_income_stream BOOLEAN DEFAULT FALSE,
);
PRAGMA foreign_keys = ON;

-- migrate:down
