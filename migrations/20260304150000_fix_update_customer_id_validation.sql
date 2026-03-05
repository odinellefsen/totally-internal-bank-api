CREATE OR REPLACE FUNCTION update_customer(
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
    IF p_customer_id NOT BETWEEN 100000000 AND 999999999 THEN
        RAISE EXCEPTION 'customer_id violates 9-digit constraint'
            USING ERRCODE = '23514',
                  CONSTRAINT = 'customer_id_must_be_9_digits_chk';
    END IF;

    RETURN QUERY
    UPDATE customer AS c
    SET
        first_name = NULLIF(BTRIM(p_first_name), ''),
        middle_name = NULLIF(BTRIM(p_middle_name), ''),
        last_name = NULLIF(BTRIM(p_last_name), ''),
        date_of_birth = p_date_of_birth
    WHERE c.customer_id = p_customer_id
    RETURNING c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth::text;
END;
$$;
