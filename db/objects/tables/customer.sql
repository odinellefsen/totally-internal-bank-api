CREATE TABLE customer (
    customer_id INTEGER PRIMARY KEY CONSTRAINT customer_id_must_be_9_digits_chk CHECK (customer_id BETWEEN 100000000 AND 999999999),
    first_name TEXT NOT NULL CONSTRAINT customer_first_name_len_chk CHECK (length(first_name) < 150),
    middle_name TEXT CONSTRAINT customer_middle_name_len_chk CHECK (length(middle_name) < 250),
    last_name TEXT NOT NULL CONSTRAINT customer_last_name_len_chk CHECK (length(last_name) < 150),
    date_of_birth DATE NOT NULL
)