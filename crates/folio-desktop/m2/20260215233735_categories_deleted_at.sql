-- migrate:up
ALTER TABLE categories
ADD COLUMN deleted_at INT NULL;
-- migrate:down
