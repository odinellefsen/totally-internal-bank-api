-- Add migration script here
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE account (
	account_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
	account_number BIGINT NOT NULL,
	account_type TEXT NOT NULL
        CONSTRAINT account_type_allowed_chk
        CHECK (account_type IN ('standard')),
	current_balance NUMERIC(14,2) NOT NULL DEFAULT 0
        CONSTRAINT account_current_balance_nonnegative_chk
        CHECK (current_balance >= 0),
    interest_rate NUMERIC(5,4) NOT NULL DEFAULT 0
        CONSTRAINT account_interest_rate_range_chk
        CHECK (interest_rate >= 0 AND interest_rate <= 1),
    CONSTRAINT account_number_unique_uq
    UNIQUE (account_number),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);