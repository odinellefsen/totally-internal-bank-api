-- Add migration script here
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
    -- we need to add this constraint check manually again because
    -- anything that is in the WHERE clause isn't checked before
    -- the lookup is done meaning that it first tries to find a matching
    -- customer_id and only after do constraint checks apply.
    -- this leads bad api error responses.
    IF p_customer_id NOT BETWEEN 100000000 AND 999999999 THEN
        RAISE EXCEPTION 'customer_id violates 9-digit constraint'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'customer_id_must_be_9_digits_chk';
    END IF;
    
    RETURN QUERY
    DELETE FROM customer AS c
    WHERE c.customer_id = p_customer_id
    RETURNING c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth::text;
END;
$$;