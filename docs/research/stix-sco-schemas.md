# STIX 2.1 Cyber-observable Objects (SCOs): schemas, constraints, vocabularies, deterministic ids

Research notes for the Fennec observable/IOC data model. Written for another agent: dense, cited, tables first.

## 0. Sources, provenance and legend

| Tag | Source | Version / location |
|---|---|---|
| [S] | OASIS `cti-stix2-json-schemas` clone at `.stix2-json-schemas` | commit `9af1db41b7b86c06324f899649ae83480134f66e` (2026-01-19). Paths below are relative to the clone root, `file:line` |
| [SPEC] | STIX 2.1 OASIS Standard, `https://docs.oasis-open.org/cti/stix/v2.1/os/stix-v2.1-os.html` | downloaded 2026-10-25 and converted to text locally; cited by section number (the HTML has no line numbers) |
| [PY] | `oasis-open/cti-python-stix2` master, files `stix2/base.py`, `stix2/v21/observables.py`, `stix2/v21/vocab.py`, `stix2/properties.py`, `stix2/hashes.py`, `stix2/test/v21/test_deterministic_ids.py` | fetched from raw.githubusercontent.com 2026-10-25 (NOT part of the reference clones); cited `file:line` of the fetched copy |
| [OCTI] | OpenCTI clone at `.opencti` | commit `294dffb07c5ee41086493827c94848b56f592a2c`; cited relative to that clone root |
| [VERIFIED] | Computation I ran (Python `uuid.uuid5` + sorted-key compact JSON) | scripts in scratchpad, see section 7.6 |
| [REC] | My own recommendation for Fennec, not a statement about any source | |

Important framing from the schema repo itself: the schemas are "non-normative"; "in case of conflict, the specification takes precedence", and they do NOT check (among others) that `*_ref`/`*_refs` point to the right object types, that `mime_type` values come from IANA, or that custom observables/extensions carry at least one custom property (`README.md`, "Some of the checks the schemas do not contain" list).

The 18 SCO schemas: `schemas/observables/{artifact, autonomous-system, directory, domain-name, email-addr, email-message, file, ipv4-addr, ipv6-addr, mac-addr, mutex, network-traffic, process, software, url, user-account, windows-registry-key, x509-certificate}.json`. All extensions and embedded types are defined inside these files under `definitions` (there are no separate extension schema files in this repo).

---

## 1. Common definitions referenced by SCO schemas (`schemas/common/*.json`)

### 1.1 Regex catalogue (decoded form, i.e. JSON escaping removed, exactly as the regex engine sees it)

| Id | Where | Regex / rule |
|---|---|---|
| R-TYPE | `common/cyber-observable-core.json:10` (+ `minLength 3` :11, `maxLength 250` :12, `not enum ["action"]` :14-18) | `^([a-z][a-z0-9]*)+(-[a-z0-9]+)*\-?$` |
| R-ID | `common/identifier.json:7` | `^[a-z][a-z0-9-]+[a-z0-9]--[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-5][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$` (each SCO adds `^<type>--`, e.g. `observables/file.json:22`) |
| R-TS | `common/timestamp.json:7` | `^[0-9]{4}-(0[1-9]\|1[012])-(0[1-9]\|[12][0-9]\|3[01])T([01][0-9]\|2[0-3]):([0-5][0-9]):([0-5][0-9]\|60)(\.[0-9]+)?Z$` |
| R-TS-SEC | `observables/file.json:287` (PE `time_date_stamp`, ANDed with R-TS) | `T\d{2}:\d{2}:\d{2}Z$` (second precision, no fraction) |
| R-BIN | `common/binary.json:7` | `^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{4}\|[A-Za-z0-9+/]{3}=\|[A-Za-z0-9+/]{2}==)$` (standard base64, padded; note: rejects empty string) |
| R-HEX | `common/hex.json:7` | `^([a-fA-F0-9]{2})+$` (even length; accepts uppercase although [SPEC] 2.8 says lowercase a-f) |
| R-DICT-KEY | `common/dictionary.json:9` (`minProperties 1` :7, `additionalProperties false` :33) | `^[a-zA-Z0-9_-]{0,250}$`; values: array(minItems 1) / string / integer / boolean / number / object (:10-30) |
| R-PROP-BIN | `common/properties.json:7` | `^[a-z][a-z0-9_]{0,245}_bin$` -> value must be R-BIN |
| R-PROP-HEX | `common/properties.json:10` | `^[a-z][a-z0-9_]{0,245}_hex$` -> value must be R-HEX |
| R-PROP | `common/properties.json:13` (`additionalProperties false` :37) | `^([a-z][a-z0-9_]{2,249})\|id$` (unanchored alternation: effectively "starts with 3 chars of `[a-z][a-z0-9_]{2}`" or "ends with `id`") |
| R-EXT-PREDEF | `common/cyber-observable-core.json:56` | `^([a-z][a-z0-9]*)+(-[a-z0-9]+)*\-ext$` (value: object, minProperties 1, custom-property rules) |
| R-EXT-DEF | `common/cyber-observable-core.json:61` | `^extension-definition--[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[1-5][0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}$` (value must satisfy `common/extension.json`) |
| R-ENC | `observables/file.json:43` (`name_enc`), `observables/directory.json:31` (`path_enc`) | `^[a-zA-Z0-9/\.+_:-]{2,250}$` |
| R-SELECTOR | `common/granular-marking.json:13` | `^([a-z0-9_-]{3,249}(\.(\[\d+\]\|[a-z0-9_-]{1,250}))*\|id)$` |

(Inside the table, `\|` is Markdown escaping of `|`; the source has a plain `|`.)

### 1.2 Common types

| Type | File | Definition |
|---|---|---|
| identifier | `common/identifier.json` | string, R-ID |
| timestamp | `common/timestamp.json` | string, R-TS. [SPEC] 2.16.1: RFC 3339 `YYYY-MM-DDTHH:mm:ss[.s+]Z`, UTC, `Z` mandatory, no decimal point without digits |
| binary | `common/binary.json` | string, R-BIN; property names MUST end in `_bin` ([SPEC] 2.1) |
| hex | `common/hex.json` | string, R-HEX; property names MUST end in `_hex`; [SPEC] 2.8: "even number of hexadecimal characters ... lower-case letters 'a' through 'f'" |
| dictionary | `common/dictionary.json` | object, minProperties 1, keys R-DICT-KEY. [SPEC] 2.3: keys ASCII `a-zA-Z0-9_-`, max 250, SHOULD be lowercase; empty dictionaries prohibited |
| list | (no file) | JSON array; [SPEC] 2.12: every list MUST have at least one value; empty lists prohibited |
| integer | (no file) | [SPEC] 2.10: signed 54-bit, `[-(2**53)+1, (2**53)-1]` unless otherwise restricted |
| float | (no file) | [SPEC] 2.6: IEEE 754 double; no NaN/Infinity |
| hashes | `common/hashes-type.json` | see 1.3 |
| extension (instance of an extension-definition extension) | `common/extension.json` | object, minProperties 1, `extension_type` REQUIRED (:13-15), enum `extension-type-enum` (:17-26) |
| extension-definition (SMO) | `common/extension-definition.json` | required `name, schema, version, extension_types` (:78-83) plus core required props; `extension_properties` only allowed when `extension_types` contains `toplevel-property-extension` (:58-76) |
| url | `common/url-regex.json:7` | string, `"format": "uri"` (annotation only under draft 2020-12 unless the validator enables format assertion) |
| granular-marking | `common/granular-marking.json` | `{selectors: [R-SELECTOR] minItems 1, marking_ref: identifier ^marking-definition--, lang?}`, required `selectors, marking_ref` (:33-36) |
| external-reference | `common/external-reference.json` | NOT applicable to SCOs ([SPEC] 3.2 table) |

### 1.3 Hashes type (`common/hashes-type.json`)

`type: object`, `allOf: dictionary` (:7), `additionalProperties: false` (:54). patternProperties (all matching patterns apply simultaneously in JSON Schema):

| Key pattern | Line | Value regex |
|---|---|---|
| `^[a-zA-Z0-9_-]{3,250}$` (custom key) | 9 | any string |
| `^MD5$` | 13-17 | `^[a-fA-F0-9]{32}$` |
| `^SHA-1$` | 18-22 | `^[a-fA-F0-9]{40}$` |
| `^SHA-256$` | 23-27 | `^[a-fA-F0-9]{64}$` |
| `^SHA-512$` | 28-32 | `^[a-fA-F0-9]{128}$` |
| `^SHA3-256$` | 33-37 | `^[a-fA-F0-9]{64}$` |
| `^SHA3-512$` | 38-42 | `^[a-fA-F0-9]{128}$` |
| `^SSDEEP$` | 43-47 | `^[a-zA-Z0-9/+:.]{1,128}$` |
| `^TLSH$` | 48-52 | `^[a-zA-Z0-9]{70}$` |

[SPEC] 2.7: keys MUST be unique, ASCII `a-zA-Z0-9-_`, length 3..250; name SHOULD come from `hash-algorithm-ov`; value MUST be in the format of the algorithm; "To enhance compatibility, the SHA-256 hash SHOULD be used whenever possible." Every SCO hashes property additionally says "Dictionary keys MUST come from the hash-algorithm-ov open vocabulary" (e.g. [SPEC] 6.7.1 `hashes`).

Observations [S]: lowercase keys such as `md5` or `sha256` match only the custom-key pattern, so their values are not validated. Values accept uppercase hex.

[PY] behaviour (`stix2/hashes.py`, `stix2/properties.py:428-486`): key normalisation `name.replace("-", "").upper()` mapped to the spec name (so `sha256`, `sha-256`, `SHA256` all become `SHA-256`); values checked case-insensitively (`re.I`, `hashes.py:54-56`) and NOT case-normalised; [PY] TLSH regex is `^[a-f0-9]{70}$` (`hashes.py:50`).

---

## 2. SCO common properties (`common/cyber-observable-core.json`)

