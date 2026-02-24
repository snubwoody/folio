PRAGMA foreign_keys = OFF;

CREATE TABLE transactions_new (
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

INSERT INTO transactions_new(id,amount,from_account_id,to_account_id,transaction_date,category_id,created_at,note)
SELECT id,amount,from_account_id,to_account_id,transaction_date,category_id,created_at,note
FROM transactions;

DROP TABLE transactions;

ALTER TABLE transactions_new RENAME TO transactions;

PRAGMA foreign_keys = ON;


