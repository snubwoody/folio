-- Add migration script here
CREATE TABLE categories(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	title TEXT NOT NULL
);