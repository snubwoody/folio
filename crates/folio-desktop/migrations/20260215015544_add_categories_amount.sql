PRAGMA foreign_keys = OFF;

CREATE TABLE budgets_new (
    id TEXT NOT NULL DEFAULT (hex(randomblob(8))),
    category_id TEXT NOT NULL UNIQUE REFERENCES categories(id),
    amount INTEGER NOT NULL DEFAULT 0,
    created_at INTEGER NOT NULL DEFAULT (unixepoch('now')),
    month INTEGER NULL,
    year INTEGER NULL,
    -- Prevent duplicate budgets
    UNIQUE(category_id, month, year)
);

-- UPDATE budgets 
-- SET year = strftime('%Y', 'now'),
--     month = CAST(strftime('%m', 'now') AS INTEGER)
-- WHERE year IS NULL OR month IS NULL;


INSERT INTO budgets_new(id,category_id,amount,created_at) 
SELECT id,category_id,amount,created_at FROM budgets;

DROP TABLE budgets;
ALTER TABLE budgets_new RENAME TO budgets;

PRAGMA foreign_keys = ON;
