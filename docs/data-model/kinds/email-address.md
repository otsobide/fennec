# Email address

One email address that Fennec has registered as an observable. Module
`libs/kernel/src/email_address/`, aggregate `EmailAddress`.

| Property | Value |
|---|---|
| Family | Network and host values |
| Identity regime | A (value-identified, UUID v5) |
| STIX type | `email-addr` |
| OpenCTI | `Email-Addr` |
| MISP | `email`, `email-src`, `email-dst` (and the role variants `target-email`, `whois-registrant-email`, `dns-soa-email`) |
| Personal data | Yes |

## Value

`EmailAddressValue` holds the canonical address as a string.

- **Canonicalisation**, in order:
  1. surrounding ASCII whitespace is dropped;
  2. the address is split at its only `@`;
  3. the local part is lowercased with the Unicode default case mapping and
     put in Normalization Form C (`lowercase_nfc` in `shared-valueobject`),
     so `JOSÉ@example.com` and its decomposed spelling (`e` followed by
     U+0301 COMBINING ACUTE ACCENT) are one address;
  4. the domain is parsed as the shared `DnsName` described in
     [domain-name.md](domain-name.md#value): one trailing dot dropped,
     UTS #46 nontransitional processing to lowercase A-labels.
     `Billing@Bücher.example` becomes `billing@xn--bcher-kva.example`.
- **Accepted canonical addresses**:
  - the local part is a dot-atom of RFC 5322 atom characters (letters,
    digits and ``!#$%&'*+-/=?^_`{|}~``), extended with non-ASCII characters
    as RFC 6531 allows. It has at most 64 octets and no leading, trailing or
    doubled dot. Subaddresses (`first.last+tag`) are kept: they are what the
    sender wrote;
  - the domain has at least two labels and follows the domain name rules;
  - the whole address has at most 254 octets (the RFC 5321 path limit).
- **Rejected**: anything without exactly one `@` (including defanged
  `john[at]example.com`), quoted local parts (`"john"@example.com`),
  comments, display names (`John <john@example.com>`), `mailto:` URIs,
  address literals (`john@[192.0.2.1]`), whitespace inside the address, and
  every domain a domain name would reject.
- **Typed components**: `local_part()`, `domain()` and
  `is_internationalised()` (a non-ASCII local part or an A-label in the
  domain).

STIX defines `value` as an RFC 5322 `addr-spec`, which also allows quoted
local parts and address literals. Fennec rejects both: they are almost never
seen in threat intelligence, and accepting them would need a canonical form
for quoting that no producer agrees on.

Lowercasing the local part departs from RFC 5321, which leaves its case to
the receiving server. Every platform Fennec exchanges data with lowercases
the whole address (OpenCTI in its GraphQL create path, MISP in
`modifyBeforeValidation`), and in practice mail systems treat local parts
as case-insensitive. Changing any of these rules changes identifiers and follows the
rekey procedure in [../identity.md](../identity.md#changing-a-canonical-form).

## Identity

`EmailAddressId` is the UUID part of the STIX identifier, computed over
`{"value": "<canonical address>"}`:

| Value | Identifier |
|---|---|
| `john@example.com` | `email-addr--7165e2a9-671f-585d-b1e1-ca59c671d934` (OpenCTI fixture) |
| `billing@xn--bcher-kva.example` (from `Billing@Bücher.example`) | `email-addr--cf72105f-c27b-5289-bada-a2f65b957880` |
| `first.last+tag@mail.example.org` | `email-addr--c3272f87-19b2-58ab-b8bb-0b646c06456b` |

`John@Example.COM` and ` john@example.com ` are the same aggregate as
`john@example.com`. OpenCTI does not convert internationalised domains to
A-labels, so for `billing@bücher.example` its identifier differs from
Fennec's and the importer records it as an alias (see
[../interoperability.md](../interoperability.md#import)).

## Aggregate and events

As for every value-identified kind: `id`, `value` and `created_at`; no
update use case. The STIX `display_name` is not part of the address: it
belongs to the email message or the sighting that carried it.
`fennec.kernel.email_address.created` carries `id`, `value` and
`created_at`; `fennec.kernel.email_address.deleted` carries `id` and
`value`.

Email addresses are personal data. The catalogue marks them so retention and
sharing policies can be attached when those aggregates exist (see
[../observable-kinds.md](../observable-kinds.md#personal-data)).

## HTTP API

| Method and path | Body or query | Responses |
|---|---|---|
| `POST /email-addresses` | `{"value": "john@example.com"}`; any other field is rejected | `201` with `Location` and `{"id", "stix_id"}`; `409` with the same body when already registered; `400` for an invalid address |
| `GET /email-addresses/{id}` | | `200` with `{"id", "stix_id", "value", "created_at"}`; `400` if the id is not a UUID v5; `404` |
| `GET /email-addresses?value=<address>` | Any accepted spelling, with `@` percent-encoded as `%40` | Same as `GET /email-addresses/{id}` |
| `DELETE /email-addresses/{id}` | | `204`; `400`; `404` |

## Storage

`InMemoryEmailAddressRepository` and `SqlxPostgresEmailAddressRepository`
run the same repository contract. The PostgreSQL table, from
`libs/kernel/migrations/20261026180001_create_email_address.sql`, stores the
canonical address as `text COLLATE "C"` and repeats what SQL can check of the
canonical form in a `CHECK`:

```sql
CREATE TABLE kernel.email_address (
    id         uuid             PRIMARY KEY,
    value      text COLLATE "C" NOT NULL,
    created_at timestamptz      NOT NULL,
    CONSTRAINT email_address_value_is_canonical CHECK (
        octet_length(value) BETWEEN 5 AND 254
        AND octet_length(split_part(value, '@', 1)) <= 64
        AND value ~ '^[^@[:space:][:upper:]]+@[a-z0-9_-]+(\.[a-z0-9_-]+)+$'
        AND value !~ '(^\.|\.\.|\.@)'
        AND value !~ '\.[0-9]+$'
    ),
    CONSTRAINT email_address_value_key UNIQUE (value)
);
```

Search by domain (every address at `example.com`) belongs to a read model;
the write side only looks addresses up by id.
