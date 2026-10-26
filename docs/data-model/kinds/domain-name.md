# Domain name

One DNS domain name that Fennec has registered as an observable. Module
`libs/kernel/src/domain_name/`, aggregate `DomainName`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `domain-name` |
| OpenCTI | `Domain-Name` |
| MISP | `domain` |
| Personal data | No |

## Value

`DomainNameValue` holds the canonical name as a string.

- **Canonicalisation**, in order:
  1. surrounding ASCII whitespace and one trailing root dot are dropped;
  2. the name goes through UTS #46 nontransitional processing as the `idna`
     crate (1.x) implements it for WHATWG URLs (`idna::domain_to_ascii`,
     no STD3 rules), which lowercases it and turns every internationalised
     label into its A-label: `faß.de` becomes `xn--fa-hia.de`,
     `Bücher.example` becomes `xn--bcher-kva.example`.
- **Accepted canonical names** have at least two labels; every label has 1
  to 63 lowercase letters, digits, hyphens or underscores (service records
  such as `_dmarc.example.com` are domains) and neither starts nor ends with
  a hyphen; the whole name has at most 253 characters; the last label is not
  numeric, so `10.0.0.10` is never taken for a domain (a known MISP
  weakness).
- **Rejected**: empty names, single labels (`localhost`, a `hostname`),
  empty labels (`example..com`), characters outside the label alphabet
  (spaces, `/`, `@`, URLs, email addresses), defanged spellings
  (`example[.]com`), names or labels over the limits.
- **Typed components**: `labels()`, `top_level_domain()` and
  `is_internationalised()`.
- **Implementation**: steps 1 and 2 and the label rules are the shared
  `DnsName` (`libs/shared/valueobject/src/domain/dns/`), which hostnames and
  the domains of email addresses reuse; `DomainNameValue` trims the input and
  adds the second-label rule.

Changing any of these rules changes identifiers and follows the rekey
procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`DomainNameId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical name>"}`:

| Value | Identifier |
|---|---|
| `hostelhotels.net` | `domain-name--07f3ac51-335f-524c-918f-86f80da2bce3` (OpenCTI fixture) |
| `example.com` | `domain-name--bedb4899-d24b-5401-bc86-8f6b4cc18ec7` |
| `xn--fa-hia.de` (from `faß.de`) | `domain-name--1c80db4c-9024-51b7-a295-c50e9bf33964` |

`HostelHotels.NET.` and ` hostelhotels.net ` are the same aggregate as
`hostelhotels.net`.

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. `fennec.kernel.domain_name.created` carries `id`, `value`
and `created_at`; `fennec.kernel.domain_name.deleted` carries `id` and
`value`.

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /domain-names` | `{"value": "hostelhotels.net"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid name |
| `GET /domain-names/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /domain-names?value=<name>` | Any accepted spelling | Same as `GET /domain-names/{id}` |
| `DELETE /domain-names/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryDomainNameRepository` and `SqlxPostgresDomainNameRepository` run
the same repository contract. The PostgreSQL table, from
`libs/kernel/migrations/20261026160001_create_domain_name.sql`, stores the
canonical name as `text COLLATE "C"` and repeats the canonical-form rules in
a `CHECK`, so a canonicalisation bug fails loudly instead of creating a
duplicate:

```sql
CREATE TABLE kernel.domain_name (
    id         uuid             PRIMARY KEY,
    value      text COLLATE "C" NOT NULL,
    created_at timestamptz      NOT NULL,
    CONSTRAINT domain_name_value_is_canonical CHECK (
        char_length(value) BETWEEN 3 AND 253
        AND value ~ '^[a-z0-9_-]+(\.[a-z0-9_-]+)+$'
        AND value !~ '\.[0-9]+$'
    ),
    CONSTRAINT domain_name_value_key UNIQUE (value)
);
```

Suffix search (every name under `example.com`) belongs to a read model; the
write side only looks names up by id.
