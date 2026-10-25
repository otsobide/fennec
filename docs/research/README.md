# Research

Source-level studies of the data models Fennec is compatible with or learns
from: OpenCTI, MISP and STIX 2.1, plus a catalogue of the pivot identifiers
analysts use and a study of how to store all of it in PostgreSQL at scale.

These notes are **inputs**, not decisions. They record what each reference
platform does, with file and line citations, and they carry recommendations
that were weighed when the Fennec data model was designed. The binding
decisions live in [../data-model/](../data-model/README.md) and
[../architecture/](../architecture/README.md); where a note and those pages
disagree, those pages win.

## Notes

| Note | What it covers |
|---|---|
| [opencti-observables.md](opencti-observables.md) | The 37 OpenCTI observable types: every attribute, embedded reference, validation rule, standard id recipe and STIX export mapping, plus the inconsistencies found in the source. |
| [opencti-identity-and-storage.md](opencti-identity-and-storage.md) | How OpenCTI identifies, deduplicates, merges, serialises, stores (Elasticsearch) and streams (Redis) observables. |
| [misp-attribute-types.md](misp-attribute-types.md) | The 194 MISP attribute types: categories, composite types, validation and normalisation rules, correlation and the MySQL schema. |
| [misp-objects.md](misp-objects.md) | The 171 observable-relevant MISP object templates attribute by attribute, and the 243 identifier kinds they carry. |
| [stix-sco-schemas.md](stix-sco-schemas.md) | The 18 STIX 2.1 cyber-observable objects, their extensions, vocabularies and regexes, and the deterministic identifier algorithm with verified test vectors. |
| [pivot-identifiers.md](pivot-identifiers.md) | 248 pivot identifiers and fingerprints (web analytics ids, TLS and SSH fingerprints, binary hashes, telecom ids, account handles, cloud ids, ...) with validation rules and pivot value. |
| [postgres-scalability.md](postgres-scalability.md) | Column types, identity and deduplication, indexing, partitioning, the domain event log and transactional consistency on PostgreSQL with sqlx. |

## Claim markers

Each note separates what a source says from what its author inferred or
recommends. The exact legend is at the top of each note; the families are:

- **Source facts**: `[SRC]`, `[S]`, `[src]`, `[misp]`, `[SPEC]`. Quoted or
  read from code or a specification, with a citation.
- **Inferences**: `[INF]`, `[Q]`, `[obs]`. Derived from reading code that was
  not executed.
- **Recommendations**: `[REC]`, `[R]`, `REC`. Proposals for Fennec.
- **Unverified**: `[VERIFY]`, `[K]`, `[ext]`. Taken from memory or from a
  secondary source; check before relying on it.

## Reference clones

Citations point into four upstream repositories cloned at the repository
root. They are ignored by git (see `.gitignore`), so clone them yourself to
follow a citation. The commits below are the ones the notes were written
against:

```bash
git clone https://github.com/OpenCTI-Platform/opencti.git .opencti
git -C .opencti checkout 294dffb07c5ee41086493827c94848b56f592a2c

git clone --branch 2.5 https://github.com/MISP/MISP.git .misp
git -C .misp checkout fc99c9e3fc85beddc6ef18835359f3c8b7f44608

git clone https://github.com/MISP/misp-objects.git .misp-objects
git -C .misp-objects checkout a063afec447d000911d667843c7493d914116ffa

git clone https://github.com/oasis-open/cti-stix2-json-schemas.git .stix2-json-schemas
git -C .stix2-json-schemas checkout 9af1db41b7b86c06324f899649ae83480134f66e
```

The notes also describe the Fennec codebase as it was when they were written
(a single generic `ioc` module, a synchronous event bus, in-memory
persistence only). Read those parts as history.
