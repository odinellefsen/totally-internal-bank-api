CREATE OR REPLACE FUNCTION delete_customer(
    p_customer_id customer.customer_id%TYPE
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
    DELETE FROM customer AS c
    WHERE c.customer_id = p_customer_id
    RETURNING c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth::text;
END;
$$;
