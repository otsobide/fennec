# OpenCTI Stix-Cyber-Observable catalogue (research notes for Fennec)

Status: research notes, source-derived. Every claim tagged **[S]** is what the OpenCTI source says (with file and line). Claims tagged **[R]** are my recommendations for Fennec. Claims tagged **[Q]** are quirks/inconsistencies observed in the source that I did not verify at runtime. Claims tagged **[K]** come from my own knowledge of the STIX 2.1 specification and were not found in the local clones.

## 0. Snapshot and path conventions

- OpenCTI clone: `.opencti/` at commit `294dffb07c5ee41086493827c94848b56f592a2c` (2026-10-25), shallow clone (no history available).
- STIX schemas clone: `.stix2-json-schemas/` (OASIS `stix2.1` branch JSON schemas).
- Path aliases used below (all relative to `.opencti/`):

| Alias | Path |
|---|---|
| `TYPES` | `opencti-platform/opencti-graphql/src/schema/stixCyberObservable.ts` |
| `ATTR` | `opencti-platform/opencti-graphql/src/modules/attributes/stixCyberObservable-registrationAttributes.ts` |
| `REF` | `opencti-platform/opencti-graphql/src/modules/relationsRef/stixCyberObservable-registrationRef.ts` |
| `REFDEF` | `opencti-platform/opencti-graphql/src/schema/stixRefRelationship.ts` |
| `ADEF` | `opencti-platform/opencti-graphql/src/schema/attribute-definition.ts` |
| `SYN` | `opencti-platform/opencti-graphql/src/utils/syntax.js` |
| `DET` | `opencti-platform/opencti-graphql/src/utils/observable.ts` |
| `DOM` | `opencti-platform/opencti-graphql/src/domain/stixCyberObservable.js` |
| `ID` | `opencti-platform/opencti-graphql/src/schema/identifier.js` |
| `GEN` | `opencti-platform/opencti-graphql/src/schema/general.js` |
| `CONV` | `opencti-platform/opencti-graphql/src/database/stix-2-1-converter.ts` |
| `SCO` | `opencti-platform/opencti-graphql/src/types/stix-2-1-sco.d.ts` |
| `COMMON` | `opencti-platform/opencti-graphql/src/types/stix-2-1-common.d.ts` |
| `EXT` | `opencti-platform/opencti-graphql/src/types/stix-2-1-extensions.ts` |
| `GQL` | `opencti-platform/opencti-graphql/config/schema/opencti.graphql` |
| `RES` | `opencti-platform/opencti-graphql/src/resolvers/stixCyberObservable.js` |
| `MW` | `opencti-platform/opencti-graphql/src/database/middleware.ts` |
| `FDA` | `opencti-platform/opencti-graphql/src/schema/fieldDataAdapter.ts` |
| `FMT` | `opencti-platform/opencti-graphql/src/utils/format.js` |
| `VOC` | `opencti-platform/opencti-graphql/src/modules/vocabulary/vocabulary-types.ts` |
| `VOCU` | `opencti-platform/opencti-graphql/src/modules/vocabulary/vocabulary-utils.ts` |
| `REL` | `opencti-platform/opencti-graphql/src/database/stix.ts` |
| `VAL` | `opencti-platform/opencti-graphql/src/schema/schema-validator.ts` |
| `ESA` | `opencti-platform/opencti-graphql/src/modules/entitySetting/entitySetting-attributeUtils.ts` |
| `PYC` | `client-python/pycti/utils/constants.py` |
| `PYO` | `client-python/pycti/entities/opencti_stix_cyber_observable.py` |
| `PYS` | `client-python/pycti/utils/opencti_stix2.py` |
| `FE` | `opencti-platform/opencti-front/src/private/components/observations/stix_cyber_observables/StixCyberObservableCreation.jsx` |

## 1. Inventory: the 37 concrete observable types

**[S]** `TYPES:4-40` defines 37 entity-type constants; `TYPES:48-87` registers all 37 under the abstract `Stix-Cyber-Observable`; `TYPES:42-47` registers 3 of them under the abstract `Hashed-Observable`. The python client enumerates the same 37 plus a pseudo type `Simple-Observable` (`PYC:31-71`, see section 11). There is nothing else: the backend list is authoritative and matches GQL, pycti and the frontend.

**[S]** STIX type string = `entity_type.toLowerCase()`, except `StixFile` which maps to `file` (`CONV:137-160`). GraphQL type name = entity type with dashes removed and each segment capitalised (`RES:59-63`, `DOM:207`).

Category legend: **STD** = STIX 2.1 standard SCO (one of the 18 files in `.stix2-json-schemas/schemas/observables/`); **EMB** = STIX 2.1 embedded type that OpenCTI promotes to a standalone SCO; **OCTI** = OpenCTI custom SCO (constant commented `// Custom` in `TYPES:24-40`). Export extension legend: **prop** = standard STIX properties plus OpenCTI SCO extension of `extension_type: 'property-extension'`; **new** = custom SCO with `extension_type: 'new-sco'` and OpenCTI-specific properties at top level (see section 10).

| # | OpenCTI `entity_type` | Constant (`TYPES` line) | STIX `type` | GraphQL type / AddInput (`GQL` lines) | Cat. | Hashed | Export ext. | Legacy STIX aliases accepted by pycti (`PYO:356-398`) |
|---|---|---|---|---|---|---|---|---|
| 1 | `Autonomous-System` | `ENTITY_AUTONOMOUS_SYSTEM` (4) | `autonomous-system` | `AutonomousSystem` 7801 / `AutonomousSystemAddInput` 7927 | STD | no | prop | |
| 2 | `Directory` | `ENTITY_DIRECTORY` (5) | `directory` | `Directory` 7939 / 8067 | STD | no | prop | |
| 3 | `Domain-Name` | `ENTITY_DOMAIN_NAME` (6) | `domain-name` | `DomainName` 8081 / 8205 | STD | no | prop | |
| 4 | `Email-Addr` | `ENTITY_EMAIL_ADDR` (7) | `email-addr` | `EmailAddr` 8215 / 8340 | STD | no | prop | |
| 5 | `Email-Message` | `ENTITY_EMAIL_MESSAGE` (8) | `email-message` | `EmailMessage` 8351 / 8481 | STD | no | prop | |
| 6 | `Email-Mime-Part-Type` | `ENTITY_EMAIL_MIME_PART_TYPE` (9) | `email-mime-part-type` | `EmailMimePartType` 8497 / 8623 | EMB | no | new | |
| 7 | `Artifact` | `ENTITY_HASHED_OBSERVABLE_ARTIFACT` (10) | `artifact` | `Artifact` 8776 / 8907 | STD | yes | prop | |
| 8 | `StixFile` | `ENTITY_HASHED_OBSERVABLE_STIX_FILE` (11, "Because File already used") | `file` | `StixFile` 8923 / 9059 | STD | yes | prop | `file` |
| 9 | `X509-Certificate` | `ENTITY_HASHED_OBSERVABLE_X509_CERTIFICATE` (12) | `x509-certificate` | `X509Certificate` 9079 / 9232 | STD | yes | prop | |
| 10 | `IPv4-Addr` | `ENTITY_IPV4_ADDR` (13) | `ipv4-addr` | `IPv4Addr` 9270 / 9395 | STD | no | prop | |
| 11 | `IPv6-Addr` | `ENTITY_IPV6_ADDR` (14) | `ipv6-addr` | `IPv6Addr` 9407 / 9532 | STD | no | prop | |
| 12 | `Mac-Addr` | `ENTITY_MAC_ADDR` (15) | `mac-addr` | `MacAddr` 9542 / 9666 | STD | no | prop | |
| 13 | `Mutex` | `ENTITY_MUTEX` (16) | `mutex` | `Mutex` 9676 / 9800 | STD | no | prop | |
| 14 | `Network-Traffic` | `ENTITY_NETWORK_TRAFFIC` (17) | `network-traffic` | `NetworkTraffic` 9810 / 9946 | STD | no | prop | |
| 15 | `Process` | `ENTITY_PROCESS` (18) | `process` | `Process` 9967 / 10114 | STD | no | prop | |
| 16 | `Software` | `ENTITY_SOFTWARE` (19) | `software` | `Software` 10147 / 10286 | STD | no | prop | |
| 17 | `Url` | `ENTITY_URL` (20) | `url` | `Url` 10302 / 10426 | STD | no | prop | |
| 18 | `User-Account` | `ENTITY_USER_ACCOUNT` (21) | `user-account` | `UserAccount` 10436 / 10574 | STD | no | prop | |
| 19 | `Windows-Registry-Key` | `ENTITY_WINDOWS_REGISTRY_KEY` (22) | `windows-registry-key` | `WindowsRegistryKey` 10597 / 10723 | STD | no | prop | |
| 20 | `Windows-Registry-Value-Type` | `ENTITY_WINDOWS_REGISTRY_VALUE_TYPE` (23) | `windows-registry-value-type` | `WindowsRegistryValueType` 10735 / 10861 | EMB | no | new | |
| 21 | `Cryptographic-Key` | `ENTITY_CRYPTOGRAPHIC_KEY` (24) | `cryptographic-key` | `CryptographicKey` 10873 / 10997 | OCTI | no | new | `x-opencti-cryptographic-key` |
| 22 | `Cryptocurrency-Wallet` | `ENTITY_CRYPTOGRAPHIC_WALLET` (25) | `cryptocurrency-wallet` | `CryptocurrencyWallet` 11007 / 11131 | OCTI | no | new | `x-opencti-cryptocurrency-wallet` |
| 23 | `Hostname` | `ENTITY_HOSTNAME` (26) | `hostname` | `Hostname` 11141 / 11265 | OCTI | no | new | `x-opencti-hostname` |
| 24 | `Text` | `ENTITY_TEXT` (27) | `text` | `Text` 11275 / 11399 | OCTI | no | new | `x-opencti-text` |
| 25 | `Credential` | `ENTITY_CREDENTIAL` (28) | `credential` | `Credential` 11819 / 11946 | OCTI | no | new | `x-opencti-credential` |
| 26 | `User-Agent` | `ENTITY_USER_AGENT` (29) | `user-agent` | `UserAgent` 11409 / 11533 | OCTI | no | new | `x-opencti-user-agent` |
| 27 | `Bank-Account` | `ENTITY_BANK_ACCOUNT` (30) | `bank-account` | `BankAccount` 11543 / 11670 | OCTI | no | new | |
| 28 | `Tracking-Number` | `ENTITY_TRACKING_NUMBER` (31) | `tracking-number` | `TrackingNumber` 11682 / 11809 | OCTI | no | new | `x-opencti-tracking-number` |
| 29 | `Phone-Number` | `ENTITY_PHONE_NUMBER` (32) | `phone-number` | `PhoneNumber` 11956 / 12080 | OCTI | no | new | |
| 30 | `Payment-Card` | `ENTITY_PAYMENT_CARD` (33) | `payment-card` | `PaymentCard` 12090 / 12217 | OCTI | no | new | `x-opencti-payment-card` |
| 31 | `Media-Content` | `ENTITY_MEDIA_CONTENT` (34) | `media-content` | `MediaContent` 12230 / 12359 | OCTI | no | new | |
| 32 | `Persona` | `ENTITY_PERSONA` (35) | `persona` | `Persona` 12373 / 12498 | OCTI | no | new | |
| 33 | `SSH-Key` | `ENTITY_SSH_KEY` (36) | `ssh-key` | `SSHKey` 12510 / 12641 | OCTI | no | new | |
| 34 | `AI-Prompt` | `ENTITY_AI_PROMPT` (37) | `ai-prompt` | `AIPrompt` 12658 / 12782 | OCTI | no | new | |
| 35 | `IMEI` | `ENTITY_IMEI` (38) | `imei` | `IMEI` 12791 / 12918 | OCTI | no | new | `x-opencti-imei` |
| 36 | `ICCID` | `ENTITY_ICCID` (39) | `iccid` | `ICCID` 12922 / 13049 | OCTI | no | new | `x-opencti-iccid` |
| 37 | `IMSI` | `ENTITY_IMSI` (40) | `imsi` | `IMSI` 13053 / 13180 | OCTI | no | new | `x-opencti-imsi` |

Counts: 18 STD + 2 EMB + 17 OCTI = 37.

## 2. Abstract parents and type hierarchy

**[S]** Abstract names (`GEN:77-78`): `Stix-Cyber-Observable` and `Hashed-Observable`. Ancestry computed by `getParentTypes` (`opencti-platform/opencti-graphql/src/schema/schemaUtils.js:114-148`): every observable gets `parent_types = [Basic-Object, Stix-Object, Stix-Core-Object, Stix-Cyber-Observable]`. `Stix-Core-Object` groups `Stix-Domain-Object` and `Stix-Cyber-Observable` (`opencti-platform/opencti-graphql/src/modules/attributes/stixCoreObject-registrationAttributes.ts:10-13`).

**[S]** `Hashed-Observable` is only a type group (registered in `TYPES:47`, tested by `isStixCyberObservableHashedObservable` `TYPES:89-90`). It is **not** pushed into `parent_types` (`schemaUtils.js:144-146` only pushes `Stix-Cyber-Observable`). No attribute is registered on it; `hashes` is registered individually on each of the 3 members (`ATTR:113,122,135`).

