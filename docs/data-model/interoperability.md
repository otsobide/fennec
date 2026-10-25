# Interoperability

How Fennec observables map to and from STIX 2.1, OpenCTI and MISP. The
mapping is part of the model: it decides which kinds exist (a kind without a
clean mapping still exists if analysts pivot on it) and what an importer must
reject.

None of these adapters exists yet. This page is the contract they will
implement, so that the kinds built before them already carry what the
mapping needs.

## STIX 2.1

### Export

Every observable exports to one STIX cyber-observable object:

| Property | Value |
|---|---|
| `type` | The kind's STIX type (see [identity.md](identity.md#stix-type-names)) |
| `id` | `<type>--<uuid>`, the deterministic identifier |
| `spec_version` | `2.1` |
| contributing and descriptive properties | The canonical values, under their STIX names |

STIX forbids `created_by_ref`, `created`, `modified`, `labels`,
`confidence` and `external_references` on cyber-observables, which matches
Fennec's rule that observables carry no assessments. Assessments export as
their own objects (indicators, notes, opinions) that reference the
observable.

Host and network kinds of the same family share a STIX type: `ipv4_address`
exports `{"type": "ipv4-addr", "value": "89.19.209.154"}` and `ipv4_network`
exports `{"type": "ipv4-addr", "value": "198.51.100.0/24"}`.

### Fennec-only kinds

Kinds with an `x-fennec-*` type are declared through one Fennec
`extension-definition` of `extension_type` `new-sco`, created by a fixed
Fennec producer identity:

```json
{
  "type": "identity",
  "spec_version": "2.1",
  "id": "identity--741a70ef-0293-4e53-8816-cdc8d155d423",
  "created": "2026-10-25T00:00:00.000Z",
  "modified": "2026-10-25T00:00:00.000Z",
  "name": "Fennec",
  "identity_class": "system"
}
```

```json
{
  "type": "extension-definition",
  "spec_version": "2.1",
  "id": "extension-definition--e36c7a52-1f1d-4040-9de6-36fb400d9b02",
  "created_by_ref": "identity--741a70ef-0293-4e53-8816-cdc8d155d423",
  "created": "2026-10-25T00:00:00.000Z",
  "modified": "2026-10-25T00:00:00.000Z",
  "name": "Fennec observable kinds",
  "description": "Cyber-observable types defined by Fennec.",
  "schema": "<URL of the published JSON schema>",
  "version": "1.0.0",
  "extension_types": ["new-sco"]
}
```

Both ids are fixed for the lifetime of the project. When a wave adds
`x-fennec-*` types, the JSON schema gains them and the extension-definition's
`version` and `modified` change; its id does not. Each exported object of
such a kind references it:

```json
{
  "type": "x-fennec-google-analytics-tracking-id",
  "spec_version": "2.1",
  "id": "x-fennec-google-analytics-tracking-id--<uuid>",
  "value": "UA-12345678-1",
  "extensions": {
    "extension-definition--e36c7a52-1f1d-4040-9de6-36fb400d9b02": {
      "extension_type": "new-sco"
    }
  }
}
```

### Import

- A STIX type that maps to one kind is built through that kind's value
  objects.
- A STIX type shared by a family of kinds is resolved inside the family by
  structure, never by guessing across families: `ipv4-addr` and `ipv6-addr`
  become the host kind when the value has no prefix shorter than the full
  width and the network kind otherwise. `cryptocurrency-wallet` cannot be
  resolved from its value and needs an explicit chain from the producer;
  without one it is rejected as untyped.
