-- Add migration script here
ALTER TABLE customer
DROP CONSTRAINT IF EXISTS customer_id_p_tal;

ALTER TABLE customer
ADD CONSTRAINT customer_id_must_be_9_digits_chk
CHECK (customer_id BETWEEN 100000000 AND 999999999);

ALTER TABLE customer
ALTER COLUMN first_name SET NOT NULL;

ALTER TABLE customer
ALTER COLUMN date_of_birth SET NOT NULL;

ALTER TABLE customer
DROP CONSTRAINT IF EXISTS customer_first_name_check;

ALTER TABLE customer
ADD CONSTRAINT customer_first_name_len_chk
CHECK (length(first_name) < 150);

ALTER TABLE customer
DROP CONSTRAINT IF EXISTS customer_middle_name_check;

ALTER TABLE customer
ADD CONSTRAINT customer_middle_name_len_chk
CHECK (length(middle_name) < 250);

ALTER TABLE customer
DROP CONSTRAINT IF EXISTS customer_last_name_check;

ALTER TABLE customer
ADD CONSTRAINT customer_last_name_len_chk
CHECK (length(last_name) < 150);