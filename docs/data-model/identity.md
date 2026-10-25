# Observable identity

How every observable aggregate is identified, why, and how the identifier is
computed. Read this before adding an observable kind: the identity regime of
a kind decides its id value object, its primary key and its create semantics.

## Two identity regimes

| Regime | Applies to | Aggregate id | STIX identifier |
|---|---|---|---|
| **A. Value-identified** | Kinds whose identity is their value and never changes: IP addresses and networks, domains, hostnames, URLs, email addresses, MAC addresses, AS numbers, every fingerprint and every platform identifier (Google Analytics ids, JA4, JARM, IMEI, ...). | The UUID part of the STIX deterministic identifier, a **UUID v5** derived from the canonical value. | `<stix-type>--<id>`, computed from the id. |
| **B. Evolving identity** | Kinds whose identifying properties can grow after creation: a file that learns a new hash, a certificate, an artifact. | A **UUID v4** chosen once (by the client, like every other aggregate in Fennec) and never changed. | A separate value object, recomputed when a better contributing property arrives. |

Regime A is the default. A kind uses regime B only when its STIX
contributing properties can legitimately change for the same real-world
object.

### Why value-identified kinds derive their id

- **Deduplication without coordination.** Every producer, every replica and
  every retry that sees `198.51.100.7` computes the same id. A create is
  idempotent by construction: the second one fails with `AlreadyExists` on
  the same id instead of creating a twin.
- **References without lookups.** A connector can reference an IP address by
  id before Fennec has ever seen it, and a STIX export needs no lookup to
  produce the identifier.
- **One index instead of two.** The primary key is the deduplication key, so
  a table needs one random-key B-tree, and a lookup by value is a lookup by
  primary key (see [../architecture/persistence.md](../architecture/persistence.md)).
- **The usual objection does not apply.** The shared rule "identity is a
  platform UUID v4, handles are separate" exists because handles change. An
  observable value never changes: a different value is a different
  observable. So the value can carry the identity.

The aggregate id value object of a regime A kind is validated as a **UUID
v5** (parse, then check the version) and has no `generate()`. It is built
from the value with `from_<value>()`, for example
`Ipv4AddressId::from_value(&Ipv4AddressValue)`, and parsed from a request
path with `new(&str)`.

## The algorithm

Fennec computes identifiers exactly as STIX 2.1 section 2.9 specifies, which
is also what `python-stix2` and OpenCTI implement for the common case:

1. Take the kind's **ID contributing properties**, in canonical form.
2. Build a JSON object with one member per contributing property that is
   present (`{"value": "198.51.100.7"}`). References contribute the
   referenced object's full identifier string.
3. Serialise it with **RFC 8785** (JSON Canonicalization Scheme): members
   sorted by key, no whitespace, minimal string escaping, UTF-8.
4. Compute a **UUID v5** with the OASIS namespace
   `00abedb4-aa42-466c-9c01-fed23315a9b7` over those bytes.
5. Prefix it with the STIX type: `ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7`.

The canonical JSON writer and the UUID v5 step live in `shared-valueobject`
because they are generic and have no CTI meaning. Each module owns its STIX
type name, its contributing properties and its canonical form.

