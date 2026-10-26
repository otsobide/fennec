-- One row per registered email address (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the canonical
-- value; the unique value is a safety net while the table is unpartitioned.

CREATE TABLE kernel.email_address (
    id         uuid             PRIMARY KEY,
    value      text COLLATE "C" NOT NULL,
    created_at timestamptz      NOT NULL,
    CONSTRAINT email_address_value_is_canonical CHECK (octet_length(value) BETWEEN 5 AND 254 AND octet_length(split_part(value, '@', 1)) <= 64 AND value ~ '^[^@[:space:][:upper:]]+@[a-z0-9_-]+(\.[a-z0-9_-]+)+$' AND value !~ '(^\.|\.\.|\.@)' AND value !~ '\.[0-9]+$'),
    CONSTRAINT email_address_value_key UNIQUE (value)
);
