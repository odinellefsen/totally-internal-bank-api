CREATE TABLE customer (
    customer_id TEXT PRIMARY KEY,
    first_name TEXT CHECK (length(first_name) < 150),
    middle_name TEXT CHECK (length(middle_name) < 250),
    last_name TEXT CHECK (length(last_name) < 150),
    date_of_birth DATE
)