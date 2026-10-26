# Autonomous system

One autonomous system, identified by its number, that Fennec has registered
as an observable. Module `libs/kernel/src/autonomous_system/`, aggregate
`AutonomousSystem`. It is the first value-identified kind with a description
that can change: its name and its Regional Internet Registry.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `autonomous-system` |
| OpenCTI | `Autonomous-System` |
| MISP | `AS` |
| Personal data | No |

## Value

`AutonomousSystemValue` holds the AS number as a `u32`.

- **Accepted**, with surrounding ASCII whitespace ignored:
  - the plain decimal number (`15169`), leading zeros allowed;
  - the number after an `AS` prefix in any case and without a space
    (`AS15169`, `as15169`), as registries and most feeds write it;
  - the asdot notation of RFC 5396, two numbers from 0 to 65535 joined by a
    dot, the first counting units of 65536 (`1.10` is `65546`), with or
    without the prefix.
- **Range**: 0 to 4294967295, the 32-bit numbers of RFC 6793.
- **Canonical form**: the number. It displays as plain decimal (`15169`),
  `label()` gives `AS15169`, and the STIX identifier hashes it as the JSON
  integer `number`, not as a string.
- **Rejected**: numbers above the range, asdot halves above 65535, signs,
  spaces inside the value (`AS 15169`), separators (`15,169`), hexadecimal,
  non-ASCII digits and empty values.
- **Typed components**: `number()`, `label()`, `is_four_byte()` (above
  65535), `is_private_use()` (RFC 6996) and `is_documentation()`
  (RFC 5398).

The HTTP API carries the number as a JSON string in `value`, like every
other kind, so `"AS15169"` and `"1.10"` are accepted as written.

## Description

The name and the registry describe an autonomous system; neither is part of
its identity, and both start empty.

- `AutonomousSystemName` is the name it is registered under (`GOOGLE`,
  `CLOUDFLARENET`), the STIX `name`. Surrounding whitespace is dropped and
  the case is kept; it must not be empty, must not hold control characters
  (a line break would let one record spoof another in exports) and has at
  most 255 characters.
- `AutonomousSystemRir` is the registry that assigned the number, the STIX
  `rir`. STIX leaves it as free text; Fennec accepts only the five
  registries, ignoring case and the spaces, hyphens or underscores inside
  the name: `AFRINIC`, `APNIC`, `ARIN`, `LACNIC` and `RIPE NCC` (also
  `RIPE`, `ripe-ncc`).

Changing the number rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).
Changing the description rules does not.

## Identity

`AutonomousSystemId` is the UUID part of the STIX identifier, computed over
`{"number": <number>}`:

| Value | Identifier |
|---|---|
| `15169` | `autonomous-system--7afc7bdf-d070-5add-90ff-f93503b1654d` (OpenCTI fixture) |
| `13335` | `autonomous-system--0204c07d-e4dd-5f14-a3d5-c93cb1c5a9fc` |

`AS15169`, `as15169` and `0.15169` are the same aggregate as `15169`. A path
id is accepted only if it is a UUID v5.

## Aggregate and events

| Field | Value object | Notes |
|---|---|---|
| `id` | `AutonomousSystemId` | Derived from `value`; never supplied |
| `value` | `AutonomousSystemValue` | The number |
| `name` | `Option<AutonomousSystemName>` | Empty until described |
| `rir` | `Option<AutonomousSystemRir>` | Empty until described |
| `created_at` | `AutonomousSystemCreatedAt` | When Fennec first registered the number |
| `updated_at` | `AutonomousSystemUpdatedAt` | When the description last changed; the creation instant until then |

`AutonomousSystem::new` registers a number without a description;
`describe` returns the same aggregate with a new name, registry and update
instant; `restore` rebuilds one from a row.

| Event | Name | Payload |
|---|---|---|
| `AutonomousSystemCreatedEvent` | `fennec.kernel.autonomous_system.created` | `id`, `value`, `created_at` |
| `AutonomousSystemUpdatedEvent` | `fennec.kernel.autonomous_system.updated` | `id`, `value`, `new_name`, `old_name`, `new_rir`, `old_rir`, `updated_at`; an absent field is `null` |
| `AutonomousSystemDeletedEvent` | `fennec.kernel.autonomous_system.deleted` | `id`, `value` |

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /autonomous-systems` | `{"value": "AS15169"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid number |
| `GET /autonomous-systems/{id}` | | `200` with `{"id", "stix_id", "value", "name", "rir", "created_at", "updated_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /autonomous-systems?value=<number>` | Any accepted spelling | Same as `GET /autonomous-systems/{id}` |
| `PUT /autonomous-systems/{id}` | `{"name": "GOOGLE", "rir": "ARIN"}`; a missing or `null` field clears it; any other field, such as `value`, is rejected | `200`; `400` for an invalid id, name or registry; `404` |
| `DELETE /autonomous-systems/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryAutonomousSystemRepository` and
`SqlxPostgresAutonomousSystemRepository` run the same repository contract,
which also covers keeping and replacing the description and updating an
unknown number. The table, from
`libs/kernel/migrations/20261026230001_create_autonomous_system.sql`,
stores the number as a `bigint` (PostgreSQL has no unsigned 32-bit type)
and repeats the range, name and registry rules in `CHECK` constraints, each
tested by name:

```sql
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
```

Prefixes announced by an autonomous system and its peers are relationships
to `ipv4_network` and `ipv6_network` aggregates, not columns here.
