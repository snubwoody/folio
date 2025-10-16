-- Add migration script here
CREATE TABLE budgets(
    id TEXT NOT NULL DEFAULT (hex(randomblob(8))),
    amount TEXT NOT NULL DEFAULT '0',
    category_id TEXT NOT NULL REFERENCES categories(id)
);