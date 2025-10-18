-- Add migration script here
CREATE TABLE expenses(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	account_id TEXT NULL REFERENCES accounts(id),
	category_id TEXT NULL REFERENCES categories(id),
	currency_code TEXT NOT NULL
);

CREATE TABLE incomes(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	account_id TEXT NULL REFERENCES accounts(id),
	currency_code TEXT NOT NULL
);

CREATE TABLE transfers(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	from_account_id TEXT NULL REFERENCES accounts(id),
	to_account_id TEXT NULL REFERENCES accounts(id)
);