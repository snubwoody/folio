-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE expenses_new(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	account_id TEXT NULL REFERENCES accounts(id) ON DELETE SET NULL,
	category_id TEXT NULL REFERENCES categories(id) ON DELETE SET NULL,
	currency_code TEXT NOT NULL,
	created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

INSERT INTO expenses_new SELECT * FROM expenses;
DROP TABLE expenses;
ALTER TABLE expenses_new RENAME TO expenses;

PRAGMA foreign_keys = ON;