**[S]** GraphQL: `interface StixCyberObservable` (`GQL:7681-7800`) and `interface HashedObservable` (`GQL:8654`, field `hashes: [Hash]` at `GQL:8749-8750`). Concrete types implement `BasicObject & StixObject & StixCoreObject & StixCyberObservable` (plus `& HashedObservable` for Artifact, StixFile, X509Certificate: `GQL:8776,8923,9079`).

**[S]** Behaviour driven by `Hashed-Observable` membership:
- Hash input list is converted to a dictionary (`DOM:239-241`).
- On create, existing observables of the same type sharing any non-fuzzy hash are looked up and upserted into (`MW:3964-3980`, `MW:1547-1567`), `resolvedInput.update = true` is forced, and the standard ids that each single hash would generate are stored in `x_opencti_stix_ids` (`MW:3971-3979`, `ID:546-562`).
- Hash collisions across merge candidates raise `Hashes collision` (`MW:1569-1584`).
- On update, other per-hash standard ids are recomputed and kept in `x_opencti_stix_ids` (`MW:2408-2430`).
- `getInstanceIds` / `getInputIds` add every hash value itself as a lookup id (`ID:564-573`, `ID:574-609`).

**[R]** In Fennec model `Hashed-Observable` as a capability (a `Hashes` value object plus a `HashedObservable` trait implemented by `Artifact`, `File`, `X509Certificate`), not as a supertype aggregate.

## 3. Attribute metadata semantics (how to read the per-type tables)

**[S]** Attribute definition fields (`ADEF:27-57`):
- `type`: `string | date | numeric | boolean | object | ref`; strings have `format` `short` (keyword-indexed, `ignore_above: 512` in `shortMapping` `ADEF:12-21`), `text`, `id`, `enum`, `vocabulary`, `json`; numerics have `precision` `integer` (32-bit), `long`, `float`.
- `mandatoryType` (`ADEF:31`): `internal` (platform-computed), `external` (always mandatory for API/UI users: `ESA:46-58` sets `mandatory: attr.mandatoryType === 'external'`), `customizable` (an admin can make it mandatory per entity type in Entity Settings), `no` (optional). Enforcement: `VAL:129-154` throws `This attribute is mandatory` unless the user has `KNOWLEDGE_KNUPDATE_KNBYPASSFIELDS`. Note: enforcement is driven by entity settings configuration, so it is not a hard storage invariant.
- `upsert` (`ADEF:43-44`): "If attribute can be upsert by the integration" (when a connector re-sends the same object, `upsert: false` fields are not overwritten).
- `editDefault`: attribute appears in Entity Settings default-value configuration (`ESA:46-49`).
- `multiple`: array value. `isFilterable`: usable as filter key.

In the per-type tables: **Mand.** is the registration `mandatoryType` (`ext` = `external`); **AddInput** shows the GraphQL input type (a `!` means GraphQL-level required); **Up.** is `upsert`; **Mult.** is `multiple`.

## 4. Attributes shared by ALL observables (inherited)

| Attribute | Registered on (file:line) | Type | Mand. | Mult. | Up. | Notes |
|---|---|---|---|---|---|---|
| `id` | Basic-Object (`basicObject-registrationAttributes.ts:22`, def `ADEF:100-112`) | string/id | no | no | no | internal handle |
| `internal_id` | Basic-Object (`ADEF:176-188`) | string/id | no | no | no | UUIDv4 |
| `standard_id` | Basic-Object (`ADEF:207-218`) | string/short | internal | no | no | deterministic STIX id, see section 9 |
| `draft_ids`, `draft_change` | Basic-Object (`ADEF:114-155`) | id / object | no | | no | draft workspace internals |
| `i_attributes` | Basic-Object (`ADEF:157-175`) | object | no | yes | no | per-attribute update metadata |
| `parent_types`, `base_type`, `entity_type` | Basic-Object (`ADEF:393-430`) | string/short | internal | parent_types multiple | no | |
| `created_at`, `updated_at`, `refreshed_at` | Basic-Object (`ADEF:600-632`) | date | internal | no | no | platform timestamps |
| `creator_id` | Basic-Object (`ADEF:190-205`) | string/id to User | no | yes | yes | |
| `metrics` | Basic-Object (`ADEF:375-389`) | nested object {name, value float} | no | yes | yes | |
| `x_opencti_stix_ids` | Stix-Object (`opencti-platform/opencti-graphql/src/modules/attributes/stixObject-registrationAttributes.ts:5-9`, def `ADEF:529-539`) | string/short | no | yes | yes | alternate STIX ids (incl. per-hash ids) |
| `x_opencti_files` | Stix-Object (`ADEF:245-...`) | object | no | yes | no | attached files |
| `opinions_metrics` | Stix-Core-Object (`stixCoreObject-registrationAttributes.ts:6-8`, def `ADEF:507-526`) | object {mean,max,min,total} | no | no | yes | |
| `x_opencti_modified_at` | Stix-Cyber-Observable (`ATTR:48`, def `ADEF:589-598`) | date | no | no | yes | label "Last update" |
| `x_opencti_description` | Stix-Cyber-Observable (`ATTR:49`) | string/short | no | no | yes | label "Observable description" |
| `x_opencti_score` | Stix-Cyber-Observable (`ATTR:50`) | numeric/integer | no | no | yes | label "Score"; validated integer 0..100 by `checkScore` (`FMT:294-302`, called `DOM:211`, `DOM:282-284`) |

**[S]** Not stored on observables (unlike SDOs): `created`, `modified`, `confidence`, `revoked`, `lang` (exception: `SSH-Key` registers `created`, `ATTR:319`). GraphQL still exposes `spec_version`, `representative`, `observable_value` (computed, `RES:64`, `FMT:251-292`).

**[S]** `x_opencti_score` side effect: editing an observable's score propagates it to every Indicator linked via `based-on` (`DOM:322-333`).

**[S]** Top-level arguments of the create mutation (`GQL:15806-15858`): `type: String!`, `stix_id`, `x_opencti_score`, `x_opencti_description`, `x_opencti_modified_at`, `createIndicator`, `createdBy`, `objectMarking`, `objectLabel`, `objectOrganization`, `externalReferences`, `clientMutationId`, `update`, `upsertOperations`, plus one per-type input argument. **[Q]** `addStixCyberObservable` (`DOM:190-231`) does not copy `x_opencti_modified_at` into the created input, so that argument appears to be ignored on this path.

**[S]** Per-type AddInput technical fields present on almost all inputs: `file: Upload`, `fileMarkings: [String]`, `files: [Upload]`, `filesMarkings: [[String]]`, `noTriggerImport: [Boolean]`, `embedded: [Boolean]`, `upsertOperations: [EditInput!]`. Exceptions: `AIPromptAddInput` has `noTriggerImport: Boolean` (scalar) and no `embedded` (`GQL:12782-12789`); `IMEIAddInput`, `ICCIDAddInput`, `IMSIAddInput` only have `value` and `file` (`GQL:12918-12920`, `13049-13051`, `13180-13182`).

### 4.1 Meta refs shared by ALL observables

**[S]** `REF:80-83` registers on `Stix-Cyber-Observable`: `createdBy, objectMarking, objectLabel, externalReferences, objectOrganization` (the first four are also inherited from `Stix-Core-Object`, `opencti-platform/opencti-graphql/src/modules/relationsRef/stixCoreObject-registrationRef.ts:5-10`).

| Input name | STIX name | DB relation | Mult. | Mand. | Up. | Target types | Def (`REFDEF`) |
|---|---|---|---|---|---|---|---|
| `createdBy` | `created_by_ref` | `created-by` | no | customizable (ED) | yes | Individual, Sector, System, Organization | 700-716 |
| `objectMarking` | `object_marking_refs` | `object-marking` | yes | customizable (ED) | yes | Marking-Definition | 718-734 |
| `objectLabel` | `labels` | `object-label` | yes | no | yes | Label (labels may be passed by value, `MW:1098-1108`) | 851-867 |
| `externalReferences` | `external_references` | `external-reference` | yes | no | yes | External-Reference | 905-921 |
| `objectOrganization` | `granted_refs` | `granted` | yes | no | yes | Organization; requires capability `KNOWLEDGE_KNUPDATE_KNORGARESTRICT` | 754-775 |

**[S]** `createdBy` must resolve to an Identity (`MW:1278-1284`). Markings must be within the user's allowed markings (`MW:1270-1277`).

**[S]** STIX placement on export (`CONV:319-337`): `object_marking_refs` top level; `labels`, `description` (from `x_opencti_description`), `score` (from `x_opencti_score`), `created_by_ref`, `external_references` go into the OpenCTI SCO extension `extension-definition--f93e2c80-4231-4f9a-af8b-95c9bd566a82` (`EXT:13`) for STD types; for OCTI and EMB types they are emitted at top level (section 10). `granted_refs` and internal ids go into the generic OpenCTI extension `extension-definition--ea279b3e-5c71-4632-ac08-831c66a786ba` (`EXT:6`, `CONV:163-211`).

## 5. Hashes (Artifact, StixFile, X509-Certificate)

**[S]** `hashDefinition` (`ATTR:54-76`): `type: 'object'`, `format: 'standard'`, not multiple, `upsert: true`, `isFilterable: true`, with fixed mappings (all `string/short`, optional, `upsert: true`):

| Key | Filterable |
|---|---|
| `MD5` | yes |
| `SHA-1` | yes |
| `SHA-256` | yes |
| `SHA-512` | yes |
| `SHA3-256` | no |
| `SHA3-512` | no |
| `SSDEEP` | yes |
| `SDHASH` | no |
| `TLSH` | no |
| `LZJD` | no |

**[S]** Fuzzy algorithms (excluded from dedup lookups): `FUZZY_HASH_ALGORITHMS = ['SSDEEP', 'SDHASH', 'TLSH', 'LZJD']` (`FDA:13`, used by `extractNotFuzzyHashValues` `FDA:20-26`).

**[S]** GraphQL shape: input `HashInput { algorithm: String! @constraint(minLength: 3), hash: String! @constraint(minLength: 5), upsertOperations }` (`GQL:8636-8640`); output `Hash { algorithm: String!, hash: String }` (`GQL:8641-8644`). Storage is a dictionary `{ALGO: value}`.

**[S]** Normalisation on input: `inputHashesToStix` upper-cases and trims the algorithm and trims the hash value (`FDA:29-37`); hash values are **not** case-normalised. Output conversion `stixHashesToInput` (`FDA:40-46`).

**[S]** Syntax checks (only for `StixFile` and `Artifact`, not `X509-Certificate`) in `SYN:183-201`:
```
MD5      /^[a-fA-F0-9]{32}$/
SHA-1    /^[a-fA-F0-9]{40}$/
SHA-256  /^[a-fA-F0-9]{64}$/
SHA-512  /^[a-fA-F0-9]{128}$/
```
Other algorithms are not validated by the backend. For comparison the STIX `hashes-type` schema (`.stix2-json-schemas/schemas/common/hashes-type.json`) also defines `SHA3-256 ^[a-fA-F0-9]{64}$`, `SHA3-512 ^[a-fA-F0-9]{128}$`, `SSDEEP ^[a-zA-Z0-9/+:.]{1,128}$`, `TLSH ^[a-zA-Z0-9]{70}$`, and custom keys `^[a-zA-Z0-9_-]{3,250}$`; it has no `SDHASH` or `LZJD`.

**[S]** Standard id uses exactly one hash, chosen by priority `MD5 > SHA-1 > SHA-256 > SHA-512 > SHA3-256 > SHA3-512 > SSDEEP > SDHASH > TLSH > LZJD` after upper-casing keys (`ID:157-172`). Display priority is different: `SHA-512 > SHA-256 > SHA-1 > MD5` (`FMT:205-216`); ES runtime field uses `SHA-256 > SHA-512 > SHA-1 > MD5` (`FMT:337-342`).

**[Q]** Because the id resolver does not lowercase values, `ABC...` and `abc...` MD5 give different `standard_id`s; dedup still works because the keyword normalizer lowercases (`string_normalizer` = `lowercase` + `asciifolding`, `opencti-platform/opencti-graphql/src/database/engine.ts:1184-1187`) and `listEntitiesByHashes` searches `hashes.*` across all algorithms.

