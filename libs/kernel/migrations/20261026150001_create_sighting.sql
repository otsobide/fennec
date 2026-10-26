-- One row per (observable kind, observable id, source id): the observations
-- of one observable by one source (docs/architecture/entities.md). No
-- foreign keys: modules relate by kind-qualified identifier only. The
-- CHECKs repeat the value object invariants.

CREATE TABLE kernel.sighting (
    id              uuid             PRIMARY KEY,
    observable_kind text COLLATE "C" NOT NULL,
    observable_id   uuid             NOT NULL,
    source_id       uuid             NOT NULL,
    first_seen      timestamptz      NOT NULL,
    last_seen       timestamptz      NOT NULL,
    count           bigint           NOT NULL,
    created_at      timestamptz      NOT NULL,
    updated_at      timestamptz      NOT NULL,
    CONSTRAINT sighting_observable_kind_is_module_name
        CHECK (observable_kind ~ '^[a-z][a-z0-9]*(_[a-z0-9]+)*$' AND char_length(observable_kind) <= 64),
    CONSTRAINT sighting_count_positive CHECK (count >= 1),
    CONSTRAINT sighting_last_seen_after_first_seen CHECK (last_seen >= first_seen),
    CONSTRAINT sighting_updated_at_after_created_at CHECK (updated_at >= created_at),
    CONSTRAINT sighting_observable_source_key UNIQUE (observable_kind, observable_id, source_id)
);

-- Listing by observable uses the leading columns of the unique key; listing
-- by source needs its own index.
CREATE INDEX sighting_source_id_idx ON kernel.sighting (source_id);
