-- Add migration script here
PRAGMA foreign_keys = OFF;

CREATE TABLE budgets_new(
    id TEXT NOT NULL DEFAULT (hex(randomblob(8))),
    amount INTEGER NOT NULL DEFAULT 0,
    category_id TEXT NOT NULL UNIQUE REFERENCES categories(id) ON DELETE CASCADE,
    created_at INT NOT NULL DEFAULT (unixepoch('now'))
);

INSERT INTO budgets_new SELECT * FROM budgets;
DROP TABLE budgets;
ALTER TABLE budgets_new RENAME TO budgets;

PRAGMA foreign_keys = ON;
