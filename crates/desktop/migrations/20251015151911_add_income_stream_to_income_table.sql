-- Add migration script here
CREATE TABLE incomes_new(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL DEFAULT 0,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	account_id TEXT NULL REFERENCES accounts(id),
    income_stream TEXT NULL REFERENCES income_streams(id),
	currency_code TEXT NOT NULL
);

-- Copy existing data from old table to new table
INSERT INTO incomes_new(id, amount, transaction_date, account_id, currency_code)
SELECT id, amount, transaction_date, account_id, currency_code
FROM incomes;

-- Drop the old table
DROP TABLE incomes;

-- Rename the new table to the original name
ALTER TABLE incomes_new RENAME TO incomes;