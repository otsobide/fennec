-- One row per DNS resolution, the asserted link from a domain name to the
-- address it resolved to (docs/data-model/links.md), and one child row per
-- source that observed it. The id is a UUID v5 in the link namespace,
-- derived from the kind-qualified endpoints; the unique endpoints are a
-- safety net while the table is unpartitioned. The endpoints are stored by
-- module kind and canonical value, with no foreign key to the observable
-- tables: modules relate by kind-qualified identifier only, and recording a
-- link does not require its endpoints to be registered. The CHECKs repeat
-- the value object invariants.

CREATE TABLE kernel.dns_resolution (
    id            uuid             PRIMARY KEY,
    subject_kind  text             NOT NULL,
    subject_value text COLLATE "C" NOT NULL,
    answer_kind   text             NOT NULL,
    answer_value  inet             NOT NULL,
    created_at    timestamptz      NOT NULL,
    -- The subject follows the canonical form of kernel.domain_name.
    CONSTRAINT dns_resolution_subject_is_domain_name CHECK (
        subject_kind = 'domain_name'
        AND char_length(subject_value) BETWEEN 3 AND 253
        AND subject_value ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)+$'
        AND subject_value !~ '\.[0-9]+$'
    ),
    -- The answer is a host address of the family its kind names, as in
    -- kernel.ipv4_address and kernel.ipv6_address.
    CONSTRAINT dns_resolution_answer_matches_kind CHECK (
        (answer_kind = 'ipv4_address' AND family(answer_value) = 4 AND masklen(answer_value) = 32)
        OR (answer_kind = 'ipv6_address' AND family(answer_value) = 6 AND masklen(answer_value) = 128)
    ),
    CONSTRAINT dns_resolution_subject_answer_key UNIQUE (subject_value, answer_value)
);

-- Listing the addresses a name resolved to uses the unique key; listing the
-- names that resolved to an address needs the reverse index.
CREATE INDEX dns_resolution_answer_subject_idx ON kernel.dns_resolution (answer_value, subject_value);

-- One row per source that observed a resolution. A resolution always has at
-- least one; deleting it deletes them.
CREATE TABLE kernel.dns_resolution_observation (
    dns_resolution_id uuid        NOT NULL REFERENCES kernel.dns_resolution (id) ON DELETE CASCADE,
    source_id         uuid        NOT NULL,
    first_seen        timestamptz NOT NULL,
    last_seen         timestamptz NOT NULL,
    count             bigint      NOT NULL,
    created_at        timestamptz NOT NULL,
    updated_at        timestamptz NOT NULL,
    CONSTRAINT dns_resolution_observation_window_is_ordered CHECK (first_seen <= last_seen),
    CONSTRAINT dns_resolution_observation_count_is_positive CHECK (count >= 1),
    PRIMARY KEY (dns_resolution_id, source_id)
);
