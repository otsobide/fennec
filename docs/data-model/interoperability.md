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
exports `{"type": "ipv4-addr", "value": "198.51.100.7"}` and `ipv4_network`
exports `{"type": "ipv4-addr", "value": "198.51.100.0/24"}`.

### Fennec-only kinds

Kinds with an `x-fennec-*` type are declared through one Fennec
`extension-definition` of `extension_type` `new-sco`:

```json
{
  "type": "extension-definition",
  "spec_version": "2.1",
  "id": "extension-definition--e36c7a52-1f1d-4040-9de6-36fb400d9b02",
  "name": "Fennec observable kinds",
  "extension_types": ["new-sco"]
}
```

Each exported object of such a kind references it:

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

The extension-definition id above is fixed for the lifetime of the project.
Its JSON schema, published with the first STIX exporter, lists every
`x-fennec-*` type and its properties.

### Import

- A STIX object whose type maps to exactly one kind is built through that
  kind's value objects. `ipv4-addr` and `ipv6-addr` map to the host or the
  network kind depending on whether the value carries a prefix shorter than
  the full width.
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
`imsi`, `iccid`, `ssh-key`, `user-agent`) round-trip one to one, and their
identifiers match OpenCTI's whenever the value arrived canonical. The
differences OpenCTI introduces for files, artifacts, certificates, processes,
software, network traffic and registry keys are listed in
[identity.md](identity.md#where-fennec-deliberately-differs-from-opencti).

The OpenCTI-specific SCO properties map as follows:

| OpenCTI property | Fennec |
|---|---|
| `x_opencti_score`, `x_opencti_description`, labels, markings, `createdBy`, external references | Assessments; not part of the observable |
| `x_opencti_stix_ids` | Alias identifiers, recorded by the importer |
| `x_opencti_files` | Not imported |

### Kinds without an OpenCTI type

OpenCTI has no typed home for web tracking ids, TLS and SSH fingerprints,
binary structural hashes, platform account ids, application ids or cloud
ids. They export to OpenCTI as a `text` observable whose `value` is the
canonical value and whose label is `fennec:<module>` (for example
`fennec:ja4_fingerprint`), so a later import can recover the kind. Because
STIX does not hash the type, the UUID part of the OpenCTI `text--<uuid>`
equals the UUID part of the Fennec identifier.

An OpenCTI `text` observable without a `fennec:<module>` label is not
imported: it carries no kind.

## MISP

### Import

| MISP construct | Fennec |
|---|---|
| Attribute of a typed kind (`ip-dst`, `domain`, `sha256`, `ja3-fingerprint-md5`, `jarm-fingerprint`, `btc`, ...) | The matching kind; role variants (`ip-src` and `ip-dst`, `email-src` and `email-dst`) collapse into one kind |
| `ip-src` and `ip-dst` with a CIDR value | The network kind; `/32` and `/128` fold to the host kind |
| Composite attribute (`domain\|ip`, `ip-dst\|port`, `filename\|sha256`, `malware-sample`) | Split into the observables it contains plus the relationship between them |
| Object template (`file`, `x509`, `tracking-id`, `ja4-plus`, `phone`, ...) | One observable per typed attribute, plus relationships; `tracking-id` dispatches on its `tracker` attribute to the right web tracking kind |
| `to_ids`, `category`, `comment`, tags, galaxies | Assessments and context; not part of the observable |
| Sightings | Fennec sightings of the imported observables |
| Correlations | Not imported: shared values are already the same aggregate |
| `text`, `other`, `comment` and the other catch-all types | Not imported; reported as untyped |

MISP validates many types loosely (URLs, cryptocurrency addresses and bank
identifiers only reject newlines or non-alphanumerics, and four-byte UTF-8
characters were replaced by `?` in older MySQL schemas). Values that fail
Fennec's grammar are rejected and reported, never stored raw.

### Export

Each kind exports to its MISP attribute type when one exists (the catalogue
lists them), and otherwise to the attribute of the MISP object template that
holds it (`tracking-id`, `ja4-plus`, `hhhash`, `cs-beacon-config`). Kinds
with no MISP counterpart at all export as a `text` attribute with the tag
`fennec:<module>`.
