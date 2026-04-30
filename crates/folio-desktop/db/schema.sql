CREATE TABLE _sqlx_migrations (
    version BIGINT PRIMARY KEY,
    description TEXT NOT NULL,
    installed_on TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    success BOOLEAN NOT NULL,
    checksum BLOB NOT NULL,
    execution_time BIGINT NOT NULL
);
CREATE TABLE accounts(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	name TEXT NOT NULL DEFAULT 'Account',
	starting_balance INTEGER NOT NULL DEFAULT 0
, created_at INT NULL);
CREATE TABLE transfers(
	id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
	amount INTEGER NOT NULL,
	transaction_date TEXT NOT NULL DEFAULT (date('now')),
	from_account_id TEXT NULL REFERENCES accounts(id),
	to_account_id TEXT NULL REFERENCES accounts(id)
, created_at INT NULL);
CREATE TABLE "budgets" (
    id TEXT NOT NULL DEFAULT (hex(randomblob(8))),
    category_id TEXT NOT NULL UNIQUE REFERENCES categories(id),
    amount INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (unixepoch('now')),
    month INTEGER NULL,
    year INTEGER NULL,
    -- Prevent duplicate budgets
    UNIQUE(category_id, month, year)
);
CREATE TABLE "transactions" (
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    amount INT NOT NULL DEFAULT 0,
    -- The sending account
    from_account_id TEXT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    -- The receiving account
    to_account_id TEXT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    -- The date the transaction occurred
    transaction_date TEXT NOT NULL,
    category_id TEXT NULL REFERENCES categories(id) ON DELETE SET NULL,
    created_at INT NOT NULL DEFAULT (unixepoch('now')),
    -- Optional context for the transaction
    note TEXT NULL,

    CHECK (amount >= 0),

    -- Prevent self transfers
    CHECK (from_account_id IS DISTINCT FROM to_account_id)
    -- At least one should be not null
    CHECK (from_account_id IS NOT NULL OR to_account_id IS NOT NULL)
);
CREATE TABLE "category_groups"(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL,
    sort_order INT NOT NULL DEFAULT (random()),
    created_at INT NOT NULL DEFAULT (unixepoch('now'))
);
CREATE TABLE "schema_migrations" (version varchar(128) primary key);
CREATE TABLE "categories"(
     id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
     title TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (unixepoch('now')),
    deleted_at INT NULL,
    is_income_stream BOOLEAN DEFAULT FALSE,
    category_group TEXT NULL REFERENCES category_groups(id) DEFAULT NULL
);
-- Dbmate schema migrations
INSERT INTO "schema_migrations" (version) VALUES
  ('20260430175918');
