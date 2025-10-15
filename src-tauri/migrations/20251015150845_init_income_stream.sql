-- Add migration script here
CREATE TABLE income_streams(
    id TEXT PRIMARY KEY NOT NULL DEFAULT (hex(randomBlob(8))),
    title TEXT NOT NULL DEFAULT 'Income'
);

INSERT INTO income_streams(title)
VALUES 
    ('Salary'),
    ('Dividends'),
    ('Refund'),
    ('Other'),
    ('Donation');