| Property | Schema type | Req | Schema constraints (file:line) | [SPEC] rules (3.2) |
|---|---|---|---|---|
| `type` | string | R | R-TYPE, len 3..250, not `action` (:8-19); each SCO pins it with `enum [<type>]` | MUST be the SCO type name |
| `id` | identifier | R | R-ID (:47-50) + per-SCO prefix `^<type>--` | SCOs SHOULD use UUIDv5 (2.9), see section 7 |
| `spec_version` | string | O | `enum ["2.0","2.1"]` (:20-26) | MUST be `2.1`; if absent the default for SCOs is 2.1 |
| `object_marking_refs` | list of identifier | O | minItems 1 (:27-34) | refs MUST be marking-definition |
| `granular_markings` | list of granular-marking | O | minItems 1 (:35-42) | |
| `defanged` | boolean | O | (:43-46) | default `false`; MUST NOT be used on non-SCOs |
| `extensions` | dictionary | O | minProperties 1; keys R-EXT-PREDEF or R-EXT-DEF only; `additionalProperties false` (:51-66) | keys SHOULD be an extension-definition id or a predefined extension name; when the extension-definition facility is used, `extension_type` MUST be present; each predefined extension at most once per SCO (3.10) |

Required: `type`, `id` (:93-96). Not applicable to SCOs ([SPEC] 3.2 table and each "Not Applicable Common Properties" row): `created_by_ref, revoked, labels, confidence, lang, external_references` and also `created, modified` (SCOs are not versioned, [SPEC] 3.6).

Reserved names: schema forbids top-level `severity`, `action`, `username`, `phone_numbers` (:69-92). [SPEC] 3.8 reserves `severity, username, phone_number, action` (singular `phone_number`: schema/spec mismatch).

Top-level custom property names must match R-PROP / R-PROP-BIN / R-PROP-HEX (`allOf properties.json`, :68; `properties.json:37` `additionalProperties false`).

[SPEC] 3.1 naming: lowercase; property names `a-z0-9_`, 3..250 chars; `_ref` for a single embedded relationship, `_refs` for a list, `_bin` binary, `_hex` hex, `_enc` encoding sibling, and "The encoding property MUST NOT be present when the original property is not present".

[SPEC] 3.9.1 `_enc` values: name from the IANA character set registry; Preferred MIME Name if defined, else Name (directory/file text adds "2013-12-20 revision").

[SPEC] 3.7 common relationships for all SDOs and SCOs: `derived-from` (same type), `duplicate-of` (same type), `related-to` (any).

Extensions on SCOs: three ways [SPEC] 7.3: new object types (`new-sco`), nested `property-extension` (key = `extension-definition--<uuid>`, value carries `extension_type`), `toplevel-property-extension`. `extension-type-enum`: `new-sdo, new-sco, new-sro, property-extension, toplevel-property-extension` ([SPEC] 10.5; `common/extension.json:17-26`). Predefined SCO extensions (`*-ext`) "do not use the extension facility described in section 7.3" (each extension section in [SPEC] 6). Every SCO-specific extension dictionary in [S] also accepts unknown keys with any dictionary value (`additionalProperties: {$ref dictionary}`: `file.json:388-391`, `network-traffic.json:327-330`, `process.json:230-233`, `user-account.json:147-150`), still subject to R-EXT-PREDEF / R-EXT-DEF from the core.

---

## 3. The 18 SCOs: properties, constraints, id-contributing properties

Column legend: Req R = required, O = optional, C = conditionally required. "Schema" column gives the JSON-schema type and constraints with `file:line`; "Spec" column gives normative text from [SPEC] section 6.x that the schema does not encode. All SCOs also carry the common properties of section 2.

### 3.1 `artifact` (`observables/artifact.json`, [SPEC] 6.1)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `mime_type` | string | O | pattern `^(application\|audio\|font\|image\|message\|model\|multipart\|text\|video)/[a-zA-Z0-9.+_-]+` (:24-28; not end-anchored) | SHOULD be from IANA media type registry "Template" column |
| `payload_bin` | binary | C | R-BIN (:29-32) | MUST NOT be present if `url` is provided |
| `url` | string | C | `format uri` (:33-36) | valid URL resolving to the unencoded content; MUST NOT be present if `payload_bin` |
| `hashes` | hashes | C | hashes-type (:37-40) | MUST be present when `url` is present; keys from hash-algorithm-ov |
| `encryption_algorithm` | enum | O | `encryption-algorithm-enum` (:41-44, :116-123) | MUST come from encryption-algorithm-enum; mime_type + encryption_algorithm = encrypted archive |
| `decryption_key` | string | O | (:45-48) | MUST NOT be present when `encryption_algorithm` is absent |

Object constraints [S]: `oneOf [{required payload_bin, not url}, {required url+hashes, not payload_bin}]` (:51-75) so exactly one of `payload_bin`/`url` is required; `oneOf` encryption (:76-113): key requires algorithm. [SPEC]: "One of payload_bin or url MUST be provided."

**ID contributing [SPEC]:** `hashes, payload_bin`; "If the hashes property is present, include only one hash. The selected hash SHOULD come from this ordered list ... [ MD5, SHA-1, SHA-256, SHA-512 ]."

### 3.2 `autonomous-system` (`observables/autonomous-system.json`, [SPEC] 6.2)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `number` | integer | R | integer, no bounds (:24-27); required (:37-39) | number assigned to the AS (typically by an RIR) |
| `name` | string | O | (:28-31) | |
| `rir` | string | O | (:32-35) | name of the RIR that assigned the number |

**ID contributing:** `number`.

### 3.3 `directory` (`observables/directory.json`, [SPEC] 6.3)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `path` | string | R | (:25-28), required (:55-57) | path as originally observed |
| `path_enc` | string | O | R-ENC (:29-33) | MUST be specified if path stored in non-Unicode encoding; IANA charset name (2013-12-20 revision; Preferred MIME Name else Name) |
| `ctime` | timestamp | O | R-TS (:34-37) | created |
| `mtime` | timestamp | O | R-TS (:38-41) | last written/modified |
| `atime` | timestamp | O | R-TS (:42-45) | last accessed |
| `contains_refs` | list of identifier | O | array of string, minItems 1 (:46-53) | targets MUST be `file` or `directory` |

**ID contributing:** `path`.

### 3.4 `domain-name` (`observables/domain-name.json`, [SPEC] 6.4)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `value` | string | R | `format idn-hostname` (:24-28), required (:38-40) | MUST conform to RFC 1034; each domain and sub-domain MUST conform to RFC 5890 |
| `resolves_to_refs` | list of identifier | O | array of string minItems 1 (:29-36) | targets MUST be `ipv4-addr`, `ipv6-addr` or `domain-name` (CNAME case) |

SCO relationship [SPEC] 6.4.2: `domain-name resolves-to {domain-name, ipv4-addr, ipv6-addr}`.

**ID contributing:** `value`.

### 3.5 `email-addr` (`observables/email-addr.json`, [SPEC] 6.5)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `value` | string | R | `format email` (:24-28), required (:38-40) | RFC 5322 3.4 `addr-spec`; MUST NOT include the display name |
| `display_name` | string | O | (:29-32) | RFC 5322 3.4 `display-name` |
| `belongs_to_ref` | identifier | O | string (:33-36) | target MUST be `user-account` |

**ID contributing:** `value`.

### 3.6 `email-message` (`observables/email-message.json`, [SPEC] 6.6)

[SPEC] 6.6: RFC 2047 encoded header values MUST be decoded before inclusion; undecodable chars SHOULD become U+FFFD; capture raw via `raw_email_ref`.

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `is_multipart` | boolean | R | only inside the top-level `oneOf` branches (:90-140); no branch without it, so effectively required | required |
| `date` | timestamp | O | R-TS (:24-27) | when sent |
| `content_type` | string | O | (:28-31) | `Content-Type` header |
| `from_ref` | identifier | O | string (:32-35) | `From:`; target MUST be email-addr (spec text says "email-address", a typo) |
| `sender_ref` | identifier | O | string (:36-39) | `Sender:`; target email-addr |
| `to_refs` | list of identifier | O | minItems 1 (:40-47) | targets email-addr |
| `cc_refs` | list of identifier | O | minItems 1 (:48-55) | targets email-addr |
| `bcc_refs` | list of identifier | O | minItems 1 (:56-63) | targets email-addr; absence is not equivalent to absent BCC header |
| `message_id` | string | O | (:64-67) | Message-ID |
| `subject` | string | O | (:68-71) | |
| `received_lines` | list of string | O | array of string, no minItems (:72-78) | same order as in the message |
| `additional_header_fields` | dictionary | O | `email-additional-header-fields` (:79-82, :176-203): values `oneOf [array of string minItems 2, string]` (:188-201); forbidden keys regex `^date\|received_lines\|content_type\|from_ref\|sender_ref\|to_refs\|cc_refs\|bcc_refs\|subject$` (:181, unanchored alternation) | excludes the headers above; keys SHOULD be case-preserved header names; "value for each dictionary key MUST always be a list of type string" (spec example nevertheless uses plain strings) |
| `body` | string | O | in branch `is_multipart: false` (:100-103); branch forbids `body_multipart` (:108-112) | MUST NOT be used if is_multipart is true |
| `body_multipart` | list of email-mime-part-type | O | in branch `is_multipart: true` (:123-129), no minItems; branch forbids `body` (:134-138) | MUST NOT be used if is_multipart is false |
| `raw_email_ref` | identifier | O | string (:83-86) | target MUST be `artifact` |

Embedded type `email-mime-part-type` (`definitions/mime-part-type` :142-175, [SPEC] 6.6.2):

| Property | Type | Req | Schema | Spec |
|---|---|---|---|---|
| `body` | string | C | (:146-149) | content if content_type absent or starts with `text/`; MUST be decoded to Unicode |
| `body_raw_ref` | identifier | C | string (:150-153) | non-text parts; target `artifact` or `file` (artifact SHOULD when data matters, file when metadata matters) |
| `content_type` | string | O | (:154-157) | include parameters such as charset |
| `content_disposition` | string | O | (:158-161) | |

`oneOf [{required body}, {required body_raw_ref}]` (:163-174): exactly one. [SPEC]: "One of body OR body_raw_ref MUST be included." No property for Content-Transfer-Encoding (body MUST be decoded).

**ID contributing:** `from_ref, subject, body` (all optional, so UUIDv4 if none present). Note `body_multipart` does not contribute.

