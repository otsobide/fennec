# Hostname

One hostname that Fennec has registered as an observable: the name of one
machine, with or without its domain. Module `libs/kernel/src/hostname/`,
aggregate `Hostname`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `hostname` (an OpenCTI custom SCO, not part of STIX 2.1) |
| OpenCTI | `Hostname` |
| MISP | `hostname` |
| Personal data | No |

## Value

`HostnameValue` holds the canonical name as a string.

- **Canonicalisation** follows [domain-name.md](domain-name.md#value) step by
  step: surrounding ASCII whitespace and one trailing dot are dropped, then
  the name goes through UTS #46 nontransitional processing
  (`idna::domain_to_ascii`, no STD3 rules), which lowercases it and turns
  every internationalised label into its A-label: `drucker-büro` becomes
  `xn--drucker-bro-1hb`.
- **Accepted canonical names** have one or more labels; every label has 1 to
  63 lowercase letters, digits, hyphens or underscores and neither starts nor
  ends with a hyphen; the whole name has at most 253 characters; the last
  label is not numeric, so `10.0.0.10` is never taken for a hostname.
- **The difference from a domain name** is the single label: `build-agent-7`
  and `localhost` are hostnames and are rejected as domain names. A
  fully qualified name such as `mail01.corp.example` is valid as both kinds;
  which one an observation is depends on what the source reported, and the
  two kinds never share an aggregate.
- **Rejected**: empty names, empty labels (`host..example`), characters
  outside the label alphabet (spaces, `/`, `@`, URLs, email addresses),
  defanged spellings (`host[.]example`), names or labels over the limits, and
  IPv4 addresses.
- **Typed components**: `labels()`, `machine_name()` (the leftmost label),
  `is_single_label()` and `is_internationalised()`.

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

### Interoperability

OpenCTI trims a hostname but does not lowercase it, and MISP lowercases it
but neither trims it nor converts it to A-labels. Fennec does all three, as
for domain names, so `Mail01.Corp.Example` from OpenCTI and
`mail01.corp.example` from MISP are one aggregate. For a name that OpenCTI
stored in mixed case, OpenCTI's identifier differs from Fennec's, and the
importer records it as an alias (see
[../interoperability.md](../interoperability.md#import)).

## Identity

`HostnameId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical name>"}`:

| Value | Identifier |
|---|---|
| `mail01.corp.example` | `hostname--38ee45a4-70cb-5ddf-9cd2-abcd7afa92a2` |
| `build-agent-7` | `hostname--46e518b9-291e-5e2e-b34c-18e18439c5d1` |
| `xn--drucker-bro-1hb` (from `drucker-büro`) | `hostname--fd23db49-ebaf-58ee-9c38-5578f59434cf` |

`MAIL01.Corp.Example.` and ` mail01.corp.example ` are the same aggregate as
`mail01.corp.example`. Because the type is not hashed, the hostname and the
domain name `mail01.corp.example` share the UUID part and differ only in the
prefix; Fennec keys every table on its own kind, so they never collide.

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.hostname.created` carries `id`, `value` and
`created_at`; `fennec.kernel.hostname.deleted` carries `id` and `value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /hostnames` | `{"value": "mail01.corp.example"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid name |
| `GET /hostnames/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /hostnames?value=<name>` | Any accepted spelling | Same as `GET /hostnames/{id}` |
| `DELETE /hostnames/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryHostnameRepository` and `SqlxPostgresHostnameRepository` run the
same repository contract. The PostgreSQL table, from
`libs/kernel/migrations/20261026170001_create_hostname.sql`, stores the
canonical name as `text COLLATE "C"` and repeats the canonical-form rules in
a `CHECK`, so a canonicalisation bug fails loudly instead of creating a
duplicate:

```sql
CREATE TABLE kernel.hostname (
    id         uuid             PRIMARY KEY,
    value      text COLLATE "C" NOT NULL,
    created_at timestamptz      NOT NULL,
    CONSTRAINT hostname_value_is_canonical CHECK (
        char_length(value) BETWEEN 1 AND 253
        AND value ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)*$'
        AND value !~ '(^|\.)[0-9]+$'
    ),
    CONSTRAINT hostname_value_key UNIQUE (value)
);
```
