-- One row per registered IPv4 network (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the canonical
-- value; the unique value is a safety net while the table is unpartitioned.
-- The cidr type refuses set host bits and stores one canonical form; the
-- CHECK keeps the family and the prefix range of the kind.

CREATE TABLE kernel.ipv4_network (
    id         uuid        PRIMARY KEY,
    value      cidr        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv4_network_value_is_network CHECK (family(value) = 4 AND masklen(value) < 32),
    CONSTRAINT ipv4_network_value_key UNIQUE (value)
);
