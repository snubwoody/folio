-- Add migration script here
UPDATE income_streams SET created_at = unixepoch('now') WHERE created_at IS NULL;
