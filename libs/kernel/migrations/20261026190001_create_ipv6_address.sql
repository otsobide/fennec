-- One row per registered IPv6 address (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the canonical
-- value; the unique value is a safety net while the table is unpartitioned.

CREATE TABLE kernel.ipv6_address (
    id         uuid        PRIMARY KEY,
    value      inet        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv6_address_value_is_host CHECK (family(value) = 6 AND masklen(value) = 128),
    CONSTRAINT ipv6_address_value_key UNIQUE (value)
);