### 3.7 `file` (`observables/file.json`, [SPEC] 6.7)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `extensions` | dictionary | O | `file-extensions-dictionary` (:24-27, :98-392) | predefined: `ntfs-ext, raster-image-ext, pdf-ext, archive-ext, windows-pebinary-ext` |
| `hashes` | hashes | C | hashes-type (:28-31) | with archive-ext: hash of the whole archive; keys from hash-algorithm-ov |
| `size` | integer | O | minimum 0 (:32-36) | MUST NOT be negative |
| `name` | string | C | (:37-40) | |
| `name_enc` | string | O | R-ENC (:41-45) | IANA charset (2013-12-20 revision; Preferred MIME Name else Name) |
| `magic_number_hex` | hex | O | R-HEX (:46-49) | |
| `mime_type` | string | O | no pattern (:50-53) | SHOULD be from IANA media type registry |
| `ctime` / `mtime` / `atime` | timestamp | O | R-TS (:54-65) | |
| `parent_directory_ref` | identifier | O | string (:66-69) | target MUST be `directory` |
| `contains_refs` | list of identifier | O | minItems 1 (:70-77) | any SCO embedded in the file (not the archive use case) |
| `content_ref` | identifier | O | string (:78-81) | target MUST be `artifact` |

Object constraint: `anyOf [required hashes, required name]` (:85-96); [SPEC]: "A File object MUST contain at least one of hashes or name."

**ID contributing [SPEC]:** `hashes, name, extensions, parent_directory_ref`, with the one-hash rule [MD5, SHA-1, SHA-256, SHA-512].

#### 3.7.1 `archive-ext` (`file.json:237-257`, [SPEC] 6.7.2)

| Property | Type | Req | Schema | Spec |
|---|---|---|---|---|
| `contains_refs` | list of identifier | R | minItems 1, required (:241-248, :254-256) | targets `file` or `directory` ("MUST contain references to one or more File objects") |
| `comment` | string | O | (:249-252) | |

#### 3.7.2 `ntfs-ext` (`file.json:101-144`, [SPEC] 6.7.3); at least one property (`anyOf` :137-142)

| Property | Type | Req | Schema |
|---|---|---|---|
| `sid` | string | O | (:107-110) |
| `alternate_data_streams` | list of alternate-data-stream-type | O | array, no minItems (:111-134) |

`alternate-data-stream-type` ([SPEC] 6.7.3.2): `name` string R (:116-119, required :130-132); `hashes` hashes O (:120-123); `size` integer O, minimum 0 (:124-128).

#### 3.7.3 `pdf-ext` (`file.json:193-236`, [SPEC] 6.7.4); at least one property (`anyOf` :226-234)

| Property | Type | Schema | Spec |
|---|---|---|---|
| `version` | string | (:199-202) | decimal version from the PDF header, e.g. `1.4` |
| `is_optimized` | boolean | (:203-206) | |
| `document_info_dict` | dictionary | keys `^[a-zA-Z0-9_-]{0,250}$`, values string (:207-215) | keys SHOULD be case-preserved DID entry names without leading `/` (e.g. `Title`); values MUST be strings |
| `pdfid0` | string | (:216-219) | first file identifier |
| `pdfid1` | string | (:220-223) | second file identifier |

#### 3.7.4 `raster-image-ext` (`file.json:145-192`, [SPEC] 6.7.5); at least one property (`anyOf` :182-190, which also lists the 2.0 property `image_compression_algorithm` at :187)

| Property | Type | Schema | Spec |
|---|---|---|---|
| `image_height` | integer | (:151-154) | pixels |
| `image_width` | integer | (:155-158) | pixels |
| `bits_per_pixel` | integer | (:159-162) | |
| `exif_tags` | dictionary | keys `^[A-Z][a-zA-Z0-9_-]+$`, values `oneOf [string, integer]`, `additionalProperties false` (:163-179) | keys MUST be case-preserved EXIF tag names (e.g. `XResolution`); value integer for int* EXIF datatypes, string otherwise |

#### 3.7.5 `windows-pebinary-ext` (`file.json:258-386`, [SPEC] 6.7.6)

`pe_type` required (:383-385) plus `anyOf` at least one other property (:326-382). [SPEC]: "MUST contain at least one property other than the required pe_type property".

| Property | Spec type | Req | Schema | Spec |
|---|---|---|---|---|
| `pe_type` | open-vocab | R | string (:262-265); the `windows-pebinary-type-ov` definition (:560-567) is NOT referenced | SHOULD come from windows-pebinary-type-ov |
| `imphash` | string | O | (:266-269) | Mandiant/FireEye imphash |
| `machine_hex` | hex | O | (:270-273) | target machine |
| `number_of_sections` | integer | O | minimum 0 (:274-278) | |
| `time_date_stamp` | timestamp | O | R-TS AND R-TS-SEC (:279-290) | MUST be precise to the second |
| `pointer_to_symbol_table_hex` | hex | O | (:291-294) | |
| `number_of_symbols` | integer | O | minimum 0 (:295-299) | |
| `size_of_optional_header` | integer | O | minimum 0 (:300-304) | |
| `characteristics_hex` | hex | O | (:305-308) | |
| `file_header_hashes` | hashes | O | (:309-312) | |
| `optional_header` | windows-pe-optional-header-type | O | (:313-316) | at least one property of the type |
| `sections` | list of windows-pe-section-type | O | minItems 1 (:317-324) | |

`windows-pe-optional-header-type` (`file.json:393-533`, `minProperties 1`, `additionalProperties false` :395-396, [SPEC] 6.7.6.2):

| Property | Type | Schema constraint |
|---|---|---|
| `magic_hex` | hex | (:399-402) |
| `major_linker_version`, `minor_linker_version` | integer | (:403-410) |
| `size_of_code`, `size_of_initialized_data`, `size_of_uninitialized_data` | integer | minimum 0 (:411-425) |
| `address_of_entry_point`, `base_of_code`, `base_of_data`, `image_base`, `section_alignment`, `file_alignment` | integer | (:426-449) |
| `major_os_version`, `minor_os_version`, `major_image_version`, `minor_image_version`, `major_subsystem_version`, `minor_subsystem_version` | integer | (:450-473) |
| `win32_version_value_hex` | hex | (:474-477) |
| `size_of_image`, `size_of_headers` | integer | minimum 0 (:478-487) |
| `checksum_hex`, `subsystem_hex`, `dll_characteristics_hex` | hex | (:488-499) |
| `size_of_stack_reserve`, `size_of_stack_commit`, `size_of_heap_reserve`, `size_of_heap_commit` | integer | minimum 0 (:500-519) |
| `loader_flags_hex` | hex | (:520-523) |
| `number_of_rva_and_sizes` | integer | (:524-527) |
| `hashes` | hashes | (:528-531) |

`windows-pe-section-type` (`file.json:534-559`, [SPEC] 6.7.6.3): `name` string R (:538-541, :556-558); `size` integer minimum 0 (:542-546); `entropy` number (:547-550; [SPEC] float, Shannon entropy over bytes, range 0..8); `hashes` hashes (:551-554).

### 3.8 `ipv4-addr` (`observables/ipv4-addr.json`, [SPEC] 6.8)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `value` | string | R | regex below (:24-28), required (:46-48) | one or more IPv4 addresses in CIDR notation; `/32` MAY be omitted for a single address |
| `resolves_to_refs` | list of identifier | O | minItems 1 (:29-36) | targets MUST be `mac-addr` |
| `belongs_to_refs` | list of identifier | O | minItems 1 (:37-44) | targets MUST be `autonomous-system` |

```
^(([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])(\/(3[0-2]|[1-2][0-9]|[0-9]))?$
```
[VERIFIED] accepts `198.51.100.3`, `198.51.100.0/24`, `1.1.1.1/0`; rejects leading zeros `010.1.1.1`, `/33`, `256.1.1.1`. It does not require the host bits to be zero for a CIDR block.

Relationships [SPEC] 6.8.2: `ipv4-addr resolves-to mac-addr`, `ipv4-addr belongs-to autonomous-system`.

**ID contributing:** `value`.

### 3.9 `ipv6-addr` (`observables/ipv6-addr.json`, [SPEC] 6.9)

Same shape as ipv4-addr (`value` R :24-28, `resolves_to_refs` -> mac-addr :29-36, `belongs_to_refs` -> autonomous-system :37-44; required :46-48). Spec: CIDR notation, `/128` MAY be omitted. Relationships [SPEC] 6.9.2 identical to IPv4.

Schema regex (`ipv6-addr.json:27`, decoded form, verbatim):
```
^s*((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]d|1dd|[1-9]?d)(.(25[0-5]|2[0-4]d|1dd|[1-9]?d)){3}))|:)))(%.+)?s*(\/(12[0-8]|1[0-1][0-9]|[1-9][0-9]|[0-9]))?$
```
This regex is corrupted (backslashes lost when copied): `s*` instead of `\s*`, `d` instead of `\d`, `.` instead of `\.`. [VERIFIED] with Python `re`: accepts the spec examples, `::1`, `::`, `fe80::1%eth0`, `2001:DB8::1`, but ALSO `sss::1` (leading `s` consumed by `^s*`); REJECTS IPv4-embedded forms such as `::ffff:192.0.2.1`; rejects `/129`.

**ID contributing:** `value`.

### 3.10 `mac-addr` (`observables/mac-addr.json`, [SPEC] 6.10)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `value` | string | R | `^([0-9a-f]{2}[:]){5}([0-9a-f]{2})$` (:24-28), required (:30-32). (Schema description at :27 wrongly says "CIDR notation") | single colon-delimited, lowercase MAC-48, with leading zeros, e.g. `00:00:ab:cd:ef:01` |

**ID contributing:** `value`.

### 3.11 `mutex` (`observables/mutex.json`, [SPEC] 6.11)

| Property | Spec type | Req | Schema |
|---|---|---|---|
| `name` | string | R | (:24-27); `required` is at top level outside `allOf` (:31-33) |

**ID contributing:** `name`.

### 3.12 `network-traffic` (`observables/network-traffic.json`, [SPEC] 6.12)

