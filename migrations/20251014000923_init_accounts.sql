-- Add migration script here
CREATE TABLE accounts(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	name TEXT NOT NULL DEFAULT 'Account',
	starting_balance INTEGER NOT NULL DEFAULT 0
);