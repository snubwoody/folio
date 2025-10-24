-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE incomes_new(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	account_id TEXT NULL REFERENCES accounts(id) ON DELETE SET NULL,
	income_stream TEXT NULL REFERENCES income_streams(id) ON DELETE SET NULL,
	currency_code TEXT NOT NULL,
	created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

INSERT INTO incomes_new SELECT * FROM incomes;
DROP TABLE incomes;
ALTER TABLE incomes_new RENAME TO incomes;

PRAGMA foreign_keys = ON;
