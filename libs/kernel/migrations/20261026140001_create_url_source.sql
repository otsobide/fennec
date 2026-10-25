-- The URL payload of a source, keyed by the same id as kernel.source
-- (docs/architecture/entities.md). No foreign key: modules relate by shared
-- identifier only. The CHECKs repeat the value object invariants.

CREATE TABLE kernel.url_source (
    id                       uuid             PRIMARY KEY,
    url                      text COLLATE "C" NOT NULL,
    format                   text COLLATE "C" NOT NULL,
    polling_interval_seconds bigint           NOT NULL,
    created_at               timestamptz      NOT NULL,
    updated_at               timestamptz      NOT NULL,
    CONSTRAINT url_source_url_is_http CHECK (url ~ '^https?://' AND char_length(url) <= 2048),
    CONSTRAINT url_source_format_known CHECK (format IN ('plain', 'csv', 'json', 'stix')),
    CONSTRAINT url_source_polling_interval_range
        CHECK (polling_interval_seconds BETWEEN 1 AND 4294967295),
    CONSTRAINT url_source_updated_at_after_created_at CHECK (updated_at >= created_at)
);