**[R]** Fennec: `Hashes` value object = map of a closed `HashAlgorithm` enum (STIX `hash-algorithm-ov` plus OpenCTI's `SDHASH`, `LZJD`, plus a `Custom(String)` matching `^[a-zA-Z0-9_-]{3,250}$`) to validated, **lower-cased** hex digests (fuzzy hashes kept verbatim with their own format checks). Validate every algorithm, including for X509.

## 6. Ref (embedded relationship) definitions reused by observables

**[S]** All in `REFDEF`. Input names `REFDEF:59-86`, DB relation names `REFDEF:88-115`. All below are `mandatoryType: 'no'`, `upsert: true`, `datable: true`, `isFilterable: true`. The `toTypes` listed in the definition are the global superset; each observable type narrows allowed targets with a checker in `REF` (section 7).

| Ref const | Input name (GraphQL/API) | STIX name | DB relation type | Mult. | Global `toTypes` | Def lines |
|---|---|---|---|---|---|---|
| `contains` | `containsObservable` | `contains_refs` | `contains` | yes | `Stix-Core-Object` | 188-204 |
| `resolvesTo` | `resolvesTo` | `resolves_to_refs` | `obs_resolves-to` | yes | Domain-Name, IPv4, IPv6, Mac | 205-221 |
| `belongsTo` | `belongsTo` | `belongs_to_refs` | `obs_belongs-to` | yes | User-Account, Autonomous-System | 222-238 |
| `emailFrom` | `emailFrom` | `from_ref` | `from` | no | Email-Addr | 239-255 |
| `sender` | `emailSender` | `sender_ref` | `sender` | no | Email-Addr | 256-272 |
| `emailTo` | `emailTo` | `to_refs` | `to` | yes | Email-Addr | 273-289 |
| `cc` | `emailCc` | `cc_refs` | `cc` | yes | Email-Addr | 290-306 |
| `bcc` | `emailBcc` | `bcc_refs` | `bcc` | yes | Email-Addr | 307-323 |
| `rawEmail` | `rawEmail` | `raw_email_ref` | `raw-email` | no | Artifact | 324-340 |
| `bodyRaw` | `bodyRaw` | `body_raw_ref` | `body-raw` | no | Artifact, StixFile | 341-357 |
| `parentDirectory` | `parentDirectory` | `parent_directory_ref` | `parent-directory` | no | Directory | 358-374 |
| `obsContent` | `obsContent` | `content_ref` | `obs_content` | no | Artifact | 375-391 |
| `src` | `networkSrc` | `src_ref` | `src` | no | Domain-Name, IPv4, IPv6, Mac | 392-408 |
| `dst` | `networkDst` | `dst_ref` | `dst` | no | Domain-Name, IPv4, IPv6, Mac | 409-425 |
| `srcPayload` | `srcPayload` | `src_payload_ref` | `src-payload` | no | Artifact | 426-442 |
| `dstPayload` | `dstPayload` | `dst_payload_ref` | `dst-payload` | no | Artifact | 443-459 |
| `encapsulates` | `networkEncapsulates` | `encapsulates_refs` | `encapsulates` | yes | Network-Traffic | 460-476 |
| `encapsulatedBy` | `encapsulatedBy` | `encapsulated_by_ref` | `encapsulated-by` | no | Network-Traffic | 477-493 |
| `openedConnections` | `openedConnections` | `opened_connection_refs` | `opened-connection` | yes | Network-Traffic | 494-510 |
| `creatorUser` | `creatorUser` | `creator_user_ref` | `creator-user` | no | User-Account | 511-527 |
| `image` | `processImage` | `image_ref` | `image` | no | StixFile | 528-544 |
| `parent` | `processParent` | `parent_ref` | `parent` | no | Process | 545-561 |
| `child` | `processChild` | `child_refs` | `child` | yes | Process | 562-578 |
| `bodyMultipart` | `bodyMultipart` | `body_multipart` | `body-multipart` | yes | Email-Mime-Part-Type | 579-595 |
| `values` | `winRegValues` | `values_refs` | `values` | yes | Windows-Registry-Value-Type | 596-612 |
| `serviceDlls` | `serviceDlls` | `service_dll_refs` | `service-dll` | yes | StixFile | 631-647 |
| `xOpenctiLinkedTo` (deprecated, migration only) | `xOpenctiLinkedTo` | `x_opencti_linked_to_refs` | `x_opencti_linked-to` | yes | Stix-Core-Object | 613-630 |

Inbound refs from non-observable entities that target observables (for completeness):

| Owner | Input | STIX name | Mult. | Targets | Source |
|---|---|---|---|---|---|
| Malware | `samples` | `sample_ref` (constant `ATTRIBUTE_SAMPLE`, `opencti-platform/opencti-graphql/src/modules/malwareAnalysis/malwareAnalysis-types.ts:20`) | yes | Artifact, StixFile | `REFDEF:171-187`, registered `opencti-platform/opencti-graphql/src/modules/relationsRef/stixDomainObject-registrationRef.ts:48-52` |
| Malware | `operatingSystems` | `operating_system_refs` | yes | Software | `REFDEF:154-170` |
| Malware-Analysis | host_vm / operating_system / installed_software / analysis_sco / sample | `host_vm_ref`, `operating_system_ref`, `installed_software_refs`, `analysis_sco_refs`, `sample_ref` | per ref | Software (x3); any observable; Artifact, StixFile, Network-Traffic, Domain-Name, Url, Hostname, ... | `opencti-platform/opencti-graphql/src/modules/malwareAnalysis/malwareAnalysis.ts:114-205` |

**[Q]** Malware `samples` uses STIX name `sample_ref` although it is multiple (STIX Malware property is `sample_refs`).

## 7. Per-type catalogue

Subsection numbers 7.1 to 7.37 equal the inventory row numbers of section 1 (rows 21 to 26, 28, 29 and 34 to 37 share section 7.21). Unless stated otherwise every attribute below is `string/short`, `isFilterable: true`, `editDefault: false`.

### 7.1 Autonomous-System (`autonomous-system`, STD)

Attributes (`ATTR:79-83`; GQL type 7898-7901, input 7927-7938):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `number` | numeric/integer | ext (ED) | `Int!` | no | yes | Autonomous system number | `number` |
| `name` | string | no | `String` | no | yes | Name | `name` |
| `rir` | string | no | `String` | no | yes | RIR | `rir` |

- Refs: none of its own (`REF:148`). Inbound: IPv4/IPv6 `belongs_to_refs`.
- Standard id: `number` (`ID:99`). Matches STIX.
- Validation: `systemChecker = /^\d{0,10}$/` on `number` (`SYN:162`, `SYN:177-179`), message "Must be numeric".
- `observable_value`: `name || number` (`FMT:253-254`).
- Export: `CONV:671-679`. pycti: `PYO:593-604`.
- **[Q]** `number` is `precision: 'integer'` (32-bit) and GraphQL `Int` (32-bit signed), while 4-byte ASNs go up to 4294967295 (10 digits, accepted by the regex). ASNs above 2147483647 cannot be represented.
- **[R]** `AsNumber(u32)`, reject 0? (AS0 is reserved but appears in data; decide policy), optional `rir` as an enum (AFRINIC, APNIC, ARIN, LACNIC, RIPE NCC) with an `Other` escape.

### 7.2 Directory (`directory`, STD)

Attributes (`ATTR:84-90`; GQL 8036-8041, input 8067-8080):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `path` | string | ext (ED) | `String!` | no | yes | Path | `path` |
| `path_enc` | string | no | `String` | no | yes | Path encoding | `path_enc` |
| `ctime` | date | no | `DateTime` | no | yes | Ctime | `ctime` |
| `mtime` | date | no | `DateTime` | no | yes | Mtime | `mtime` |
| `atime` | date | no | `DateTime` | no | yes | Atime | `atime` |

- Refs: `contains` (`containsObservable` / `contains_refs`, multiple) restricted to `Directory`, `StixFile` (`REF:85-87`; also re-registered with `[]` at `REF:152`, which keeps the existing map). Inbound: `StixFile.parent_directory_ref`.
- Standard id: `path` (`ID:100`). Matches STIX.
- Validation: none in backend.
- `observable_value`: `path` (`FMT:255-256`). Export `CONV:714-725`. pycti `PYO:605-626`.
- **[K]** STIX `path_enc` must be an IANA character-set name; OpenCTI does not check it.

### 7.3 Domain-Name (`domain-name`, STD)

Attributes (`ATTR:91-93`; GQL 8178-8179, input 8205-8214):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `value` | string | **no** | `String!` | no | yes | Value | `value` |

- Refs: `resolvesTo` (`resolves_to_refs`, multiple) restricted to `Domain-Name`, `IPv4-Addr`, `IPv6-Addr` (`REF:88-90`).
- Standard id: `value` (`ID:101`).
- Normalisation: value lower-cased on GraphQL create only (`DOM:212-215`: `lowerCaseTypes = ['Domain-Name', 'Email-Addr']`); not on edit, not elsewhere.
- Validation (`SYN:163`, `SYN:180-182`, message "Valid domain name"):
```
domainChecker = /^(?=.{1,253}$)(?!-)(?:[^\s.](?:[^\s.]{0,61}[^\s.])?\.)+[^\s.]{2,63}$/
```
  (requires at least one dot, labels up to 63 non-space non-dot chars, so IDN unicode labels pass; no punycode conversion.)
- `observable_value`: `value` (default branch `FMT:289-290`). Export `CONV:726-733`. pycti `PYO:627-636`.
- **[Q]** Registration says not mandatory while GraphQL input requires it.
- **[R]** `DomainName` value object: trim, strip trailing dot, lowercase, IDNA to A-label (store both A-label canonical and U-label display), validate with the regex above plus label rules.

### 7.4 Email-Addr (`email-addr`, STD)

Attributes (`ATTR:94-97`; GQL 8312-8314, input 8340-8350):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `value` | string | ext (ED) | `String` | no | yes | Value | `value` |
| `display_name` | string | no | `String` | no | yes | Display name | `display_name` |

- Refs: `belongsTo` (`belongs_to_refs` STIX name, **multiple**) restricted to `User-Account` (`REF:106-108`). Export emits singular `belongs_to_ref` = first element (`CONV:740`); STIX defines `belongs_to_ref` (single) (`.stix2-json-schemas/schemas/observables/email-addr.json:33`).
- Standard id: `value` (`ID:102`). Lower-cased on create (`DOM:212-215`).
- Validation (`SYN:165`, `SYN:205-207`, "Valid email address"):
```
emailChecker = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
```
- `observable_value`: `value`. Export `CONV:734-742`. pycti `PYO:637-649`.
- **[R]** Model `belongs_to` as `Option<UserAccountId>` (single) to stay STIX-exact.

### 7.5 Email-Message (`email-message`, STD)

Attributes (`ATTR:98-106`; GQL 8448-8455, input 8481-8496):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `is_multipart` | boolean | no | `Boolean` | no | yes | Multipart | `is_multipart` |
| `attribute_date` | date | no | `DateTime` | no | yes | Email date | `date` (renamed) |
| `content_type` | string | no | `String` | no | yes | Content type | `content_type` |
| `message_id` | string | no | `String` | no | yes | Message ID | `message_id` |
| `subject` | string | no | `String` | no | yes | Subject | `subject` |
| `received_lines` | string | no | `[String]` | **yes** | yes | Received lines | `received_lines` |
| `body` | string | no | `String` | no | yes | Body | `body` |

- Refs (`REF:91-105`): `emailFrom` (`from_ref`) Email-Addr; `sender` (`emailSender`/`sender_ref`) Email-Addr; `emailTo` (`to_refs`) Email-Addr; `cc` (`emailCc`/`cc_refs`) Email-Addr; `bcc` (`emailBcc`/`bcc_refs`) Email-Addr; `bodyMultipart` (`body_multipart`) Email-Mime-Part-Type; `rawEmail` (`raw_email_ref`) Artifact; `contains` (`contains_refs`) restricted to StixFile, Artifact, Url, Email-Addr (**OpenCTI extension**: exported inside the OCTI SCO extension, `CONV:765-768`; STIX `email-message` has no `contains_refs`).
- Standard id: `from` (renamed `from_ref`), `subject`, `body` (`ID:103`, resolver `from` returns `standard_id` `ID:148-150`). STIX: `from_ref, subject, body`.
- Not supported: `additional_header_fields` (always exported as `{}`, "TODO Implement" `CONV:759`).
- `observable_value`: `body || subject` (`FMT:257-258`). Export `CONV:743-771`. pycti `PYO:650-682` (maps STIX `date` to `attribute_date`).
- Indicator-pattern extraction skips Email-Message when the pattern uses `from_ref` or `sender_ref` (`SYN:108-110`, issue #5293).
- **[Q]** The id contribution key is `from` but the create input key is `emailFrom` (`REFDEF:65`), so `from_ref` most likely never contributes to the OpenCTI standard id (it would then be computed from `subject`+`body` only).
- **[Q]** STIX export of embedded `body_multipart[].body_raw_ref` takes the *email message's* `bodyRaw`, not each part's (`CONV:268-278`, line 274).
- **[R]** Model `body_multipart` as an ordered `Vec<MimePart>` value-object collection inside the EmailMessage aggregate (STIX semantics) while still allowing a MIME part to reference an Artifact/File; keep `contains` as an explicit `x_opencti` extension if needed.

### 7.6 Email-Mime-Part-Type (`email-mime-part-type`, EMB)

Attributes (`ATTR:107-111`; GQL 8594-8597, input 8623-8634):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `body` | string | no | `String` | no | yes | Body | `body` |
| `content_type` | string | no | `String` | no | yes | Content type | `content_type` |
| `content_disposition` | string | no | `String` | no | yes | Content disposition | `content_disposition` |

- Refs: `bodyRaw` (`body_raw_ref`) restricted to Artifact, StixFile (`REF:109-111`). Inbound: `Email-Message.body_multipart`.
- Standard id: `body`, `content_type`, `content_disposition` (`ID:144`, "Types embedded").
- `observable_value`: default branch `value || name || 'Unknown'` so always `Unknown` (`FMT:289-290`).
- Export: as standalone SCO with top-level `labels/description/score/created_by_ref/external_references` and `new-sco` (`CONV:1461-1484`), and also embedded in the parent message (`CONV:268-278`). pycti `PYO:683-702`.
- **[K]** In STIX 2.1 this is not a top-level SCO; it is the `email-mime-part-type` sub-object of `email-message.body_multipart`.

### 7.7 Artifact (`artifact`, STD, hashed)

Attributes (`ATTR:112-120`; GQL 8873-8881, input 8907-8922):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `hashes` | object (section 5) | no | `[HashInput]` | no | yes | Hashes | `hashes` (defaults to `{}`) |
| `mime_type` | string | no | `String` | no | yes | Mime type | `mime_type` |
| `payload_bin` | string | no | `String` | no | yes | Payload | `payload_bin` |
| `url` | string | no | `String` | no | yes | URL | `url` |
| `encryption_algorithm` | string | no | `String` | no | yes | Encryption algorithm | `encryption_algorithm` |
| `decryption_key` | string | no | `String` | no | yes | Decryption key | `decryption_key` |
| `x_opencti_additional_names` | string | no | `[String]` | **yes** | yes | Additional names | OCTI SCO ext `additional_names` |

- Refs: none of its own. Inbound: StixFile `content_ref`, Email-Message `raw_email_ref`/`contains_refs`, Email-Mime-Part `body_raw_ref`, Network-Traffic `src/dst_payload_ref`, Malware samples.
- Standard id: first non-empty way among `[hashes]`, then `[url]` (`ID:104`; comment: payload_bin excluded "Cause of volume"). STIX: `hashes, payload_bin`.
- Invariants: `payload_bin` and `url` mutually exclusive on create (`DOM:235-237`) and on edit (`DOM:285-309`). Hash syntax check (section 5).
- Upload path: if the AddInput carries `file` and no `payload_bin`, `artifactImport` is used (`DOM:216-218`, `DOM:428-483`): computes MD5, SHA-1, SHA-256, SHA-512 of the content (`DOM:466-471`), sets `mime_type` from `file-type` sniffing, `x_opencti_additional_names = [filename]`, stores the binary under `import/Artifact/<id>`; password-protected ZIP is auto-extracted with `app:artifact_zip_password` up to `ARTIFACT_ZIP_MAX_EXTRACTED_SIZE = 104857600` bytes (`DOM:395-417`), Office Open XML files are never extracted (`DOM:419-438`).
- Upsert: name/additional-names preservation logic applies to Artifact and StixFile (`MW:2374-2393`).
- `observable_value`: hash (`SHA-512>SHA-256>SHA-1>MD5`) `|| payload_bin || url` (`FMT:259-260`). Export `CONV:651-670`. pycti `PYO:703-743` (also uploads decoded `payload_bin` as a file, `PYO:1508-1519`).
- **[K]** STIX requires `mime_type` to match the `mime-type` pattern `^(application|audio|font|image|message|model|multipart|text|video)/[a-zA-Z0-9.+_-]+` (`.stix2-json-schemas/schemas/observables/artifact.json`); OpenCTI does not validate it.
- **[R]** Model `ArtifactContent` as an enum `Payload(Bytes) | Url(Url)` (never both), `encryption_algorithm` as STIX `encryption-algorithm-enum` (`AES-256-GCM`, `ChaCha20-Poly1305`, `mime-type-indicated`).

### 7.8 StixFile (`file`, STD, hashed)

Attributes (`ATTR:121-133`; GQL 9020-9033, input 9059-9078):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `hashes` | object | no | `[HashInput]` | no | yes | Hashes | `hashes` |
| `extensions` | string | no | (absent) | no | yes | Extensions | not exported |
| `size` | numeric/long | no | `Int` | no | yes | File size | `size` |
| `name` | string | no | `String` | no | yes | Name | `name` |
| `name_enc` | string | no | `String` | no | yes | Name encoding | `name_enc` |
| `magic_number_hex` | string | no | `String` | no | yes | Magic number hex | `magic_number_hex` |
| `ctime` / `mtime` / `atime` | date | no | `DateTime` | no | yes | Ctime/Mtime/Atime | same |
| `x_opencti_additional_names` | string | no | `[String]` | **yes** | yes | Additional names | OCTI SCO ext `additional_names` |
| `mime_type` | string | no | `String` | no | yes | Mime type | `mime_type` |

- Refs (`REF:112-116`): `contains` (`contains_refs`) to **any** Stix-Cyber-Observable; `parentDirectory` (`parent_directory_ref`) to Directory; `obsContent` (`content_ref`) to Artifact (GraphQL exposes it as typed field `obsContent: Artifact`, `GQL:9033`, resolver `RES:83-85`; input `obsContent: ID`, `GQL:9070`).
- Standard id: first non-empty of `[hashes]`, then `[name]` (`ID:105`). Names are case-sensitive in the id (`ID:173-180`, test `opencti-platform/opencti-graphql/tests/01-unit/domain/identifier-test.js:216-217`). STIX: `hashes, name, extensions, parent_directory_ref` (OpenCTI ignores the last two).
- Name history: on upsert, a replaced `name` is pushed into `x_opencti_additional_names`, which is never shrunk by upsert (`MW:2374-2393`); on merge the source names are appended (`MW:1936-1938`).
- Not supported: STIX file extensions `archive-ext`, `ntfs-ext`, `pdf-ext`, `raster-image-ext`, `windows-pebinary-ext` (declared only in `SCO:127-167` type definitions, never registered nor exported by `CONV:772-797`).
- Validation: hashes (section 5). Frontend requires at least one of MD5/SHA-1/SHA-256/SHA-512/name (`FE:546-603`).
- `observable_value`: hash `|| name` (`FMT:261-262`). pycti `PYO:744-798`.
- **[Q]** GraphQL `size: Int` (32-bit) while the attribute is `long`.
- **[R]** `File` aggregate with `FileName` (case-preserving, no path separators), `FileSize(u64)`, `MagicNumberHex` (even-length hex), `MimeType`; File extensions as typed optional value objects if Fennec wants STIX completeness.

### 7.9 X509-Certificate (`x509-certificate`, STD, hashed)

Attributes (`ATTR:134-163`; GQL 9176-9206, input 9232-9269). All optional, `upsert: true`:

| Attribute | Type | AddInput | Label | STIX export |
|---|---|---|---|---|
| `hashes` | object | `[HashInput]` | Hashes | `hashes` |
| `is_self_signed` | boolean | `Boolean` | Self signed | `is_self_signed` |
| `version` | string | `String` | Version | `version` |
| `serial_number` | string | `String` | Serial number | `serial_number` |
| `signature_algorithm` | string | `String` | Signature algorithm | `signature_algorithm` |
| `issuer` | string | `String` | Issuer | `issuer` |
| `validity_not_before` | date | `DateTime` | Validity not before | `validity_not_before` |
| `validity_not_after` | date | `DateTime` | Validity not after | `validity_not_after` |
| `subject` | string | `String` | Subject | `subject` |
| `subject_public_key_algorithm` | string | `String` | Subject public key algorithm | same |
| `subject_public_key_modulus` | string | `String` | Subject public key modulus | same |
| `subject_public_key_exponent` | numeric/integer | `Int` | Subject public key exponent | same |
| `basic_constraints` | string | `String` | Basic constraints | `x509_v3_extensions.basic_constraints` |
| `name_constraints` | string | `String` | Name constraints | `x509_v3_extensions.*` |
| `policy_constraints` | string | `String` | Policy constraints | `x509_v3_extensions.*` |
| `key_usage` | string | `String` | Key usage | `x509_v3_extensions.*` |
| `extended_key_usage` | string | `String` | Extended key usage | `x509_v3_extensions.*` |
| `subject_key_identifier` | string | `String` | Subject key identifier | `x509_v3_extensions.*` |
| `authority_key_identifier` | string | `String` | Authority key identifier | `x509_v3_extensions.*` |
| `subject_alternative_name` | string | `String` | Subject alternative name | `x509_v3_extensions.*` |
| `issuer_alternative_name` | string | `String` | Issuer alternative name | `x509_v3_extensions.*` |
| `subject_directory_attributes` | string | `String` | Subject directory attributes | `x509_v3_extensions.*` |
| `crl_distribution_points` | string | `String` | CRL distribution points | `x509_v3_extensions.*` |
| `inhibit_any_policy` | string | `String` | Inhibit any policy | `x509_v3_extensions.*` |
| `private_key_usage_period_not_before` | date | `DateTime` | Private key usage not before | `x509_v3_extensions.*` |
| `private_key_usage_period_not_after` | date | `DateTime` | Private key usage not after | `x509_v3_extensions.*` |
| `certificate_policies` | string | `String` | Certifiate policies (sic) | `x509_v3_extensions.*` |
| `policy_mappings` | string | `String` | Policy mappings | `x509_v3_extensions.*` |

- OpenCTI stores the v3 extension fields flat; export nests them under `x509_v3_extensions` (`CONV:1130-1147`). pycti reads them flat from the STIX object (`PYO:799-941`), defaulting `is_self_signed` to `False`.
- Refs: none (`REF:165`).
- Standard id ways: `[hashes]`, else `[serial_number]`, else `[subject]` (`ID:106`). STIX: `hashes, serial_number` (both together); OpenCTI only uses the first non-empty way and adds a `subject` fallback.
- Validation: none (hash regexes are not applied to X509, `SYN:183-201`).
- `observable_value`: hash `|| subject || issuer` (`FMT:263-264`). Export `CONV:1114-1149`.
- **[R]** Serial number as normalised lowercase hex without separators; `subject_public_key_exponent` as `u64`; dates as `OffsetDateTime`; v3 extensions as a nested value object.

### 7.10 IPv4-Addr (`ipv4-addr`, STD)

Attributes (`ATTR:164-166`; GQL 9367-9369, input 9395-9406):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `value` | string | ext | `String` | no | yes | Value | `value` |

- AddInput additionally accepts `belongsTo: [String]` and `resolvesTo: [String]` (`GQL:9397-9398`, IPv4 only). GraphQL output also has computed `countries: CountryConnection` (`GQL:9369`, `RES:89-91`, via `located-at`).
- Refs (`REF:117-120`): `resolvesTo` (`resolves_to_refs`) restricted to `Mac-Addr`; `belongsTo` (`belongs_to_refs`) restricted to `Autonomous-System`.
- Standard id: `value` (`ID:107`).
- Validation (`SYN:168`, `SYN:208-210`, "Valid IPv4 address"), CIDR allowed:
```
ipv4Checker = /^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])(?:\/([0-9]|[1-2][0-9]|3[0-2]))?$/
```
  No normalisation (e.g. host bits in a CIDR are not cleared). STIX schema pattern is equivalent (`.stix2-json-schemas/schemas/observables/ipv4-addr.json`).
- `observable_value`: `value`. Export `CONV:815-823`. pycti `PYO:989-998`.
- **[R]** `Ipv4Value = Address(Ipv4Addr) | Network(Ipv4Net)`; canonical string form; decide whether to reject or normalise non-zero host bits.

### 7.11 IPv6-Addr (`ipv6-addr`, STD)

Same as IPv4 (`ATTR:167-169`; GQL 9504-9506 (comment says `# IPv4Addr`), input 9532-9541, no `belongsTo/resolvesTo` in input). Refs `REF:121-124` (resolves_to Mac, belongs_to AS). Id `value` (`ID:108`). Validation (`SYN:166`, `SYN:211-213`, "Valid IPv6 address"), CIDR `/0-128` and `fe80::...%zone` allowed:
```
ipv6Checker = /^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))(?:\/([0-9]|[1-9][0-9]|1[0-1][0-9]|12[0-8]))?$/
```
No canonicalisation (RFC 5952 compression, lowercase): `2001:DB8::1` and `2001:db8:0::1` produce different standard ids. Export `CONV:824-832`. pycti `PYO:999-1008`.
- **[R]** Parse with `std::net::Ipv6Addr` / `ipnet`, store RFC 5952 canonical form, lowercase.

### 7.12 Mac-Addr (`mac-addr`, STD)

`value` string, ext, `upsert: true` (`ATTR:170-172`; GQL 9639-9640, input 9666-9675). Refs: none (`REF:154`); inbound IPv4/IPv6 `resolves_to_refs`, Network-Traffic `src/dst_ref`, IMEI `has` (core rel). Id `value` (`ID:109`). Validation (`SYN:167`, `SYN:214-216`, "Valid MAC address"):
```
macAddrChecker = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/
```
No normalisation. STIX schema requires `^([0-9a-f]{2}[:]){5}([0-9a-f]{2})$` (lowercase, colon only; `.stix2-json-schemas/schemas/observables/mac-addr.json:26`). So OpenCTI can store and export STIX-invalid values (`AA-BB-...`).
- **[R]** Accept `:`, `-`, `.` (Cisco `aabb.ccdd.eeff`) and bare 12-hex forms, normalise to lowercase colon form (STIX canonical). EUI-64 optional.

### 7.13 Mutex (`mutex`, STD)

`name` string, ext, upsert true (`ATTR:173-175`; GQL 9773-9774, input 9800-9809, `String` optional). Refs none (`REF:156`). Id `name`, case-sensitive (`ID:110`, test `identifier-test.js:220-221`). No validation. `observable_value`: `name` (`FMT:265-266`). Export `CONV:840-846`. pycti `PYO:1019-1028`.

### 7.14 Network-Traffic (`network-traffic`, STD)

Attributes (`ATTR:176-188`; GQL 9907-9920, input 9946-9966):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `extensions` | string | no | (absent) | no | yes | Extensions | not exported |
| `start` | date | no | `DateTime` | no | yes | Network traffic start date | `start` |
| `end` | date | no | `DateTime` | no | yes | Network traffic end date | `end` |
| `is_active` | boolean | no | `Boolean` | no | yes | Network traffic active | `is_active` |
| `src_port` | numeric/integer | no | `Int` | no | yes | SRC port | `src_port` |
| `dst_port` | numeric/integer | no | `Int` | no | yes | DST port | `dst_port` |
| `protocols` | string | no | `[String]` | **yes** | yes | Protocols | `protocols` |
| `src_byte_count` | numeric/integer | no | `Int` | no | yes | SRC byte count | `src_byte_count` |
| `dst_byte_count` | numeric/integer | no | `Int` | no | yes | DST byte count | `dst_byte_count` |
| `src_packets` | numeric/integer | no | `Int` | no | yes | SRC packets | `src_packets` |
| `dst_packets` | numeric/integer | no | `Int` | no | yes | DST packets | `dst_packets` |

- AddInput refs: `networkSrc: String`, `networkDst: String` (`GQL:9950-9951`). GraphQL output also declares `src_ref: Int`, `dst_ref: Int` (`GQL:9914-9915`) which are not stored attributes **[Q]**.
- Refs (`REF:125-132`): `src` (`networkSrc`/`src_ref`) and `dst` (`networkDst`/`dst_ref`) restricted to Domain-Name, IPv4, IPv6, Mac; `srcPayload` (`src_payload_ref`) and `dstPayload` (`dst_payload_ref`) Artifact; `encapsulates` (`networkEncapsulates`/`encapsulates_refs`, multiple) Network-Traffic; `encapsulatedBy` (`encapsulated_by_ref`) Network-Traffic. Inbound: Process `opened_connection_refs`.
- Standard id: `start, end, networkSrc as src_ref, networkDst as dst_ref, src_port, dst_port, protocols` (`ID:111-119`). STIX also includes `extensions`.
- **[Q]** The id resolvers are keyed `src`/`dst` (`ID:151-156`) but the contribution `src` keys are `INPUT_SRC = 'networkSrc'` / `INPUT_DST = 'networkDst'`, so `filteredIdContributions` (`ID:384-413`) finds no resolver and would embed the resolved ref object itself rather than its `standard_id`. Not runtime-verified; treat OpenCTI Network-Traffic standard ids as non-reproducible.
- Not supported: `ipfix` (exported from `instance.ipfix` but no attribute registered), and the STIX extensions `http-request-ext`, `icmp-ext`, `socket-ext`, `tcp-ext` (only declared in `SCO:439-471`, never exported by `CONV:847-869`).
- Validation: none. Indicator-pattern extraction skips Network-Traffic when the pattern contains `src_ref`/`dst_ref` (`SYN:105-107`).
- `observable_value`: `src_port || dst_port` (`FMT:267-268`) but the ES runtime script uses `dst_port` only (`FMT:345-346`) **[Q]**.
- pycti `PYO:1029-1089` (`src_ref` to `networkSrc`, `dst_ref` to `networkDst`; refs `src_ref`/`dst_ref` are then excluded from the nested-ref loop `PYS:1640-1650`).
- **[R]** Ports `u16`, byte/packet counts `u64` (OpenCTI's 32-bit integers overflow on real traffic), `protocols` as an ordered `Vec<Protocol>` lower-cased (STIX: IANA service names, ordered from outer to inner layer), require at least one of `src_ref`/`dst_ref` ([K] STIX constraint), `end >= start`.

### 7.15 Process (`process`, STD)

Attributes (`ATTR:189-215`; GQL 10064-10088, input 10114-10146):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `extensions` | string | no | (absent) | no | yes | Extensions | not exported |
| `is_hidden` | boolean | no | `Boolean` | no | yes | Hidden | `is_hidden` |
| `pid` | numeric/long | no | `Int` | no | yes | PID | `pid` |
| `created_time` | date | no | `DateTime` | no | yes | Process creation date | `created_time` |
| `cwd` | string | no | `String` | no | yes | CWD | `cwd` |
| `command_line` | string | no | **`String!`** | no | yes | Command line | `command_line` |
| `environment_variables` | string | no | `[String]` | **yes** | yes | Environment variables | `environment_variables` (STIX: dictionary) |
| `aslr_enabled` | boolean | no | `Boolean` | no | **no** | ASLR enabled | `extensions.windows-process-ext.*` |
| `dep_enabled` | boolean | no | `Boolean` | no | **no** | DEP enabled | windows-process-ext |
| `priority` | vocabulary `case_priority_ov` | no | `String` | no | **no** | Priority | windows-process-ext |
| `owner_sid` | string | no | `String` | no | **no** | Owner sid | windows-process-ext |
| `window_title` | string | no | `String` | no | **no** | Window title | windows-process-ext |
| `startup_info` | (commented out, "TODO introduce later", `ATTR:203-204`) | | `[DictionaryInput]` | | | | windows-process-ext |
| `integrity_level` | vocabulary `integrity_level_ov` | no | `String` | no | **no** | Integrity level | windows-process-ext |
| `service_name` | string | no | `String` | no | **no** | Service name | `extensions.windows-service-ext.*` |
| `descriptions` | string | no | `[String]` | **yes** | **no** | Descriptions | windows-service-ext |
| `display_name` | string | no | `String` | no | **no** | Display name | windows-service-ext |
| `group_name` | string | no | `String` | no | **no** | Group name | windows-service-ext |
| `start_type` | vocabulary `start_type_ov` | no | `String` | no | **no** | Start type | windows-service-ext |
| `service_type` | vocabulary `service_type_ov` | no | `String` | no | **no** | Service type | windows-service-ext |
| `service_status` | vocabulary `service_status_ov` | no | `String` | no | **no** | Service status | windows-service-ext |

- ProcessAddInput also carries its own `x_opencti_description: String` (`GQL:10119`).
- Refs (`REF:133-140`): `openedConnections` (`opened_connection_refs`) Network-Traffic; `creatorUser` (`creator_user_ref`) User-Account; `image` (`processImage`/`image_ref`) StixFile; `parent` (`processParent`/`parent_ref`) Process; `child` (`processChild`/`child_refs`) Process; `serviceDlls` (`service_dll_refs`, windows-service-ext) StixFile (GraphQL typed field `serviceDlls: StixFileConnection`, `GQL:10086`, `RES:80-82`).
- Standard id: `pid` (only when `command_line` is present, `dependencies: ['command_line']`) and `command_line` (`ID:120`, comment `// v4`). **[K]** STIX 2.1 defines no id-contributing properties for `process` (UUIDv4); OpenCTI makes it deterministic.
- Export: always writes both `windows-process-ext` and `windows-service-ext` objects (`CONV:886-907`).
- Vocabulary bindings: `integrity_level_ov`, `service_status_ov`, `service_type_ov`, `start_type_ov` are bound to Process (`VOC:201-208`, `VOC:357-383`). Values: integrity `low, medium, high, system` (`VOCU:432-436`); start type `SERVICE_AUTO_START, SERVICE_BOOT_START, SERVICE_DEMAND_START, SERVICE_DISABLED, SERVICE_SYSTEM_ALERT`; service type `SERVICE_KERNEL_DRIVER, SERVICE_FILE_SYSTEM_DRIVER, SERVICE_WIN32_OWN_PROCESS, SERVICE_WIN32_SHARE_PROCESS`; status `SERVICE_CONTINUE_PENDING, SERVICE_PAUSE_PENDING, SERVICE_PAUSED, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOP_PENDING, SERVICE_STOPPED` (`VOCU:924-975`).
- **[Q]** `priority` is bound to `case_priority_ov` (`P1..P4`, `VOCU:123-128`) although STIX `windows-process-ext.priority` is a Windows priority class string (e.g. `HIGH_PRIORITY_CLASS`), and `case_priority_ov.entity_types` does not list Process (`VOC:106-113`).
- **[Q]** `environment_variables` stored as list of strings, STIX expects a dictionary.
- `observable_value`: `pid || command_line` (`FMT:269-270`). pycti maps only the core fields (`PYO:1090-1118`).
- **[R]** `Pid(u32)` or `u64`, `CommandLine` non-empty, env vars as `BTreeMap<String,String>`, Windows extensions as optional typed sub-structs with STIX enums; Process identity should be UUIDv4 per STIX unless Fennec deliberately mirrors OpenCTI dedup.

### 7.16 Software (`software`, STD)

Attributes (`ATTR:216-224`; GQL 10244-10252, input 10286-10301):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `name` | string | no | `String` | no | yes | Name | `name` |
| `cpe` | string | no | `String` | no | yes | CPE | `cpe` |
| `swid` | string | no | `String` | no | yes | SWID | `swid` |
| `languages` | string | no | `[String]` | **yes** | yes | Languages | `languages` |
| `vendor` | string | no | `String` | no | yes | Vendor | `vendor` |
| `version` | string | no | `String` | no | yes | Version | `version` |
| `x_opencti_product` | string | no | `String` | no | yes | Product name | OCTI SCO ext `product` (`CONV:923-926`) |

- Refs: none (`REF:160`). Inbound: Malware `operating_system_refs`, Malware-Analysis host_vm/operating_system/installed_software. Typed GraphQL field `vulnerabilities` (`GQL:10252`, `RES:86-88`).
- Standard id: `name, cpe, swid, vendor, version` (`ID:121`); `name` is lower-cased **for the id only** (`ID:173-180`, test `identifier-test.js:172-178`); all string contributions are trimmed (`ID:408-410`).
- Validation (`SYN:217-219`, "Valid Software attributes"): fails only if `!name && !cpeChecker.test(cpe) && !swid`:
```
cpeChecker = /^cpe:\/\/[a-zA-Z0-9_./:-]+|^cpe:\/[a-zA-Z0-9_./:-]+$/
```
  (CPE 2.2 URI form only). STIX 2.1 requires `name` and requires `cpe` to be a CPE **2.3** formatted string (`.stix2-json-schemas/schemas/observables/software.json:24-31,57-59`), and `languages` to be RFC 5646 tags (`software.json:42`). OpenCTI thus rejects a Software that only has a CPE 2.3 string, and accepts a Software without `name`.
- `observable_value`: `(name || cpe || swid) (+ " (version)")` (`FMT:243-249`, `FMT:271-272`). pycti `PYO:1119-1164`.
- **[R]** `SoftwareName` required (STIX), `Cpe23` value object parsed into its 13 components (accept CPE 2.2 URI on import and convert), `LanguageTag` (BCP 47), case-insensitive name comparison for identity.

### 7.17 Url (`url`, STD)

`value` string, **mandatoryType no**, upsert true (`ATTR:225-227`; GQL 10399-10400, input 10426-10435 `String` optional). Refs none (`REF:145`). Id `value` (`ID:122`). **No backend validation and no normalisation.** `observable_value`: `value`. Export adds a non-standard top-level `score` (`CONV:1058-1065`, typed in `SCO:542-545`) in addition to the OCTI extension. pycti `PYO:1165-1174`.
- **[R]** Parse with the `url` crate (WHATWG), lowercase scheme and host, IDNA host, keep path/query as-is; reject relative URLs; refang before parse.

### 7.18 User-Account (`user-account`, STD)

Attributes (`ATTR:228-255`; GQL 10533-10548, input 10574-10596):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `extensions` | string | no | (absent) | yes | Extensions | not exported |
| `user_id` | string, `format: 'id'`, `entityTypes: [User]` (`ATTR:230-241`) | no | `String` | yes | User ID | `user_id` |
| `credential` | string | no | `String` | yes | Credential | `credential` |
| `account_login` | string | no | `String` | yes | Account login | `account_login` |
| `account_type` | vocabulary `account_type_ov` | no | `String` | yes | Account type | `account_type` |
| `display_name` | string | no | `String` | yes | Display name | `display_name` |
| `is_service_account` | boolean | no | `Boolean` | yes | Service account | same |
| `is_privileged` | boolean | no | `Boolean` | yes | Privileged | same |
| `can_escalate_privs` | boolean | no | `Boolean` | yes | Can escalate privs | same |
| `is_disabled` | boolean | no | `Boolean` | yes | Disabled | same |
| `account_created` | date | no | `DateTime` | yes | Account creation date | same |
| `account_expires` | date | no | `DateTime` | yes | Account expiration date | same |
| `credential_last_changed` | date | no | `DateTime` | yes | Credential last modification date | same |
| `account_first_login` | date | no | `DateTime` | yes | Account first login date | same |
| `account_last_login` | date | no | `DateTime` | yes | Account last login date | same |

- `account_type_ov` (`VOC:52-60`, values `VOCU:14-26`): `facebook, ldap, nis, openid, radius, skype, tacacs, twitter, unix, windows-local, windows-domain` (open vocabulary).
- Refs: none of its own (`REF:163`). Inbound: Email-Addr `belongs_to`, Process/Windows-Registry-Key `creator_user_ref`, core rels `authored-by`/`publishes` with Media-Content.
- Standard id: `account_type, user_id, account_login` (`ID:123`). Matches STIX.
- **[Q]** `user_id` is declared as an `id`-format attribute targeting the internal OpenCTI `User` entity, while in STIX it is the account identifier on the target system (UID, SID, ...).
- Not supported: `unix-account-ext` (`gid, groups, home_dir, shell`) is only declared in `SCO:570-576`.
- `observable_value`: `account_login || user_id` (`FMT:273-274`). Export `CONV:1083-1102`. pycti `PYO:1175-1251`.
- **[R]** Require at least one of `user_id`/`account_login` (OpenCTI does not). `credential` is a secret: consider a separate protected value object.

### 7.19 Windows-Registry-Key (`windows-registry-key`, STD)

Attributes (`ATTR:256-260`; GQL 10694-10697, input 10723-10734):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `attribute_key` | string | no | `String` | yes | Key | `key` (renamed, `CONV:1107`) |
| `modified_time` | date | no | `DateTime` | yes | Key modification date | `modified_time` |
| `number_of_subkeys` | numeric/integer | no | `Int` | yes | Number of sub-keys | `number_of_subkeys` |

- Refs (`REF:141-144`): `values` (`winRegValues`, STIX name `values_refs`, multiple) to Windows-Registry-Value-Type, exported as the STIX embedded `values` list (`CONV:258-267`, `CONV:1108`); `creatorUser` (`creator_user_ref`) User-Account.
- Standard id: `[{ src: 'attribute_key', dst: 'key' }, { src: 'values' }]` (`ID:124`). STIX: `key, values`.
- **[Q]** The rename uses `dst` but the code reads `dest` (`ID:396`), so the id payload key is `attribute_key`, not `key`; and `values` is never a key of the input (the input key is `winRegValues`). OpenCTI registry-key standard ids therefore do not equal STIX-spec ids.
- pycti maps STIX `key` to `attribute_key` and re-creates each embedded STIX value as a separate `windows-registry-value-type` observable linked by a nested ref (`PYO:1252-1271`, `PYO:1520+`).
- `observable_value`: `attribute_key` (`FMT:279-280`). Validation: none. STIX schema expects the key to start with a hive (`^HKLM|HKCC|HKCR|HKCU|HKU|hklm|hkcc|hkcr|hkcu|hku`, `.stix2-json-schemas/schemas/observables/windows-registry-key.json`).
- **[R]** `RegistryKeyPath` with parsed `Hive` enum (accept long names `HKEY_LOCAL_MACHINE` and short forms, normalise per policy; [K] STIX says key matching is case-insensitive), `values: Vec<RegistryValue>` as a value-object list inside the aggregate.

### 7.20 Windows-Registry-Value-Type (`windows-registry-value-type`, EMB)

`name`, `data`, `data_type` strings, optional, upsert true (`ATTR:261-265`; GQL 10832-10835, input 10861-10872). Refs none (`REF:164`); inbound Windows-Registry-Key `values`. Id `name, data, data_type` (`ID:145`). `observable_value`: `name || data` (`FMT:281-282`). Export standalone `new-sco` (`CONV:1443-1460`) and embedded in the key. pycti `PYO:1272-1289`.
- **[K]** STIX `data_type` is the closed `windows-registry-datatype-enum` (`REG_NONE, REG_SZ, REG_EXPAND_SZ, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_DWORD_LITTLE_ENDIAN, REG_LINK, REG_MULTI_SZ, REG_RESOURCE_LIST, REG_FULL_RESOURCE_DESCRIPTION, REG_RESOURCE_REQUIREMENTS_LIST, REG_QWORD, REG_INVALID_TYPE`); OpenCTI stores free text.

### 7.21 OpenCTI custom single-value types (inventory rows 21 to 26, 28, 29, 34 to 37)

**[S]** Common shape for: Cryptographic-Key (`ATTR:266-268`), Cryptocurrency-Wallet (`ATTR:269-271`), Hostname (`ATTR:272-274`), Text (`ATTR:275-277`), User-Agent (`ATTR:278-280`), Phone-Number (`ATTR:286-288`), Credential (`ATTR:302-304`), Tracking-Number (`ATTR:305-307`), AI-Prompt (`ATTR:322-324`), IMEI (`ATTR:325-327`), ICCID (`ATTR:328-330`), IMSI (`ATTR:331-333`):

| Attribute | Type | Mand. | AddInput | Mult. | Up. | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `value` | string/short | ext | `String` (optional at GraphQL level) | no | yes | Value | top-level `value` |

- No refs of their own (`REF:146-170`). Standard id: `value` (`ID:126-142`). `observable_value`: `value`. Export: top-level `value, labels, description, score, created_by_ref, external_references` and OCTI SCO ext `{extension_type: 'new-sco'}` (converters listed in section 10). pycti STIX definitions (`PYC:337-893`): `value` required, `spec_version` fixed `2.1`, id-contributing `["value"]`.

Type-specific details:

| Type | Backend validation (`SYN`) | Normalisation | Notable core relationships (`REL`) | pycti STIX class (`PYC` line) | Notes |
|---|---|---|---|---|---|
| Cryptographic-Key | none | none | | 706 | Detection heuristic only: PEM header or `ssh-*` key (`DET:150-157`) |
| Cryptocurrency-Wallet | none | none | | 508 | Detection heuristics BTC/ETH/LTC/bech32 (`DET:158-167`); constant is named `ENTITY_CRYPTOGRAPHIC_WALLET` |
| Hostname | `hostnameChecker` (`SYN:164`, `SYN:202-204`, "Valid hostname") | none (not lower-cased, unlike Domain-Name) | Hostname to Domain/IPv4/IPv6 `communicates-with`, `resolves-to`; to Artifact/File `drops`; to Attack-Pattern `uses`; to Hostname `derived-from` (`REL:1037-1060`); Domain-Name to Hostname `resolves-to`, `derived-from` (`REL:1213`) | 337 | Hostname may be dot-less, allows `_` |
| Text | none | none | Text to Narrative `uses`, Text to Text `derived-from` (`REL:1071-1076`) | 367 | Fallback type of `detectObservableType` (`DET:41-313`) |
| User-Agent | none | none | | 598 | |
| Phone-Number | none | none | Phone to IMSI `uses`, Phone to ICCID `resolves-to` (`REL:1136-1141`) | 538 | Detection includes E.164 `^\+?[1-9]\d{1,14}$` (`DET:92-100`) |
| Credential | none | none | | 478 | Detection `user:password` (`DET:216-222`) |
| Tracking-Number | none | none | | 568 | |
| AI-Prompt | none | none | AI-Prompt to Software `targets` (`REL:1295`) | 766 | AddInput minimal (`GQL:12782-12789`) |
| IMEI | `imeiChecker = /(^[0-9]{15,16})$/` (`SYN:171`, `SYN:220-222`, "Valid IMEI") | none (no Luhn check) | IMEI to ICCID/IMSI `uses`, IMEI to Mac `has` (`REL:1114-1121`) | 796 (doc: 14 digits + check digit, 16 legacy) | Frontend regex `/^([0-9]{15,16}\n*)+$/i` (`FE:605-609`) |
| ICCID | `iccidChecker = /(^[0-9]{18,22})$/` (`SYN:172`, `SYN:223-225`) | none (no Luhn) | ICCID to IMEI `uses`, to IMSI `has`, to Phone `resolves-to` (`REL:1103-1110`) | 830 (doc: 18 to 20 digits) | Frontend `/^([0-9]{18,22}\n*)+$/i` (`FE:615-619`) |
| IMSI | `imsiChecker = /(^[0-9]{14,15})$/` (`SYN:173`, `SYN:226-228`) | none | IMSI to IMEI `uses`, to ICCID `has`, to Phone `uses` (`REL:1125-1132`) | 863 (doc: MCC+MNC+MSIN) | Frontend `/^([0-9]{14,15}\n*)+$/i` (`FE:625-629`) |

```
hostnameChecker = /^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-_]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-_]*[A-Za-z0-9])$/
```

Converters: Cryptographic-Key `CONV:697-713`, Cryptocurrency-Wallet `CONV:680-696`, Hostname `CONV:798-814`, Text `CONV:930-946`, Credential `CONV:966-982`, Tracking-Number `CONV:983-999`, Phone-Number `CONV:1000-1016`, User-Agent `CONV:1066-1082`, AI-Prompt `CONV:1188-1204`, IMEI `CONV:1206-1222`, ICCID `CONV:1224-1240`, IMSI `CONV:1242-1258`. pycti input mapping `PYO:1290-1338` and `PYO:1355-1364`, `PYO:1455-1505`.

**[R]** Strong value objects: `HostnameValue` (lowercase, IDNA, label rules), `PhoneNumberValue` (E.164 via `phonenumber` crate, keep raw), `ImeiValue` (15 digits + Luhn, accept 16-digit IMEISV as separate variant), `IccidValue` (19 or 20 digits, `89` prefix, Luhn), `ImsiValue` (14 or 15 digits, parse MCC/MNC), `CryptocurrencyWalletAddress` (enum by chain with checksum validation: Base58Check, Bech32/Bech32m, EIP-55), `CryptographicKey` (PEM/OpenSSH/JWK detection, fingerprint computed), `Credential` (secret, never logged), `UserAgentString` (non-empty, length cap), `TrackingNumber` (carrier enum optional), `AiPrompt` and `Text` (non-empty, length cap, NFC).

### 7.27 Bank-Account (`bank-account`, OCTI)

Attributes (`ATTR:281-285`; GQL 11641-11644, input 11670-11681):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `iban` | string | ext | `String` | yes | IBAN | top-level `iban` |
| `bic` | string | no | `String` | yes | BIC | `bic` |
| `account_number` | string | no | `String` | **no** | Account number | `account_number` |

- Refs none (`REF:147`). Core rel: Bank-Account `transferred-to` Bank-Account (`REL:1032`).
- Standard id: `iban` (`ID:131`). pycti STIX class requires both `value` and `iban`, id `["iban"]` (`PYC:439-454`).
- Backend validation: none. Frontend only (`FE:532-544`): `bic /^[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?$/i`, `iban /^[A-Z]{2}[0-9]{2}[A-Z0-9]{1,30}$/i` (no mod-97 check).
- `observable_value`: `iban || number` (`FMT:275-276`) **[Q]** reads `number`, which does not exist on this type (should be `account_number`).
- Export `CONV:947-965`. pycti `PYO:1339-1354`.
- **[R]** `Iban` value object: strip spaces, uppercase, country-length table, ISO 7064 mod-97 check; `Bic` 8/11 chars uppercase; `iban` optional if `account_number` present (OpenCTI forces IBAN, which excludes many non-IBAN countries).

### 7.30 Payment-Card (`payment-card`, OCTI)

Attributes (`ATTR:289-294`; GQL 12187-12191, input 12217-12229):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `card_number` | string | ext (ED) | **`String!`** | **no** | Card number | `card_number` |
| `expiration_date` | date | no | `DateTime` | yes | Expiration date | `expiration_date` |
| `cvv` | numeric/integer | no | `Int` | yes | CVV | `cvv` |
| `holder_name` | string | no | `String` | yes | Holder name | `holder_name` |

- Refs none (`REF:157`). Standard id `card_number` (`ID:135`). pycti STIX class: `value` required, `card_number` required, `expiration_date` and `cvv` as strings, id `["card_number"]` (`PYC:397-414`).
- Validation: none in backend (no Luhn). Detection heuristics `DET:225-235`.
- `observable_value`: `card_number || holder_name` (`FMT:277-278`). Export `CONV:1038-1057`. pycti `PYO:1365-1387` (duplicated branch `PYO:1431-1454`).
- **[Q]** `cvv` as integer loses leading zeros (`012`). `expiration_date` as full timestamp although cards carry month/year.
- **[R]** `Pan` (digits only, 12 to 19, Luhn, derived `CardBrand` and BIN), `CardExpiry { year, month }`, `Cvv` as string of 3 or 4 digits, treat PAN/CVV as sensitive (masking, encryption at rest).

### 7.31 Media-Content (`media-content`, OCTI)

Attributes (`ATTR:295-301`; GQL 12327-12333, input 12359-12372):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `title` | string | no | `String` | yes | Title | `title` |
| `content` | string | no | `String` | yes | Content | `content` |
| `media_category` | string | no | `String` | yes | Media category | `media_category` |
| `url` | string | ext (ED) | **`String!`** | **no** | URL | `url` |
| `publication_date` | date | no | `DateTime` | yes | Media publication date | `publication_date` |

- GraphQL output also has `description: String` (`GQL:12329`) which is not a registered attribute **[Q]**; STIX export fills `description` from `x_opencti_description` (`CONV:1023`).
- Refs none (`REF:155`). Core rels: `authored-by` Individual/Organization/User-Account, `uses` Narrative, `derived-from` Media-Content; User-Account `publishes` Media-Content (`REL:1272-1291`).
- Standard id `url` (`ID:136`). No validation. `observable_value`: `content || title || url` (`FMT:283-284`). Export `CONV:1017-1037`. pycti STIX class `PYC:628-646` (`url` required, id `["url"]`), mapping `PYO:1388-1413`.
- **[R]** Reuse the `Url` value object for `url`; `media_category` as an open vocabulary.

### 7.32 Persona (`persona`, OCTI)

Attributes (`ATTR:308-311`; GQL 12470-12472, input 12498-12508):

| Attribute | Type | Mand. | AddInput | Up. | Label | STIX export |
|---|---|---|---|---|---|---|
| `persona_name` | string | ext | `String! @constraint(minLength: 2, format: "not-blank")` | yes | Persona name | `persona_name` |
| `persona_type` | string (bound to vocabulary `persona_type_ov` by `VOC:302-308`) | ext | `String!` | yes | Persona type | `persona_type` |

- `persona_type_ov` values (`VOCU:862-870`): `true name, previous true name, nickname, moniker, assumed name, alternative spelling/transliteration, unknown`.
- Refs none (`REF:158`). Standard id `persona_name, persona_type` (`ID:139`). `observable_value`: `persona_name` (`FMT:285-286`). Export `CONV:1150-1166`; pycti `PYC:673-687`, `PYO:1414-1430`.
- **[Q]** Export does not emit `description` (no top-level `description`, and the OCTI SCO extension is replaced by `{extension_type: 'new-sco'}`), so `x_opencti_description` is lost in STIX output.

### 7.33 SSH-Key (`ssh-key`, OCTI)

Attributes (`ATTR:312-321`; GQL 12607-12615, input 12641-12657):

| Attribute | Type | Mand. | AddInput | Up. | Filterable | Label | STIX export |
|---|---|---|---|---|---|---|---|
| `key_type` | vocabulary `key_type_ov` (`rsa, ecdsa, ed25519, dsa`, `VOCU:1212-1217`, `VOC:384-391`) | no | `String` | yes | yes | Key type | `key_type` |
| `public_key` | string | no | `String` | yes | yes | Public key | `public_key` |
| `fingerprint_sha256` | string | ext | **`String! @constraint(minLength: 5)`** | yes | yes | Fingerprint sha256 | `fingerprint_sha256` |
| `fingerprint_md5` | string | no | `String @constraint(minLength: 5)` | yes | yes | Fingerprint MD5 | `fingerprint_md5` |
| `key_length` | **string** | no | `String` | yes | yes | Key length | `key_length` |
| `comment` | string | no | `String` | yes | **no** | Comment | `comment` |
| `created` | date (shared `created` def, `ADEF:569-578`) | no | `DateTime` | yes | yes | Original creation date | `created` |
| `expiration_date` | date | no | `DateTime` | yes | yes | Expiration date | `expiration_date` |

- Refs none (`REF:166`). Standard id `fingerprint_sha256` (`ID:137`).
- Indicator generation keys: `SSH-Key_sha256` / `SSH-Key_md5` (`DOM:106-116`). Pattern extraction maps `Ssh-Key` to `SSH-Key` (`SYN:59-61`).
- `observable_value`: `fingerprint_sha256 || fingerprint_md5 || key_type` (`FMT:287-288`). Export `CONV:1167-1186`; pycti mapping `PYO:942-988`.
- **[Q]** pycti's STIX class declares only `value` (required) with id `["value"]` (`PYC:736-749`), inconsistent with the backend (id from `fingerprint_sha256`).
- **[Q]** Export drops `labels`, `description`, `score`, `created_by_ref` (only `external_references` kept at top level and the OCTI SCO extension replaced) (`CONV:1170-1185`).
- **[Q]** `key_length` is string in registration/GraphQL but `number` in the TS STIX type (`SCO:703`).
- **[R]** `SshPublicKey` parsed from OpenSSH format (`ssh-keys` crate), fingerprints **derived** from the key when present (SHA256 base64 without padding, MD5 colon hex), `KeyLength(u32)`, `SshKeyType` closed enum plus `Other`.

### 7.38 Summary of mandatory and identity attributes per type

| Type | Registration `external` | GraphQL AddInput `!` | OpenCTI standard-id fields (`ID:96-146`) | STIX 2.1 spec id fields [K] |
|---|---|---|---|---|
| Autonomous-System | number | number | number | number |
| Directory | path | path | path | path |
| Domain-Name | (none) | value | value | value |
| Email-Addr | value | (none) | value | value |
| Email-Message | (none) | (none) | from(_ref), subject, body | from_ref, subject, body |
| Email-Mime-Part-Type | (none) | (none) | body, content_type, content_disposition | n/a (embedded) |
| Artifact | (none) | (none) | hashes, else url | hashes, payload_bin |
| StixFile | (none) | (none) | hashes, else name | hashes, name, extensions, parent_directory_ref |
| X509-Certificate | (none) | (none) | hashes, else serial_number, else subject | hashes, serial_number |
| IPv4-Addr / IPv6-Addr / Mac-Addr | value | (none) | value | value |
| Mutex | name | (none) | name | name |
| Network-Traffic | (none) | (none) | start, end, src_ref, dst_ref, src_port, dst_port, protocols | + extensions |
| Process | (none) | command_line | pid (needs command_line), command_line | none (UUIDv4) |
| Software | (none) | (none) | name (lowercased), cpe, swid, vendor, version | name, cpe, swid, vendor, version |
| Url | (none) | (none) | value | value |
| User-Account | (none) | (none) | account_type, user_id, account_login | account_type, user_id, account_login |
| Windows-Registry-Key | (none) | (none) | attribute_key (+ values, unreachable) | key, values |
| Windows-Registry-Value-Type | (none) | (none) | name, data, data_type | n/a (embedded) |
| 12 single-value OCTI types | value | (none) | value | n/a |
| Bank-Account | iban | (none) | iban | n/a |
| Payment-Card | card_number | card_number | card_number | n/a |
| Media-Content | url | url | url | n/a |
| Persona | persona_name, persona_type | both | persona_name, persona_type | n/a |
| SSH-Key | fingerprint_sha256 | fingerprint_sha256 | fingerprint_sha256 | n/a |

## 8. Syntax validation and normalisation (backend)

**[S]** `checkObservableSyntax(type, data)` (`SYN:175-233`) returns `true` or an error label; called on create (`DOM:243-246`, error `Observable is not correctly formatted`, doc code `DOC_INCORRECT_OBSERVABLE_FORMAT`), on every update after merge (`MW:2726-2732`), and when forms materialise entities (`opencti-platform/opencti-graphql/src/modules/form/form-entity-materializer.ts:82`). Types not listed fall to `default: return true`, i.e. **no validation** for: Directory, Email-Message, Email-Mime-Part-Type, X509-Certificate, Mutex, Network-Traffic, Process, Url, User-Account, Windows-Registry-Key, Windows-Registry-Value-Type, and all OCTI types except Hostname, IMEI, ICCID, IMSI.

All checker constants (`SYN:162-173`), verbatim:
```
systemChecker   = /^\d{0,10}$/
domainChecker   = /^(?=.{1,253}$)(?!-)(?:[^\s.](?:[^\s.]{0,61}[^\s.])?\.)+[^\s.]{2,63}$/
hostnameChecker = /^(([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-_]*[a-zA-Z0-9])\.)*([A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9\-_]*[A-Za-z0-9])$/
emailChecker    = /^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/
ipv6Checker     = (see 7.11)
macAddrChecker  = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/
ipv4Checker     = /^(?:(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\.){3}(?:25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])(?:\/([0-9]|[1-2][0-9]|3[0-2]))?$/
cpeChecker      = /^cpe:\/\/[a-zA-Z0-9_./:-]+|^cpe:\/[a-zA-Z0-9_./:-]+$/
imeiChecker     = /(^[0-9]{15,16})$/
iccidChecker    = /(^[0-9]{18,22})$/
imsiChecker     = /(^[0-9]{14,15})$/
```

Normalisations actually applied by the backend:

| Normalisation | Where | Scope |
|---|---|---|
| Lower-case `value` | `DOM:212-215` | Domain-Name, Email-Addr; GraphQL create only |
| Hash algorithm upper-case + trim, hash value trim | `FDA:29-37` | Hashed types, GraphQL create |
| Trim of string id contributions | `ID:408-410` | All types, id computation only |
| Software `name` lower-case | `ID:173-180` | id computation only |
| Hash key upper-case | `ID:157-160` | id computation only |
| Score integer 0..100 | `FMT:294-302` | All, create and edit |
| Artifact url XOR payload_bin | `DOM:235-237`, `DOM:285-309` | Artifact |
| Keyword search lowercase/asciifolding | `engine.ts:1184-1187` | Search/dedup lookups only |
| Search input refang | `engine.ts:1948-1951` via `opencti-platform/opencti-graphql/src/utils/refang.js:15+` | Search only |

**[S]** GraphQL-level constraints: `HashInput.algorithm minLength 3`, `HashInput.hash minLength 5` (`GQL:8637-8638`), `Persona.persona_name minLength 2, not-blank` (`GQL:12499`), `SSHKey.fingerprint_sha256/md5 minLength 5` (`GQL:12644-12645`).

## 9. Standard id (deterministic STIX id) generation

**[S]** `generateStandardId` (`ID:509-520`): observables use `generateStixId` with the OASIS namespace `00abedb4-aa42-466c-9c01-fed23315a9b7` (`GEN:56`); `id = <stix-type>--uuidv5(JCS-canonicalised JSON of contributing data, OASIS_NAMESPACE)` (`ID:331-347`, uses `canonicalize` RFC 8785). The type is injected as `opencti_type` for resolver use only (`ID:415-417`).

**[S]** Algorithm (`ID:384-447`):
- A definition is either a flat list (all listed fields that are present contribute) or a list of alternative "ways" (array of arrays): the first way producing a non-empty payload wins (`ID:428-439`).
- For each present field: if `dependencies` are listed and any is empty, the whole way yields `{}` (`ID:397-401`); key renamed to `dest` if set (`ID:402`); value passed through a resolver when one exists for the field name (`ID:403-407`), else strings are trimmed (`ID:409`); empty/nil values are dropped (`ID:412`).
- Resolvers (`ID:147-181`): `from`/`src`/`dst` return the ref's `standard_id`; `hashes` keeps a single hash by priority (section 5); `name` lower-cases for Software only.
- On update, if a contributing field changes, `standard_id` is regenerated and the previous one is kept in `x_opencti_stix_ids` (`MW:2397-2430`, `ID:449-480` for upgrade/downgrade detection).

**[Q]** Quirks affecting STIX-id fidelity (from reading the code, not runtime-verified): Windows-Registry-Key (`dst` typo, `values` key mismatch), Network-Traffic (resolver key mismatch on `networkSrc`/`networkDst`), Email-Message (`from` vs `emailFrom` input key), Artifact (`url` instead of `payload_bin`), File (no `extensions`/`parent_directory_ref`), X509 (ways instead of combined), Process (deterministic instead of UUIDv4).

**[R]** Fennec should compute STIX-spec ids exactly (RFC 8785 + UUIDv5 with the OASIS namespace, spec contributing properties, spec hash selection) as the canonical `stix_id`, and if OpenCTI interop requires it, additionally compute an `opencti_compat_id` with the OpenCTI rules above, stored as an alias (like `x_opencti_stix_ids`).

## 10. STIX 2.1 export shape (`CONV`)

**[S]** Common builder `buildStixCyberObservable` (`CONV:319-337`): `id = standard_id`, `spec_version: '2.1'`, `type = convertTypeToStixType(entity_type)`, `defanged = instance.defanged` (**[Q]** no `defanged` attribute is registered anywhere, so it is effectively always absent), `object_marking_refs`, and two extensions:
- `extension-definition--ea279b3e-5c71-4632-ac08-831c66a786ba` (`STIX_EXT_OCTI`, `EXT:6`): OpenCTI internals (`id`, `type`, `created_at`, `updated_at`, `modified_at`, `files`, `stix_ids`, `is_inferred`, `granted_refs`, `creator_ids`, `labels_ids`, `created_by_ref_id`, `workflow_id`, `metrics`, ...; `CONV:163-211`).
- `extension-definition--f93e2c80-4231-4f9a-af8b-95c9bd566a82` (`STIX_EXT_OCTI_SCO`, `EXT:13`): `{extension_type: 'property-extension', labels, description, score, created_by_ref, external_references}`.

Per-type deviations:

| Type(s) | Extra in OCTI SCO extension | Top-level extras | Converter |
|---|---|---|---|
| Artifact | `additional_names` | | `CONV:651-670` |
| StixFile | `additional_names` (default `[]`) | `parent_directory_ref`, `contains_refs`, `content_ref` | `CONV:772-797` |
| Software | `product` (from `x_opencti_product`) | | `CONV:910-929` |
| Email-Message | `contains_refs` | `date` (from `attribute_date`), embedded `body_multipart`, `additional_header_fields: {}` | `CONV:743-771` |
| Windows-Registry-Key | | `key` (from `attribute_key`), embedded `values` | `CONV:1103-1113` |
| X509-Certificate | | nested `x509_v3_extensions` | `CONV:1114-1149` |
| Process | | `windows-process-ext`, `windows-service-ext` (inside `extensions`) | `CONV:870-909` |
| Url | | `score` (non-standard) | `CONV:1058-1065` |
| 17 OCTI types + 2 EMB types | replaced by `{extension_type: 'new-sco'}` | `labels`, `description`, `score`, `created_by_ref`, `external_references` | listed in 7.x |
| Persona | `{extension_type: 'new-sco'}` | `labels`, `score`, `created_by_ref`, `external_references` (**no `description`**) | `CONV:1150-1166` |
| SSH-Key | `{extension_type: 'new-sco'}` | `external_references` only (**no labels/description/score/created_by_ref**) | `CONV:1167-1186` |

**[S]** Dispatch for observables: `CONV:1603-1720`.

## 11. STIX import mapping (python client / connectors)

**[S]** pycti `import_observable` (`PYS:1470-1690`): extracts meta refs (`created_by_ref` or `x_opencti_created_by_ref` or OCTI extension `created_by_ref`; `labels` or `x_opencti_labels`; `external_references` or `x_opencti_external_references`; `granted_refs` or `x_opencti_granted_refs`), files from `x_opencti_files` or OCTI extension `files`, then calls `stix_cyber_observable.create` and afterwards turns every remaining `*_ref` / `*_refs` key (except `created_by_ref`, `object_marking_refs`, `x_opencti_created_by_ref`, `x_opencti_granted_refs`, `src_ref`, `dst_ref`) into nested ref relationships with relationship type = key minus suffix, `_` replaced by `-` (`PYS:1640-1685`).

**[S]** Type normalisation (`PYO:351-398`): STIX `type.title()`, then special cases: `file` to `StixFile`, `ipv4-addr` to `IPv4-Addr`, `ipv6-addr` to `IPv6-Addr`, `persona`, `ssh-key`, `ai-prompt`, `hostname`/`x-opencti-hostname`, `payment-card`/`x-opencti-payment-card`, `credential`/`x-opencti-credential`, `tracking-number`/`x-opencti-tracking-number`, `cryptocurrency-wallet`/`x-opencti-cryptocurrency-wallet`, `user-agent`/`x-opencti-user-agent`, `cryptographic-key`/`x-opencti-cryptographic-key`, `imei`/`x-opencti-imei`, `iccid`/`x-opencti-iccid`, `imsi`/`x-opencti-imsi`, `text`/`x-opencti-text`.

**[S]** Field renames on import: STIX `date` to `attribute_date` (Email-Message), `key` to `attribute_key` (Windows-Registry-Key), `src_ref`/`dst_ref` to `networkSrc`/`networkDst`, OCTI extension `additional_names` to `x_opencti_additional_names`, OCTI extension `x_opencti_product` to `x_opencti_product`, `description`/`score` read from top level `x_opencti_description`/`x_opencti_score` or from the OCTI extension (`PYO:400-420`). `x_opencti_create_indicator` triggers indicator creation (`PYO:338-342`).

**[S]** `simple-observable` (pycti-only pseudo STIX type, `PYC:65`, `PYS:1548-1590`): `{key: "<Type>.<attribute>", value}` (e.g. `Domain-Name.value`, `File.hashes.SHA-256`), converted to a real observable (`PYO:344-350`, `PYO:420-436`).

**[S]** Indicator pattern to observable (`SYN:40-117`): object types are pascalised with special cases `File` to `StixFile`, `Ipv4-Addr`, `Ipv6-Addr`, `Ssh-Key` to `SSH-Key` (`SYN:49-61`); only properties that contribute to the standard id are kept (`SYN:66-81`). **[Q]** `ai-prompt`, `imei`, `iccid`, `imsi` pascalise to `Ai-Prompt`, `Imei`, `Iccid`, `Imsi`, which are not entity types, so they are silently dropped.

**[S]** Indicator from observable (`DOM:80-163`): pattern key `<Type>_<attr>`; hashed types use `sha256`, `sha512`, `sha1`, `md5` (joined by `__` when several), `StixFile` and `Artifact` keys are rewritten to `File`; `x_opencti_main_observable_type` set to the observable type.

## 12. Core relationships touching observables (summary)

**[S]** `related-to` and `revoked-by` are allowed between any entities (`REL:1324-1326`). Generic: Indicator `based-on` any observable (`REL:472-474`); Infrastructure `consists-of` any observable (`REL:516-518`). Observable-sourced mappings are at `REL:1030-1300` (e.g. Domain-Name `resolves-to` Domain/Hostname/IPv4/IPv6; IPv4/IPv6 `located-at` locations, `resolves-to` Mac, `belongs-to` AS/Organization/Individual; File `drops`/`downloads`/`interpreted-by`/`derived-from` File, `communicates-with` IPs/Domain, `targets`/`demonstrates` Vulnerability; Software `has`/`remediates` Vulnerability; Bank-Account `transferred-to` Bank-Account; mobile identifiers among IMEI/ICCID/IMSI/Phone/Mac). Note the overlap: `resolves-to`/`belongs-to` exist both as embedded refs (`obs_resolves-to`, `obs_belongs-to`, section 6) and as core relationships.

## 13. Type-detection heuristics and refang (input helpers)

**[S]** `detectObservableType` (`DET:41-313`) is used by Forms (`opencti-platform/opencti-graphql/src/modules/form/form-bundle-builder.ts:186-191`, `:328-333`), not by the core create path. Patterns sorted by priority (higher first), first match wins, fallback `Text`:

| Priority | Type | Patterns (abridged; full text at `DET` lines) |
|---|---|---|
| 10 | IPv4-Addr | dotted quad, optional `/\d{1,2}` (`DET:51-58`) |
| 10 | IPv6-Addr | long regex, CIDR variant (`DET:59-66`) |
| 10 | Mac-Addr | `^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$`, `^([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4}$` (`DET:67-74`) |
| 9 | Url | `^(https?\|ftp):\/\/[^\s/$.?#].[^\s]*$` (i) and a stricter one (`DET:75-82`) |
| 9 | Email-Addr | same as `emailChecker` (`DET:85-91`) |
| 9 | Artifact | 32/40/64/128 hex (`DET:140-149`) |
| 8 | Phone-Number | 3 patterns incl. E.164 (`DET:92-100`) |
| 8 | StixFile | path with extension, extension list (`DET:129-137`) |
| 8 | Cryptographic-Key | `-----BEGIN ... KEY-----`, `ssh-(rsa\|dss\|ed25519\|ecdsa) ...` (`DET:150-157`) |
| 8 | Cryptocurrency-Wallet | `^[13][a-km-zA-HJ-NP-Z1-9]{25,34}$`, `^0x[a-fA-F0-9]{40}$`, `^[LM3][a-km-zA-HJ-NP-Z1-9]{25,34}$`, `^bc1[a-z0-9]{39,59}$` (`DET:158-167`) |
| 8 | Windows-Registry-Key | `^(HKEY_\|HKLM\\\|HKCU\\\|HKU\\\|HKCR\\\|HKCC\\)` (i) ... (`DET:197-204`) |
| 8 | Autonomous-System | `^AS\d{1,10}$` (i), `^ASN?\s*\d{1,10}$` (i) (`DET:281-288`) |
| 8 | X509-Certificate | PEM certificate/CRL header, `^MII[A-Za-z0-9+/]+={0,2}$` (`DET:291-299`) |
| 7 | Domain-Name | 2 patterns (`DET:103-110`) |
| 7 | Directory | Windows/Unix paths (`DET:120-128`) |
| 7 | Credential | `user:password` (`DET:216-222`) |
| 7 | Payment-Card | `^[0-9]{13,19}$`, Visa/MC/Amex/Discover (`DET:225-235`) |
| 7 | Network-Traffic | protocol prefix, `port:`, `IP:port` (`DET:258-267`) |
| 6 | Hostname, Process, Bank-Account, Tracking-Number, User-Agent | (`DET:111-117`, `170-178`, `236-244`, `247-256`, `269-278`) |
| 5 | Mutex, Software | (`DET:179-194`) |
| 4 | User-Account | (`DET:207-215`) |

**[S]** `refangValue` (`DET:358-408`): `hxxp(s)://` to `http(s)://`, `[://]` to `://`, `[.]`/`(.)`/`{.}` to `.`, `[@]`/`(@)`/`{@}` to `@`, `[:]`/`(:)` to `:`, `[/]`/`(/)` to `/`, ` dot ` to `.`, ` at ` to `@`, whitespace around dots removed, trim. `isDefanged` (`DET:420-440`). A separate, broader `refang` exists for search and Defender (`opencti-platform/opencti-graphql/src/utils/refang.js`).

**[R]** Put refanging in the application layer (command handlers/parsers), never inside value-object constructors; value objects should reject defanged input.

## 14. Frontend extras (only for UX parity)

**[S]** `FE:208-230` `BULK_OBSERVABLES` (types allowing bulk multi-line creation and their keys): Credential, Cryptocurrency-Wallet, Cryptographic-Key, Domain-Name, Email-Addr, Hostname, ICCID, IMEI, IMSI, IPv4-Addr, IPv6-Addr, Mac-Addr, Phone-Number, Text, Tracking-Number, Url, User-Agent, AI-Prompt (`value`); StixFile (`name`, `hashes_MD5`, `hashes_SHA-1`, `hashes_SHA-256`, `hashes_SHA-512`); Artifact and X509-Certificate (hash keys). `FE:232-250` `SCO_DEFAULT_FIELD` (main field per complex type): Bank-Account `iban`, Autonomous-System `name`, Email-Message `body`, Email-Mime-Part-Type `body`, Mutex `name`, Network-Traffic `dst_port`, Payment-Card `card_number`, Persona `persona_name`, Process `command_line`, Software `name`, User-Account `account_login`, Windows-Registry-Key `attribute_key`, Windows-Registry-Value-Type `name`, StixFile `name`, Artifact/X509 `hashes_MD5`. Frontend hash regexes allow up to 50 newline-separated values (`FE:550-553`).

## 15. Consolidated list of OpenCTI inconsistencies relevant to Fennec

1. Registration vs GraphQL mandatory mismatch: Domain-Name `value` (reg `no`, GQL `!`), Email-Addr/IPv4/IPv6/Mac/Mutex/custom `value` (reg `external`, GQL optional), Process `command_line` (reg `no`, GQL `!`) (sections 7.3, 7.4, 7.10, 7.15).
2. Lower-casing only on GraphQL create for Domain-Name and Email-Addr; no canonicalisation for IPv6, MAC, URL, Hostname, hashes (section 8).
3. MAC values may violate the STIX pattern; Software CPE 2.3 rejected when alone; X509 hashes unvalidated (sections 7.12, 7.16, 5).
4. Standard-id deviations from STIX for Artifact, File, X509, Network-Traffic, Process, Windows-Registry-Key, Email-Message (section 9).
5. Integer overflows: AS number (u32 > i32), Network-Traffic byte/packet counts (32-bit), File size GraphQL `Int` (sections 7.1, 7.14, 7.8).
6. Embedded STIX types promoted to SCOs: Email-Mime-Part-Type, Windows-Registry-Value-Type (sections 7.6, 7.20).
7. Unsupported STIX properties/extensions: File extensions, Network-Traffic `ipfix` and extensions, User-Account `unix-account-ext`, Email-Message `additional_header_fields`, Process `startup_info` (registration commented out) (sections 7.5, 7.8, 7.14, 7.15, 7.18).
8. Type mismatches vs STIX: `environment_variables` list instead of dict, `belongs_to_ref` stored multiple, Process `priority` on `case_priority_ov`, SSH `key_length` string, Payment-Card `cvv` integer (sections 7.4, 7.15, 7.30, 7.33).
9. Export losses: SSH-Key (labels, description, score, author), Persona (description); `defanged` never populated (section 10).
10. `observable_value` bugs: Bank-Account uses non-existent `number`; Network-Traffic display vs ES runtime inconsistent (sections 7.27, 7.14).
11. pycti SSH-Key STIX class keyed on `value` while backend keys on `fingerprint_sha256` (section 7.33).
12. Pattern-to-observable extraction drops AI-Prompt, IMEI, ICCID, IMSI (section 11).

## 16. Recommendations for Fennec (all [R])

- One aggregate per concrete type (37 aggregates, or 35 if Email-Mime-Part-Type and Windows-Registry-Value-Type are modelled as value objects inside their parents, which is STIX-exact; if OpenCTI round-trip of those standalone SCOs is required, keep them as aggregates with an `embedded_in` back-reference).
- A shared `ObservableCommon` value (score 0..100 as `Score(u8)`, description, labels, markings, author, external references, organisation sharing, alternate stix ids) composed into every aggregate, not inherited.
- Every "value" observable gets its own validated, canonicalising newtype; constructors are total functions returning `Result<Self, DomainError>`; canonical form is what feeds the id.
- Embedded refs as typed id newtypes per allowed target set (e.g. `ResolvesToTarget = Domain(DomainNameId) | Ipv4(Ipv4AddrId) | Ipv6(Ipv6AddrId)`), with multiplicity matching STIX (single for `belongs_to_ref` on Email-Addr, `parent_directory_ref`, `content_ref`, `src_ref`, `dst_ref`, `image_ref`, `parent_ref`, `creator_user_ref`, `raw_email_ref`, `from_ref`, `sender_ref`, `encapsulated_by_ref`, payload refs).
- Deterministic ids per STIX spec (section 9), with optional OpenCTI-compat alias ids.
- Domain events per aggregate: `Created`, `ValueCorrected` (id-affecting change, carries old and new stix id), `AttributesUpdated`, `RefAdded/RefRemoved`, `Merged`, `ScoreChanged` (OpenCTI propagates score to based-on indicators, `DOM:322-333`), `Deleted`.
- PostgreSQL: one table per aggregate with typed columns (u32 as `bigint`, u64 as `numeric(20)` or `bigint` with check), a shared `observable` registry table (id, stix_id, type discriminator, alt ids) for cross-type lookups and ref integrity, hashes in a child table `(observable_id, algorithm, value)` with a unique index on `(observable_type, algorithm, value)` for non-fuzzy algorithms to reproduce OpenCTI's hash-based dedup.
