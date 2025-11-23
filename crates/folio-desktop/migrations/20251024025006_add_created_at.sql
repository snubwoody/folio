-- Add migration script here
ALTER TABLE accounts
ADD COLUMN created_at INT NULL;
UPDATE accounts SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE expenses
ADD COLUMN created_at INT NULL;
UPDATE expenses SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE incomes
ADD COLUMN created_at INT NULL;
UPDATE incomes SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE transfers
ADD COLUMN created_at INT NULL;
UPDATE transfers SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE categories
ADD COLUMN created_at INT NULL;
UPDATE categories SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE income_streams
ADD COLUMN created_at INT NULL;
UPDATE incomes SET created_at = unixepoch('now') WHERE created_at IS NULL;

ALTER TABLE budgets
ADD COLUMN created_at INT NULL;
UPDATE budgets SET created_at = unixepoch('now') WHERE created_at IS NULL;
