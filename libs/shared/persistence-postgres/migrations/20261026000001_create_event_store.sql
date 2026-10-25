-- The append-only domain event log, written in the same transaction as the
-- aggregate state (docs/architecture/event-store.md).

CREATE TABLE event_store.domain_event (
    position       bigint      GENERATED ALWAYS AS IDENTITY,
    transaction_id xid8        NOT NULL DEFAULT pg_current_xact_id(),
    recorded_at    timestamptz NOT NULL DEFAULT now(),
    occurred_on    timestamptz NOT NULL,
    event_id       uuid        NOT NULL,
    event_name     text COLLATE "C" NOT NULL,
    aggregate_type text COLLATE "C" NOT NULL,
    aggregate_id   uuid        NOT NULL,
    payload        jsonb       NOT NULL,
    CONSTRAINT domain_event_pkey PRIMARY KEY (position, recorded_at),
    CONSTRAINT domain_event_event_name_shape
        CHECK (event_name ~ '^fennec\.[a-z0-9_]+\.[a-z0-9_]+\.[a-z_]+$')
) PARTITION BY RANGE (recorded_at);

CREATE INDEX domain_event_transaction_position_idx
    ON event_store.domain_event (transaction_id, position);
CREATE INDEX domain_event_aggregate_idx
    ON event_store.domain_event (aggregate_type, aggregate_id, position);
CREATE INDEX domain_event_event_id_idx
    ON event_store.domain_event (event_id);

CREATE TABLE event_store.consumer_checkpoint (
    consumer_name       text COLLATE "C" PRIMARY KEY,
    last_transaction_id xid8        NOT NULL DEFAULT '0',
    last_position       bigint      NOT NULL DEFAULT 0,
    updated_at          timestamptz NOT NULL DEFAULT now()
);

-- Creates the monthly partitions from the current UTC month to months_ahead
-- months later. There is no default partition: a write outside every
-- partition must fail loudly. Concurrent callers are serialised.
CREATE FUNCTION event_store.ensure_partitions(months_ahead integer) RETURNS void
LANGUAGE plpgsql AS $$
DECLARE
    first_month    date := date_trunc('month', now() AT TIME ZONE 'UTC')::date;
    partition_from date;
    partition_to   date;
    partition_name text;
BEGIN
    PERFORM pg_advisory_xact_lock(hashtextextended('fennec.event_store.ensure_partitions', 0));

    FOR month_offset IN 0..months_ahead LOOP
        partition_from := (first_month + make_interval(months => month_offset))::date;
        partition_to   := (partition_from + interval '1 month')::date;
        partition_name := format('domain_event_%s', to_char(partition_from, 'YYYY_MM'));

        IF to_regclass(format('event_store.%I', partition_name)) IS NULL THEN
            EXECUTE format(
                'CREATE TABLE event_store.%I PARTITION OF event_store.domain_event
                     FOR VALUES FROM (%L) TO (%L)',
                partition_name,
                partition_from::text || ' 00:00:00+00',
                partition_to::text || ' 00:00:00+00'
            );
        END IF;
    END LOOP;
END $$;

SELECT event_store.ensure_partitions(12);
