# DNS resolution

The asserted link "a domain name resolved to an address", with one
observation per source that reported it. Module
`libs/kernel/src/dns_resolution/`, aggregate `DnsResolution`. The rules every
link follows are in [../links.md](../links.md).

| Property | Value |
|---|---|
| Subject | `domain_name` |
| Answer | `ipv4_address` or `ipv6_address` |
| Identity | UUID v5 in the link namespace over the kind-qualified endpoints |
| STIX | `relationship` with `relationship_type` `resolves-to` |
| OpenCTI | `resolves-to` |
| MISP | `domain\|ip` attributes and `a-record` values of `dns-record` objects |

## Endpoints

- **Subject**: `DnsResolutionSubject::DomainName` is parsed with the rules of
  the `domain_name` module (a shared `DnsName` with at least two labels,
  surrounding whitespace and one root dot dropped, A-labels).
- **Answer**: `DnsResolutionAnswer::{Ipv4Address, Ipv6Address}` is parsed with
  the rules of the address modules (an IPv4 dotted quad optionally `/32`;
  any RFC 4291 form optionally `/128`, no zone index). The caller states the
  kind through the field it uses; it is never inferred.
- Both derive the id their own module would, and conformance tests in
  `tests/libs/kernel/src/dns_resolution/domain/value_objects/` assert, for
  more than a hundred spellings, the same acceptance, canonical value, id
  and error message as `DomainNameValue`, `Ipv4AddressValue` and
  `Ipv6AddressValue`.

## Identity

`DnsResolutionId` is the UUID v5, in the namespace
`eda8415d-6a75-5fdc-9eb0-72698098a92f`, of the RFC 8785 serialisation of
`{"answer": {"id": "<uuid>", "kind": "<module>"}, "kind": "dns_resolution", "subject": {"id": "<uuid>", "kind": "domain_name"}}`.

| Resolution | Identifier |
|---|---|
| `example.com` to `93.184.216.34` | `relationship--2da44331-2011-5539-b65e-9ff44db9c139` |

`Example.COM.` to `93.184.216.34/32` is the same link.

## Aggregate and observations

| Field | Value object | Notes |
|---|---|---|
| `id` | `DnsResolutionId` | Derived from the endpoints; never supplied |
| `subject` | `DnsResolutionSubject` | Never changes |
| `answer` | `DnsResolutionAnswer` | Never changes |
| `created_at` | `DnsResolutionCreatedAt` | When Fennec first recorded the link |
| `observations` | `DnsResolutionObservation`, one per source | At least one |

Each observation holds the `source_id` (`DnsResolutionSourceId`, a UUID v4),
a `DnsResolutionWindow` (`first_seen` <= `last_seen`, client instants bounded
to 1970..9999), a `DnsResolutionCount` (cumulative, at least 1) and its
created and updated instants.

- `record` creates the link with its first observation; a missing count is 1
  and a missing `last_seen` is `first_seen`.
- `observe` merges a report into the source's observation (earliest
  `first_seen`, latest `last_seen`, larger count) or adds a new source's
  observation, and returns `None` when the report changes nothing.
- `retract` removes a source's observation; retracting the last one deletes
  the link.

## Events

| Event | Name | Payload |
|---|---|---|
| `DnsResolutionCreatedEvent` | `fennec.kernel.dns_resolution.created` | `id` and, for both endpoints, `*_kind`, `*_value`, `*_id`; `created_at` |
| `DnsResolutionObservedEvent` | `fennec.kernel.dns_resolution.observed` | `id`, `source_id`, the merged `first_seen`, `last_seen`, `count`, the `reported_first_seen`, `reported_last_seen`, `reported_count` (`null` when absent) and `updated_at` |
| `DnsResolutionRetractedEvent` | `fennec.kernel.dns_resolution.retracted` | `id`, `source_id` |
| `DnsResolutionDeletedEvent` | `fennec.kernel.dns_resolution.deleted` | `id` and both endpoints |

A new link publishes `created` then `observed`; a merge publishes
`observed`; a report that changes nothing publishes nothing, and on
PostgreSQL the transactional command bus rolls the unit of work back without
an error. Because `observed` carries the report as well as the merged
window, the log keeps "seen in January 2019 and again in March 2024" even
though the observation only stores the window that covers both.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /dns-resolutions` | `{"domain_name", exactly one of "ipv4_address" or "ipv6_address", "source_id", "first_seen", "last_seen"?, "count"?}` (unix seconds; any other field is rejected) | `201` with `Location` and `{"id", "stix_id", "relationship_type", "outcome": "created"}`; `200` with `"outcome": "observed"` or `"unchanged"`; `400` |
| `GET /dns-resolutions/{id}` | | `200` with `id`, `stix_id`, `relationship_type`, `subject` and `answer` (`{kind, value, id}`), the overall `first_seen` and `last_seen`, `created_at` and `observations`; `400` if the id is not a UUID v5; `404` |
| `GET /dns-resolutions?domain_name=` or `?ipv4_address=` or `?ipv6_address=` | Exactly one | `200` with `{"dns_resolutions": [...], "truncated"}`, at most 1000 links, ordered by address or by name |
| `GET /dns-resolutions/co-resolved?domain_name=` | | `200` with the domain names that resolved to the same addresses in overlapping windows, their shared answers and `truncated` (at most 100 answers of the queried name and 1000 links per answer) |
| `DELETE /dns-resolutions/{id}/observations/{source_id}` | | `204`; `400`; `404` when the link or the source's observation is absent |
| `DELETE /dns-resolutions/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryDnsResolutionRepository` and `SqlxPostgresDnsResolutionRepository`
run one repository contract. The aggregate spans two tables, from
`libs/kernel/migrations/20261027000001_create_dns_resolution.sql`:

- `kernel.dns_resolution`: `id`, `subject_kind`, `subject_value`
  (`text COLLATE "C"`), `answer_kind`, `answer_value` (`inet`) and
  `created_at`, with named CHECKs that repeat the canonical forms
  (`dns_resolution_subject_is_domain_name`,
  `dns_resolution_answer_matches_kind`), a unique key on
  `(subject_value, answer_value)` and an index on
  `(answer_value, subject_value)` for the reverse lookups.
- `kernel.dns_resolution_observation`: one row per `(dns_resolution_id,
  source_id)` with `first_seen`, `last_seen`, `count`, `created_at` and
  `updated_at`, the CHECKs `dns_resolution_observation_window_is_ordered`
  and `dns_resolution_observation_count_is_positive`, and `ON DELETE
  CASCADE` from its link.

Every write takes the link's advisory lock, so two sources recording the
same new link converge: one row, two observations, one `created` and two
`observed` events (tested on PostgreSQL). Storing the endpoint values lets a
query return names without reading another module and filter by network
(`answer_value << '198.51.100.0/24'`).
