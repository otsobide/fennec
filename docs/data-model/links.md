# Links between observables

Observables become intelligence when they are connected: a domain resolves
to an address, an autonomous system announces a network, a file contacts a
domain. This page fixes how Fennec models those connections. The design was
chosen among three alternatives (one generic relationship module, one module
per relationship type with one aggregate per source, and the split below);
the reasons are recorded at the end.

## Two kinds of link

| | Intrinsic link | Asserted link |
|---|---|---|
| What it is | A fact the values themselves prove | A claim a source makes at some time |
| Examples | An email address and its domain; a URL and its host; an address inside a network | A domain resolved to an address (passive DNS); an AS announced a prefix; a file contacted a domain |
| Stored | Never | As an aggregate in its own module |
| Comes from | A read model derived from the observables' `created` and `deleted` events | A command: an import, a connector or an analyst |
| Provenance | None needed | One observation per source, with its time window |

An intrinsic link is never registered, because it cannot be wrong and
cannot change: `john@example.com` belongs to `example.com` by its value
alone. Storing it would only create a second copy that can drift.

## One module per asserted fact

Each asserted fact is its own `kernel` module, built like an observable
kind and listed in the link catalogue below: `dns_resolution` first. There
is no generic relationship aggregate and no `relationship_type` column,
for the same reason there is no generic observable (non-negotiable 1 in
[README.md](README.md)): every fact has its own endpoints and its own
attributes, and a closed set of modules keeps them typed.

The endpoints are typed by value, not only by id. A link stores the
canonical values of its endpoints (`text COLLATE "C"` for names, `inet` for
addresses) next to their module kind, and derives their ids with the same
algorithm as the observable modules. In regime A an id and a value are one
to one, so storing the value adds no ambiguity, and it lets a link query
return names without reading another module and filter by network
(`answer_value << '198.51.100.0/24'`).

## The aggregate and its observations

- The **link** is the root: its id, its kind-qualified endpoints and when
  Fennec first registered it. It never changes.
- Each **observation** is a child entity, one per source: `source_id`,
  `first_seen`, `last_seen`, `count`, and when Fennec created and last
  changed it.
- A link has at least one observation. Retracting the last one deletes
  the link.

Recording an observation merges it into the source's existing one:

- `first_seen` takes the earliest instant and `last_seen` the latest;
- `count` is the number of times the source saw the fact, cumulative and
  optional; the stored count is the larger of the stored and the reported
  one, so sending the same report twice changes nothing;
- when nothing changes, no event is published, and the transactional
  command bus rolls the unit of work back without an error.

## Identity

A link is regime A: its id is a UUID v5 derived from its kind and its
kind-qualified endpoint ids, so every source and every retry that reports
the same fact converges on one aggregate.

- **Namespace**: `eda8415d-6a75-5fdc-9eb0-72698098a92f`, the UUID v5 of
  `urn:fennec:link` in the RFC 4122 URL namespace. STIX 2.1 says
  relationship objects "MUST NOT use a namespace of
  00abedb4-aa42-466c-9c01-fed23315a9b7 if generating a UUIDv5" (see
  [../research/stix-sco-schemas.md](../research/stix-sco-schemas.md)), so
  a link exported as a STIX `relationship` keeps this id under its own
  namespace.
- **Name**: the RFC 8785 serialisation of
  `{"answer": {"id": "<uuid>", "kind": "<module>"}, "kind": "<link module>", "subject": {"id": "<uuid>", "kind": "<module>"}}`.
- Time and source are not part of the id: "resolved in 2019 and again in
  2024" is one link whose observations and `observed` events keep the
  history.

| Link | Identifier |
|---|---|
| `dns_resolution` of `example.com` to `93.184.216.34` | `2da44331-2011-5539-b65e-9ff44db9c139` |

## Endpoints, integrity and deletion

- A link never imports an observable module. It parses its endpoint
  values with the shared primitives (`DnsName`, `std::net`), and
  conformance tests in `tests/` assert that it derives the same canonical
  value and id as the observable module for every sample.
- Recording a link does not check that its endpoints are registered:
  checking would take each endpoint's advisory lock and queue every
  ingestion towards a shared address (a CDN, a sinkhole, a parking
  service). Importers register observables first, as a STIX bundle lists
  them.
- Deleting an observable leaves its links in place for now. A cascade
  arrives with the first consumer of the kernel event log, which has to
  exist before any link reaches a kind marked as personal data.

## Untyped links

`related-to`, MISP's free `relationship_type` strings and similar untyped
links are rejected and reported on import, as untyped observables are. A
link that cannot say what it means is not a fact Fennec can reason about.

## Link catalogue

| Module | Meaning | Subject | Answer or target | STIX `relationship_type` | OpenCTI | MISP | Status |
|---|---|---|---|---|---|---|---|
| `dns_resolution` | A domain name resolved to an address | `domain_name` | `ipv4_address`, `ipv6_address` | `resolves-to` | `resolves-to` | `domain\|ip`, `a-record` objects | planned |
| `dns_alias` | A domain name is a CNAME of another | `domain_name` | `domain_name` | `resolves-to` | `resolves-to` | `cname` in `dns-record` objects | planned |
| `autonomous_system_announcement` | An autonomous system announced a network | `autonomous_system` | `ipv4_network`, `ipv6_network` | `belongs-to` (inverse) | `belongs-to` | `asn` objects | planned |

## Why this design

- A **generic relationship module** would need a `type` column and a table
  of allowed combinations, which the non-negotiables forbid, and it cannot
  give one link type attributes of its own (a TTL, a port).
- **One aggregate per source** makes the same fact N aggregates for N
  sources and leaves "is this fact known?" to a `GROUP BY`; one aggregate
  per fact gives a stable id that indicators and assessments can point at.
- **Storing intrinsic links** would duplicate what the values already say.
