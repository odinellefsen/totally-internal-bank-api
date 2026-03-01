CREATE TABLE customer (
    customer_id INTEGER PRIMARY KEY CONSTRAINT customer_id_p_tal CHECK (customer_id BETWEEN 100000000 AND 999999999),
    first_name TEXT CHECK (length(first_name) < 150),
    middle_name TEXT CHECK (length(middle_name) < 250),
    last_name TEXT CHECK (length(last_name) < 150),
    date_of_birth DATE
)