# IPv6 address

One IPv6 host address that Fennec has registered as an observable. Module
`libs/kernel/src/ipv6_address/`, aggregate `Ipv6Address`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `ipv6-addr` |
| OpenCTI | `IPv6-Addr` |
| MISP | `ip-src`, `ip-dst` (role collapsed) |
| Personal data | No |

## Value

`Ipv6AddressValue` wraps a `std::net::Ipv6Addr`.

- **Accepted**: every RFC 4291 text form (full, compressed with `::`, mixed
  with a dotted IPv4 tail, any case, groups with leading zeros), optionally
  followed by `/128`, with surrounding ASCII whitespace ignored.
- **Canonical form**: the RFC 5952 text, as Rust's `Ipv6Addr` displays it:
  lowercase hexadecimal, leading zeros dropped, the longest run of two or
  more zero groups compressed to `::` (the leftmost on a tie, a single zero
  group never compressed), and IPv4-mapped addresses written with a dotted
  tail (`::ffff:192.0.2.1`). Other dotted tails are written in hexadecimal:
  `64:ff9b::192.0.2.1` becomes `64:ff9b::c000:201`.
- **Rejected**: any prefix other than `/128` (a network belongs to the
  `ipv6_network` kind), malformed prefixes (`/0128`, `/+128`), zone indices
  (`fe80::1%eth0`: an interface name is local to one host and says nothing
  about the address), brackets (`[2001:db8::7]` is URL host syntax), groups
  of more than four digits, more or fewer than eight groups, two `::`,
  IPv4 addresses, defanged spellings and empty values.
- **Typed components**: `is_loopback()`, `is_documentation()` (RFC 3849,
  `2001:db8::/32`), `is_unique_local()` (RFC 4193, `fc00::/7`),
  `is_link_local()` (`fe80::/10`) and `ipv4_mapped()`, which returns the
  IPv4 address an IPv4-mapped address stands for.

An IPv4-mapped address stays an IPv6 address: Fennec never turns one kind
into another. The link between `::ffff:192.0.2.1` and `192.0.2.1` belongs to
a relationship or a read model.

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`Ipv6AddressId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical address>"}`:

| Value | Identifier |
|---|---|
| `2001:db8::1` | `ipv6-addr--6469e3a9-b053-5e34-a025-9396ae051d26` (OpenCTI, compressed input) |
| `2001:db8:85a3::8a2e:370:7334` | `ipv6-addr--f5344921-72d2-5619-945d-8757bca4b7cd` |
| `::ffff:192.0.2.1` | `ipv6-addr--3cbdde9e-d72f-52ab-9a38-fc8b2fd4714d` |

`2001:0DB8:85A3:0000:0000:8A2E:0370:7334/128` and
` 2001:db8:85a3::8a2e:370:7334 ` are the same aggregate as
`2001:db8:85a3::8a2e:370:7334`. OpenCTI only trims IPv6 values, so the
expanded spelling `2001:0db8:0000:0000:0000:0000:0000:0001` gets the
OpenCTI id `ipv6-addr--6607d35d-eadd-5c67-b67a-c8bdcdf90d45`, which the
importer records as an alias of Fennec's `2001:db8::1` (see
[../interoperability.md](../interoperability.md#import)). A path id is
accepted only if it is a UUID v5.

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.ipv6_address.created` carries `id`, `value`
and `created_at`; `fennec.kernel.ipv6_address.deleted` carries `id` and
`value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /ipv6-addresses` | `{"value": "2001:db8::1"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid value |
| `GET /ipv6-addresses/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /ipv6-addresses?value=<address>` | Any accepted spelling, with `/` percent-encoded as `%2F` | Same as `GET /ipv6-addresses/{id}` |
| `DELETE /ipv6-addresses/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryIpv6AddressRepository` and `SqlxPostgresIpv6AddressRepository` run
the same repository contract. The PostgreSQL repository binds the address
as an `inet` host, so PostgreSQL compares addresses, not spellings. Its
table, from `libs/kernel/migrations/20261026190001_create_ipv6_address.sql`:

```sql
CREATE TABLE kernel.ipv6_address (
    id         uuid        PRIMARY KEY,
    value      inet        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv6_address_value_is_host CHECK (family(value) = 6 AND masklen(value) = 128),
    CONSTRAINT ipv6_address_value_key UNIQUE (value)
);
```

Containment in networks (every address inside `2001:db8::/32`) belongs to a
read model; the write side only looks addresses up by id.
