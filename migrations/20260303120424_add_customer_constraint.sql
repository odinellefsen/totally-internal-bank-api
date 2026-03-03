-- Add migration script here
ALTER TABLE customer
ALTER COLUMN last_name SET NOT NULL;