The canonical JSON writer supports exactly what contributing properties
need: objects, arrays, strings and integers. A kind that needs another JSON
type (the floating-point entropy inside a file's PE extension) extends the
writer first, with RFC 8785 test vectors.

### Hash selection

For kinds that identify by hashes (file, artifact, X.509 certificate), only
one hash contributes, chosen in the order **MD5, SHA-1, SHA-256, SHA-512**
(the STIX preference list), then SHA3-256, SHA3-512, SSDEEP, TLSH, as
OpenCTI extends it. Hash values contribute in lowercase hex.

### Verified test vectors

Every module that computes an identifier carries these vectors in its id
value object tests. They were checked against `python-stix2` and OpenCTI
fixtures (see
[../research/stix-sco-schemas.md](../research/stix-sco-schemas.md) section 7.3
and [../research/opencti-identity-and-storage.md](../research/opencti-identity-and-storage.md)
section 1.8).

| Contributing JSON | Identifier |
|---|---|
| `{"value":"89.19.209.154"}` | `ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7` |
| `{"value":"198.51.100.3"}` | `ipv4-addr--28bb3599-77cd-5a82-a950-b5bc3caf07c4` |
| `{"value":"198.51.100.0/24"}` | `ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a` |
| `{"value":"hostelhotels.net"}` | `domain-name--07f3ac51-335f-524c-918f-86f80da2bce3` |
| `{"value":"example.com"}` | `domain-name--bedb4899-d24b-5401-bc86-8f6b4cc18ec7` |
| `{"value":"d2:fb:49:24:37:18"}` | `mac-addr--757b1725-9903-54f5-a855-1240691d7659` |
| `{"value":"john@example.com"}` | `email-addr--7165e2a9-671f-585d-b1e1-ca59c671d934` |
| `{"value":"https://example.com/research/index.html"}` | `url--47c3cf9a-5027-5bf0-997a-017c7edc7c55` |
| `{"number":15139}` | `autonomous-system--3aa27478-50b5-5ab8-9da9-cdc12b657fff` |
| `{"name":"__CLEANSWEEP__"}` | `mutex--f93fe911-e545-5239-b9b0-597840d0c871` |
| `{"hashes":{"MD5":"025ad219ece1125a8f5a0e74e32676cb"}}` | `file--cd03138e-eb70-5409-b5df-2f53bee7a1e1` |
| `{"hashes":{"SHA-256":"aba8b59281faa8c1c43a4ca7af075edd3e3516d3cef058a1f43b093177b8f83c"}}` | `file--1b6bb8bc-bc12-5fe7-9d5c-43fce2a4ac84` |

Do not use the example identifiers printed in the STIX 2.1 specification
itself as test expectations: none of the 31 can be reproduced by the
specification's own algorithm.

## STIX type names

| Kind origin | STIX type | Example |
|---|---|---|
| One of the 18 STIX 2.1 cyber-observable objects | The STIX type | `ipv4-addr`, `domain-name`, `file` |
| An OpenCTI custom observable | OpenCTI's type, so identifiers match OpenCTI | `hostname`, `phone-number`, `cryptocurrency-wallet`, `imei` |
| A Fennec-only kind | `x-fennec-<kind>` in kebab case, contributing property `value` (a string) | `x-fennec-google-analytics-tracking-id`, `x-fennec-ja4-fingerprint` |

Fennec-only types carry the `x-fennec-` prefix so that a future official
STIX or OpenCTI type with the same name cannot collide with them. They are
exported through a Fennec `extension-definition` of type `new-sco` (see
[interoperability.md](interoperability.md)). Because the type is not part of
the hashed name, the UUID part of `x-fennec-ja3-fingerprint--<uuid>` equals
the UUID part of an OpenCTI `text--<uuid>` with the same value, which keeps
the lossy OpenCTI mapping stable.

## Qualify every reference by kind

STIX hashes only the contributing properties, never the type. A
`domain-name` and a `hostname` with the value `example.com` therefore share
the UUID part of their identifiers, and so do a `url` and a
`x-fennec-...` kind whose value happens to be the same string.

Inside one module this is harmless: each kind has its own aggregate, its own
id value object and its own table. Outside a module it is not, so:

- A reference to an observable from another module (a sighting, a future
  relationship or indicator) stores the **full `<stix-type>--<uuid>`
  identifier** or an explicit kind next to the UUID, never a bare UUID.
- The domain event log records the aggregate type next to the aggregate id.

## Canonical forms that feed the identifier

The identifier is computed from the canonical form, so the canonical form is
part of the identity contract. Changing a canonicalisation rule changes
identifiers and needs a migration of every stored id of that kind.

| Kind | Canonical form | OpenCTI does |
|---|---|---|
| IPv4 host | Dotted quad, no leading zeros, no `/32` | Takes the value as received |
| IPv6 host | RFC 5952 compressed lowercase, no `/128` | Takes the value as received |
| IPv4 or IPv6 network | Network address with host bits zero, prefix kept | Takes the value as received |
| Domain name | Lowercase A-label (punycode), no trailing dot | Lowercases only |
| Hostname | Lowercase A-label, no trailing dot | Takes the value as received |
| Email address | Lowercase, domain part as A-label | Lowercases |
| URL | WHATWG URL serialisation (lowercase scheme and host, default port removed, normalised percent-encoding) | Takes the value as received |
| MAC address | Lowercase, colon separated (`d2:fb:49:24:37:18`) | Validates any case and separator |
| Hex digests and fingerprints | Lowercase hex | Takes the value as received |

For values that already arrive canonical, which is the common case in feeds,
Fennec and OpenCTI compute the same identifier. When they differ, Fennec's
identifier is the one the STIX algorithm produces for the canonical value.

## Where Fennec deliberately differs from OpenCTI

OpenCTI deviates from the specification for some kinds. Fennec follows the
specification and documents the difference; an OpenCTI-compatible alias
identifier can be stored next to the Fennec one when round-tripping requires
it.

| Kind | Specification and Fennec | OpenCTI |
|---|---|---|
| File | `hashes` (one), `name`, `extensions`, `parent_directory_ref` | `hashes` only if present, else `name` |
| Artifact | `hashes` (one), `payload_bin` | `hashes`, else `url` |
| X.509 certificate | `hashes` (one), `serial_number` | First present of `hashes`, `serial_number`, `subject` |
| Process | No contributing properties: always a UUID v4 | Deterministic over `pid` and `command_line` |
| Software | `name`, `cpe`, `swid`, `vendor`, `version` | Same, with `name` lowercased for the id only |
| Network traffic | Includes `extensions` | Leaves `extensions` out |
| Windows registry key | `key`, `values` | Appears to hash the key under the member name `attribute_key` |

The full comparison, with source lines, is in
[../research/stix-sco-schemas.md](../research/stix-sco-schemas.md) section 7.5.
