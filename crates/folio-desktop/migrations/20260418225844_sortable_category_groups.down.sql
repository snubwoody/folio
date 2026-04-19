PRAGMA foreign_keys = OFF;

CREATE TABLE category_groups_new(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL
);

INSERT INTO category_groups_new(id,title)
SELECT id,title FROM category_groups;

DROP TABLE category_groups;

ALTER TABLE category_groups_new
RENAME TO category_groups;

PRAGMA foreign_keys = ON;
