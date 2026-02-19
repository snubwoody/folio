CREATE TABLE transactions (
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    amount INT NOT NULL DEFAULT 0,
    -- The sending account
    from_account_id TEXT NULL REFERENCES accounts(id),
    -- The receiving account
    to_account_id TEXT NULL REFERENCES accounts(id),
    -- The date the transaction occurred
    transaction_date TEXT NOT NULL,
    category_id TEXT NULL REFERENCES categories(id),
    created_at INT NOT NULL DEFAULT (unixepoch('now')),
    -- Optional context for the transaction
    note TEXT NULL,

    CHECK (amount >= 0),

    -- Prevent self transfers
    CHECK (from_account_id IS DISTINCT FROM to_account_id)
    -- At least one should be not null
    CHECK (from_account_id IS NOT NULL OR to_account_id IS NOT NULL)
);


