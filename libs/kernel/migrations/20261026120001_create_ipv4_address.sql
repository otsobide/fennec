-- One row per registered IPv4 host address (docs/data-model/kinds/ipv4-address.md).
-- The id is the UUID part of the STIX deterministic identifier, derived from
-- the value; the unique value is a safety net while the table is unpartitioned.

CREATE TABLE kernel.ipv4_address (
    id         uuid        PRIMARY KEY,
    value      inet        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv4_address_value_is_host CHECK (family(value) = 4 AND masklen(value) = 32),
    CONSTRAINT ipv4_address_value_key UNIQUE (value)
);
