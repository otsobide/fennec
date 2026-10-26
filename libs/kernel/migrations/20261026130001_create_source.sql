-- One row per intelligence source (docs/architecture/entities.md). The CHECKs
-- repeat the value object invariants; a new source type or status needs a
-- migration that widens them.

CREATE TABLE kernel.source (
    id          uuid             PRIMARY KEY,
    source_type text COLLATE "C" NOT NULL,
    status      text COLLATE "C" NOT NULL,
    description text             NOT NULL,
    created_at  timestamptz      NOT NULL,
    updated_at  timestamptz      NOT NULL,
    CONSTRAINT source_source_type_known CHECK (source_type IN ('url')),
    CONSTRAINT source_status_known CHECK (status IN ('active', 'inactive')),
    CONSTRAINT source_description_length CHECK (char_length(description) <= 1024),
    CONSTRAINT source_updated_at_after_created_at CHECK (updated_at >= created_at)
);
