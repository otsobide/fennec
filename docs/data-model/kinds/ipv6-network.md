# IPv6 network

One IPv6 network, a CIDR block, that Fennec has registered as an
observable. Module `libs/kernel/src/ipv6_network/`, aggregate
`Ipv6Network`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `ipv6-addr` (shared with `ipv6_address`) |
| OpenCTI | `IPv6-Addr` |
| MISP | `ip-src`, `ip-dst` with a CIDR value (role collapsed) |
| Personal data | No |

## Value

`Ipv6NetworkValue` holds the network address and the prefix length.

- **Accepted**: any RFC 4291 text form of the network address followed by a
  prefix length from `/0` to `/127`, with surrounding ASCII whitespace
  ignored.
- **Canonical form**: the RFC 5952 text of the address (see
  [ipv6-address.md](ipv6-address.md#value)), a slash and the prefix length:
  `2001:db8::/32`.
- **Rejected**: a value without a prefix and a `/128` (a single host, which
  is an `ipv6_address`); an address with host bits set (`2001:db8::7/32`),
  for the reasons given in [ipv4-network.md](ipv4-network.md#value), with
  the error naming the network; malformed prefixes, zone indices, brackets,
  IPv4 and empty values.
- **Typed components**: `network_address()`, `prefix_length()`,
  `last_address()`, `contains()`, `is_documentation()` (the whole block
  inside `2001:db8::/32`) and `is_unique_local()` (inside `fc00::/7`).

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`Ipv6NetworkId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical block>"}`:

| Value | Identifier |
|---|---|
| `2001:db8::/32` | `ipv6-addr--8da31feb-e0f9-54c3-ab30-bd8ccea361dd` |
| `2001:db8:85a3::/48` | `ipv6-addr--2a50470b-c9dc-5b93-8fa0-d2c92c80e12e` |

`2001:0DB8:0000:0000:0000:0000:0000:0000/32` is the same aggregate as
`2001:db8::/32`. OpenCTI only trims IPv6 values, so its identifier for an
uncompressed block differs and the importer records it as an alias (see
[../interoperability.md](../interoperability.md#import)). The STIX type is
shared with `ipv6_address`; references carry the module kind (see
[../identity.md](../identity.md#qualify-every-reference-by-module-kind)).

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.ipv6_network.created` carries `id`, `value`
and `created_at`; `fennec.kernel.ipv6_network.deleted` carries `id` and
`value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /ipv6-networks` | `{"value": "2001:db8::/32"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid value |
| `GET /ipv6-networks/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /ipv6-networks?value=<block>` | Any accepted spelling, with `/` percent-encoded as `%2F` | Same as `GET /ipv6-networks/{id}` |
| `DELETE /ipv6-networks/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryIpv6NetworkRepository` and `SqlxPostgresIpv6NetworkRepository` run
the same repository contract. The table, from
`libs/kernel/migrations/20261026220001_create_ipv6_network.sql`, uses a
`cidr` column as for IPv4 networks:

```sql
CREATE TABLE kernel.ipv6_network (
    id         uuid        PRIMARY KEY,
    value      cidr        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv6_network_value_is_network CHECK (family(value) = 6 AND masklen(value) < 128),
    CONSTRAINT ipv6_network_value_key UNIQUE (value)
);
```
