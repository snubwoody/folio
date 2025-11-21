-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE income_streams_new(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	title TEXT NOT NULL,
	sort_order REAL NOT NULL DEFAULT ((random() + 9223372036854775808) / 18446744073709551615.0),
	created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

INSERT INTO income_streams_new(id,title,created_at)
SELECT id,title,created_at FROM income_streams;

DROP TABLE income_streams;
ALTER TABLE income_streams_new RENAME TO income_streams;

PRAGMA foreign_keys = ON;

