# Observable kinds

What an observable kind is, which families of kinds exist, and the contract
every kind module fulfils. The list of kinds itself is in
[observable-catalogue.md](observable-catalogue.md).

## Families

Analysts pivot on four different sorts of value. They look alike (a string,
often hex) but they behave differently, so the family decides the domain
contract.

| Family | What it is | Equality | Examples | Becomes |
|---|---|---|---|---|
| **Network and host values** | Addresses and names that locate something | Exact, on the canonical form | IPv4 and IPv6 hosts and networks, domains, hostnames, URLs, email addresses, MAC addresses, AS numbers | An aggregate per kind |
| **Identifiers** | Values assigned by a platform or a standard to an account, a tenant, a device, an app or a site | Exact, on the canonical form | Google Analytics and Tag Manager ids, AdSense publisher ids, Telegram chat ids, IMEI, IBAN, Android package names, Entra tenant ids | An aggregate per kind |
| **Fingerprints** | Deterministic digests computed from an observed artefact | Exact, on the canonical form | MD5 and SHA-256 of a file, imphash, JA3, JA4, JARM, HASSH, favicon MurmurHash3, X.509 SHA-256 | An aggregate per kind, or a value object of the aggregate that owns the artefact (a file's hashes) |
| **Similarity digests** | Digests designed for distance comparison | Distance or score; exact match only as a degenerate case | ssdeep, TLSH, telfhash, impfuzzy | Exact-match identity on the write side; similarity search is a read-side projection |

A fifth sort of value is **not** an observable: **measurements** that only
describe one session or are reallocated constantly (JA4L latency, JA4SSH,
Community ID flow hashes, TMSI and GUTI radio identities). They belong to the
sighting or network session they were measured in, as value objects.

## Modelling rules

1. **One kind, one module.** A kind is a module under `libs/kernel/src/<kind>/`
   with the full `domain` / `application` / `infrastructure` tree. It never
   imports a sibling module, and it is listed in
   `tests/libs/kernel/src/module_isolation_tests.rs`.
2. **Collapse roles, keep kinds.** MISP distinguishes `ip-src` from `ip-dst`
   and `email-src` from `email-dst`. Direction is a property of how a value was
   seen, not of the value, so Fennec has one `ipv4_address` kind and the
   direction lives on the sighting or relationship that saw it.
3. **Split hosts from networks.** A host is a point and a network is a range:
   they have different invariants, queries and storage types. STIX encodes both
   as `ipv4-addr`; Fennec has `ipv4_address` and `ipv4_network` (and the IPv6
   pair), which both export to `ipv4-addr`. A `/32` (or `/128`) network is
   normalised to the host kind.
4. **Split composite values.** No kind holds two values. An importer takes
   MISP composite types apart by what they describe:
   - `domain|ip` is two observables (a `domain_name` and an address) plus the
     relationship between them;
   - `filename|<hash>` and `malware-sample` are one `file` with a name and a
     hash;
   - `ip-src|port` and `ip-dst|port` are the address observable, with the port
     carried by the network traffic or sighting it was seen in (a port is not
     an observable).
5. **Expose typed components.** A value object that has structure exposes it:
   `GoogleAnalyticsTrackingId::account_number()`, `Imei::type_allocation_code()`,
   `MacAddress::is_locally_administered()`, `Ja4Fingerprint::alpn()`. Nothing
   downstream re-parses the canonical string.
6. **Numbers are numbers.** Kinds whose domain is numeric (AS numbers, Telegram
   ids, Cobalt Strike watermarks) hold an integer, not a string, and the
   database column is an integer type.
7. **Secrets are not observables yet.** Values whose disclosure grants access
   (bot tokens, webhook URLs with their token, API keys, private keys,
   passwords) need storage and access rules that do not exist yet. Until they
   do, Fennec models only the non-secret identifiers derived from them (a
   Telegram bot id, a Discord webhook id, a Slack workspace id, an AWS account
   id).
8. **No catch-alls.** There is no `text`, `other`, `comment`, `hex` or
   `anonymised` kind. A value Fennec cannot type yet is not stored as an
   observable.

## Validation rules

- **Parse, normalise, validate, canonicalise**, in that order, inside the value
  object constructor: trim, fold case and separators, check the grammar, check
  numeric ranges, then build the canonical form.
- **Value objects do not refang.** `hxxp://example[.]com` is a defanged
  spelling for humans, not a URL. Refanging belongs to the importer or
  extractor that reads the report, before any value object is built.
- **Strict grammars.** A value that does not match the kind's grammar is
  rejected with a `ValueObjectValidationError` naming the violated rule. When a
  vendor does not publish a format, the grammar is the observed one, documented
  as such in the kind's page, and widened by a dedicated commit when a new
  variant appears.
- **Encoding checksums are enforced; assignment checksums are reported.**
  Checksums that are part of an encoding (Base58Check, Bech32, EIP-55 mixed
  case) are validated, because a value that fails them does not decode. Checksums
  over an assigned number (Luhn on IMEI and card numbers, mod 97 on IBAN) are
  computed and exposed (`has_valid_check_digit()`), not enforced: spoofed and
  fabricated numbers are intelligence too.
- **Never infer the kind.** A value object validates the kind it is asked to
  build; it never tries other kinds. Detecting candidate kinds in free text is
  an import concern with its own use case.

## Personal data

Some kinds identify people: email addresses, phone numbers, IMEI, IMSI,
ICCID, MAC addresses, account handles and account ids. The catalogue marks
them, so retention and sharing policies can be attached when those
aggregates exist.

## The module contract

Every observable kind ships the same slice, built inside-out with one commit
per step (see
[../guides/adding-a-bounded-context.md](../guides/adding-a-bounded-context.md)
for the order):

| Layer | Contents |
|---|---|
| Domain | The aggregate (`Ipv4Address`), its id value object (`Ipv4AddressId`), one value object per property (`Ipv4AddressValue`, `Ipv4AddressCreatedAt`), the repository error enum, the repository trait, the `Created` and `Deleted` events with their factories, and the `STIX_TYPE` constant. |
| Application | `create_<kind>`, `find_<kind>` and `delete_<kind>`, plus any use case the kind needs (updating the name of an autonomous system, adding a hash to a file). |
| Infrastructure | The in-memory repository, the PostgreSQL repository (`SqlxPostgres<Kind>Repository` under `infrastructure/persistence/sqlx_postgres/`) and the table migration. |
| Delivery | `POST /<kinds>`, `GET /<kinds>/{id}`, `GET /<kinds>?value=<value>` and `DELETE /<kinds>/{id}` in `cti_api`, with the create semantics in [identity.md](identity.md#create-semantics). |
| Tests | Value object tests (every accepted spelling, every rejection, the canonical form, the identifier test vectors), the four standard service tests per use case, the in-memory and PostgreSQL repository contract tests, and the e2e tests. |

Events follow the shared payload conventions:

- `Created` carries the full state: the id and every property.
- `Deleted` carries the id **and the canonical value** (or, for regime B,
  the natural keys). A value-identified id is a one-way hash, so without the
  value a subscriber could not tell what was deleted.
- `Updated` carries the id plus `old_<field>` and `new_<field>` for every
  field that changed.

The aggregate of a value-identified kind records `created_at` (when Fennec
first registered it) and no `updated_at`: nothing about it can change. Kinds
with mutable descriptive properties (an autonomous system's name, a file's
size) add `updated_at` and an update use case. Concurrent commands on one
aggregate are serialised by the unit of work (see
[../architecture/event-store.md](../architecture/event-store.md#one-command-at-a-time-per-aggregate)),
so an update reads the current state, changes it and writes it back without a
version check.
