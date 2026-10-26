# MAC address

One EUI-48 (MAC-48) address that Fennec has registered as an observable.
Module `libs/kernel/src/mac_address/`, aggregate `MacAddress`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `mac-addr` |
| OpenCTI | `Mac-Addr` |
| MISP | `mac-address` |
| Personal data | Yes |

## Value

`MacAddressValue` holds the six octets.

- **Accepted**, with surrounding ASCII whitespace ignored and hexadecimal
  digits in any case:
  - six groups separated by colons (`d2:fb:49:24:37:18`) or by hyphens
    (`D2-FB-49-24-37-18`, as Windows prints them), each of one or two
    digits, because some tools drop leading zeros (`0:1b:63:84:45:e6` from
    macOS `arp`);
  - three groups of four digits separated by dots (`001b.6384.45e6`, as
    Cisco equipment prints them);
  - twelve digits without separators (`02005E100000`).
- **Canonical form**: the one the STIX schema requires, six lowercase
  two-digit groups separated by colons: `d2:fb:49:24:37:18`.
- **Rejected**: fewer or more than six octets, EUI-64 identifiers (eight
  octets; MISP's `mac-eui-64` has no STIX counterpart and would be a kind of
  its own), groups of the wrong length, mixed separators
  (`d2:fb-49:24-37:18`), non-ASCII digits, defanged spellings and empty
  values.
- **Typed components**: `octets()`, `oui()` (the first three octets, the
  manufacturer prefix when the address is universally administered),
  `is_multicast()` (the group bit), `is_locally_administered()` (the local
  bit, set by phones that randomise their address and by virtual machines)
  and `is_broadcast()`.

MAC addresses are personal data: a universally administered address follows
one device, and so usually one person. The catalogue marks the kind so
retention and sharing policies can be attached (see
[../observable-kinds.md](../observable-kinds.md#personal-data)).

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`MacAddressId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical address>"}`:

| Value | Identifier |
|---|---|
| `00:11:22:33:44:55` | `mac-addr--56c46416-7118-5199-a206-1d701ab3f69f` (OpenCTI, canonical input) |
| `d2:fb:49:24:37:18` | `mac-addr--757b1725-9903-54f5-a855-1240691d7659` |
| `00:1b:63:84:45:e6` | `mac-addr--46e4e100-1877-5fe6-a06f-c77917889a2b` |

`D2-FB-49-24-37-18`, `d2fb.4924.3718` and ` d2:fb:49:24:37:18 ` are the same
aggregate as `d2:fb:49:24:37:18`. OpenCTI accepts hyphens and upper case and
hashes the value as received, so `00-11-22-33-44-55` gets its own OpenCTI id
(`mac-addr--012f1f5c-47cd-55b2-9cd5-1c424882372f`), which the importer
records as an alias of Fennec's `00:11:22:33:44:55` (see
[../interoperability.md](../interoperability.md#import)).

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.mac_address.created` carries `id`, `value`
and `created_at`; `fennec.kernel.mac_address.deleted` carries `id` and
`value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /mac-addresses` | `{"value": "d2:fb:49:24:37:18"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid value |
| `GET /mac-addresses/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /mac-addresses?value=<address>` | Any accepted spelling | Same as `GET /mac-addresses/{id}` |
| `DELETE /mac-addresses/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryMacAddressRepository` and `SqlxPostgresMacAddressRepository` run
the same repository contract. The PostgreSQL column is a `macaddr`, which
parses every common spelling and stores one canonical form, so the table
needs no `CHECK` and a test asserts that a raw hyphenated value written
straight into it reads back canonical. The table, from
`libs/kernel/migrations/20261026200001_create_mac_address.sql`:

```sql
CREATE TABLE kernel.mac_address (
    id         uuid        PRIMARY KEY,
    value      macaddr     NOT NULL,
    created_at timestamptz NOT NULL,
    CONSTRAINT mac_address_value_key UNIQUE (value)
);
```

Search by manufacturer prefix belongs to a read model; the write side only
looks addresses up by id.
