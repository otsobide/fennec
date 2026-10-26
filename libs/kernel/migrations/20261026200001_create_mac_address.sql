-- One row per registered MAC address (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the canonical
-- value; the unique value is a safety net while the table is unpartitioned.
-- The macaddr type stores every spelling in one canonical form, so the table
-- needs no CHECK of its own.

CREATE TABLE kernel.mac_address (
    id         uuid        PRIMARY KEY,
    value      macaddr     NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT mac_address_value_key UNIQUE (value)
);
