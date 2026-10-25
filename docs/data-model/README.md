# Data model

How Fennec models cyber threat intelligence data. The first slice is the
**observable**: a fact that can be seen on a network, a host or the web (an
IP address, a domain, a file hash, a Google Analytics tracking id, a JA4
fingerprint), stored in the `kernel` bounded context on PostgreSQL.

The model is compatible with STIX 2.1 and OpenCTI on export and import, and
it learns from MISP, but it is not a copy of any of them. The research it was
designed from is in [../research/](../research/README.md).

## Documents

- [identity.md](identity.md): how every observable is identified. The
  deterministic STIX identifier, the canonical JSON it is computed from, the
  namespaces, the verified test vectors, and where Fennec deliberately
  differs from OpenCTI.
- [observable-kinds.md](observable-kinds.md): the taxonomy of observable
  kinds (identifiers, fingerprints, similarity digests, measurements) and the
  rules each kind has to follow to become a module.
- [observable-catalogue.md](observable-catalogue.md): every observable kind
  Fennec models or plans to model, wave by wave, with its STIX type, its
  OpenCTI and MISP counterparts, its canonical form and its storage type.
- [kinds/](kinds/README.md): one page per implemented kind, with its
  grammar, canonical form, identity, events, API and storage.
- [interoperability.md](interoperability.md): how Fennec observables map to
  and from STIX 2.1 bundles, OpenCTI and MISP, including what is lossy.

## Non-negotiables

1. **No generic types.** There is no `Observable { type, value }` table, no
   `Text` catch-all and no `type` column. Every kind is its own aggregate in
   its own `kernel` module, with value objects that validate and canonicalise
   its exact syntax, its own repository, its own events and its own table.
   Adding a kind means adding a module, never adding a row to an enum.
2. **Canonicalise on construction.** A value object turns every accepted
   spelling of a value into one canonical form (lowercase hex, RFC 5952 IPv6,
   punycode domains, no `/32` on hosts). Equality, uniqueness and the
   deterministic identifier all work on the canonical form only.
3. **Value-identified kinds derive their identity from their value.** The
   aggregate id of an IPv4 address is the UUID part of its STIX 2.1
   deterministic identifier, computed from the canonical value. Nobody
   chooses it, so two producers that see the same address create the same
   aggregate. Kinds whose identity can grow (a file learning a new hash) keep
   a stable UUID v4 and carry the STIX identifier as a separate value object.
   See [identity.md](identity.md).
4. **References are qualified by module kind.** The STIX algorithm does not
   hash the type, and several kinds share a STIX type (`ipv4_address` and
   `ipv4_network` both export `ipv4-addr`), so neither a bare UUID nor a STIX
   identifier says which aggregate it points to. Anything that references an
   observable outside its own module (a sighting, the event log, a future
   relationship) stores the **module kind** (`ipv4_network`) next to the
   UUID. The STIX identifier is an export and import form only.
5. **Observables are facts, not judgements.** An observable records what was
   seen and when Fennec first registered it. Scores, labels, markings,
   confidence, authorship and detection rules are assessments about an
   observable and belong to their own aggregates that reference it. This keeps
   observables append-mostly, which is what lets them scale.
6. **Never infer the kind from the value alone.** Thirty-two hex characters
   can be an MD5, a JA3, a HASSH or an imphash; a run of digits can be a Meta
   pixel, an AS number or a Telegram chat. The kind is always stated by the
   caller (an endpoint per kind, a field per kind in an import), never
   guessed by Fennec.

## Retiring the generic `ioc` module

The `ioc` module that predated this model was exactly the generic
`{type, value}` shape rule 1 forbids: an `IocType` enum and an unvalidated
string. It was deleted in one dedicated commit when the first observable kind
(`ipv4_address`) landed, and `/iocs` disappeared with it. Nothing had ever
been persisted (every repository was in memory), so there was no data to
migrate.

The `sighting` module referenced what was seen through `SightingIocId`, a
bare UUID v4. In the same change it was replaced by an observable reference
made of two value objects, `SightingObservableKind` (the module kind, a
validated snake case name rather than an enum, so adding a kind never touches
the sighting module) and `SightingObservableId` (a UUID v4 or v5), so a
sighting can point at any kind, including value-identified ones whose ids
are UUID v5. The uniqueness
pair became (observable kind, observable id, source id), and
`/iocs/{ioc_id}/sightings` became
`/observables/{kind}/{id}/sightings`.
