-- One row per registered autonomous system (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the number;
-- the unique value is a safety net while the table is unpartitioned.

CREATE TABLE kernel.autonomous_system (
    id         uuid        PRIMARY KEY,
    value      bigint      NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT autonomous_system_value_is_in_range CHECK (value BETWEEN 0 AND 4294967295),
    CONSTRAINT autonomous_system_value_key UNIQUE (value)
);
