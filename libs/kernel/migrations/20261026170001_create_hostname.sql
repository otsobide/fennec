-- One row per registered hostname (docs/data-model/kinds/). The id is the
-- UUID part of the STIX deterministic identifier, derived from the canonical
-- value; the unique value is a safety net while the table is unpartitioned.

CREATE TABLE kernel.hostname (
    id         uuid             PRIMARY KEY,
    value      text COLLATE "C" NOT NULL,
    created_at timestamptz      NOT NULL,
    CONSTRAINT hostname_value_is_canonical CHECK (char_length(value) BETWEEN 1 AND 253 AND value ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)*$' AND value !~ '(^|\.)[0-9]+$'),
    CONSTRAINT hostname_value_key UNIQUE (value)
);
