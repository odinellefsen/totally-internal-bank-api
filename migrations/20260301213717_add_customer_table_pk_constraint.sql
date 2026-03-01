-- Add migration script here
ALTER TABLE customer
ALTER COLUMN customer_id TYPE INTEGER
USING customer_id::INTEGER;

ALTER TABLE customer
ADD CONSTRAINT customer_id_p_tal
CHECK (customer_id BETWEEN 100000000 AND 999999999);