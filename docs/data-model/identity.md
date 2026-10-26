# Observable identity

How every observable aggregate is identified, why, and how the identifier is
computed. Read this before adding an observable kind: the identity regime of
a kind decides its id value object, its primary key and its create semantics.

## Two identity regimes

| Regime | Applies to | Aggregate id | STIX identifier |
|---|---|---|---|
| **A. Value-identified** | Kinds whose contributing properties are a pure function of one immutable value: IP addresses and networks, domains, hostnames, URLs, email addresses, MAC addresses, AS numbers, mutexes, directories, every fingerprint and every platform identifier (Google Analytics ids, JA4, JARM, IMEI, ...). | The UUID part of the STIX deterministic identifier, a **UUID v5** derived from the canonical value. | `<stix-type>--<id>`, computed from the id. |
| **B. Evolving identity** | Kinds whose contributing properties can legitimately change or grow for the same real-world object: a file that learns a new hash, a certificate, an artifact, a process, an email message, network traffic that ends, a registry key whose values change, software that gains a CPE, a user account whose login is renamed. | A **UUID v4** chosen once and never changed. | A separate value object, recomputed when a contributing property changes. |

A kind is regime A only when every contributing property is part of one
value that can never change for that aggregate. If any contributing property
can be learned later or change, the kind is regime B.

### Why value-identified kinds derive their id

- **Deduplication without coordination.** Every producer, every replica and
  every retry that sees `89.19.209.154` computes the same id. A create is
  idempotent by construction: the second one fails with `AlreadyExists` on
  the same id instead of creating a twin.
- **References without lookups.** A connector can reference an IP address by
  id before Fennec has ever seen it, and a STIX export needs no lookup to
  produce the identifier.
- **Lookups by value are lookups by primary key.** The primary key is the
  deduplication key, so finding an observable by its value never scans and,
  once a table is partitioned by id, always prunes to one partition (see
  [../architecture/persistence.md](../architecture/persistence.md)).
