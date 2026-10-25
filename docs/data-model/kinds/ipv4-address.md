# IPv4 address

One IPv4 host address that Fennec has registered as an observable. Module
`libs/kernel/src/ipv4_address/`, aggregate `Ipv4Address`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `ipv4-addr` |
| OpenCTI | `IPv4-Addr` |
| MISP | `ip-src`, `ip-dst` (role collapsed) |
| Personal data | No |

## Value

`Ipv4AddressValue` wraps a `std::net::Ipv4Addr`.

- **Accepted**: four decimal octets (`198.51.100.7`), optionally followed by
  `/32`, with surrounding whitespace ignored.
- **Canonical form**: the dotted quad without leading zeros and without a
  prefix, as Rust's `Ipv4Addr` displays it: `198.51.100.7`.
- **Rejected**: any prefix other than `/32` (a network belongs to the
  `ipv4_network` kind), octets with leading zeros (`198.051.100.7` is
  ambiguous between decimal and octal), fewer or more than four octets,
  octets above 255, IPv6, defanged spellings (`198.51.100[.]7`) and empty
  values.
- **Typed components**: `is_private()` (RFC 1918), `is_loopback()` and
  `is_documentation()` (RFC 5737).

## Identity

`Ipv4AddressId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical address>"}`:

| Value | Identifier |
|---|---|
| `89.19.209.154` | `ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7` (OpenCTI fixture) |
| `198.51.100.3` | `ipv4-addr--28bb3599-77cd-5a82-a950-b5bc3caf07c4` |

Every accepted spelling of an address gives the same id: `198.51.100.3/32`
and ` 198.51.100.3 ` are the same aggregate. A path id is accepted only if it
is a UUID v5.

## Aggregate

| Field | Value object | Notes |
|---|---|---|
| `id` | `Ipv4AddressId` | Derived from `value` by `Ipv4Address::new`; never supplied |
| `value` | `Ipv4AddressValue` | Canonical address |
| `created_at` | `Ipv4AddressCreatedAt` | When Fennec first registered the address, in microseconds |

Nothing about an IPv4 address changes, so the aggregate has no `updated_at`
and no update use case.

## Events

| Event | Name | Payload |
|---|---|---|
| `Ipv4AddressCreatedEvent` | `fennec.kernel.ipv4_address.created` | `id`, `value`, `created_at` |
| `Ipv4AddressDeletedEvent` | `fennec.kernel.ipv4_address.deleted` | `id`, `value` |

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /ipv4-addresses` | `{"value": "89.19.209.154"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid value |
| `GET /ipv4-addresses/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}` (`created_at` in Unix seconds); `400` if the id is not a UUID v5; `404` |
| `GET /ipv4-addresses?value=<address>` | Any accepted spelling | Same as `GET /ipv4-addresses/{id}` |
| `DELETE /ipv4-addresses/{id}` | | `204`; `400`; `404` |

```bash
curl -i -X POST localhost:8081/ipv4-addresses \
  -H 'Content-Type: application/json' -d '{"value": "89.19.209.154"}'
curl localhost:8081/ipv4-addresses?value=89.19.209.154
```

## Storage

In memory today (`InMemoryIpv4AddressRepository`, keyed by id). The
PostgreSQL table, from [../../architecture/persistence.md](../../architecture/persistence.md):

```sql
CREATE TABLE kernel.ipv4_address (
    id         uuid        PRIMARY KEY,
    value      inet        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv4_address_value_is_host CHECK (family(value) = 4 AND masklen(value) = 32),
    CONSTRAINT ipv4_address_value_key UNIQUE (value)
);
```
