-- migrate:up
PRAGMA foreign_keys = OFF;

CREATE TABLE categories_new(
     id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
     title TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (unixepoch('now')),
    deleted_at INT NULL,
    is_income_stream BOOLEAN DEFAULT FALSE,
    category_group TEXT NULL REFERENCES category_groups(id) DEFAULT NULL
);

INSERT INTO categories_new(id,title,created_at,deleted_at,is_income_stream)
SELECT id,title,created_at,deleted_at,is_income_stream FROM categories;

DROP TABLE categories;

ALTER TABLE categories_new
RENAME TO categories;

PRAGMA foreign_keys = ON;

-- migrate:down
