CREATE OR REPLACE FUNCTION create_customer(
    p_customer_id customer.customer_id%TYPE,
    p_first_name customer.first_name%TYPE,
    p_middle_name customer.middle_name%TYPE,
    p_last_name customer.last_name%TYPE,
    p_date_of_birth customer.date_of_birth%TYPE
)
RETURNS TABLE (
    customer_id customer.customer_id%TYPE,
    first_name customer.first_name%TYPE,
    middle_name customer.middle_name%TYPE,
    last_name customer.last_name%TYPE,
    date_of_birth TEXT
)
LANGUAGE plpgsql
AS $$
BEGIN
    RETURN QUERY
    INSERT INTO customer AS c (customer_id, first_name, middle_name, last_name, date_of_birth)
    VALUES (
        p_customer_id,
        NULLIF(BTRIM(p_first_name), ''),
        NULLIF(BTRIM(p_middle_name), ''),
        NULLIF(BTRIM(p_last_name), ''),
        p_date_of_birth
    )
    RETURNING c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth::text;
END;
$$;