- The incoming `id` is not trusted: Fennec recomputes the identifier from the
  canonical value. When the two differ (the producer did not canonicalise,
  or used OpenCTI's variant), the importer records the incoming id as an
  alias so later references to it still resolve.
- A value that fails the kind's validation is rejected and reported with the
  object id and the violated rule. It is never stored raw.

## OpenCTI

### Kinds with an OpenCTI type

Kinds whose STIX type is an OpenCTI type (the 18 STIX objects plus
`hostname`, `phone-number`, `cryptocurrency-wallet`, `bank-account`, `imei`,
`imsi`, `iccid`, `ssh-key`, `user-agent`) round-trip one to one. Their
identifiers match OpenCTI's when the value arrived canonical, except for the
kinds where OpenCTI deviates from the specification (file, artifact, X.509
certificate, process, software, email message, network traffic and registry
key, listed in
[identity.md](identity.md#where-fennec-deliberately-differs-from-opencti)):
those round-trip by alias, never by matching identifier.

Known lossy cases:

| Case | Behaviour |
|---|---|
| `ssh-key` | OpenCTI takes `fingerprint_sha256` as a free string from the producer. Identifiers match only when the producer used Fennec's `SHA256:<base64 without padding>` form. |
| `imsi` shorter than 14 digits | OpenCTI rejects it, so it does not export. |
| A 16-digit OpenCTI `imei` | Imports as `imeisv`. |
| `bic` alone | OpenCTI only knows a BIC as an attribute of a Bank-Account keyed by IBAN, so a standalone BIC exports as labelled `text`. |

The OpenCTI-specific SCO properties map as follows:

| OpenCTI property | Fennec |
|---|---|
| `x_opencti_score`, `x_opencti_description`, labels, markings, `createdBy`, external references | Assessments; not part of the observable |
| `x_opencti_stix_ids` | Alias identifiers, recorded by the importer |
| `x_opencti_files` | Not imported |

### Kinds without an OpenCTI type

OpenCTI has no typed home for web tracking ids, TLS fingerprints, HASSH,
binary structural hashes, platform account ids, application ids or cloud
ids. They export to OpenCTI as a `text` observable whose `value` is the
canonical value and which carries the label `fennec:<module>` (for example
`fennec:ja4_fingerprint`). Because STIX does not hash the type, the UUID part
of the OpenCTI `text--<uuid>` equals the UUID part of the Fennec identifier.

The mapping is many to one. Different kinds can share a canonical string (32
lowercase hex is a valid imphash, JA3, HASSH and MISP dom-hash; a run of
digits is a valid Yandex, Hotjar and Microsoft UET id), and OpenCTI merges
them into one `text` that carries every label. On import:

- a `text` with one or more `fennec:<module>` labels becomes one observable
  per label;
- a `text` with no `fennec:` label carries no kind and is rejected as
  untyped.

## MISP

### Import

| MISP construct | Fennec |
|---|---|
| Attribute of a typed kind (`ip-dst`, `domain`, `sha256`, `ja3-fingerprint-md5`, `jarm-fingerprint`, `btc`, `github-username`, `onion-address`, ...) | The matching kind; role variants (`ip-src` and `ip-dst`, `email-src` and `email-dst`) collapse into one kind |
| `ip-src` and `ip-dst` with a CIDR value | The network kind; `/32` and `/128` fold to the host kind |
| `domain\|ip` | A `domain_name`, an address, and the relationship between them |
| `filename\|<hash>`, `malware-sample` | One `file` with a name and a hash |
| `ip-src\|port`, `ip-dst\|port` | The address; the port goes to the network traffic or sighting it was seen in |
| Object template (`file`, `x509`, `ja4-plus`, `phone`, ...) | One observable per typed attribute, plus relationships |
| `tracking-id` object | Its `tracker` attribute is optional free text that cannot tell Google's id families apart. The importer builds the web tracking kind whose grammar matches the `id`, and only when exactly one kind of the tracking family matches (`UA-`, `G-`, `GT-`, `GTM-`, `AW-` and `pub-` grammars are disjoint). Otherwise the value is reported as untyped. This is the one place where the kind is chosen by grammar, and only inside a family the producer declared. |
| `to_ids`, `category`, `comment`, tags, galaxies | Assessments and context; not part of the observable |
| Sightings | Fennec sightings of the imported observables |
| Correlations | Not imported: shared values are already the same aggregate |
| `text`, `other`, `comment` and the other catch-all types | Not imported; reported as untyped |

MISP validates many types loosely (URLs, cryptocurrency addresses and bank
identifiers only reject newlines or non-alphanumerics, and four-byte UTF-8
characters were replaced by `?` in older MySQL schemas). Values that fail
Fennec's grammar are rejected and reported, never stored raw.

### Export

Each kind exports to the MISP attribute type the catalogue lists for it, and
otherwise to the attribute of the MISP object template that holds it
(`tracking-id`, `ja4-plus`, `hhhash`, `cs-beacon-config`). Kinds with no MISP
counterpart at all export as a `text` attribute with the tag
`fennec:<module>`.
