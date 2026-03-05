CREATE OR REPLACE FUNCTION update_customer(
    p_customer_id customer.customer_id%TYPE,
    p_first_name customer.first_name%TYPE,
    p_middle_name customer.middle_name%TYPE,
    p_last_name customer.last_name%TYPE,
    p_date_of_birth customer.date_of_birth%TYPE
)
RETURNS TABLE (
    old_customer_id customer.customer_id%TYPE,
    old_first_name customer.first_name%TYPE,
    old_middle_name customer.middle_name%TYPE,
    old_last_name customer.last_name%TYPE,
    old_date_of_birth TEXT,
    new_customer_id customer.customer_id%TYPE,
    new_first_name customer.first_name%TYPE,
    new_middle_name customer.middle_name%TYPE,
    new_last_name customer.last_name%TYPE,
    new_date_of_birth TEXT
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
    WITH old_row AS (
        SELECT c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth
        FROM customer AS c
        WHERE c.customer_id = p_customer_id
        FOR UPDATE
    ),
    updated AS (
        UPDATE customer AS c
        SET
            first_name = CASE
                WHEN p_first_name IS NULL THEN c.first_name
                ELSE NULLIF(BTRIM(p_first_name), '')
            END,
            middle_name = CASE
                WHEN p_middle_name IS NULL THEN c.middle_name
                ELSE NULLIF(BTRIM(p_middle_name), '')
            END,
            last_name = CASE
                WHEN p_last_name IS NULL THEN c.last_name
                ELSE NULLIF(BTRIM(p_last_name), '')
            END,
            date_of_birth = COALESCE(p_date_of_birth, c.date_of_birth)
        FROM old_row AS o
        WHERE c.customer_id = o.customer_id
        RETURNING c.customer_id, c.first_name, c.middle_name, c.last_name, c.date_of_birth
    )
    SELECT
        o.customer_id,
        o.first_name,
        o.middle_name,
        o.last_name,
        o.date_of_birth::text,
        u.customer_id,
        u.first_name,
        u.middle_name,
        u.last_name,
        u.date_of_birth::text
    FROM old_row AS o
    JOIN updated AS u ON u.customer_id = o.customer_id;
END;
$$;
