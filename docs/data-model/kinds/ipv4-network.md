# IPv4 network

One IPv4 network, a CIDR block, that Fennec has registered as an
observable. Module `libs/kernel/src/ipv4_network/`, aggregate
`Ipv4Network`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `ipv4-addr` (shared with `ipv4_address`) |
| OpenCTI | `IPv4-Addr` |
| MISP | `ip-src`, `ip-dst` with a CIDR value (role collapsed) |
| Personal data | No |

## Value

`Ipv4NetworkValue` holds the network address and the prefix length.

- **Accepted**: a dotted quad followed by a prefix length from `/0` to
  `/31`, with surrounding ASCII whitespace ignored.
- **Canonical form**: the dotted quad without leading zeros, a slash and the
  prefix length: `198.51.100.0/24`.
- **Rejected**:
  - a value without a prefix, and a `/32`: both are a single host, which is
    an `ipv4_address`;
  - an address with host bits set (`198.51.100.7/24`). That spelling
    usually names the host `198.51.100.7` on its network, so truncating it
    to `198.51.100.0/24` would merge two different observations; the error
    names the network so the producer can choose. PostgreSQL's `cidr`
    refuses the same values;
  - malformed prefixes (`/024`, `/+24`), octets with leading zeros,
    abbreviated addresses (`198.51.100/24`), IPv6, defanged spellings and
    empty values.
- **Typed components**: `network_address()`, `prefix_length()`,
  `last_address()`, `size()`, `contains()` and `is_private()` (the whole
  block inside one RFC 1918 range).

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`Ipv4NetworkId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical block>"}`:

| Value | Identifier |
|---|---|
| `198.51.100.0/24` | `ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a` |
| `10.0.0.0/8` | `ipv4-addr--86428b07-0841-50f5-884f-260544a6ebee` |

The STIX type is shared with `ipv4_address`, so the full identifier does not
say which module owns the aggregate; every reference carries the module kind
next to the id (see
[../identity.md](../identity.md#qualify-every-reference-by-module-kind)). A
STIX `ipv4-addr` is imported as a network when its value has a prefix
shorter than `/32` (see [../interoperability.md](../interoperability.md#import)).
A path id is accepted only if it is a UUID v5.

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.ipv4_network.created` carries `id`, `value`
and `created_at`; `fennec.kernel.ipv4_network.deleted` carries `id` and
`value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /ipv4-networks` | `{"value": "198.51.100.0/24"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid value |
| `GET /ipv4-networks/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /ipv4-networks?value=<block>` | Any accepted spelling, with `/` percent-encoded as `%2F` | Same as `GET /ipv4-networks/{id}` |
| `DELETE /ipv4-networks/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryIpv4NetworkRepository` and `SqlxPostgresIpv4NetworkRepository` run
the same repository contract. The PostgreSQL column is a `cidr`, which
refuses set host bits and stores one canonical form; the `CHECK` keeps the
family and the prefix range, and a test expects it by name for an IPv6
block and a `/32`. The table, from
`libs/kernel/migrations/20261026210001_create_ipv4_network.sql`:

```sql
CREATE TABLE kernel.ipv4_network (
    id         uuid        PRIMARY KEY,
    value      cidr        NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT ipv4_network_value_is_network CHECK (family(value) = 4 AND masklen(value) < 32),
    CONSTRAINT ipv4_network_value_key UNIQUE (value)
);
```

Containment queries (the networks that hold an address, the addresses inside
a network) belong to a read model, where a GiST index on `inet_ops` serves
them; the write side only looks networks up by id.