- **The usual objection does not apply.** The shared DDD rule "identity is a
  platform UUID v4, handles are separate" exists because handles change. An
  observable value does not: a different value is a different observable.
  What can change is Fennec's canonicalisation code, and
  [Changing a canonical form](#changing-a-canonical-form) defines how that is
  handled without ever rewriting history.

## Create semantics

### Regime A

- `POST /ipv4-addresses` takes `{"value": "89.19.209.154"}`. The body has no
  `id`; a request that sends one is rejected with `400`.
- The controller builds the value object (`Ipv4AddressValue`) and dispatches
  `CreateIpv4AddressCommand { value }`.
- The aggregate has one constructor, `Ipv4Address::new(value, created_at)`,
  which derives its id from the value. There is no constructor that takes an
  id, so an aggregate whose id does not match its value cannot exist. A
  repository rebuilding an aggregate from a row calls the same constructor and
  reports `Unexpected` if the stored id differs from the derived one.
- The id value object (`Ipv4AddressId`) is validated as a **UUID v5** (parse,
  then check the version), has no `generate()`, and is built either from the
  value (`Ipv4AddressId::from_value(&Ipv4AddressValue)`) or from a request
  path (`Ipv4AddressId::new(&str)`).
- A successful create returns `201 Created` with
  `Location: /ipv4-addresses/<uuid>` and the body
  `{"id": "<uuid>", "stix_id": "ipv4-addr--<uuid>"}`, both computed by the
  controller from the value object. A duplicate returns `409 Conflict` with
  the same body, so the caller learns the id it collided with.

### Regime B

Regime B kinds follow the convention every other Fennec aggregate already
follows: the client supplies a UUID v4 `id` in the POST body, which makes a
retried create idempotent, and the id value object keeps `generate()` for
server-side creation. Their STIX identifier is a separate value object that
the aggregate recomputes whenever a contributing property changes;
deduplication uses the natural keys of the kind (one unique index per hash
for a file).

### Deviations from the shared DDD guide

The shared guide asks for a platform-generated UUID v4 identity with a
`generate()` constructor on every aggregate. Fennec deviates in two
documented ways:

- Regime A observable ids are UUID v5 derived from the canonical value, with
  no `generate()`.
- Regime B observables and every non-observable aggregate (sources,
  sightings) take a client-supplied UUID v4, as Fennec has done since its
  first aggregate; `generate()` stays for server-side creation.

## The algorithm

Fennec computes identifiers exactly as STIX 2.1 section 2.9 specifies, which
is also what `python-stix2` implements and what OpenCTI implements for the
common case:

1. Take the kind's **ID contributing properties**, in canonical form.
2. Build a JSON object with one member per contributing property that is
   present (`{"value": "89.19.209.154"}`). References contribute the
   referenced object's full identifier string.
3. Serialise it with **RFC 8785** (JSON Canonicalization Scheme): members
   sorted by key, no whitespace, minimal string escaping, UTF-8.
4. Compute a **UUID v5** with the OASIS namespace
   `00abedb4-aa42-466c-9c01-fed23315a9b7` over those bytes.
5. Prefix it with the STIX type: `ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7`.

The canonical JSON writer, the OASIS namespace constant and the UUID v5 step
live in `shared-valueobject` because they are generic and have no CTI
meaning. Each module owns its STIX type name, its contributing properties and
its canonical form.

The canonical JSON writer supports what contributing properties need:
objects, arrays, strings, integers and booleans (booleans appear inside
`socket-ext` and `pdf-ext` extensions). A kind that needs floating-point
numbers (the entropy inside a file's PE extension) extends the writer first,
with RFC 8785 test vectors.

### Hash selection

For kinds that identify by hashes (file, artifact, X.509 certificate), only
one hash contributes, chosen in the order **MD5, SHA-1, SHA-256, SHA-512**
(the STIX preference list), then SHA3-256, SHA3-512, SSDEEP, TLSH, as
OpenCTI extends it. MD5, SHA-1, SHA-2 and SHA-3 values contribute in
lowercase hex; SSDEEP contributes as received (its alphabet is
case-sensitive); TLSH contributes in uppercase with its `T1` prefix.

### Verified test vectors

Every module that computes an identifier carries the vectors of its kind in
its id value object tests. **Fixture** vectors come from the test suites of
OpenCTI or `python-stix2`; **computed** vectors were produced with the
algorithm above, which reproduces every fixture (see
[../research/stix-sco-schemas.md](../research/stix-sco-schemas.md) section 7.3
and [../research/opencti-identity-and-storage.md](../research/opencti-identity-and-storage.md)
section 1.8).

| Contributing JSON | Identifier | Source |
|---|---|---|
| `{"value":"89.19.209.154"}` | `ipv4-addr--abfb438a-8baf-5f54-b19e-b85ee74b3ed7` | Fixture (OpenCTI) |
| `{"value":"hostelhotels.net"}` | `domain-name--07f3ac51-335f-524c-918f-86f80da2bce3` | Fixture (OpenCTI) |
| `{"hashes":{"MD5":"025ad219ece1125a8f5a0e74e32676cb"}}` | `file--cd03138e-eb70-5409-b5df-2f53bee7a1e1` | Fixture (OpenCTI) |
| `{"hashes":{"SHA-256":"aba8b59281faa8c1c43a4ca7af075edd3e3516d3cef058a1f43b093177b8f83c"}}` | `file--1b6bb8bc-bc12-5fe7-9d5c-43fce2a4ac84` | Fixture (OpenCTI) |
| `{"values":[{"data":"qwerty","name":"Foo"},{"data":"42","name":"Bar"}]}` | `windows-registry-key--36594eba-bcc7-5014-9835-0e154264e588` | Fixture (`python-stix2`) |
| A `file` with a `windows-pebinary-ext` extension (full JSON in the research note, section 7.3) | `file--ced31cd4-bdcb-537d-aefa-92d291bfc11d` | Fixture (`python-stix2`) |
| `{"value":"198.51.100.3"}` | `ipv4-addr--28bb3599-77cd-5a82-a950-b5bc3caf07c4` | Computed |
| `{"value":"198.51.100.0/24"}` | `ipv4-addr--1ea47840-aa39-5aa8-b4a2-cd203eb6662a` | Computed |
| `{"value":"example.com"}` | `domain-name--bedb4899-d24b-5401-bc86-8f6b4cc18ec7` | Computed |
| `{"value":"xn--fa-hia.de"}` (canonical form of `faß.de`) | `domain-name--1c80db4c-9024-51b7-a295-c50e9bf33964` | Computed |
| `{"value":"d2:fb:49:24:37:18"}` | `mac-addr--757b1725-9903-54f5-a855-1240691d7659` | Computed |
| `{"value":"john@example.com"}` | `email-addr--7165e2a9-671f-585d-b1e1-ca59c671d934` | Computed |
| `{"value":"https://example.com/research/index.html"}` | `url--47c3cf9a-5027-5bf0-997a-017c7edc7c55` | Computed |
| `{"value":"http://example.com/"}` (canonical form of `http://example.com`) | `url--3013563b-9e18-5ef6-917a-204463f5f419` | Computed |
| `{"number":15139}` | `autonomous-system--3aa27478-50b5-5ab8-9da9-cdc12b657fff` | Computed |
| `{"name":"__CLEANSWEEP__"}` | `mutex--f93fe911-e545-5239-b9b0-597840d0c871` | Computed |

Do not use the example identifiers printed in the STIX 2.1 specification
itself as test expectations: none of the 31 can be reproduced by the
specification's own algorithm.

## STIX type names

| Kind origin | STIX type | Example |
|---|---|---|
| One of the 18 STIX 2.1 cyber-observable objects | The STIX type | `ipv4-addr`, `domain-name`, `file` |
| An OpenCTI custom observable | OpenCTI's type and contributing property, so identifiers match OpenCTI | `hostname`, `phone-number`, `cryptocurrency-wallet`, `imei` |
| A Fennec-only kind | `x-fennec-<kind>` in kebab case, contributing property `value` (a string) | `x-fennec-google-analytics-tracking-id`, `x-fennec-ja4-fingerprint` |

Fennec-only types carry the `x-fennec-` prefix so that a future official
STIX or OpenCTI type with the same name cannot collide with them. They are
exported through a Fennec `extension-definition` of type `new-sco` (see
[interoperability.md](interoperability.md)).

## Qualify every reference by module kind

STIX hashes only the contributing properties, never the type, and a STIX
type is not a Fennec kind:

- A `domain-name` and a `hostname` with the value `example.com` share the
  UUID part of their identifiers, and so do a `url` and any `x-fennec-*`
  kind whose value happens to be the same string.
- `ipv4_address` and `ipv4_network` both export `ipv4-addr`, and five
  cryptocurrency kinds export `cryptocurrency-wallet`, so even a full STIX
  identifier does not say which module owns the aggregate.

Inside one module this is harmless: each kind has its own aggregate, its own
id value object and its own table. Outside a module, every reference carries
the **module kind** (`ipv4_network`, `litecoin_address`) next to the UUID:
the sighting's observable reference, the event log's `aggregate_type`, the
endpoints of every link. The `<stix-type>--<uuid>` form is used only on
export and import.

Links between observables are value-identified too, under their own
namespace (`eda8415d-6a75-5fdc-9eb0-72698098a92f`, the UUID v5 of
`urn:fennec:link`), because STIX forbids the OASIS namespace outside
cyber-observables; [links.md](links.md#identity) gives the name they are
derived from and a test vector.

## Canonical forms that feed the identifier

The identifier is computed from the canonical form, so the canonical form is
part of the identity contract. Each kind's page pins its rules exactly
(library, version and flags), with edge-case vectors.

| Kind | Canonical form | OpenCTI does |
|---|---|---|
| IPv4 host | Dotted quad, no leading zeros, no `/32` | Takes the value as received |
| IPv6 host | RFC 5952 compressed lowercase, no `/128` | Takes the value as received |
| IPv4 or IPv6 network | Network address with host bits zero, prefix kept | Takes the value as received |
| Domain name | UTS #46 nontransitional processing (`faß.de` becomes `xn--fa-hia.de`), lowercase A-labels, no trailing dot | Lowercases only |
| Hostname | Same as domain name | Takes the value as received |
| Email address | Lowercase, domain part as A-labels | Lowercases |
| URL | WHATWG serialisation as produced by the `url` crate: lowercase scheme and host, A-label host, default port removed, `/` for an empty path, percent-encodings left as received | Takes the value as received |
| MAC address | Lowercase, colon separated (`d2:fb:49:24:37:18`) | Validates any case and separator |
| Hex digests and fingerprints | Lowercase hex | Takes the value as received |

For a value that arrives already canonical, Fennec and OpenCTI compute the
same identifier. Some common inputs are not canonical: a bare origin such as
`http://example.com` canonicalises to `http://example.com/`, which has a
different identifier. The importer records the producer's identifier as an
alias, so references to it still resolve.

### Changing a canonical form

A change to a canonicalisation rule (a new library version that normalises
differently, a widened grammar that maps an old spelling elsewhere) is a
breaking change for that kind, because it changes identifiers. It is
shipped with a rekey use case that, for every affected aggregate:

1. creates the aggregate under its new id,
2. records the old id as an alias of the new one,
3. publishes `fennec.kernel.<kind>.rekeyed` with the old and the new id,
4. deletes the old aggregate.

The event log is never rewritten. Consumers that hold the old id learn the
new one from the `rekeyed` event, and identifiers already exported keep
resolving through the alias.

## Where Fennec deliberately differs from OpenCTI

OpenCTI deviates from the specification for some kinds, in some cases
because of defects in its identifier code. Fennec follows the specification.
For these kinds, OpenCTI round-trips by alias, never by matching identifier.

| Kind | Specification and Fennec | OpenCTI |
|---|---|---|
| File | `hashes` (one), `name`, `extensions`, `parent_directory_ref` | `hashes` only if present, else `name` |
| Artifact | `hashes` (one), `payload_bin` | `hashes`, else `url` |
| X.509 certificate | `hashes` (one), `serial_number` | First present of `hashes`, `serial_number`, `subject` |
| Process | No contributing properties: always a UUID v4 | Deterministic over `pid` and `command_line` |
| Software | `name`, `cpe`, `swid`, `vendor`, `version` | Same, with `name` lowercased for the id only |
| Email message | `from_ref`, `subject`, `body` | `subject` and `body` only: the sender is stored under another name and never contributes |
| Network traffic | `start`, `end`, `src_ref`, `dst_ref`, ports, `protocols`, `extensions` | Leaves `extensions` out, and appears to serialise the resolved source and destination objects instead of their ids, so its ids with references are not reproducible |
| Windows registry key | `key`, `values` | Appears to hash the key under the member name `attribute_key`, and `values` never contribute |

The full comparison, with source lines, is in
[../research/stix-sco-schemas.md](../research/stix-sco-schemas.md) section 7.5
and [../research/opencti-identity-and-storage.md](../research/opencti-identity-and-storage.md)
section 1.6.