[SPEC] 6.12: "MUST contain the protocols property and at least one of the src_ref or dst_ref properties and SHOULD contain the src_port and dst_port properties". Traffic may be non-established (e.g. SYN flood).

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `extensions` | dictionary | O | `network-traffic-extensions-dictionary` (:24-27, :177-331) | predefined `http-request-ext, tcp-ext, icmp-ext, socket-ext` |
| `start` | timestamp | O | R-TS (:28-31) | |
| `end` | timestamp | O | R-TS (:32-35) | MUST NOT be present if `is_active` true; MUST be >= `start` (not enforced by schema) |
| `is_active` | boolean | O | only via top-level `oneOf` (:134-175): `false`, or `true` with no `end`, or absent | if `end` is present, MUST be false |
| `src_ref` | identifier | C | string (:36-39) | target `ipv4-addr`, `ipv6-addr`, `mac-addr` or `domain-name` |
| `dst_ref` | identifier | C | string (:40-43) | same targets |
| `src_port` | integer | O | 0..65535 (:44-49) | |
| `dst_port` | integer | O | 0..65535 (:50-55) | |
| `protocols` | list of string | R | array of string minItems 1 (:56-63), required (:119-121) | low to high, outer to inner encapsulation (e.g. `ipv4, tcp, http`); SHOULD use IANA Service Name registry names, lowercase recommended; the protocol of a present extension SHOULD be listed |
| `src_byte_count` / `dst_byte_count` | integer | O | integer, no minimum (:64-71) | "as a positive integer" |
| `src_packets` / `dst_packets` | integer | O | integer, no minimum (:72-79) | "as a positive integer" |
| `ipfix` | dictionary | O | keys `^[a-zA-Z0-9_-]{0,250}$`, values `anyOf [string, integer]` (:80-95) | keys SHOULD be case-preserved IPFIX element names (e.g. `octetDeltaCount`) |
| `src_payload_ref` | identifier | O | string (:96-99) | target `artifact` |
| `dst_payload_ref` | identifier | O | string (:100-103) | target `artifact` |
| `encapsulates_refs` | list of identifier | O | minItems 1 (:104-111) | targets `network-traffic` |
| `encapsulated_by_ref` | identifier | O | string (:112-115) | target `network-traffic` |

Object constraints [S]: `required [protocols]` (:119-121), `anyOf [src_ref, dst_ref]` (:122-133), `oneOf is_active` (:134-175). [PY] additionally enforces `end >= start` (`observables.py:580-582`).

**ID contributing:** `start, end, src_ref, dst_ref, src_port, dst_port, protocols, extensions`.

#### 3.12.1 `http-request-ext` (`network-traffic.json:180-222`, [SPEC] 6.12.2; protocol value `http`)

| Property | Type | Req | Schema | Spec |
|---|---|---|---|---|
| `request_method` | string | R | (:184-187, required :218-221) | lowercase string |
| `request_value` | string | R | (:188-191) | typically a resource path |
| `request_version` | string | O | (:192-195) | lowercase string |
| `request_header` | dictionary | O | keys `^.+$`, values array of string, `additionalProperties false` (:196-208) | keys MUST be header names (SHOULD preserve case); values MUST always be list of string (spec example uses plain strings) |
| `message_body_length` | integer | O | (:209-212) | bytes |
| `message_body_data_ref` | identifier | O | string (:213-216) | target `artifact` |

#### 3.12.2 `icmp-ext` (`network-traffic.json:223-240`, [SPEC] 6.12.3; protocol `icmp`)

`icmp_type_hex` hex R (:227-230), `icmp_code_hex` hex R (:231-234); required (:236-239).

#### 3.12.3 `socket-ext` (`network-traffic.json:241-301`, [SPEC] 6.12.4)

| Property | Type | Req | Schema | Spec |
|---|---|---|---|---|
| `address_family` | enum | R | enum (:245-258), required (:298-300) | network-socket-address-family-enum |
| `is_blocking` | boolean | O | (:259-262) | |
| `is_listening` | boolean | O | (:263-266) | |
| `options` | dictionary | O | keys `^(SO\|ICMP\|ICMP6\|IP\|IPV6\|MCAST\|TCP\|IRLMP)(_[A-Z]+)+$`, values integer, `additionalProperties false` (:267-276) | keys SHOULD be case-preserved option names (e.g. `SO_ACCEPTCONN`); values MUST be integers; `SO_RCVTIMEO`, `SO_SNDTIMEO`, `SO_LINGER` in milliseconds; presence of `SO_LINGER` means active |
| `socket_type` | enum | O | enum (:277-287) | network-socket-type-enum |
| `socket_descriptor` | integer | O | minimum 0 (:288-292) | non-negative |
| `socket_handle` | integer | O | (:293-296) | handle or inode |

[VERIFIED] the `options` key regex rejects names with digits after the first underscore, e.g. `IPV6_V6ONLY`. [PY] only checks the prefix (`observables.py:505-516`).

#### 3.12.4 `tcp-ext` (`network-traffic.json:302-325`, [SPEC] 6.12.5; protocol `tcp`)

`src_flags_hex` hex O (:308-311), `dst_flags_hex` hex O (:312-315); `anyOf` at least one (:318-323). Spec: union of all TCP flags observed between `start` and `end` (whole traffic if unspecified).

### 3.13 `process` (`observables/process.json`, [SPEC] 6.13)

[SPEC]: "A Process object MUST contain at least one property (other than type) from this object (or one of its extensions)."

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `extensions` | dictionary | O | `process-extensions-dictionary` (:24-27, :100-234) | `windows-process-ext`, `windows-service-ext` |
| `is_hidden` | boolean | O | (:28-31) | |
| `pid` | integer | O | (:32-35) | |
| `created_time` | timestamp | O | R-TS (:36-39) | |
| `cwd` | string | O | (:40-43) | |
| `command_line` | string | O | (:44-47) | full command line incl. process name (schema description still says `binary_ref.name`, the STIX 2.0 name; spec says `image_ref.name`) |
| `environment_variables` | dictionary | O | dictionary (:48-51) | keys case-preserved variable names; values MUST be strings |
| `opened_connection_refs` | list of identifier | O | minItems 1 (:52-59) | targets `network-traffic` |
| `creator_user_ref` | identifier | O | string (:60-63) | target `user-account` |
| `image_ref` | identifier | O | string (:64-67) | target `file` |
| `parent_ref` | identifier | O | string (:68-71) | target `process` |
| `child_refs` | list of identifier | O | minItems 1 (:72-79) | targets `process` |

Object constraint [S]: `anyOf` (:83-98) lists `extensions, is_hidden, pid, name, created, cwd, arguments, command_line, environment_variables, opened_connection_refs, creator_user_ref, image_ref, parent_ref, child_refs`. `name`, `created`, `arguments` are STIX 2.0 property names and `created_time` is missing, so a 2.1 process carrying only `created_time` fails this schema (schema bug).

**ID contributing [SPEC]:** none: "Since all properties on this object are optional, please use a UUIDv4 for the ID of this object."

#### 3.13.1 `windows-process-ext` (`process.json:103-150`, [SPEC] 6.13.2); at least one property

| Property | Type | Schema | Spec |
|---|---|---|---|
| `aslr_enabled` | boolean | (:109-112) | |
| `dep_enabled` | boolean | (:113-116) | |
| `priority` | string | (:117-120) | SHOULD end in `_CLASS` (e.g. `HIGH_PRIORITY_CLASS`) |
| `owner_sid` | string | (:121-124) | |
| `window_title` | string | (:125-128) | |
| `startup_info` | dictionary | `startup-info-dictionary` (:129-132, :235-254) | STARTUP_INFO struct, keys case-preserved original names (e.g. `lpDesktop`) |
| `integrity_level` | enum | `windows-integrity-level-enum` (:133-136, :255-263) | |

`anyOf` (:139-148) omits `integrity_level` (an extension with only `integrity_level` fails the schema; schema bug). `startup-info-dictionary` patternProperties (all unanchored alternations): `^lpDesktop|lpTitle|dwFillAttribute|dwFlags|wShowWindow|hStdInput|hStdOutput|hStdError$` -> string (:238-240); `^lpReserved|lpReserved2$` -> `null` (:241-243); `^cb|dwX|dwY|dwXSize|dwYSize|dwXCountChars|dwYCountChars$` -> integer (:244-246); `^cbReserved2$` -> integer 0..0 (:247-251); `additionalProperties false` (:253).

#### 3.13.2 `windows-service-ext` (`process.json:151-228`, [SPEC] 6.13.3); at least one property (`anyOf` :218-227)

| Property | Type | Schema | Spec |
|---|---|---|---|
| `service_name` | string | (:155-158) | |
| `descriptions` | list of string | minItems 1 (:159-166) | |
| `display_name` | string | (:167-170) | |
| `group_name` | string | (:171-174) | load ordering group |
| `start_type` | enum | enum (:175-185) | windows-service-start-type-enum |
| `service_dll_refs` | list of identifier | minItems 1 (:186-193) | targets `file` |
| `service_type` | enum | enum (:194-203) | windows-service-type-enum |
| `service_status` | enum | enum (:204-216) | windows-service-status-enum |

### 3.14 `software` (`observables/software.json`, [SPEC] 6.14)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `name` | string | R | (:24-27), required (top level :57-59) | |
| `cpe` | string | O | CPE 2.3 regex below (:28-32), NOT anchored | MUST be a CPE v2.3 entry from the NVD CPE Dictionary; SHOULD be present when the dictionary has one |
| `swid` | string | O | (:33-36) | SWID tag; `tagId` SHOULD be used as proxy identifier |
| `languages` | list of string | O | items regex below, minItems 1 (:37-45) | each MUST be RFC 5646; (schema description: ISO 639-2 accepted for backward compatibility) |
| `vendor` | string | O | (:46-49) | |
| `version` | string | O | (:50-53) | |

