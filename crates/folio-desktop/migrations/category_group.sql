PRAGMA foreign_keys = OFF;

CREATE TABLE category_groups_new(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL,
    sort_order INT NOT NULL DEFAULT (random()),
    created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

-- Use rowid to keep the existing order
INSERT INTO category_groups_new(id,title,sort_order)
SELECT id,title,rowid FROM category_groups;

DROP TABLE category_groups;

ALTER TABLE category_groups_new
RENAME TO category_groups;

PRAGMA foreign_keys = ON;
