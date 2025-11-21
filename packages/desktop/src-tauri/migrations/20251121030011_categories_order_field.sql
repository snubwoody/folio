-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE categories_new(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	title TEXT NOT NULL,
	sort_order REAL NOT NULL DEFAULT ((random() + 9223372036854775808) / 18446744073709551615.0),
	created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

INSERT INTO categories_new(id,title,created_at)
SELECT id,title,created_at FROM categories;

DROP TABLE categories;
ALTER TABLE categories_new RENAME TO categories;

PRAGMA foreign_keys = ON;