CPE regex (`software.json:30`, decoded):
```
cpe:2\.3:[aho\*\-](:(((\?*|\*?)([a-zA-Z0-9\-\._]|(\\[\\\*\?!"#$$%&'\(\)\+,/:;<=>@\[\]\^`\{\|}~]))+(\?*|\*?))|[\*\-])){5}(:(([a-zA-Z]{2,3}(-([a-zA-Z]{2}|[0-9]{3}))?)|[\*\-]))(:(((\?*|\*?)([a-zA-Z0-9\-\._]|(\\[\\\*\?!"#$$%&'\(\)\+,/:;<=>@\[\]\^`\{\|}~]))+(\?*|\*?))|[\*\-])){4}
```
(part, then 5 components vendor/product/version/update/edition, then language, then 4 components sw_edition/target_sw/target_hw/other.) [VERIFIED] unanchored: `"xx cpe:2.3:a:... trailing"` passes; CPE 2.2 URI `cpe:/a:...` fails.

Languages regex (`software.json:42`, decoded):
```
^([a-z]{2}(-[A-Z]{2})?(-[a-z]{4})?(-[A-Z]{2}|[0-9]{3})?(-[a-z0-9]{5,8}|[0-9][a-z0-9]{3})*(-[a-z0-9]{1,8})*|[a-z]{3})$
```
[VERIFIED] accepts `en`, `en-US`, `eng`, `es-419`, `fr-CA`; rejects `zh-Hant` (RFC 5646 canonical script casing is title case).

**ID contributing:** `name, cpe, swid, vendor, version` (note: `languages` excluded).

### 3.15 `url` (`observables/url.json`, [SPEC] 6.15)

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `value` | string | R | `format uri` via url-regex (:24-27), required (top level :31-33) | MUST conform to RFC 3986, section 1.1.3 "Uniform Resource Locator" |

**ID contributing:** `value`.

### 3.16 `user-account` (`observables/user-account.json`, [SPEC] 6.16)

[SPEC]: all optional, at least one MUST be included. [S] `anyOf` over all 15 properties incl. `extensions` (:87-103).

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `extensions` | dictionary | O | (:24-27, :105-151) | `unix-account-ext` |
| `user_id` | string | O | (:28-31) | unique identifier in the account's system (UID on Unix, GUID, name, email, ...) |
| `credential` | string | O | (:32-35) | cleartext credential, only for malware-analysis metadata; SHOULD NOT be used for sharing PII |
| `account_login` | string | O | (:36-39) | what a user types to log in (e.g. `root` for user_id 0) |
| `account_type` | open-vocab | O | plain string (:40-43); `account-type-ov` definition (:152-168) NOT referenced | SHOULD come from account-type-ov |
| `display_name` | string | O | (:44-47) | GECOS on Unix |
| `is_service_account` | boolean | O | (:48-51) | |
| `is_privileged` | boolean | O | (:52-55) | |
| `can_escalate_privs` | boolean | O | (:56-59) | |
| `is_disabled` | boolean | O | (:60-63) | |
| `account_created` / `account_expires` / `credential_last_changed` / `account_first_login` / `account_last_login` | timestamp | O | R-TS (:64-83) | |

**ID contributing:** `account_type, user_id, account_login`.

#### 3.16.1 `unix-account-ext` (`user-account.json:108-145`, [SPEC] 6.16.2); at least one property (`anyOf` :136-143)

| Property | Spec type | Schema |
|---|---|---|
| `gid` | integer | `"type": "number"` (:114-117) (mismatch: spec integer) |
| `groups` | list of string | minItems 1 (:118-125) |
| `home_dir` | string | (:126-129) |
| `shell` | string | (:130-133) |

### 3.17 `windows-registry-key` (`observables/windows-registry-key.json`, [SPEC] 6.17)

[SPEC]: all optional, at least one MUST be included.

| Property | Spec type | Req | Schema | Spec rules |
|---|---|---|---|---|
| `key` | string | O | `not pattern ^HKLM\|HKCC\|HKCR\|HKCU\|HKU\|hklm\|hkcc\|hkcr\|hkcu\|hku` (:24-28) | full key incl. hive; SHOULD be case-preserved; hive MUST be fully expanded (`HKEY_LOCAL_MACHINE` not `HKLM`) |
| `values` | list of windows-registry-value-type | O | array, no minItems (:29-35) | |
| `modified_time` | timestamp | O | R-TS (:36-39) | |
| `creator_user_ref` | identifier | O | string (:40-43) | target `user-account` |
| `number_of_subkeys` | integer | O | (:44-47) | |

Object constraint [S] `anyOf` (:51-57) uses `modified` (line 54) instead of `modified_time` (schema bug). [VERIFIED] the `not` regex is an unanchored alternation: it forbids `HKLM\...` but also any key containing `HKCU`, `HKU`, `hku`, ... anywhere (e.g. `HKEY_CURRENT_USER\hkuser` is rejected).

`windows-registry-value-type` (`windows-registry-key.json:59-96`, [SPEC] 6.17.2; at least one property, `anyOf` :91-95):

| Property | Type | Schema | Spec |
|---|---|---|---|
| `name` | string | (:62-65) | empty string MUST be used for the default value |
| `data` | string | (:66-69) | |
| `data_type` | enum | enum (:70-89) | windows-registry-datatype-enum |

**ID contributing:** `key, values` "(all items defined in the values property MUST be included)".

### 3.18 `x509-certificate` (`observables/x509-certificate.json`, [SPEC] 6.18)

[SPEC]: "MUST contain at least one object specific property (other than type)". [S] `anyOf` over the 13 properties (:79-93).

| Property | Spec type | Schema (line) | Spec |
|---|---|---|---|
| `is_self_signed` | boolean | :24-27 | |
| `hashes` | hashes | :28-31 | hashes of the entire certificate; keys from hash-algorithm-ov |
| `version` | string | :32-35 | |
| `serial_number` | string | :36-39 | unique id as issued by the CA (spec examples use colon-separated hex `36:f7:...`) |
| `signature_algorithm` | string | :40-43 | |
| `issuer` | string | :44-47 | |
| `validity_not_before` / `validity_not_after` | timestamp | :48-55 | |
| `subject` | string | :56-59 | |
| `subject_public_key_algorithm` | string | :60-63 | |
| `subject_public_key_modulus` | string | :64-67 | RSA modulus |
| `subject_public_key_exponent` | integer | :68-71 | RSA exponent |
| `x509_v3_extensions` | x509-v3-extensions-type | :72-75 | |

`x509-v3-extensions-type` (`x509-certificate.json:97-190`, [SPEC] 6.18.2; at least one property, `anyOf` :169-186; this is the X.509 notion of "extension", not a STIX extension):

| Property | Type | OID ([SPEC]) | Schema line |
|---|---|---|---|
| `basic_constraints` | string | 2.5.29.19 (`CA:TRUE/FALSE`, optional `pathlen`) | 102 |
| `name_constraints` | string | 2.5.29.30 | 106 |
| `policy_constraints` | string | 2.5.29.36 | 110 |
| `key_usage` | string | 2.5.29.15 | 114 |
| `extended_key_usage` | string | 2.5.29.37 | 118 |
| `subject_key_identifier` | string | 2.5.29.14 | 122 |
| `authority_key_identifier` | string | 2.5.29.35 | 126 |
| `subject_alternative_name` | string | 2.5.29.17 | 130 |
| `issuer_alternative_name` | string | 2.5.29.18 | 134 |
| `subject_directory_attributes` | string | 2.5.29.9 | 138 |
| `crl_distribution_points` | string | 2.5.29.31 | 142 |
| `inhibit_any_policy` | string | 2.5.29.54 | 146 |
| `private_key_usage_period_not_before` | timestamp | (none) | 150 |
| `private_key_usage_period_not_after` | timestamp | (none) | 154 |
| `certificate_policies` | string | 2.5.29.32 | 158 |
| `policy_mappings` | string | 2.5.29.33 | 162 |

**ID contributing:** `hashes, serial_number` with the one-hash rule [MD5, SHA-1, SHA-256, SHA-512].

---

## 4. Summary table: required properties, object-level rules, ID contributing properties

| SCO | Required (besides type, id) | Object-level rule | ID contributing ([SPEC]) | All contributing optional? |
|---|---|---|---|---|
| artifact | one of `payload_bin` / `url` | `url` => `hashes`; `decryption_key` => `encryption_algorithm` | `hashes` (one), `payload_bin` | no (one is always present) |
| autonomous-system | `number` | | `number` | no |
| directory | `path` | | `path` | no |
| domain-name | `value` | | `value` | no |
| email-addr | `value` | | `value` | no |
| email-message | `is_multipart` | `body` only if false; `body_multipart` only if true | `from_ref`, `subject`, `body` | yes (UUIDv4 if none) |
| file | one of `hashes` / `name` | | `hashes` (one), `name`, `extensions`, `parent_directory_ref` | no |
| ipv4-addr | `value` | | `value` | no |
| ipv6-addr | `value` | | `value` | no |
| mac-addr | `value` | | `value` | no |
| mutex | `name` | | `name` | no |
| network-traffic | `protocols`, one of `src_ref` / `dst_ref` | `is_active=true` => no `end`; `end` => `is_active` false; `end >= start` | `start`, `end`, `src_ref`, `dst_ref`, `src_port`, `dst_port`, `protocols`, `extensions` | no (`protocols` required) |
| process | at least one property | | none: always UUIDv4 | n/a |
| software | `name` | | `name`, `cpe`, `swid`, `vendor`, `version` | no |
| url | `value` | | `value` | no |
| user-account | at least one property | | `account_type`, `user_id`, `account_login` | yes |
| windows-registry-key | at least one property | | `key`, `values` (all items) | yes |
| x509-certificate | at least one property | | `hashes` (one), `serial_number` | yes |

Predefined extension minimums: `archive-ext` requires `contains_refs`; `icmp-ext` requires both hex fields; `http-request-ext` requires `request_method` and `request_value`; `socket-ext` requires `address_family`; `windows-pebinary-ext` requires `pe_type` plus one more; `ntfs-ext`, `pdf-ext`, `raster-image-ext`, `tcp-ext`, `windows-process-ext`, `windows-service-ext`, `unix-account-ext` require at least one property.

---

## 5. Embedded reference matrix (from [SPEC] property text; the schemas only check "string")

| Source property | Cardinality | Allowed target types |
|---|---|---|
| `directory.contains_refs` | list | file, directory |
| `domain-name.resolves_to_refs` | list | ipv4-addr, ipv6-addr, domain-name |
| `email-addr.belongs_to_ref` | single | user-account |
| `email-message.from_ref`, `sender_ref` | single | email-addr |
| `email-message.to_refs`, `cc_refs`, `bcc_refs` | list | email-addr |
| `email-message.raw_email_ref` | single | artifact |
| `email-mime-part-type.body_raw_ref` | single | artifact, file |
| `file.parent_directory_ref` | single | directory |
| `file.contains_refs` | list | any SCO |
| `file.content_ref` | single | artifact |
| `archive-ext.contains_refs` | list | file, directory |
| `ipv4-addr.resolves_to_refs`, `ipv6-addr.resolves_to_refs` | list | mac-addr |
| `ipv4-addr.belongs_to_refs`, `ipv6-addr.belongs_to_refs` | list | autonomous-system |
| `network-traffic.src_ref`, `dst_ref` | single | ipv4-addr, ipv6-addr, mac-addr, domain-name |
| `network-traffic.src_payload_ref`, `dst_payload_ref` | single | artifact |
| `network-traffic.encapsulates_refs` | list | network-traffic |
| `network-traffic.encapsulated_by_ref` | single | network-traffic |
| `http-request-ext.message_body_data_ref` | single | artifact |
| `process.opened_connection_refs` | list | network-traffic |
| `process.creator_user_ref` | single | user-account |
| `process.image_ref` | single | file |
| `process.parent_ref` | single | process |
| `process.child_refs` | list | process |
| `windows-service-ext.service_dll_refs` | list | file |
| `windows-registry-key.creator_user_ref` | single | user-account |

SCO-specific SRO relationship types: `resolves-to` (domain-name -> domain-name/ipv4/ipv6; ipv4/ipv6 -> mac-addr) and `belongs-to` (ipv4/ipv6 -> autonomous-system) ([SPEC] 6.4.2, 6.8.2, 6.9.2), plus the common `derived-from`, `duplicate-of`, `related-to`.

---

## 6. Vocabularies and enumerations used by SCOs

Open vocabulary (`-ov`) semantics [SPEC] 2.14: value SHOULD be from the list but MAY be any string; non-listed values SHOULD be lowercase with hyphens. Enumeration (`-enum`) semantics [SPEC] 2.4: closed, "MUST NOT be expanded by implementations".

| Vocabulary | Kind | Used by | Values ([SPEC] section 10) | Schema location / differences |
|---|---|---|---|---|
| `hash-algorithm-ov` | open | hashes keys (artifact, file, ADS, PE file header/optional header/section, x509, external-reference) | `MD5, SHA-1, SHA-256, SHA-512, SHA3-256, SHA3-512, SSDEEP, TLSH` (10.7; MD5 per RFC 1321, SHA-1 RFC 3174, SHA-2 RFC 6234, SHA3 FIPS 202, SSDEEP piecewise hash, TLSH "valid 35 byte long hash") | encoded as patternProperties in `common/hashes-type.json:13-52` (see 1.3) |
| `encryption-algorithm-enum` | enum | artifact.encryption_algorithm | `AES-256-GCM` (NIST SP800-38D), `ChaCha20-Poly1305` (RFC 7539), `mime-type-indicated` (format self-describes, e.g. password zip, GPG armored) (10.4) | `artifact.json:116-123`, identical |
| `account-type-ov` | open | user-account.account_type | `facebook, ldap, nis, openid, radius, skype, tacacs, twitter, unix, windows-local, windows-domain` (10.1; `unix` = POSIX account) | `user-account.json:152-168` lists `unix, windows local, windows domain, ldap, tacacs, radius, nis, openid, facebook, skype, twitter, kavi`: spaces instead of hyphens, extra `kavi`, and not referenced by any property. [PY] `vocab.py:6-16` uses the spec hyphenated forms |
| `windows-pebinary-type-ov` | open | windows-pebinary-ext.pe_type | `dll, exe, sys` (10.28) | `file.json:560-567` (`exe, dll, sys`), not referenced |
| `network-socket-address-family-enum` | enum | socket-ext.address_family | `AF_UNSPEC, AF_INET, AF_IPX, AF_APPLETALK, AF_NETBIOS, AF_INET6, AF_IRDA, AF_BTH` (10.16) | `network-traffic.json:248-257`, identical |
| `network-socket-type-enum` | enum | socket-ext.socket_type | `SOCK_STREAM, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET` (10.17; the summary row contains the typo `AF_ISOCK_DGRAMNET`, the value table says `SOCK_DGRAM`) | `network-traffic.json:280-286`, identical to the value table |
| `windows-integrity-level-enum` | enum | windows-process-ext.integrity_level | `low, medium, high, system` (10.27) | `process.json:255-263`, identical |
| `windows-registry-datatype-enum` | enum | windows-registry-value-type.data_type | `REG_NONE, REG_SZ, REG_EXPAND_SZ, REG_BINARY, REG_DWORD, REG_DWORD_BIG_ENDIAN, REG_DWORD_LITTLE_ENDIAN, REG_LINK, REG_MULTI_SZ, REG_RESOURCE_LIST, REG_FULL_RESOURCE_DESCRIPTION, REG_RESOURCE_REQUIREMENTS_LIST, REG_QWORD, REG_INVALID_TYPE` (10.29) | `windows-registry-key.json:73-88`, identical |
| `windows-service-start-type-enum` | enum | windows-service-ext.start_type | `SERVICE_AUTO_START, SERVICE_BOOT_START, SERVICE_DEMAND_START, SERVICE_DISABLED, SERVICE_SYSTEM_ALERT` (10.30) | `process.json:178-184`, identical |
| `windows-service-type-enum` | enum | windows-service-ext.service_type | `SERVICE_KERNEL_DRIVER, SERVICE_FILE_SYSTEM_DRIVER, SERVICE_WIN32_OWN_PROCESS, SERVICE_WIN32_SHARE_PROCESS` (10.31) | `process.json:197-202`, identical |
| `windows-service-status-enum` | enum | windows-service-ext.service_status | `SERVICE_CONTINUE_PENDING, SERVICE_PAUSE_PENDING, SERVICE_PAUSED, SERVICE_RUNNING, SERVICE_START_PENDING, SERVICE_STOP_PENDING, SERVICE_STOPPED` (10.32) | `process.json:207-215`, identical |
| `extension-type-enum` | enum | extension instances, extension-definition | `new-sdo, new-sco, new-sro, property-extension, toplevel-property-extension` (10.5) | `common/extension.json:17-26`, `common/extension-definition.json:85-94` |

External registries referenced (no in-spec list): IANA Media Types (artifact/file `mime_type`), IANA Character Sets 2013-12-20 (`name_enc`, `path_enc`), IANA Service Name and Port Number Registry (`protocols`), NVD CPE 2.3 Dictionary (`software.cpe`), RFC 5646 (`software.languages`), SWID tags (`software.swid`), IPFIX information elements (`ipfix` keys), EXIF tag names (`exif_tags` keys), PDF DID entry names (`document_info_dict`), Windows `*_CLASS` priority names, `AF_*`/`SO_*` socket constants.

---

## 7. Deterministic SCO identifiers

### 7.1 Normative text ([SPEC] 2.9, quoted)

- "All identifiers, excluding those used in the deprecated Cyber Observable Container, MUST follow the form object-type--UUID ... the UUID MUST be an RFC 4122-compliant UUID."
- "The UUID part of the identifier MUST be unique across all objects produced by a given producer regardless of the type identified by the object-type prefix."
- "STIX Cyber-observable Objects SHOULD use UUIDv5 for the UUID portion of the identifier and the UUID portion of the UUIDv5-based identifier SHOULD be generated according to the following rules:"
  - "The namespace SHOULD be 00abedb4-aa42-466c-9c01-fed23315a9b7. This defined namespace is necessary to support the goal of deduplication and semantic equivalence of some STIX objects in the community of producers."
  - "The value of the name portion SHOULD be the list of "ID Contributing Properties" (property-name and property value pairs) as defined on each SCO object and SHOULD be represented as a JSON object that is then serialized / stringified according to [RFC8785] to ensure a canonical representation of the JSON data."
  - "If the contributing properties are all optional, and none are present on the SCO, then a UUIDv4 MUST be used."
  - "Producers not following these rules MUST NOT use a namespace of 00abedb4-aa42-466c-9c01-fed23315a9b7 and SHOULD use UUIDv4 in cases where the id would not be unique."
- SDOs/SROs/SMOs/Bundles SHOULD use UUIDv4 and "MUST NOT use a namespace of 00abedb4-aa42-466c-9c01-fed23315a9b7 if generating a UUIDv5."
- [SPEC] 3.4: "In some cases, additional selection of extension properties that contribute to the ID may be described in the ID Contributing Properties section listed on each SCO. The default algorithm ... is a UUIDv5 ... however, other algorithms for creating the SCO ID MAY be used."
- [SPEC] 3.6: "producers versioning SCOs MUST NOT use the default namespace for deterministic ID creation."
- Hash rule (in the ID Contributing row of artifact, file, x509-certificate): "If the hashes property is present, include only one hash. The selected hash SHOULD come from this ordered list (based on the following order of preference) [ MD5, SHA-1, SHA-256, SHA-512 ]."
- Appendix D (Security Considerations): SHA-1 (inside UUIDv5) is not collision resistant; different contributing properties may yield the same UUIDv5; SCOs are not tied to an author via `created_by_ref`, so feed filtering may not catch colliding objects.

### 7.2 Algorithm (as implemented by [PY] `stix2/base.py:439-474` and [OCTI] `opencti-platform/opencti-graphql/src/schema/identifier.js:331-347`; both agree on the core)

1. Let `C` = the ID contributing properties of the SCO type (section 4). If the type has none (process), use UUIDv4.
2. Build a JSON object `O` containing, for each `p` in `C` that is present on the SCO, `p: value`:
   - `hashes`: replace by a single-entry object `{alg: value}` chosen by precedence `MD5 > SHA-1 > SHA-256 > SHA-512`. [PY] `_choose_one_hash` (`base.py:484-498`): if none of the four is present, the FIRST key in dictionary iteration order (insertion order, i.e. producer dependent). [OCTI] `hashes` resolver (`identifier.js:157-171`): uppercases keys first, then `MD5, SHA-1, SHA-256, SHA-512, SHA3-256, SHA3-512, SSDEEP, SDHASH, TLSH, LZJD`.
   - `*_ref` / `*_refs`: the referenced object's id string (so referenced SCO ids must be computed first).
   - lists (`protocols`, `values`) keep their order; dictionaries (`extensions`) are embedded whole, recursively (all keys, including any custom extension).
   - timestamps (network-traffic `start`/`end`): their string serialisation. [PY] re-serialises datetimes and strips trailing zeros (test `test_json_incompatible_timestamp_value`: `678900` microseconds -> `"1987-01-02T03:04:05.6789Z"`, `test_deterministic_ids.py:93-116`).
3. If `O` is empty, use UUIDv4 (all-optional case).
4. `name` = RFC 8785 JCS serialisation of `O`: members sorted by UTF-16 code units of the key, no insignificant whitespace, ECMAScript `JSON.stringify` string escaping (only `"`, `\`, and control characters; non-ASCII emitted literally), ECMAScript number formatting, UTF-8 bytes. [PY] uses `canonicalize(obj, utf8=False)` from `stix2/canonicalization/Canonicalize.py`; [OCTI] uses npm `canonicalize` (`identifier.js:4`).
5. `uuid = UUIDv5(namespace = 00abedb4-aa42-466c-9c01-fed23315a9b7, name)` per RFC 4122 (SHA-1 over the 16 namespace bytes followed by the name bytes; first 16 bytes; version nibble 5; RFC 4122 variant).
6. `id = "<type>--" + uuid` (lowercase hex). The type is NOT part of the hashed name: two different SCO types with identical contributing JSON (e.g. `domain-name` and `url` both `{"value":"x"}`) share the UUID part, in tension with the 2.9 cross-type uniqueness rule.

Example: `{"type":"domain-name","value":"hostelhotels.net"}` -> name `{"value":"hostelhotels.net"}` -> `domain-name--07f3ac51-335f-524c-918f-86f80da2bce3` ([VERIFIED], matches [OCTI] test `opencti-platform/opencti-graphql/tests/01-unit/parser/json-parser-test.js:95-96`).

### 7.3 Verified test vectors (algorithm output equals the third-party expected value)

| Input (contributing JSON as canonicalised) | Expected id | Source of expectation |
|---|---|---|
| `{"value":"hostelhotels.net"}` | `domain-name--07f3ac51-335f-524c-918f-86f80da2bce3` | [OCTI] `tests/01-unit/parser/json-parser-test.js:95-96` |
| `{"hashes":{"SHA-256":"aba8b59281faa8c1c43a4ca7af075edd3e3516d3cef058a1f43b093177b8f83c"}}` | `file--1b6bb8bc-bc12-5fe7-9d5c-43fce2a4ac84` | [OCTI] same file :107-109 |
| `{"hashes":{"MD5":"025ad219ece1125a8f5a0e74e32676cb"}}` | `file--cd03138e-eb70-5409-b5df-2f53bee7a1e1` | [OCTI] `tests/03-integration/01-database/identifier-test.js:193-196, 225` |
| `{"hashes":{"SHA-1":"c1750bee9c1f7b5dd6f025b645ab6eba5df94175"}}` | `file--0c28767c-5f72-5036-8cc5-21c055c2b9e9` | [OCTI] same file :226 |
| file with MD5 `757a71f0fbd6b3d993be2a213338d1f2`, SHA-1 and SHA-256 present -> `{"hashes":{"MD5":"757a71f0fbd6b3d993be2a213338d1f2"}}` (MD5 wins) | `file--702e320e-43b6-552a-b7e7-d045bf9c887d` | [OCTI] `tests/01-unit/database/stix-2-0-converter-fixtures/SDOs/arsenal/malware.ts:39-45` |
| `{"extensions":{"windows-pebinary-ext":{"machine_hex":"014c","pe_type":"exe","sections":[{"entropy":7.980693,"hashes":{"SHA-256":"6e3b6f3978e5cd96ba7abee35c24e867b7e64072e2ecb22d0ee7a6e6af6894d0"},"name":".data","size":4096}]}},"hashes":{"SHA-256":"841a8921140aba50671ebb0770fecc4ee308c4952cfeff8de154ab14eeef4649"},"name":"example.exe"}` | `file--ced31cd4-bdcb-537d-aefa-92d291bfc11d` | [PY] `test_deterministic_ids.py:273-297` |
| `{"values":[{"data":"qwerty","name":"Foo"},{"data":"42","name":"Bar"}]}` | `windows-registry-key--36594eba-bcc7-5014-9835-0e154264e588` | [PY] same file :300-314 |

### 7.4 The example ids printed in [SPEC] section 6 are NOT reproducible

[VERIFIED] I recomputed all 31 deterministic example ids of [SPEC] section 6 with the algorithm above (which reproduces every [PY] and [OCTI] vector in 7.3): 0/31 match, and variants (with `type`, with `spec_version`, spaced JSON, other namespaces) also do not match. Do NOT use the spec example ids as test expectations. Algorithmically correct ids for those example inputs:

| SCO / contributing JSON | Id printed in [SPEC] | Correct id per algorithm |
|---|---|---|
| domain-name `{"value":"example.com"}` | `3c10e93f-798e-5a26-a0c1-08156efab7f5` | `bedb4899-d24b-5401-bc86-8f6b4cc18ec7` |
| ipv4-addr `{"value":"198.51.100.3"}` | `ff26c055-6336-5bc5-b98d-13d6226742dd` | `28bb3599-77cd-5a82-a950-b5bc3caf07c4` |
| ipv4-addr `{"value":"198.51.100.0/24"}` | `5853f6a4-638f-5b4e-9b0f-ded361ae3812` | `1ea47840-aa39-5aa8-b4a2-cd203eb6662a` |
| ipv6-addr `{"value":"2001:0db8:85a3:0000:0000:8a2e:0370:7334"}` | `1e61d36c-a16c-53b7-a80f-2a00161c96b1` | `85a85a8c-ee99-5722-946d-3c3a3270fc6f` |
| ipv6-addr `{"value":"2001:0db8::/96"}` | `5daf7456-8863-5481-9d42-237d477697f4` | `084d3b4c-7785-568a-a569-0c61c95754ab` |
| mac-addr `{"value":"d2:fb:49:24:37:18"}` | `65cfcf98-8a6e-5a1b-8f61-379ac4f92d00` | `757b1725-9903-54f5-a855-1240691d7659` |
| mutex `{"name":"__CLEANSWEEP__"}` | `eba44954-d4e4-5d3b-814c-2b17dd8de300` | `f93fe911-e545-5239-b9b0-597840d0c871` |
| url `{"value":"https://example.com/research/index.html"}` | `c1477287-23ac-5971-a010-5c287877fa60` | `47c3cf9a-5027-5bf0-997a-017c7edc7c55` |
| email-addr `{"value":"john@example.com"}` | `2d77a846-6264-5d51-b586-e43822ea1ea3` | `7165e2a9-671f-585d-b1e1-ca59c671d934` |
| email-addr `{"value":"jdoe@example.com"}` | `89f52ea8-d6ef-51e9-8fce-6a29236436ed` | `6deb37bd-12b7-54ae-805f-5f7146f3d171` |
| autonomous-system `{"number":15139}` | `f720c34b-98ae-597f-ade5-27dc241e8c74` | `3aa27478-50b5-5ab8-9da9-cdc12b657fff` |
| directory `{"path":"C:\\Windows\\System32"}` | `93c0a9b0-520d-545d-9094-1a08ddf46b05` | `0a58d0c1-59e6-5afd-8252-dcd3f13e5622` |
| software `{"cpe":"cpe:2.3:a:microsoft:word:2000:*:*:*:*:*:*:*","name":"Word","vendor":"Microsoft","version":"2002"}` | `a1827f6d-ca53-5605-9e93-4316cd22a00a` | `710b0b41-d4d0-5d6c-a400-fc9254554ffc` |
| user-account `{"account_login":"jdoe","account_type":"unix","user_id":"1001"}` | `0d5b424b-93b8-5cd8-ac36-306e1789d63c` | `f94d689e-707d-58c3-b803-c720bb6ed096` |
| windows-registry-key `{"key":"HKEY_LOCAL_MACHINE\\System\\Foo\\Bar"}` | `9d60798d-4e3e-5fe4-af8a-0e4986f0f90b` | `1884b770-b679-522a-a9c7-45fadaaec7bb` |
| file `{"hashes":{"SHA-256":"fe90a7e9...681db"},"name":"foo.dll"}` | `e277603e-1060-5ad4-9937-c26c97f1ca68` | `949eb97a-da1e-5292-8959-03c4249dc9f3` |
| x509-certificate `{"serial_number":"36:f7:d4:32:f4:ab:70:ea:d3:ce:98:6e:ea:99:93:49:32:0a:b7:06"}` | `463d7b2a-8516-5a50-a3d7-6f801465d5de` | `bf86f034-2390-54d7-8426-d08433db3fbe` |
| network-traffic `{"dst_ref":"ipv4-addr--ff26c055-...","protocols":["tcp"],"src_ref":"ipv4-addr--4d22aae0-..."}` | `2568d22a-8998-58eb-99ec-3c8ca74f527d` | `6e0cb830-0305-57d4-8536-7be874d42005` (computed over the spec's own, also wrong, ref ids) |
| email-message `{"from_ref":"email-addr--89f52ea8-...","subject":"Saying Hello"}` | `72b7698f-10c2-565a-a2a6-b4996a2f2265` | `92fa1bfd-2c62-5a30-8b7c-5b91ea73cf29` (same caveat) |

(Directory/registry JSON shown with JSON escaping: the actual string has single backslashes.)

### 7.5 Reference implementations compared ([SPEC] vs [PY] vs [OCTI])

| SCO | [SPEC] contributing | [PY] `_id_contributing_properties` (`stix2/v21/observables.py`) | [OCTI] `stixBaseCyberObservableContribution` (`opencti-platform/opencti-graphql/src/schema/identifier.js:96-146`) |
|---|---|---|---|
| artifact | hashes, payload_bin | same (:51) | `[[hashes],[url]]` (:104): hashes, else url; payload_bin never ("Cause of volume") |
| autonomous-system | number | same (:77) | number (:99) |
| directory | path | same (:102) | path (:100) |
| domain-name | value | same (:122) | value (:101) |
| email-addr | value | same (:143) | value (:102) |
| email-message | from_ref, subject, body | same (:193) | from_ref (resolved from the `from` entity's standard_id), subject, body (:103) |
| file | hashes, name, extensions, parent_directory_ref | hashes, name, parent_directory_ref, extensions (:374) | `[[hashes],[name]]` (:105): hashes ONLY if present, else name; never extensions/parent_directory_ref |
| ipv4-addr / ipv6-addr / mac-addr | value | same (:399, :420, :439) | value (:107-109) |
| mutex | name | same (:458) | name (:110) |
| network-traffic | start, end, src_ref, dst_ref, src_port, dst_port, protocols, extensions | same (:563) | same minus extensions (:111-119) |
| process | none (UUIDv4) | `[]` (:648) | pid (only if command_line present) + command_line (:120): deterministic, deviates from spec |
| software | name, cpe, swid, vendor, version | same (:686) | same, but `name` lowercased for id only (:121, resolver :173-179) |
| url | value | same (:705) | value (:122) |
| user-account | account_type, user_id, account_login | same (:751) | same (:123) |
| windows-registry-key | key, values | same (:788) | `{ src: 'attribute_key', dst: 'key' }` + values (:124). `filteredIdContributions` reads `dest`, not `dst` (`identifier.js:396`, `destKey = dest \|\| src`), so the canonical member name appears to be `attribute_key` instead of `key` (my reading of the code; no test vector found) |
| x509-certificate | hashes, serial_number | same (:845) | `[[hashes],[serial_number],[subject]]` (:106): first non-empty "way" wins |

Other [OCTI] behaviours that change ids: string values are `trim()`med before hashing (`identifier.js:409`); hash keys uppercased (`:159`); for hashed observables OpenCTI also computes one alternative standard id per individual hash (`generateHashedObservableStandardIds`, `identifier.js:546-560`) and uses them, plus raw hash values, for dedup (`getInputIds`, `getInstanceIds`). OpenCTI attaches its own extensions (`extension-definition--ea279b3e-5c71-4632-ac08-831c66a786ba`, SCO: `extension-definition--f93e2c80-4231-4f9a-af8b-95c9bd566a82`, `opencti-platform/opencti-graphql/src/types/stix-2-1-extensions.ts:6,13`) to every SCO; since [PY] hashes the whole `extensions` dictionary for file and network-traffic, a [PY] recomputation of an OpenCTI-exported file with extensions would give a different id than OpenCTI's.

[PY] constraint checks worth mirroring: artifact `payload_bin`/`url` mutually exclusive and `url` requires `hashes` (`observables.py:53-56`); email-mime-part at least one of body/body_raw_ref (:158-160); email-message `body_multipart` requires `is_multipart`, `body` forbidden when true (:195-200); file at least one of hashes/name (:376-378); network-traffic src/dst, is_active/end, end >= start (:565-582); process at least one property (:650-662); x509 at least one property (:847-857).

### 7.6 Scripts used

- Converter and verifier: throwaway Python scripts, not kept in the repository. The verifier uses `json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=False)`, equivalent to JCS for ASCII keys and integer/string/simple float values.
- Spec text and [PY] sources: throwaway local downloads, not kept in the repository.

---

## 8. Schema vs spec discrepancies and schema defects (consolidated)

| # | Location | Issue |
|---|---|---|
| 1 | `observables/user-account.json:152-168` | account-type-ov values `windows local`, `windows domain` (spaces) and extra `kavi`; spec uses `windows-local`, `windows-domain`; definition unreferenced |
| 2 | `observables/file.json:560-567` | windows-pebinary-type-ov defined but `pe_type` (:262-265) does not reference it |
| 3 | `observables/process.json:83-98` | anyOf uses 2.0 names `name`, `created`, `arguments`; missing `created_time` |
| 4 | `observables/process.json:46` | description mentions `binary_ref` (2.0); spec 2.1 `image_ref` |
| 5 | `observables/process.json:139-148` | windows-process-ext anyOf omits `integrity_level` |
| 6 | `observables/windows-registry-key.json:54` | anyOf uses `modified` instead of `modified_time` |
| 7 | `observables/windows-registry-key.json:26` | unanchored `not` alternation forbids hive abbreviations anywhere in the key, not only as prefix |
| 8 | `observables/file.json:187` | raster-image-ext anyOf lists undefined 2.0 property `image_compression_algorithm` |
| 9 | `observables/user-account.json:115` | `gid` is `number`; spec integer |
| 10 | `observables/ipv6-addr.json:27` | corrupted regex (`s*`, `d`, `.`), rejects IPv4-embedded IPv6, accepts leading `s` |
| 11 | `common/hex.json:7`, `common/hashes-type.json:16-51` | accept uppercase hex; spec says lowercase for `hex` (spec's own example `magic_number_hex: "504B0304"` is uppercase) |
| 12 | `common/hashes-type.json:9` | lowercase/variant algorithm keys (`md5`, `sha256`) accepted as custom without value validation |
| 13 | `common/hashes-type.json:51` | TLSH fixed at 70 chars; [REC] note: TLSH digests with the `T1` version prefix are 72 chars (my knowledge of TLSH 4.x, not in sources) |
| 14 | `observables/email-message.json:188-201` | header values may be a single string; spec says MUST always be list of string (spec example uses strings) |
| 15 | `observables/network-traffic.json:200-205` | request_header values must be arrays; spec example uses strings |
| 16 | `observables/email-message.json:181` | unanchored alternation in forbidden header-name regex |
| 17 | `observables/network-traffic.json:271` | socket option key regex rejects digits (e.g. `IPV6_V6ONLY`) |
| 18 | `observables/process.json:238-251` | startup-info patterns unanchored; `lpReserved*` typed `null` (STIX has no null values) |
| 19 | `observables/network-traffic.json:64-79` | byte/packet counts lack `minimum`; spec "positive integer"; `end >= start` not enforced |
| 20 | all `*_ref`/`*_refs` | plain `string`, no identifier pattern or target-type check (README acknowledges) |
| 21 | `observables/email-addr.json:26`, `domain-name.json:27`, `common/url-regex.json:7` | `format` keywords are annotations by default in draft 2020-12 |
| 22 | `observables/software.json:30`, `artifact.json:26` | CPE and mime regexes not fully anchored; mime limited to 9 top-level types |
| 23 | `observables/software.json:42` | language regex rejects title-case script subtags (`zh-Hant`) |
| 24 | `common/cyber-observable-core.json:88` | reserves `phone_numbers`; spec 3.8 reserves `phone_number` |
| 25 | `common/cyber-observable-core.json:22-24` | `spec_version` allows `2.0`; spec says MUST be `2.1` |
| 26 | `observables/email-message.json:72-78`, `:123-129`, `windows-registry-key.json:29-35`, `file.json:111-134` | lists without `minItems` (spec 2.12 forbids empty lists) |
| 27 | `observables/mac-addr.json:27` | description says "CIDR notation" (copy error) |
| 28 | `observables/network-traffic.json:26` | description says `http-ext`; key is `http-request-ext` |
| 29 | [SPEC] 6.6.1 | `from_ref` etc. "MUST be of type email-address" (typo for `email-addr`) |
| 30 | [SPEC] 10.17 | summary row typo `AF_ISOCK_DGRAMNET` |
| 31 | [SPEC] section 6 examples | deterministic example ids not reproducible (section 7.4) |
| 32 | [SPEC] 3.1 vs schemas | `_enc` MUST NOT be present without its base property: not enforced |

---

## 9. Recommendations for Fennec (my own, not from sources) [REC]

1. One aggregate per SCO type (18) plus one value object per embedded type and per predefined extension (12 extensions: archive, ntfs, pdf, raster-image, windows-pebinary, http-request, icmp, socket, tcp, windows-process, windows-service, unix-account; 6 embedded types: alternate-data-stream, windows-pe-optional-header, windows-pe-section, email-mime-part, windows-registry-value, x509-v3-extensions). Encode "at least one property" rules as smart constructors returning errors, not as runtime checks scattered in handlers.
2. Closed Rust enums for every `-enum` in section 6. For each `-ov`, an enum with the spec values plus `Other(LowercaseHyphenatedString)` (spec 2.14), parsing the schema variants (`windows local`) and normalising to spec form (`windows-local`).
3. `HashAlgorithm` enum (`Md5, Sha1, Sha256, Sha512, Sha3_256, Sha3_512, Ssdeep, Tlsh, Custom(Key)`), key parsing tolerant like [PY] (`replace("-","").upper()`), serialisation to spec names; per-algorithm value objects with length checks; store hex digests lowercase. Decide (open question) whether ids are computed on the normalised lowercase value: [PY]/[OCTI] hash the value as received.
4. Validate stricter than the schemas where the schemas are buggy: use a real IP parser (std `Ipv4Addr`/`Ipv6Addr` + prefix) instead of regex 3.9; registry hive check as prefix match on `HKEY_*`; socket option keys by prefix like [PY].
5. Deterministic id service: `uuid::Uuid::new_v5(&NS, jcs_bytes)` with NS `00abedb4-aa42-466c-9c01-fed23315a9b7`, JCS via an RFC 8785 crate (e.g. `serde_jcs`; crate choice to verify). Compute ids bottom-up because refs contribute (email-message `from_ref`, network-traffic `src_ref`/`dst_ref`, file `parent_directory_ref`, archive-ext `contains_refs` inside file `extensions`). Use the test vectors of 7.3, not the spec examples.
6. OpenCTI compatibility: to reproduce OpenCTI `standard_id`s, implement OpenCTI's variant (section 7.5: file = hashes-or-name, artifact = hashes-or-url, x509 = hashes-or-serial-or-subject, network-traffic without extensions, process pid+command_line, software lowercase name, trim strings, extended hash precedence). Consider storing both the spec id and the OpenCTI id (OpenCTI itself keeps alternative ids in `x_opencti_stix_ids` and per-hash ids) and treating either as an alias for dedup.
7. Identity semantics for DDD: SCOs are unversioned; a change to any id-contributing property yields a new identity (new aggregate), changes to other properties are merges on the same aggregate. Never set `created`/`modified` on SCOs under the OASIS namespace.
8. Canonicalise inputs before id computation in a documented way (IPv4: drop `/32`; IPv6: RFC 5952 lowercase compressed, drop `/128`; MAC: lowercase colon form, which the spec mandates; registry key hive expanded). Each normalisation diverges from producers that do not normalise, so pair it with alias ids.

---

## 10. Catalogue count

18 SCOs + 12 predefined extensions + 6 embedded types + 12 vocabularies/enumerations + 12 common types/definitions (identifier, timestamp, binary, hex, dictionary, list, integer, float, hashes, extension, extension-definition, granular-marking) = 60 catalogued items.
