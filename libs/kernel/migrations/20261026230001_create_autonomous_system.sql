-- One row per registered autonomous system (docs/data-model/kinds/). The id is
-- the UUID part of the STIX deterministic identifier, derived from the number;
-- the unique value is a safety net while the table is unpartitioned. The name
-- and the registry describe the system and may change; they never change its
-- id.

CREATE TABLE kernel.autonomous_system (
    id         uuid        PRIMARY KEY,
    value      bigint      NOT NULL,
    name       text,
    rir        text,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL,
    CONSTRAINT autonomous_system_value_is_in_range CHECK (value BETWEEN 0 AND 4294967295),
    CONSTRAINT autonomous_system_name_is_valid CHECK (
        char_length(name) BETWEEN 1 AND 255 AND name !~ '[[:cntrl:]]'
    ),
    CONSTRAINT autonomous_system_rir_is_known CHECK (
        rir IN ('AFRINIC', 'APNIC', 'ARIN', 'LACNIC', 'RIPE NCC')
    ),
    CONSTRAINT autonomous_system_value_key UNIQUE (value)
);
