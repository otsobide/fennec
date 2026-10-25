# Pivot identifiers and fingerprints: research notes for Fennec

Task: catalogue the pivot identifiers and fingerprints (beyond IP, domain, URL and file hash) that CTI analysts use to link infrastructure, so that Fennec can model each one as its own strongly typed observable (aggregate + validated value objects). These notes separate **source facts** (what the MISP, OpenCTI and STIX clones and the cited web sources say) from **recommendations** (marked `REC`).

Date of research: 2026-10-25. Clones are read-only references inside the Fennec repository.

---

## 0. Scope, sources and conventions

### 0.1 Local sources (clone root in parentheses)

| Short name | Clone root | Files used |
|---|---|---|
| MISP core | `.misp` | `describeTypes.json`, `app/Lib/Tools/AttributeValidationTool.php`, `app/Lib/Tools/ComplexTypeTool.php`, `app/Model/MispAttribute.php`, `app/Test/AttributeValidationToolTest.php` |
| MISP objects | `.misp-objects` | `objects/<name>/definition.json` (ja3, ja3s, ja4-plus, jarm, hhhash, favicon, dom-hash, tracking-id, cookie, cs-beacon-config, muonfp, passive-ssh, x509, pe, file, phone, phone-number, imsi-catcher, device, wifi-connection, android-app, apk, browser-extension, telegram-account, telegram-bot, twitter-account, github-user, github-repo, gitlab-user, pgp-key, pgp-meta, ssh-authorized-keys, coin-address, network-profile, cert-pl-phishing, rmm, credential, chat, ...) |
| OpenCTI | `.opencti` | `opencti-platform/opencti-graphql/src/schema/stixCyberObservable.ts`, `.../modules/attributes/stixCyberObservable-registrationAttributes.ts`, `.../utils/syntax.js`, `.../utils/observable.ts`, `.../schema/identifier.js`, `.../database/stix.ts`, `.../modules/vocabulary/vocabulary-utils.ts`, `.../schema/fieldDataAdapter.ts`, `client-python/pycti/utils/constants.py`, `client-python/pycti/utils/opencti_stix2_utils.py`, `opencti-platform/opencti-front/src/private/components/observations/stix_cyber_observables/StixCyberObservableCreation.jsx` |
| STIX 2.1 schemas | `.stix2-json-schemas` | `schemas/common/hashes-type.json`, `schemas/observables/mac-addr.json`, `schemas/observables/x509-certificate.json`, `schemas/observables/user-account.json`, `schemas/observables/file.json` |

External sources (web, fetched 2026-10-25) are listed in section 23 and referenced inline as `[W:<key>]`.

### 0.2 Conventions used in the tables

- **Regex dialect.** Validation regexes are written for the Rust `regex` crate: anchored full match (`^...$`), no look-around, no back-references. ASCII classes such as `[0-9]` are used on purpose, because in the Rust `regex` crate `\d` is Unicode-aware and would accept non-ASCII digits. Rules that need look-around (for example "no two consecutive periods") are stated in prose and must be implemented in code.
- **Pipes inside tables.** In table cells a literal regex alternation pipe is written `\|` (GitHub Flavoured Markdown escape). Long regexes are given in fenced blocks instead.
- **Evidence level** for a syntax: `Doc` = documented by the owner/standard; `Src` = taken from source code of a cited tool (MISP, OpenCTI, gitleaks, TruffleHog, FoxIO, Mastodon, etc.); `Obs` = observed in the wild or in third-party guides, vendor does not publish a format. `Obs` regexes are deliberately lenient and must not be used to reject data on ingest without an override path (`REC`).
- **Pivot value** scale: `H` = strong attribution or ownership linkage, rarely shared by unrelated parties; `M` = useful clustering signal, needs corroboration; `L` = weak or very common; `X` = not an identifier (measurement or transient), should not be an observable.
- **MISP column** codes: `type:x` = dedicated MISP attribute type `x`; `obj:o.a(t)` = attribute `a` of object template `o` stored with MISP type `t`; `text` = only free text; `none` = not modelled.
- **OpenCTI column** codes: `SCO:X` = dedicated OpenCTI observable type; `X.prop` = property of an observable; `Text` = can only be stored as the generic `Text` observable (fallback, see 1.3); `none`.

### 0.3 Family index (248 catalogue entries)

| Section | Family | IDs | Entries |
|---|---|---|---|
| 2 | Web analytics, advertising, tag and monitoring identifiers | A01-A38 | 38 |
| 3 | Site ownership verification tokens | B01-B09 | 9 |
| 4 | Embedded third-party service keys (captcha, maps, chat widgets, payments) | C01-C13 | 13 |
| 5 | Messaging, webhook and exfiltration endpoints | D01-D14 | 14 |
| 6 | Crypto-mining keys, cryptocurrency addresses, financial identifiers | E01-E17 | 17 |
| 7 | TLS and X.509 fingerprints (JA3, JA3S, JA4, JA4S, JA4X, JA4L, JARM, certificate fingerprints, serial, SPKI, SKI/AKI) | F01-F17 | 17 |
| 8 | SSH (HASSH, HASSHServer, JA4SSH, host key fingerprints, public keys, banner) | G01-G08 | 8 |
| 9 | HTTP and web content fingerprints (JA4H, HHHash, favicon mmh3, Shodan hashes, DOM hashes, ETag, Community ID) | H01-H14 | 14 |
| 10 | TCP, OS, DHCP, NTP fingerprints (JA4T, JA4TS, JA4TScan, MuonFP, p0f, JA4D, JA4D6, JA4N) | I01-I08 | 8 |
| 11 | Binary similarity and structural hashes (imphash, impfuzzy, pehash, authentihash, vhash, Rich, TLSH, ssdeep, sdhash, LZJD, telfhash, permhash, dHash, CDHash) | J01-J14 | 14 |
| 12 | Build and code-signing provenance (PDB, .NET GUIDs, TypeRefHash, build IDs, Team ID, signing certs) | K01-K11 | 11 |
| 13 | C2 framework and remote-access identifiers (Cobalt Strike watermark and public key, AnyDesk, TeamViewer, RMM) | L01-L06 | 6 |
| 14 | Cryptographic identity keys and messenger key IDs (PGP, Tox, Session) | M01-M08 | 8 |
| 15 | Telecom and device identifiers (IMEI, IMEISV, MEID, ESN, IMSI, ICCID, EID, E.164, MAC, EUI-64, Bluetooth, Wi-Fi) | N01-N17 | 17 |
| 16 | Account handles and platform IDs (Telegram, Discord, GitHub, GitLab, X, Mastodon, other social and messaging) | O01-O26 | 26 |
| 17 | Mobile apps, browser extensions, software packages | P01-P11 | 11 |
| 18 | Cloud, SaaS tenant and credential identifiers (Entra, AWS, GCP, prefixed tokens) | Q01-Q10 | 10 |
| 19 | Hosting, dead-drop and anonymity-network identifiers | R01-R07 | 7 |
| 20 | Checksum, decoding and derivation algorithms | | |
| 21 | Recommendations for Fennec (`REC`) | | |
| 22 | Open questions | | |
| 23 | External sources | | |

---

## 1. What the reference platforms do today (source facts)

### 1.1 MISP attribute types that are pivot identifiers

All line numbers: `describeTypes.json` (sane_defaults block) and `app/Lib/Tools/AttributeValidationTool.php` (AVT).

| MISP type | describeTypes line | Default category / to_ids | Normalisation (AVT `modifyBeforeValidation`) | Validation (AVT `validate`) |
|---|---|---|---|---|
| `ja3-fingerprint-md5` | 104 | Network activity / 1 | lowercase (l.86, 107) | 32 hex (l.15, 277, 286) |
| `jarm-fingerprint` | 108 | Network activity / 1 | lowercase (l.87) | 62 hex (l.16, 278) |
| `favicon-mmh3` | 112 | Network activity / 1 | none | any single line (l.578, 582-585) |
| `hassh-md5` | 116 | Network activity / 1 | lowercase (l.88) | 32 hex (l.17, 279) |
| `hasshserver-md5` | 120 | Network activity / 1 | lowercase (l.89) | 32 hex (l.18, 280) |
| `community-id` | 152 | Network activity / 1 | none | anything (l.484, 513) |
| `dom-hash` | 156 | Network activity / 1 | lowercase (l.104) | 32 hex (l.30, 285) |
| `pgp-public-key` | 176 | Person / 0 | none | anything (l.490) |
| `ssh-fingerprint` | 184 | Network activity / 0 | none | `SHA256:` + base64 of 32 bytes, or optional `MD5:` + 16 colon-hex bytes (l.586-590, 745-757) |
| `cookie` | 216 | Network activity / 0 | none | anything (l.499) |
| `btc`, `dash`, `xmr` | 296, 300, 304 | Financial fraud / 1 | none | `^[a-zA-Z0-9]+$` only, no checksum (l.614-619) |
| `iban`, `bic` | 308, 312 | Financial fraud / 1 | uppercase, strip non `[0-9A-Z]` (l.171-174) | `^[a-zA-Z0-9]+$` only, no mod-97 check (l.614-619) |
| `phone-number` | 336 | Person / 0 | `00` prefix to `+`, drop `(0)`, strip non `[+0-9]` (l.176-182) | PHP `is_numeric` (l.627-630) |
| `authentihash` | 360 | Payload delivery / 1 | lowercase (l.92) | 64 hex (l.6, 276) |
| `vhash` | 364 | Payload delivery / 1 | lowercase (l.93) | `^.+$` (l.620-621) |
| `ssdeep` | 368 | Payload delivery / 1 | none | `^([0-9]+):([0-9a-zA-Z/+]*):([0-9a-zA-Z/+]*)$` (l.736-739) |
| `imphash` | 372 | Payload delivery / 1 | lowercase (l.94) | 32 hex (l.8, 264) |
| `telfhash` | 376 | Payload delivery / 1 | lowercase (l.95) | length 70 or 72 and all hex (l.296-300, 706-709) |
| `pehash` | 380 | Payload delivery / 1 | lowercase (l.91) | 40 hex, "MISP currently only supports SHA1 for PEhashes" (l.19, 301-305) |
| `impfuzzy` | 384 | Payload delivery / 1 | none | `imports:hash:hash`, first part integer (l.311-318) |
| `tlsh` | 424 | Payload delivery / 1 | lowercase (l.96) | optional leading `t` stripped, then more than 35 chars, all hex (l.291-295, 694-700) |
| `cdhash` | 428 | Payload delivery / 1 | lowercase (l.98) | `^[0-9a-f]{40,}$` (l.319-323) |
| `x509-fingerprint-sha1` / `-md5` / `-sha256` | 532 / 536 / 540 | Network activity / 1 | strip `:` then lowercase (l.183-187) | 40 / 32 / 64 hex (l.12-14) |
| `mac-address` | 580 | Network activity / 0 | lowercase, strip `.` `:` `-` space, re-insert `:` every 2 chars (l.217-220) | `^([a-f0-9]{2}:){5}[a-f0-9]{2}$` (l.421-422) |
| `mac-eui-64` | 584 | Network activity / 0 | same as mac-address (l.218-220) | `^([a-f0-9]{2}:){3}ff:fe(:[a-f0-9]{2}){3}$` (l.423-424): only MAC-derived EUI-64 accepted |
| `github-username` / `github-repository` / `github-organisation` | 620 / 624 / 628 | Social network / 0 | none | single line only (l.572-574, 582-585) |
| `jabber-id` | 632 | Social network / 0 | none | email-like `^.[^\s]*\@.*\..*$` (l.456-461) |
| `twitter-id` | 636 | Social network / 0 | none | single line only (l.575) |
| `mobile-application-id` | 744 | Payload delivery / 1 | none | single line only (l.579) |
| `azure-application-id` | 748 | Payload delivery / 1 | none | single line only, no GUID check (l.580) |
| `chrome-extension-id` / `edge-extension-id` | 752 / 756 | Payload delivery / 1 | none | `^[a-p]{32}$` (l.642-645) |
| `onion-address` | 776 | Network activity / 1 | lowercase (l.105) | `^([a-z2-7]{16}\|[a-z2-7]{56})\.onion$` (l.416-420), no v3 checksum |
| `pdb` | 20 | Artifacts dropped | none | single line (l.539) |

Descriptions that define an algorithm (`app/Model/MispAttribute.php`):

- `dom-hash` (l.4270): "a structural fingerprint of an HTML Document Object Model where all tag names are contained in a single string separated by a pipe. The truncated SHA252 [sic] value by the first 32-character serves as fingerprint."
- `favicon-mmh3` (l.4259): "the murmur3 hash of a favicon as used in Shodan."
- `cdhash` (l.4338): "An Apple Code Directory Hash, identifying a code-signed Mach-O executable file".
- `community-id` (l.4269), `telfhash` (l.4325), `pehash` (l.4326), JA3/JARM/HASSH (l.4257-4261).

MISP freetext import (`app/Lib/Tools/ComplexTypeTool.php`):

- 32 hex is ambiguous by design: `HEX_HASH_TYPES[32]` = `md5`, `imphash`, `x509-fingerprint-md5`, `ja3-fingerprint-md5` (l.35); 40 hex = `sha1`, `pehash`, `x509-fingerprint-sha1`, `cdhash` (l.36); 64 hex = `sha256`, `authentihash`, `sha512/256`, `x509-fingerprint-sha256` (l.38).
- Bitcoin detector regex (l.376): `#^([13][a-km-zA-HJ-NP-Z1-9]{25,34})|(bc|tb)1([023456789acdefghjklmnpqrstuvwxyz]{11,71})$#i`. Observation: operator precedence makes the first alternative anchored only at the start and the second only at the end.
- Browser extension detector (l.388): `/^[a-p]{32}$/` mapped to `chrome-extension-id` only.
- Phone detector (l.500-502): needs `+` or `-`, regex `#^(\+)?([0-9]{1,3}(\(0\))?)?[0-9\/\-]{5,}[0-9]$#i`, excluding dates.

### 1.2 MISP object templates relevant to pivots (`.misp-objects/objects/<name>/definition.json`)

| Object (version) | Key pivot attributes (MISP type) | Required | Notes |
|---|---|---|---|
| `ja3` (v4) | `ja3-fingerprint-md5` (ja3-fingerprint-md5) l.24 | `ja3-fingerprint-md5` | plus ip-src/ip-dst, first/last-seen |
| `ja3s` (v5) | `ja3-fingerprint-md5` l.34 and `ja3s-fingerprint-md5` stored as MISP type `ja3-fingerprint-md5` l.39-43 | both | MISP has no distinct JA3S attribute type |
| `ja4-plus` (v2) | `ja4-fingerprint` (text) l.14-18, `ja4-type` (text) with sane_default `JA4, JA4S, JA4H, JA4L, JA4X, JA4SSH, JA4T, JA4TS, JA4TScan` l.19-33 | both | no JA4LS, JA4D, JA4D6, JA4N |
| `jarm` (v1) | `jarm` (jarm-fingerprint) l.3-7; `scope` values `Malicious - C2`, `Malicious - Client`, `Malicious - Unknown`, `Legitimate`, `Undefined` l.14-26 | oneOf `jarm` | |
| `hhhash` (v2) | `hhhash` (text) "in format hhh:version:hash_value" l.8-12; `hhhash-headers`, `hhhash-query-headers`, `hhhash-tool` l.13-39 | oneOf `hhhash` | crawler parameters are part of the object |
| `favicon` (v1) | `favicon` (attachment), `favicon-mmh3` (favicon-mmh3) l.8-12, `link` | oneOf any | |
| `dom-hash` (v4) | `dom-hash` (dom-hash) l.3-7, `url` | oneOf url/dom-hash | |
| `tracking-id` (v3) | `id` (text) l.21-25; `tracker` (text) sane_default `Google Analytics, Piwik, Kissmetrics, Woopra, Chartbeat` l.32-43; `hostname`, `url` | oneOf `id` | the only MISP home for web analytics IDs; untyped |
| `network-profile` (v6) | `google-analytics-id` (text) l.80; `jarm` (jarm-fingerprint) l.96 | | |
| `cookie` (v6) | `cookie` (cookie), `cookie-name`, `cookie-value` (text); `type` includes `Tracking`, `Exfiltration`, `Beaconing` l.56-68 | all three | |
| `cs-beacon-config` (v7) | `watermark` (text) l.182, `license-id` (text) l.131, `jar-md5`, `config-md5/sha1/sha256`, `c2` (url) | oneOf `jar-md5, md5, sha1, sha256, watermark` | no public-key attribute |
| `muonfp` (v1) | `muonfp-fingerprint` (text) "TCPWindowSize:TCPOptions:TCPMSS:TCPWindowScale (e.g. 26847:2-4-8-1-3:1460:8)" l.32-36 | oneOf | |
| `passive-ssh` (v2) | `fingerprint` (ssh-fingerprint) l.20-26, `hassh` (hassh-md5) l.33-37, `banner` (text) l.3-12, `base64` key | oneOf host/base64/fingerprint/hassh | |
| `ssh-authorized-keys` (v1) | `key` (text), `key-id`, `full-line` | oneOf | |
| `x509` (v14) | `x509-fingerprint-*`, `serial-number` (text) l.211, `subject-key-id` l.267, `issuer-key-id` l.90, `pubkey-info-modulus` (hex, no 0x, no :) l.179 | oneOf fingerprints/serial/issuer | |
| `pe` (v11) | `imphash` l.78, `impfuzzy` l.73, `pehash` l.161, `authentihash` l.3, `richpe` (md5) "RichPE metadata hash" l.184, `pdb` l.155 | | |
| `file` | `telfhash` l.471, `tlsh` l.484, `vhash` l.489, `ssdeep` l.451, `dom-hash` l.34 | | |
| `cert-pl-phishing` (v1) | `favicon-mmh3` (text) "in Murmurhash3 format (base64)" l.3-6; `phash-dct-base64` (text) l.14; `html-structure`, `truncated-hash-html-structure` | | representation differs from `favicon` object |
| `pgp-key` (v1) | `key-id` (text) "Hexadecimal key ID without 0x. Short IDs can collide" l.45; `fingerprint` (text) "Full OpenPGP fingerprint in uppercase hexadecimal, without 0x or spaces." l.24-25 | `key-id` | |
| `phone` (v6) | `imei` l.30, `imsi` l.36, `iccid` l.25, `msisdn` l.53, `tmsi` l.69, `guti` l.20 (all text) | oneOf | |
| `phone-number` (v1) | `phone-number` (phone-number) "E.164 format (e.g., +14155552671)" l.27-30 plus country-code, NDC, subscriber-number | `phone-number` | |
| `imsi-catcher` (v1) | `imsi`, `mcc`, `mnc`, `lac`, `cellid`, `tmsi-1/2` (text) | | |
| `device` (v10) | `MAC-address` (mac-address) l.3 | | |
| `wifi-connection` (v2) | `bssid` (mac-address) l.21, `ssid` (text) l.139 | oneOf ssid/bssid | |
| `android-app` (v2) | `appid` (text) l.3, `certificate` (sha1), `certificate-sha256` (sha256) l.15 | oneOf | |
| `apk` (v2) | `package-name` (text) l.47, `vhash`, `tlsh`, `ssdeep` | | |
| `browser-extension` (v1) | `id` (text) "Chrome Web Store item ID or Firefox AMO slug" l.70; `ecosystem` `chrome/firefox/edge` l.42; `crx-sha256` | oneOf `id` | |
| `telegram-account` (v2) | `id`, `username`, `phone` (text) | oneOf | |
| `telegram-bot` (v2) | `token` (text) l.19, `chat-id` (text) l.3, `username` (text) "must end with bot" l.24-27 | oneOf | |
| `twitter-account` (v7) | `id` "Numeric account id" l.62, `name` "screen name (without the @)" l.103 | oneOf | |
| `github-user` (v3) | `username` (github-username) l.103, `id` l.35, `node_id` l.52, `ssh-public-key` l.87 | oneOf | |
| `github-repo` (v2) | `full-name` "Username/Repository name" l.48, `id` l.73 | oneOf | |
| `gitlab-user` (v1) | `id` l.8, `username` l.29 | oneOf `username` | |
| `coin-address` (v7) | `address` (btc) l.3, `address-xmr` (xmr) l.13, `address-crypto` (text) l.8, `symbol` | oneOf | |
| `rmm` (v4) | `rmm-type` (AnyDesk, Atera, ... ), `guid`, `api-key`, `hostname`, `url` | oneOf | |
| `chat` (v1) | `id` (text) "scoped to its service", `protocol` (telegram, matrix, discord, ...) | `id` | |
| `instant-message`, `instant-message-group`, `facebook-account`, `instagram-account`, `reddit-account`, `youtube-channel`, `keybase-account`, `google-account` | account ids and names, all `text` | | untyped platform ids |

### 1.3 OpenCTI observable model (source facts)

- Observable types (`opencti-platform/opencti-graphql/src/schema/stixCyberObservable.ts` l.4-40): STIX core types plus custom `Cryptographic-Key`, `Cryptocurrency-Wallet`, `Hostname`, `Text`, `Credential`, `User-Agent`, `Bank-Account`, `Tracking-Number`, `Phone-Number`, `Payment-Card`, `Media-Content`, `Persona`, `SSH-Key`, `AI-Prompt`, `IMEI`, `ICCID`, `IMSI`.
- **There is no OpenCTI type for JA3, JA3S, JA4+, JARM, HASSH, favicon hash, DOM hash, HHHash, imphash, any web tracking ID, any messaging handle, any extension or app ID, any cloud ID.** A grep of `.opencti` for `imphash|ja3|jarm|hassh|telfhash|authentihash|vhash` finds no model code. The automatic type detector returns `Text` when nothing matches (`.../utils/observable.ts` l.37-46 and the final `return ENTITY_TEXT`).
- `Tracking-Number` is **parcel** tracking, not web tracking: detector regexes are UPS `^1Z[0-9A-Z]{16}$`, USPS `^[0-9]{20,22}$`, FedEx `^[0-9]{12,14}$`, international post `^[A-Z]{2}[0-9]{9}[A-Z]{2}$` (`.../utils/observable.ts` l.246-256). Do not map analytics IDs to it.
- Single-`value` custom observables (registration in `.../modules/attributes/stixCyberObservable-registrationAttributes.ts`): `Cryptographic-Key` l.266-268, `Cryptocurrency-Wallet` l.269-271, `Text` l.275-277, `Phone-Number` l.286-288, `Credential` l.302-304, `Tracking-Number` l.305-307, `IMEI` l.325-327, `ICCID` l.328-330, `IMSI` l.331-333. Identity (deterministic id) is computed from `value` (`.../schema/identifier.js` l.125-142).
- `SSH-Key`: `key_type` (vocabulary `key_type_ov` = rsa, ecdsa, ed25519, dsa; `.../modules/vocabulary/vocabulary-utils.ts` l.1212-1217), `public_key`, `fingerprint_sha256` (mandatory external), `fingerprint_md5`, `key_length`, `comment`, `created`, `expiration_date` (registration l.312-321); identity = `fingerprint_sha256` (`identifier.js` l.137).
- `X509-Certificate`: `hashes`, `serial_number` (string), `subject`, `issuer`, `subject_public_key_modulus`, `subject_key_identifier`, `authority_key_identifier`, etc. (registration l.134-163); identity = hashes, else serial_number, else subject (`identifier.js` l.105).
- `Mac-Addr`: `value` (registration l.170-172).
- `User-Account`: `user_id`, `account_login`, `account_type` (vocabulary `account_type_ov`: facebook, ldap, nis, openid, radius, skype, tacacs, twitter, unix, windows-local, windows-domain; `vocabulary-utils.ts` l.14-26), identity = account_type + user_id + account_login (`identifier.js` l.120).
- `hashes` dictionary keys supported: MD5, SHA-1, SHA-256, SHA-512, SHA3-256, SHA3-512, SSDEEP, SDHASH, TLSH, LZJD (registration l.54-77); fuzzy algorithms `['SSDEEP', 'SDHASH', 'TLSH', 'LZJD']` (`.../schema/fieldDataAdapter.ts` l.13).
- Built-in telecom relationships (`.../database/stix.ts` l.1102-1142): ICCID uses IMEI, ICCID has IMSI, ICCID resolves-to Phone-Number, IMEI uses IMSI, IMEI has Mac-Addr, IMSI uses Phone-Number, Phone-Number uses IMSI.
- Server-side syntax checks (`.../utils/syntax.js` l.162-233): `macAddrChecker = /^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$/` (l.167), `imeiChecker = /(^[0-9]{15,16})$/` (l.171), `iccidChecker = /(^[0-9]{18,22})$/` (l.172), `imsiChecker = /(^[0-9]{14,15})$/` (l.173). Same rules in the UI (`StixCyberObservableCreation.jsx` l.605-629). pycti docstrings: IMEI "14 digits + 1 check digit ... (can be 16 for legacy digits total)", ICCID "up to 18-20 digits", IMSI "usually 15 digits (can be 14-15) ... MCC+MNC+MSIN" (`client-python/pycti/utils/constants.py` l.810-826, 844-859, 877-892). No checksum is verified anywhere.
- Detector patterns worth knowing (`.../utils/observable.ts`): MAC also accepts Cisco dotted `^([0-9A-Fa-f]{4}\.){2}[0-9A-Fa-f]{4}$` (l.71); Phone includes E.164 `^\+?[1-9]\d{1,14}$` (l.96); Cryptocurrency-Wallet only BTC base58, ETH `^0x[a-fA-F0-9]{40}$`, LTC, BTC bech32 (l.159-166); Cryptographic-Key matches PEM headers and `ssh-(rsa|dss|ed25519|ecdsa)` lines (l.151-156).

### 1.4 STIX 2.1 JSON schema constraints

- `schemas/common/hashes-type.json` l.9-52: custom hash keys allowed by `^[a-zA-Z0-9_-]{3,250}$`; `SSDEEP` pattern `^[a-zA-Z0-9/+:.]{1,128}$` (l.46); `TLSH` pattern `^[a-zA-Z0-9]{70}$` (l.51). The schema therefore rejects the current 72-character `T1`-prefixed TLSH digest.
- `schemas/observables/mac-addr.json` l.26: `^([0-9a-f]{2}[:]){5}([0-9a-f]{2})$` (lowercase, colon only). EUI-64 is not representable.
- `schemas/observables/x509-certificate.json` l.36-39: `serial_number` is a free string.
- `schemas/observables/user-account.json` l.28-43: `user_id`, `account_login`, `account_type` (open vocabulary).
- `schemas/observables/file.json` l.266-268: `imphash` exists only inside `windows-pebinary-ext`.

### 1.5 Source-level observations that matter for Fennec

1. MISP `ja3s` object stores the JA3S hash with MISP type `ja3-fingerprint-md5` (`.misp-objects/objects/ja3s/definition.json` l.39-43): at attribute level JA3 and JA3S are indistinguishable.
2. MISP `telfhash` validation requires all 70 or 72 characters to be hex (AVT l.706-709) after lowercasing; a `t1`-prefixed 72-character value would fail `ctype_xdigit`. Telfhash README examples are 70 hex without prefix [W:telfhash].
3. MISP `tlsh` strips only a leading `t` (AVT l.696-698), so `T1...` (uppercase, before lowercasing at l.96 it becomes `t1`) passes. STIX schema rejects it (1.4).
4. MISP `azure-application-id`, `twitter-id`, `github-*`, `mobile-application-id`, `favicon-mmh3` have no syntax validation (AVT l.572-585).
5. MISP `mac-eui-64` only accepts the `ff:fe`-infixed, MAC-derived EUI-64 (AVT l.424).
6. MISP `iban`/`btc`/`xmr` validation is `^[a-zA-Z0-9]+$` (AVT l.619): no mod-97, base58check or bech32 verification.
7. Two incompatible favicon-mmh3 representations in MISP objects: `favicon` uses MISP type `favicon-mmh3` (Shodan integer), `cert-pl-phishing` says "Murmurhash3 format (base64)" as text.
8. OpenCTI `IMEI` accepts 15 or 16 digits, so IMEI and IMEISV collide in one type; no Luhn check (`syntax.js` l.171).
9. MISP `dom-hash` (32 hex, truncated SHA-256 of tag names) and urlscan `dom.hash` (full SHA-256 of the DOM) share a name but are different algorithms [W:urlscan-search].
10. Numeric IDs (Meta pixel, Facebook app, Yandex counter, Hotjar, LinkedIn, Pinterest, Discord snowflake, Telegram id, X id, AnyDesk, Cobalt Strike watermark) and 32-hex IDs (MD5, JA3, HASSH, imphash, Mixpanel, Amplitude, Baidu, Cloudflare token, Sentry key) are not distinguishable by syntax. The type can only come from extraction context (`REC`: never infer the type from the value alone).

---

## 2. Family A: web analytics, advertising, tag and monitoring identifiers

Common facts for the family:

- Extracted from HTML source, inline JavaScript, `<script src>` query strings, `<noscript>` image beacons, tag-manager container files (GTM `gtm.js?id=` returns JS that embeds further IDs, a second-order pivot), `ads.txt` / `app-ads.txt`, and from HAR captures (beacon requests carry the ID in query parameters such as `tid=`, `ti=`, `id=`).
- Reverse-lookup services pivot on them: DomainTools Iris Investigate exposes search parameters `google_analytics`, `ga4`, `google_tag_manager`, `adsense`, `facebook`, `hotjar`, `matomo`, `baidu_analytics`, `statcounter_project`, `statcounter_security`, `yandex_metrica` [W:dt-params], and started capturing GA4, GTM, Baidu, Facebook, Hotjar, Matomo, Statcounter and Yandex in September 2023 [W:dt-fall2023]. Bellingcat documents the method (Google Analytics, AdSense, Amazon and AddThis IDs; SameID and SpyOnWeb lookups) [W:bellingcat-2015] and its 2017 case where the `-4` suffix of a UA ID revealed a sequence of properties under the same account [W:bellingcat-2017]. HackerTarget offers reverse analytics for `UA-` and `pub-` IDs and warns that IDs can be planted on unrelated sites [W:hackertarget]. urlscan.io indexes "content.identifiers" ("Generic field for string identifiers") on paid plans [W:urlscan-search].
- Caveat for every row: anyone can copy a public ID into a page (false flag or template reuse); site builders and themes ship with the template author's ID. Pivot value assumes the ID was set by the operator.
- MISP: only `tracking-id` object (`id` text + `tracker` text) and `network-profile.google-analytics-id` (text). OpenCTI: `Text` only (and not `Tracking-Number`, see 1.3).

### 2.1 Syntax

| ID | Identifier | Validation regex (canonical) | Case and canonical form | Length | Checksum | Evidence |
|---|---|---|---|---|---|---|
| A01 | Google Analytics Universal tracking ID | `^UA-[0-9]{4,10}-[0-9]{1,4}$` | Prefix uppercase `UA-`; digits as-is (no leading-zero stripping) | 8-18 | none | Doc ("UA-XXXXXXXX", Google [W:ga-ua]); digit bounds Obs |
| A02 | Google Analytics account (derived from A01) | `^UA-[0-9]{4,10}$` | A01 without the trailing `-N` property index | 7-13 | none | Obs (Bellingcat 2017 [W:bellingcat-2017]) |
| A03 | GA4 measurement ID (web data stream) | `^G-[A-Z0-9]{6,12}$` | uppercase | 8-14 (typ. `G-` + 10) | none | Doc prefix [W:google-tag-ids]; length Obs |
| A04 | Google tag ID | `^GT-[A-Z0-9]{6,12}$` | uppercase | | none | Doc prefix (Google tag, may route to several products) [W:google-tag-ids]; length Obs |
| A05 | Google Tag Manager container | `^GTM-[A-Z0-9]{4,10}$` | uppercase | | none | Doc (`GTM-XXXXXXX`) [W:google-tag-ids] |
| A06 | Google Ads conversion ID | `^AW-[0-9]{6,12}$` | uppercase prefix; the conversion label after `/` (e.g. `AW-123456789/AbC_dEf`) is a separate attribute, not identity | | none | Doc (`AW-98765` example) [W:google-tag-ids] |
| A07 | Floodlight / Campaign Manager advertiser | `^DC-[0-9]{6,10}$` | uppercase | | none | Doc prefix [W:google-tag-ids]; length Obs |
| A08 | Google Optimize container | `^OPT-[A-Z0-9]{6,8}$` | uppercase | | none | Obs; product sunset 30 Sep 2023 (historical pivot only) |
| A09 | Google AdSense publisher ID | `^pub-[0-9]{16}$`; accept input `^(?:ca-\|partner-)?pub-[0-9]{16}$` | lowercase `pub-` + 16 digits; strip `ca-` / `partner-` | 20 | none | Doc ("pub-1234567890123456") [W:adsense] |
| A10 | Google AdMob app ID / ad unit ID | `^ca-app-pub-[0-9]{16}~[0-9]{10}$` / `^ca-app-pub-[0-9]{16}/[0-9]{10}$` | as-is; publisher part derives A09 | 38 | none | Obs |
| A11 | Meta (Facebook) Pixel ID | `^[0-9]{15,16}$` | decimal string | 15-16 | none | Obs (`fbq('init', 'ID')`) [W:apify-pixel] |
| A12 | Facebook App ID | `^[0-9]{14,17}$` | decimal string | | none | Obs (`<meta property="fb:app_id">`, `FB.init({appId})`) |
| A13 | Yandex Metrica counter (tag) ID | `^[0-9]{5,10}$` | decimal | | none | Doc numeric (`ym(XXXXXX, "init", {...})`) [W:yandex-metrica]; length Obs |
| A14 | Microsoft Clarity project ID | `^[a-z0-9]{10}$` | lowercase | 10 | none | Obs (`clarity.ms/tag/<id>`, example `55u86t0118`) [W:clarity] |
| A15 | Microsoft Advertising UET tag ID | `^[0-9]{6,10}$` | decimal | | none | Doc (`ti` value, `bat.bing.com/action/0?ti=`) [W:uet]; length Obs |
| A16 | Hotjar site ID (`hjid`) | `^[0-9]{5,9}$` | decimal | | none | Doc (hjid = site ID) [W:hotjar]; length Obs |
| A17 | Matomo / Piwik site | composite: tracker base URL (normalised URL observable) + `^[0-9]{1,9}$` | site id meaningless without tracker URL (id 1 exists on every instance) | | none | Obs (`_paq.push(['setSiteId', 'N'])`, `setTrackerUrl`, `matomo.php` / `piwik.php`) |
| A18 | Baidu Tongji (Analytics) site key | `^[0-9a-f]{32}$` | lowercase | 32 | none | Obs (`hm.baidu.com/hm.js?<key>`) [W:baidu] |
| A19 | TikTok Pixel ID | `^[A-Z0-9]{20}$` | uppercase | 20 | none | Obs; vendor documents no format and third-party examples vary (19-21) [W:tiktok] |
| A20 | LinkedIn Insight partner ID | `^[0-9]{4,9}$` | decimal | | none | Obs (`_linkedin_partner_id = "N"`, `px.ads.linkedin.com/collect/?pid=`) |
| A21 | X (Twitter) website tag / pixel ID | `^[a-z0-9]{5,6}$` | lowercase | 5-6 | none | Obs (`twq('config','o1abc')`) [W:apify-pixel] |
| A22 | Pinterest tag ID | `^[0-9]{10,17}$` | decimal | | none | Obs (`pintrk('load', 'N')`) |
| A23 | Snap Pixel ID | `^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$` | lowercase UUID | 36 | none | Obs (`snaptr('init', '<uuid>')`) |
| A24 | Reddit Pixel ID | `^(?:t2\|a2)_[a-z0-9]{4,12}$` | lowercase | | none | Obs (`rdt('init','t2_...')`) |
| A25 | Quantcast publisher ID | `^p-[A-Za-z0-9_-]{13}$` | as-is | 15 | none | Obs |
| A26 | VK (VKontakte) retargeting pixel | `^VK-RTRG-[0-9]+-[A-Za-z0-9]+$` | as-is | | none | Obs |
| A27 | Statcounter project + security code | `^[0-9]{6,9}$` and `^[0-9a-f]{8}$` (pair) | decimal / lowercase hex | | none | Doc (two DomainTools parameters) [W:dt-params]; lengths Obs |
| A28 | HubSpot portal (hub) ID | `^[0-9]{5,9}$` | decimal | | none | Obs (`js.hs-scripts.com/<id>.js`, `portalId`) |
| A29 | Mixpanel project token | `^[0-9a-f]{32}$` | lowercase | 32 | none | Obs; TruffleHog uses `[a-zA-Z0-9-]{32}` [W:th-mixpanel] |
| A30 | Segment write key | `^[A-Za-z0-9]{32}$` | case-sensitive | 32 | none | Obs (`analytics.load("<key>")`, `cdn.segment.com/analytics.js/v1/<key>/`) |
| A31 | Amplitude API key | `^[0-9a-f]{32}$` | lowercase | 32 | none | Src (TruffleHog `\b([0-9a-f]{32})\b` with "amplitude" prefix) [W:th-amplitude] |
| A32 | New Relic browser licence key + application ID | key `^NRJS-[0-9a-f]{19}$`; app `^[0-9]{6,12}$` | key case-sensitive prefix | 24 | none | Src (gitleaks `new-relic-browser-api-token`) [W:gitleaks]; app id Obs (`NREUM.info={licenseKey,applicationID}`) |
| A33 | Sentry DSN | see block A33 | lowercase public key; drop legacy secret part | | none | Doc (`https://<public_key>@o<org_id>.ingest.sentry.io/<project_id>`) [W:sentry-dsn] |
| A34 | Sentry organisation ID / project ID / public key (derived from A33) | org `^[0-9]{1,12}$`, project `^[0-9]{1,19}$`, key `^[0-9a-f]{32}$` | | | none | Doc [W:sentry-dsn] |
| A35 | Cloudflare Web Analytics site token | `^[0-9a-f]{32}$` | lowercase | 32 | none | Doc (token in `data-cf-beacon` JSON on `static.cloudflareinsights.com/beacon.min.js`) [W:cf-wa]; length Obs |
| A36 | Umami website ID | UUID (as A23) | lowercase | 36 | none | Obs (`data-website-id`; self-hosted script host is itself a pivot) |
| A37 | Amazon Associates tag | `^[A-Za-z0-9-]{1,40}-2[0-2]$` | as-is | | none | Obs (`tag=` URL parameter, `-20` US, `-21` EU, `-22` JP); cited by Bellingcat as pivot [W:bellingcat-2015] |
| A38 | AddThis profile ID | `^ra-[0-9a-f]{16}$` | lowercase | 19 | none | Obs; service discontinued 2023 (historical) [W:bellingcat-2015] |

Block A33 (Sentry DSN, Rust `regex` with named groups):

```
^https://(?P<key>[0-9a-f]{32})(?::(?P<secret>[0-9a-f]{32}))?@(?P<host>(?:o(?P<org>[0-9]+)\.)?ingest(?:\.[a-z]{2})?\.sentry\.io|[A-Za-z0-9.-]+)(?::(?P<port>[0-9]{1,5}))?(?P<path>/[A-Za-z0-9._/-]*)?/(?P<project>[0-9]+)/?$
```

Canonical: `https://<key>@<host>/<project>` lowercase host, secret removed (legacy DSNs carried `key:secret`). A self-hosted Sentry host is also an infrastructure observable (hostname). Older DSNs without the `o<org>.` subdomain stay valid [W:sentry-dsn].

### 2.2 Extraction, pivot value, MISP and OpenCTI

| ID | Typical extraction (context regex, case-insensitive on JS keywords) | Pivot value | MISP | OpenCTI |
|---|---|---|---|---|
| A01 | `ga\(\s*['"]create['"]\s*,\s*['"](UA-[0-9]+-[0-9]+)`, `_setAccount`, `gtag\('config',\s*'UA-`, `google-analytics.com/(collect\|r/collect)\?.*tid=UA-` | H (account linkage across properties; Universal Analytics stopped processing in July 2023, so mostly historical crawls) | obj:tracking-id.id(text) with tracker `Google Analytics`; obj:network-profile.google-analytics-id(text) | Text |
| A02 | derived from A01 | H | none (must be computed) | none |
| A03 | `googletagmanager.com/gtag/js\?id=(G-[A-Z0-9]+)`, `gtag\('config',\s*'(G-[A-Z0-9]+)'` | M-H (per data stream, no account number exposed) | obj:tracking-id.id(text) | Text |
| A04 | `gtag/js\?id=(GT-[A-Z0-9]+)` | M | obj:tracking-id.id(text) | Text |
| A05 | `googletagmanager.com/(gtm.js\|ns.html)\?id=(GTM-[A-Z0-9]+)`; fetch `gtm.js` to extract embedded A01/A03/A06/A11 | H (container belongs to one GTM account) | obj:tracking-id.id(text) | Text |
| A06 | `gtag\('config',\s*'(AW-[0-9]+)`, `googleadservices.com/pagead/conversion/([0-9]+)/` | H (advertiser account) | obj:tracking-id.id(text) | Text |
| A07 | `gtag\('config',\s*'(DC-[0-9]+)` | M | text | Text |
| A08 | `googleoptimize.com/optimize.js\?id=(OPT-[A-Z0-9]+)` | M (historical) | text | Text |
| A09 | `data-ad-client="ca-pub-[0-9]{16}"`, `adsbygoogle.js\?client=ca-pub-`, `ads.txt` line `google.com, pub-[0-9]{16}, (DIRECT\|RESELLER)` | H (payment account; strongest classic pivot) | obj:tracking-id.id(text) | Text |
| A10 | APK/IPA resources and manifests (`com.google.android.gms.ads.APPLICATION_ID`), `app-ads.txt` | H (links apps to the same AdMob publisher) | text | Text |
| A11 | `fbq\(\s*'init'\s*,\s*'([0-9]{15,16})'`, `facebook.com/tr\?id=([0-9]+)` | H (ad account) | obj:tracking-id.id(text) | Text |
| A12 | `<meta property="fb:app_id" content="([0-9]+)"`, `appId\s*:\s*'([0-9]+)'` | M-H | text | Text |
| A13 | `ym\(\s*([0-9]+)\s*,\s*"init"`, `mc.yandex.ru/watch/([0-9]+)`, `yaCounter([0-9]+)` | H | obj:tracking-id.id(text) | Text |
| A14 | `clarity.ms/tag/([a-z0-9]+)` | M-H | text | Text |
| A15 | `ti\s*:\s*"?([0-9]+)"?`, `bat.bing.com/action/0\?ti=([0-9]+)` | H (ad account) | text | Text |
| A16 | `hjid\s*:\s*([0-9]+)`, `static.hotjar.com/c/hotjar-([0-9]+)\.js\?sv=` | M-H | text | Text |
| A17 | `setSiteId',\s*'?([0-9]+)`, `setTrackerUrl', *u *\+ *'(matomo\|piwik)\.php'` with `var u=` base URL | H when tracker host is self-hosted (the host is the real pivot); L for id alone | obj:tracking-id.id(text), tracker `Piwik` | Text |
| A18 | `hm.baidu.com/hm.js\?([0-9a-f]{32})` | H | text | Text |
| A19 | `ttq.load\('([A-Z0-9]+)'\)`, `analytics.tiktok.com/i18n/pixel/events.js\?sdkid=` | H | text | Text |
| A20 | `_linkedin_partner_id\s*=\s*"([0-9]+)"` | M-H | text | Text |
| A21 | `twq\('(config\|init)',\s*'([a-z0-9]+)'\)` | M | text | Text |
| A22 | `pintrk\('load',\s*'([0-9]+)'` | M | text | Text |
| A23 | `snaptr\('init',\s*'([0-9a-f-]{36})'` | M | text | Text |
| A24 | `rdt\('init',\s*'([at]2_[a-z0-9]+)'` | M | text | Text |
| A25 | `_qevents.push\(\{qacct:"(p-[A-Za-z0-9_-]+)"` | M | text | Text |
| A26 | `VK-RTRG-[0-9]+-[A-Za-z0-9]+` | M | text | Text |
| A27 | `sc_project\s*=\s*([0-9]+)`, `sc_security\s*=\s*"([0-9a-f]+)"` | M | text | Text |
| A28 | `js.hs-scripts.com/([0-9]+)\.js`, `portalId:\s*"?([0-9]+)` | M-H | text | Text |
| A29 | `mixpanel.init\(\s*["']([0-9a-f]{32})` | H (project-level, public by design) | text | Text |
| A30 | `analytics.load\(\s*["']([A-Za-z0-9]{32})`, `cdn.segment.com/analytics.js/v1/([A-Za-z0-9]+)/` | H | text | Text |
| A31 | `amplitude(\.getInstance\(\))?\.init\(\s*["']([0-9a-f]{32})` | H | text | Text |
| A32 | `NREUM.info=\{.*licenseKey:"([^"]+)".*applicationID:"([0-9]+)"` | M-H | text | Text |
| A33 | `Sentry.init\(\{\s*dsn:\s*"(https://[^"]+)"`, `Raven.config\('` (legacy), stealer/panel configs | H (org and project are tenant-unique; seen in phishing kits and malware panels) | text | Text |
| A35 | `data-cf-beacon='\{"token":\s*"([0-9a-f]{32})"` | M-H (one token per site/apex) [W:cf-wa] | text | Text |
| A36 | `data-website-id="([0-9a-f-]{36})"` | M (H with self-hosted script host) | text | Text |
| A37 | `[?&]tag=([A-Za-z0-9-]+-2[0-2])` | M | text | Text |
| A38 | `addthis_widget.js#pubid=(ra-[0-9a-f]{16})` | M (historical) | text | Text |

---

## 3. Family B: site ownership verification tokens

Found in HTML `<meta name=... content=...>` and in DNS TXT records (also as files such as `BingSiteAuth.xml`, `googleXXXX.html`). Pivot rationale: a webmaster or tenant account that verified several domains often reuses the same token; passive DNS / TXT history services expose them. MISP: none (text). OpenCTI: Text.

| ID | Identifier | Where | Validation regex (token only) | Case / canonical | Length | Evidence | Pivot |
|---|---|---|---|---|---|---|---|
| B01 | Google site verification | meta `google-site-verification`; TXT `google-site-verification=<token>` | `^[A-Za-z0-9_-]{43}$` | case-sensitive, as-is | 43 (record 68) | Doc: "68-character string that begins with google-site-verification=, followed by 43 additional characters" [W:google-txt] | M-H (reuse indicates same Search Console or Workspace account; reliability not documented) |
| B02 | Yandex Webmaster verification | meta `yandex-verification`; TXT `yandex-verification: <token>` | `^[0-9a-f]{16}$` | lowercase | 16 | Obs | M-H |
| B03 | Bing Webmaster verification | meta `msvalidate.01` | `^[0-9A-F]{32}$` | uppercase | 32 | Obs | M-H |
| B04 | Microsoft 365 / Entra domain verification | TXT `MS=ms<digits>` | `^MS=ms[0-9]{8}$` | as-is | 13 | Obs | H (one tenant) |
| B05 | Meta domain verification | meta `facebook-domain-verification`; TXT `facebook-domain-verification=` | `^[a-z0-9]{20,40}$` | lowercase | Obs ~30 | Obs; third-party guides say value is unique per domain [W:fb-dv] | L-M (may be per domain) |
| B06 | Pinterest domain verification | meta `p:domain_verify` | `^[0-9a-f]{32}$` | lowercase | 32 | Obs | M |
| B07 | Baidu site verification | meta `baidu-site-verification` | `^code-[A-Za-z0-9]{10}$` | as-is | 15 | Obs | M |
| B08 | Apple domain verification | TXT `apple-domain-verification=` | `^[A-Za-z0-9]{16}$` | as-is | 16 | Obs | M |
| B09 | Other TXT tokens (Atlassian `atlassian-domain-verification=`, DocuSign `docusign=<uuid>`, Adobe `adobe-idp-site-verification=`, Zoom `ZOOM_verify_`, OpenAI `openai-domain-verification=dv-`, GlobalSign `_globalsign-domain-verification`, Stripe, etc.) | TXT | provider specific | as-is | | Obs | M (tenant-level) |

---

## 4. Family C: embedded third-party service keys (non-analytics)

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum | Extraction | Pivot | Evidence | MISP / OpenCTI |
|---|---|---|---|---|---|---|---|---|---|
| C01 | Google API key (Maps, Firebase, etc.) | `^AIza[0-9A-Za-z_-]{35}$` | case-sensitive | 39 | none | page JS (`maps.googleapis.com/maps/api/js?key=`), Firebase config, APK `google_api_key` resource | H (one GCP project) | Src gitleaks `gcp-api-key` `\b(AIza[\w-]{35})` [W:gitleaks] | text / Text |
| C02 | Firebase app ID; messaging sender ID; project ID | app `^1:[0-9]{6,15}:(?:web\|android\|ios):[0-9a-f]{8,40}$`; sender `^[0-9]{6,15}$`; project `^[a-z][a-z0-9-]{4,28}[a-z0-9]$` | lowercase | | none | `firebaseConfig = {...}` in phishing kits and apps; `<project>.firebaseapp.com`, `<project>.web.app` | H | Obs (project-id rule from GCP naming) | text / Text |
| C03 | reCAPTCHA site key | `^6L[0-9A-Za-z_-]{38}$` | case-sensitive | 40 | none | `data-sitekey=`, `grecaptcha.render`, `recaptcha/api.js?render=` | M-H (phishing kits reuse) | Obs | text / Text |
| C04 | hCaptcha site key | UUID | lowercase | 36 | none | `data-sitekey=` with `hcaptcha.com` | M-H | Obs | text / Text |
| C05 | Cloudflare Turnstile site key | `^0x4[0-9A-Za-z_-]{20,}$` | case-sensitive | ~24 | none | `data-sitekey="0x4AAAAAAA..."` with `challenges.cloudflare.com/turnstile` | M-H | Obs | text / Text |
| C06 | Stripe publishable key | `^pk_(?:live\|test)_[0-9A-Za-z]{24,99}$` | case-sensitive | | none | `Stripe('pk_live_...')` | H (merchant account) | Obs (secret `sk_`/`rk_` regex in gitleaks [W:gitleaks]) | text / Text |
| C07 | Tawk.to property ID + widget ID | `^[0-9a-f]{24}$` and `^[0-9a-z]{8,10}$` | lowercase | 24 | none | `embed.tawk.to/<property>/<widget>` | H (scam support sites) | Obs | text / Text |
| C08 | JivoChat widget ID | `^[A-Za-z0-9]{10}$` | case-sensitive | 10 | none | `code.jivosite.com/widget/<id>`, `code.jivo.ru/widget/<id>` | H | Obs | text / Text |
| C09 | Crisp website ID | UUID | lowercase | 36 | none | `CRISP_WEBSITE_ID="<uuid>"` | H | Obs | text / Text |
| C10 | Intercom app ID | `^[a-z0-9]{8}$` | lowercase | 8 | none | `app_id: "<id>"`, `widget.intercom.io/widget/<id>` | H | Obs | text / Text |
| C11 | LiveChat licence | `^[0-9]{6,9}$` | decimal | | none | `__lc.license = <n>` | H | Obs | text / Text |
| C12 | Zendesk Web Widget key | UUID | lowercase | 36 | none | `static.zdassets.com/ekr/snippet.js?key=<uuid>` | H | Obs | text / Text |
| C13 | Supabase project ref | `^[a-z]{20}$` | lowercase | 20 | none | `https://<ref>.supabase.co` plus anon JWT in phishing kits | H | Obs | text / Text |

---

## 5. Family D: messaging, webhook and exfiltration endpoints

Context: phishing kits, infostealers, RATs and extortion tooling exfiltrate to chat platforms. These values are **credentials** (anyone holding them can post, and often read). The non-secret identity embedded in them (bot ID, webhook ID, team ID, tenant ID) is the long-lived pivot.

### 5.1 Syntax

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum / decoding | Evidence |
|---|---|---|---|---|---|---|
| D01 | Telegram bot token | `^([0-9]{5,16}):([A-Za-z0-9_-]{35})$` | case-sensitive | 41-52 | none; group 1 = bot user ID (D02) | Doc example `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` [W:tg-botapi]; Src TruffleHog `\b([0-9]{8,10}:[a-zA-Z0-9_-]{35})\b` [W:th-telegram]; Src gitleaks `([0-9]{5,16}:(?-i:A)[a-z0-9_\-]{34})` (case-insensitive, secret starts with `A`) [W:gitleaks] |
| D02 | Telegram bot user ID (derived) | `^[0-9]{5,16}$` | decimal, u64 | | none | Doc: IDs have "at most 52 significant bits" [W:tg-botapi] |
| D03 | Telegram chat ID (Bot API form) | `^-?[0-9]{1,16}$`; supergroup/channel `^-100[0-9]{9,13}$` | signed i64 | | none; `-100` prefix = Bot API encoding of channel IDs (Obs) | Doc: "at most 52 significant bits, so a signed 64-bit integer" [W:tg-botapi] |
| D04 | Discord webhook URL | see block D04 | host canonicalised to `discord.com`, API version dropped | | webhook ID is a snowflake (decode, section 20) | Src TruffleHog `(https:\/\/discord\.com\/api\/webhooks\/[0-9]{18,19}\/[0-9a-zA-Z-]{68})` [W:th-discordwebhook] |
| D05 | Discord webhook ID (derived) | `^[0-9]{17,20}$` | decimal u64 | | snowflake timestamp | Doc snowflake [W:discord-ref] |
| D06 | Discord bot token | `^[A-Za-z0-9_-]{23,28}\.[A-Za-z0-9_-]{6}\.[A-Za-z0-9_-]{27,40}$` | case-sensitive | ~59-72 | segment 1 = base64url of the bot user ID (Obs) | Src TruffleHog `([A-Za-z0-9_-]{24}\.[A-Za-z0-9_-]{6}\.[A-Za-z0-9_-]{27})` [W:th-discordbot] |
| D07 | Slack incoming webhook URL | `^https://hooks\.slack\.com/services/(T[A-Z0-9]{8,12})/(B[A-Z0-9]{8,12})/([A-Za-z0-9]{23,25})$` | case-sensitive | | none; group 1 = workspace (team) ID | Src TruffleHog `https://hooks\.slack\.com/services/T[A-Z0-9]+/B[A-Z0-9]+/[A-Za-z0-9]{23,25}`; workflow form `/workflows/T.../A.../[0-9]{17,19}/...` [W:th-slackwebhook]; gitleaks `(?:https?://)?hooks.slack.com/(?:services\|workflows\|triggers)/[A-Za-z0-9+/]{43,56}` [W:gitleaks] |
| D08 | Slack workspace (team) ID (derived) | `^T[A-Z0-9]{8,12}$` | uppercase | | none | Src (as D07) |
| D09 | Microsoft Teams incoming webhook URL (Office 365 connector) | see block D09 | lowercase GUIDs | | none; embeds group GUID and **tenant GUID** (Q02) | Src TruffleHog v1 regex [W:th-teams] |
| D10 | Google Apps Script web-app deployment ID | `^AKfycb[A-Za-z0-9_-]{30,90}$` | case-sensitive | | none | Obs (`script.google.com/macros/s/<id>/exec`) |
| D11 | Formspree form ID | `^[A-Za-z0-9]{8}$` | case-sensitive | 8 | none | Obs (`formspree.io/f/<id>`) |
| D12 | EmailJS service / template / public key | `^service_[a-z0-9]{7}$`, `^template_[a-z0-9]{7}$`, public key `^[A-Za-z0-9_-]{15,22}$` | as-is | | none | Obs |
| D13 | Web3Forms access key | UUID | lowercase | 36 | none | Obs |
| D14 | ngrok authtoken | `^[A-Za-z0-9]{27}_[A-Za-z0-9]{20,21}$` | case-sensitive | ~49 | none | Src TruffleHog [W:th-ngrok] |

Block D04 (Discord webhook):

```
^https://(?:(?:canary|ptb)\.)?discord(?:app)?\.com/api(?:/v[0-9]{1,2})?/webhooks/(?P<id>[0-9]{17,20})/(?P<token>[A-Za-z0-9_-]{60,80})$
```

Block D09 (Teams, TruffleHog v1, verbatim):

```
(https:\/\/[a-zA-Z-0-9]+\.webhook\.office\.com\/webhookb2\/[a-zA-Z-0-9]{8}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{12}\@[a-zA-Z-0-9]{8}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{12}\/IncomingWebhook\/[a-zA-Z-0-9]{32}\/[a-zA-Z-0-9]{8}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{4}-[a-zA-Z-0-9]{12})
```

TruffleHog v2 also matches the Power Automate replacement: `https://[a-z0-9]+\.\d+\.environment\.api\.powerplatform\.com(?::\d+)?/powerautomate/automations/direct/workflows/[a-f0-9]{32}/triggers/manual/paths/invoke\?[^\s"'<>]+` [W:th-teams].

### 5.2 Extraction, pivot value, MISP, OpenCTI

| ID | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|
| D01-D03 | phishing kit PHP/JS (`api.telegram.org/bot<token>/sendMessage?chat_id=`), stealer configs, sandbox network traffic | H (bot ID survives token revocation; chat ID identifies the operator's channel) | obj:telegram-bot.token(text) l.19, obj:telegram-bot.chat-id(text) l.3 | Text (or `Credential` for the token) |
| D04-D06 | stealer configs (Discord-based stealers, grabbers), JS in kits | H | text | Text / Credential |
| D07-D08 | scripts, configs | H (workspace ID) | text | Text / Credential |
| D09 | configs, scripts | H (tenant GUID) | text | Text / Credential |
| D10-D14 | phishing kit form `action=` targets and JS | H | text | Text |

---

## 6. Family E: crypto-mining keys, cryptocurrency addresses and financial identifiers

Note: cryptocurrency and banking identifiers may be covered in more depth by sibling research; included here for checksum completeness.

### 6.1 Syntax

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum | Evidence |
|---|---|---|---|---|---|---|
| E01 | CoinHive site key | `^[A-Za-z0-9]{32}$` | case-sensitive | 32 | none | Obs (`new CoinHive.Anonymous('<key>')`, `coinhive.min.js`); service closed March 2019, historical |
| E02 | CoinImp (and similar web-miner) site key | `^[0-9a-f]{64}$` | lowercase | 64 | none | Obs (`Client.Anonymous('<key>')`) |
| E03 | Bitcoin legacy address (P2PKH `1...`, P2SH `3...`) | `^[13][1-9A-HJ-NP-Za-km-z]{25,33}$` | case-sensitive (base58) | 26-34 | Base58Check: last 4 bytes = first 4 bytes of SHA-256(SHA-256(version+payload)); version 0x00 / 0x05 | Doc (Bitcoin) |
| E04 | Bitcoin SegWit address | v0 `^bc1q[02-9ac-hj-np-z]{38}$` (P2WPKH) or `^bc1q[02-9ac-hj-np-z]{58}$` (P2WSH); v1 taproot `^bc1p[02-9ac-hj-np-z]{58}$` | lowercase canonical (all-uppercase accepted, mixed case invalid) | 42 / 62 | Bech32 (BIP-173) for v0; Bech32m (BIP-350) for v1+ | Doc |
| E05 | Ethereum / EVM address (also BSC, Polygon, contract addresses) | `^0x[0-9a-fA-F]{40}$` | identity: lowercase; display: EIP-55 mixed case | 42 | EIP-55: if mixed case, letter i uppercase iff nibble i of keccak256(lowercase hex) >= 8 | Doc |
| E06 | Monero address | standard / subaddress `^[48][1-9A-HJ-NP-Za-km-z]{94}$`; integrated `^4[1-9A-HJ-NP-Za-km-z]{105}$` | case-sensitive | 95 / 106 | Monero base58 (8-byte blocks) with 4-byte Keccak-256 checksum | Doc |
| E07 | TRON address | `^T[1-9A-HJ-NP-Za-km-z]{33}$` | case-sensitive | 34 | Base58Check, version byte 0x41 | Doc |
| E08 | Litecoin | `^[LM3][1-9A-HJ-NP-Za-km-z]{25,33}$` or `^ltc1[02-9ac-hj-np-z]{39,59}$` | as E03 / E04 | | Base58Check / Bech32 | Doc |
| E09 | Dogecoin | `^D[1-9A-HJ-NP-Za-km-z]{25,33}$` | | 34 | Base58Check | Doc |
| E10 | Bitcoin Cash CashAddr | `^(?:bitcoincash:)?[qp][02-9ac-hj-np-z]{41}$` | lowercase, prefix optional | | CashAddr BCH checksum (40-bit) | Doc |
| E11 | XRP classic address | `^r[1-9A-HJ-NP-Za-km-z]{24,34}$` | case-sensitive (Ripple base58 alphabet differs) | | double SHA-256 checksum | Doc |
| E12 | Solana address | `^[1-9A-HJ-NP-Za-km-z]{32,44}$` | case-sensitive | | none (base58 of 32-byte key; validate decodes to 32 bytes) | Doc |
| E13 | Mining pool login | composite: pool host:port + wallet (E03-E06) + worker name | | | | Obs (xmrig `"user"`, `--user`) |
| E14 | IBAN | `^[A-Z]{2}[0-9]{2}[A-Z0-9]{11,30}$` | uppercase, no spaces | 15-34 | ISO 13616 / ISO 7064 MOD 97-10: move first 4 chars to end, letters to 10..35, number mod 97 = 1 | Doc |
| E15 | BIC (SWIFT) | `^[A-Z]{4}[A-Z]{2}[A-Z0-9]{2}(?:[A-Z0-9]{3})?$` | uppercase; `XXX` branch equals 8-char form | 8 / 11 | none | Doc (ISO 9362) |
| E16 | Payment card PAN | `^[0-9]{12,19}$` | digits only | 12-19 | Luhn | Doc (ISO/IEC 7812) |
| E17 | Card BIN / IIN | `^[0-9]{6,8}$` | | 6-8 | none | Doc |

### 6.2 Context

| ID | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|
| E01-E02 | page JS, compromised CMS injections | H historically (payout account) | text (or tracking-id) | Text |
| E03-E12 | ransom notes, scam pages, clipper malware configs, miner configs, blockchain C2 (contract addresses) | H | type:btc / xmr / dash (format-only validation, AVT l.614-619); obj:coin-address.address(btc), address-xmr(xmr), address-crypto(text) | SCO:Cryptocurrency-Wallet (`value`) |
| E13 | miner command lines and JSON configs | H | text | Text |
| E14-E15 | money-mule and BEC lures | H | type:iban / bic | SCO:Bank-Account (`iban` identity, `bic`) |
| E16-E17 | carding dumps | H (PII/PCI) | type:cc-number / bin | SCO:Payment-Card (`card_number`) |

---

## 7. Family F: TLS and X.509 fingerprints

Facts:

- JA3 string = `SSLVersion,Cipher,SSLExtension,EllipticCurve,EllipticCurvePointFormat`, decimal values joined by `-` inside a field and `,` between fields, GREASE ignored, MD5 of the string; example string `769,47-53-5-10-49161-49162-49171-49172-50-56-19-4,0-10-11,23-24-25,0` gives `ada70206e40642a3e4461f35503241d5`. JA3S = `SSLVersion,Cipher,SSLExtension`. The Salesforce repository was archived on 1 May 2025 and points to JA4 [W:ja3].
- Chrome began permuting ClientHello extension order (shipped 20 Jan 2023 per Fastly; experiment from Chrome 108 per Peakhour), so a single browser yields many JA3 values; sorted-extension variants were proposed as a fix [W:chrome-permute]. JA4 sorts ciphers and extensions for this reason.
- JA4 = `a_b_c`, e.g. `t13d1516h2_8daaf6152771_e5627efa2ab1`. Section a: protocol `t` (TCP) / `q` (QUIC) / `d` (DTLS); TLS version 2 chars (`13`, `12`, `11`, `10`, `s3`, `s2`, `d1`, `d2`, `d3`, `00` unknown; taken from highest `supported_versions` value ignoring GREASE); SNI `d` present / `i` absent; cipher count 2 digits capped at 99; extension count 2 digits; ALPN first and last character of the first ALPN value (`00` if none; hex fallback for non-alphanumeric). Section b: first 12 hex of SHA-256 of sorted 4-hex-digit cipher list (GREASE excluded), `000000000000` if empty. Section c: first 12 hex of SHA-256 of sorted extensions (SNI `0000` and ALPN `0010` excluded) + `_` + signature algorithms in original order. Raw variants: `ja4_r` (sorted, unhashed), `ja4_o` (hashed, original order), `ja4_ro` (unhashed, original order) [W:ja4-spec]. JA4 is BSD 3-Clause; every other JA4+ method is under FoxIO License 1.1 (patent pending; not for monetisation) [W:ja4-repo].
- JA4S = `t120400_c030_4e8089b08790`: protocol, TLS version, extension count (2 digits), chosen ALPN (`00` if none); chosen cipher (4 hex); truncated SHA-256 of extensions in order [W:ja4-img-s].
- JA4X = `96a6439c8f5c_96a6439c8f5c_aae71e8db6d7`: hash of issuer RDN OIDs in order, hash of subject RDN OIDs in order, hash of extension OIDs in order (values not included; "fingerprints how a cert is created") [W:ja4-img-x]. Published examples: Sliver/Havoc `000000000000_7c32fa18c13e_bf0f0589fc03`, Cobalt Strike `2166164053c1_2166164053c1_30d204a01551`, Qakbot `2bab15409345_af684594efb4_000000000000` [W:ja4-repo].
- JA4L = `5191_42_45014`: one-way TCP latency in microseconds, observed TTL, one-way application handshake latency. Used to estimate distance, not identity [W:ja4-img-l].
- JARM = 62 characters: first 30 = cipher and TLS version chosen for each of 10 probe ClientHellos (3 chars each, `000` = refused), last 32 = truncated SHA-256 of the cumulative server extensions; example `2ad2ad0002ad2ad00042d42d00000069d641f34fe76acdc05c40262f8815e5` [W:jarm]. An all-zero JARM means no TLS answer (REC: treat as null, never as a pivot).
- Shodan exposes `ssl.jarm` and `ssl.ja3s` ("Useful for pivoting"), `ssl.cert.fingerprint.{sha1,sha256}`, `ssl.cert.serial` as **integer**, `ssl.chain_sha256` [W:shodan-ssl].
- RFC 5280 section 4.1.2.2: serialNumber is a positive integer, CAs must not use more than 20 octets; issuer + serial identifies a certificate.

### 7.1 Syntax

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum / derivation | Evidence |
|---|---|---|---|---|---|---|
| F01 | JA3 full string | `^[0-9]{1,5},(?:[0-9]{1,5}(?:-[0-9]{1,5})*)?,(?:[0-9]{1,5}(?:-[0-9]{1,5})*)?,(?:[0-9]{1,5}(?:-[0-9]{1,5})*)?,(?:[0-9]{1,5}(?:-[0-9]{1,5})*)?$` | as-is | var | MD5 gives F02 | Doc [W:ja3] |
| F02 | JA3 hash | `^[0-9a-f]{32}$` | lowercase | 32 | MD5(F01) | Doc [W:ja3]; MISP AVT l.15 |
| F03 | JA3S hash (+ string `^[0-9]{1,5},[0-9]{1,5},(?:[0-9]{1,5}(?:-[0-9]{1,5})*)?$`) | `^[0-9a-f]{32}$` | lowercase | 32 | MD5(string) | Doc [W:ja3] |
| F04 | JA4 | `^[tqd](?:13\|12\|11\|10\|s3\|s2\|d1\|d2\|d3\|00)[di][0-9]{2}[0-9]{2}[0-9A-Za-z]{2}_[0-9a-f]{12}_[0-9a-f]{12}$` | lowercase hex sections; ALPN chars as computed | 36 | b, c = truncated SHA-256 | Doc [W:ja4-spec] |
| F05 | JA4 raw / original-order variants (`ja4_r`, `ja4_o`, `ja4_ro`) | `ja4_o`: same as F04; `ja4_r` / `ja4_ro`: `^[tqd][0-9a-z]{2}[di][0-9]{4}[0-9A-Za-z]{2}_[0-9a-f]{4}(?:,[0-9a-f]{4})*_(?:[0-9a-f]{4}(?:,[0-9a-f]{4})*)?(?:_[0-9a-f]{4}(?:,[0-9a-f]{4})*)?$` | lowercase | var | `ja4_o` is NOT comparable to `ja4` | Doc [W:ja4-spec]; raw regex REC |
| F06 | JA4S | `^[tq](?:13\|12\|11\|10\|s3\|s2\|00)[0-9]{2}[0-9A-Za-z]{2}_[0-9a-f]{4}_[0-9a-f]{12}$` | lowercase | 25 | c = truncated SHA-256 | Doc [W:ja4-img-s]; examples `t120300_c030_5e2616a54c73` (IcedID), `t130200_1301_a56c5b993250` (Sliver) [W:ja4-repo] |
| F07 | JA4X | `^[0-9a-f]{12}_[0-9a-f]{12}_[0-9a-f]{12}$` | lowercase | 38 | truncated SHA-256 of OID lists | Doc [W:ja4-img-x] |
| F08 | JA4L / JA4LS | `^[0-9]{1,9}_[0-9]{1,3}_[0-9]{1,9}$` | decimal | var | measurement | Doc [W:ja4-img-l] |
| F09 | JARM | `^[0-9a-f]{62}$`; null value `0{62}` | lowercase | 62 | last 32 = truncated SHA-256 | Doc [W:jarm]; MISP AVT l.16 |
| F10 | X.509 certificate SHA-256 fingerprint | `^[0-9a-f]{64}$` (input may contain `:`) | lowercase, no separators | 64 | SHA-256 of DER | Src MISP AVT l.183-187 normalisation |
| F11 | X.509 SHA-1 fingerprint | `^[0-9a-f]{40}$` | as F10 | 40 | SHA-1 of DER | Src MISP |
| F12 | X.509 MD5 fingerprint | `^[0-9a-f]{32}$` | as F10 | 32 | MD5 of DER | Src MISP |
| F13 | X.509 serial number (scoped by issuer DN) | hex `^[0-9a-f]{1,40}$` (canonical); accept `0x`, colons, uppercase, or decimal (Shodan) | lowercase minimal hex (strip leading zero octets, keep `0`); keep raw form | <=40 hex | none | Doc RFC 5280; Shodan integer [W:shodan-ssl] |
| F14 | SubjectPublicKeyInfo SHA-256 (key pin) | hex `^[0-9a-f]{64}$`; base64 form `^[A-Za-z0-9+/]{43}=$` | store raw 32 bytes; render hex | 64 / 44 | SHA-256 of DER SPKI | Doc (RFC 7469 `pin-sha256`) |
| F15 | Subject Key Identifier | `^[0-9a-f]{2,128}$` (commonly 40) | lowercase hex, no `:` | typ. 40 | often SHA-1 of subjectPublicKey (RFC 5280 method 1) | Doc |
| F16 | Authority Key Identifier (key id) | as F15 | | | equals issuer SKI | Doc |
| F17 | JA4Scan-TLS / JA4Scan-QUIC | format not published in the sources reviewed | | | | Doc (listed only) [W:ja4-repo] |

### 7.2 Context

| ID | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|
| F01-F03 | passive: ClientHello / ServerHello in PCAP, Zeek, Suricata, Arkime; Shodan `ssl.ja3s` | M for malware clients with custom TLS stacks; L for browsers and common libraries; degraded for Chrome since 2023 | type:ja3-fingerprint-md5 (describeTypes l.104); obj:ja3, obj:ja3s (JA3S stored as ja3 type) | Text |
| F04-F07 | passive capture (Zeek/Suricata/Wireshark JA4 plugins), active scans | M (JA4, JA4S), M-H (JA4X for default C2 certificate generators) | obj:ja4-plus.ja4-fingerprint(text) + ja4-type | Text |
| F08 | passive timing | X (measurement attribute of a session) | obj:ja4-plus (JA4L) | none |
| F09 | active scanning (Shodan `ssl.jarm`, Censys, own scanner) | M (default C2 TLS stacks; collisions with CDNs and common servers) | type:jarm-fingerprint (l.108); obj:jarm; obj:network-profile.jarm | Text |
| F10-F12 | TLS handshake, CT logs (crt.sh), scanners, PE Authenticode signatures | H (exact certificate reuse across hosts, self-signed C2 defaults) | type:x509-fingerprint-* (l.532-540); obj:x509 | X509-Certificate.hashes (identity) |
| F13 | as above | L alone (default serials in tooling), H with issuer | obj:x509.serial-number(text) l.211; obj:authenticode-signerinfo.serial-number | X509-Certificate.serial_number |
| F14 | as above | H (key reuse across re-issued certificates and across hosts) | obj:x509.pubkey-info-modulus(text) l.179 (modulus, not hash) | X509-Certificate.subject_public_key_modulus (modulus only) |
| F15-F16 | as above | H (SKI = key reuse; AKI = private CA used by an operator) | obj:x509.subject-key-id l.267 / issuer-key-id l.90 | X509-Certificate.subject_key_identifier / authority_key_identifier (registration l.152-153) |

---

## 8. Family G: SSH fingerprints and keys

Facts:

- HASSH = MD5 of `kex;enc;mac;cmp` (algorithm lists from the client `SSH_MSG_KEXINIT`, comma-separated within, `;` between); HASSHServer = same over the server KEXINIT. Example HASSH `8a8ae540028bf433cd68356c1b9e8d5b` (Cyberduck). Salesforce repo archived 1 May 2025; maintained fork at Corelight [W:hassh].
- JA4SSH = `c36s36_c55s75_c70s0`: mode of client/server packet length, SSH packets sent by client/server, bare ACKs from client/server, computed every 200 SSH packets by default. Examples: interactive `c36s36_c51s80_c69s0`, reverse shell `c76s76_c71s59_c0s70` [W:ja4-img-ssh]. Behavioural, not an identity.
- OpenSSH host key fingerprints: `SHA256:` + unpadded base64 of SHA-256 (43 chars) is the modern default; legacy MD5 is 16 colon-separated hex bytes. MISP accepts both (AVT l.745-757; test values `7b:e5:6f:...`, `MD5:7b:e5:...`, `SHA256:mVPwvezndPv/ARoIadVY98vAC0g+P/5633yTC4d/wXE` in `app/Test/AttributeValidationToolTest.php` l.79-83).
- Shodan SSH banner properties: `ssh.fingerprint`, `ssh.hassh`, `ssh.key`, `ssh.type`, `ssh.kex.*` [W:shodan-ssh].

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum / derivation | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|---|---|
| G01 | HASSH (client) | `^[0-9a-f]{32}$` | lowercase | 32 | MD5 of G03 | passive (Zeek `hassh` package), honeypots (Cowrie) | M (bot/scanner tooling) | type:hassh-md5 (l.116); obj:passive-ssh.hassh, obj:cowrie.hassh | Text |
| G02 | HASSHServer | `^[0-9a-f]{32}$` | lowercase | 32 | MD5 | passive, Shodan `ssh.hassh` | M | type:hasshserver-md5 (l.120) | Text |
| G03 | hasshAlgorithms string (client/server) | `^[^;]*;[^;]*;[^;]*;[^;]*$` | as-is | var | | passive | M (explains G01/G02) | text | Text |
| G04 | JA4SSH | `^c[0-9]+s[0-9]+_c[0-9]+s[0-9]+_c[0-9]+s[0-9]+$` | lowercase | var | | passive (per 200 packets) | X (session behaviour) | obj:ja4-plus | none |
| G05 | SSH host key fingerprint SHA-256 | `^SHA256:[A-Za-z0-9+/]{43}$` | case-sensitive base64; store algorithm + 32 raw bytes | 50 | SHA-256 of the wire-format public key blob | banner scans (Shodan, Censys), `ssh-keyscan`, passive-ssh (CIRCL D4) | H (VPS images cloned with the same host key; operator reuse) | type:ssh-fingerprint (l.184); obj:passive-ssh.fingerprint | SSH-Key.fingerprint_sha256 (identity) |
| G06 | SSH host key fingerprint MD5 | `^(?:MD5:)?[0-9a-f]{2}(?::[0-9a-f]{2}){15}$` | lowercase hex with colons | 47 (+4) | MD5 of key blob | as G05 | H | type:ssh-fingerprint | SSH-Key.fingerprint_md5 |
| G07 | SSH public key (authorized_keys line) | `^(?:ssh-ed25519\|ssh-rsa\|ssh-dss\|ecdsa-sha2-nistp(?:256\|384\|521)\|sk-ssh-ed25519@openssh\.com\|sk-ecdsa-sha2-nistp256@openssh\.com) [A-Za-z0-9+/]+={0,3}(?: .*)?$` | identity = key type + base64 blob (comment excluded) | var | derive G05 and G06 | malware persistence (`~/.ssh/authorized_keys`), GitHub `https://github.com/<user>.keys`, C2 configs | H | obj:ssh-authorized-keys.key(text); obj:github-user.ssh-public-key(text) | SSH-Key.public_key; Cryptographic-Key (detector `^ssh-(rsa\|dss\|ed25519\|ecdsa) ...`, observable.ts l.154) |
| G08 | SSH identification banner | `^SSH-[0-9.]+-[\x21-\x2C\x2E-\x7E]+(?: [\x20-\x7E]*)?$` (RFC 4253 section 4.2, max 255 chars incl. CRLF) | as-is (without CRLF) | <=253 | | scans, honeypots | L-M (custom banners in implants or honeypots) | obj:passive-ssh.banner(text) | Text |

---

## 9. Family H: HTTP and web content fingerprints

Facts:

- Shodan favicon hash: MurmurHash3 x86 32-bit, seed 0, computed over Python `base64.encodebytes(favicon_bytes)` (MIME base64 with a newline every 76 chars and a trailing newline), reported as a **signed** 32-bit integer; query `http.favicon.hash:<int>` [W:favicon]. Shodan Datapedia lists `http.favicon.{data,hash,location}` with `hash` integer, and example `http.favicon.hash:708578229` [W:shodan-http]. FOFA `icon_hash` uses the same mmh3 convention (Obs).
- Shodan HTTP numeric hashes [W:shodan-http]: `http.html_hash` ("Numeric hash of the http.html property. Useful for finding other IPs with the exact same website."), `http.headers_hash` ("Numeric hash of the HTTP headers with the order preserved."), `http.components_hash`, `http.robots_hash`, `http.title_hash`, `http.server_hash`, `http.securitytxt_hash`, `http.sitemap_hash`, `http.dom_hash` (no description); banner-level `hash` ("Numeric hash of the data property") [W:shodan-root]. The algorithm is not documented by Shodan (commonly reported as mmh3; Obs).
- urlscan.io search fields: `hash` ("Any SHA256 hash of any HTTP response"), `dom.hash` ("SHA256 hash of the DOM before truncation"), `text.hash`, `page.tlsIssuer`, `page.title`, `content.identifiers` [W:urlscan-search].
- HHHash: SHA-256 over the ordered list of HTTP response header **names**, joined with `:`, case preserved, output `hhh:1:<sha256hex>`; values are only comparable when produced with the same crawler / request headers [W:hhhash]. MISP object stores `hhhash` as text "in format hhh:version:hash_value" plus the query headers and tool (`hhhash/definition.json` l.8-39).
- JA4H = `ge20cr13enus_974ebe531c03_b66fa821d02c_e97928733c74`: method (2 chars: `ge`, `po`, `pu`, ...), HTTP version (`10`, `11`, `20`, `30`), cookie `c`/`n`, referer `r`/`n`, header count (2 digits, excludes Cookie and Referer, capped at 99), first 4 chars of primary Accept-Language (`0000` if absent); b = truncated SHA-256 of header names in order; c = of sorted cookie names; d = of sorted cookie name=value pairs [W:ja4-img-h] [W:ja4h-md]. Published malware examples are often truncated to `a_b` (e.g. Darkgate `po10nn060000_cdb958d032b0`) [W:ja4-repo].
- MISP dom-hash: 32 hex = first 32 chars of SHA-256 over all tag names joined by `|` (MispAttribute.php l.4270); example value in tests `0cc175b9c0f1b6a831c399e269772661` (AttributeValidationToolTest.php l.41-43).
- Community ID: flow hash `1:` + base64(SHA-1(seed + flow tuple)) for cross-sensor flow correlation (MISP type `community-id`, describeTypes l.152).

| ID | Identifier | Validation regex | Case / canonical | Length | Derivation | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|---|---|
| H01 | JA4H | `^[a-z]{2}(?:10\|11\|20\|30)[cn][rn][0-9]{2}[0-9a-z]{4}_[0-9a-f]{12}(?:_[0-9a-f]{12}_[0-9a-f]{12})?$` | lowercase | 51 (or 25 for `a_b`) | truncated SHA-256 | HTTP requests in PCAP / proxy logs, sandbox traffic | M (malware HTTP clients) | obj:ja4-plus | Text |
| H02 | HHHash | `^hhh:1:[0-9a-f]{64}$` | lowercase hex; version prefix kept | 70 | SHA-256 of header names | active crawl (python-hhhash, Lacus, AIL, Common Crawl) | M (server stack, C2 profile); only within one crawler profile | obj:hhhash.hhhash(text) | Text |
| H03 | Favicon MurmurHash3 (Shodan convention) | `^-?[0-9]{1,10}$` and value within i32 range | canonical signed i32 decimal; accept unsigned u32 and convert | <=11 | mmh3_32(base64.encodebytes(bytes)) | crawl favicon (`/favicon.ico`, `<link rel=icon>`), Shodan, FOFA | M-H for bespoke panels and phishing kits; L for framework defaults; 32-bit space means accidental collisions are expected at scale (REC: corroborate) | type:favicon-mmh3 (l.112); obj:favicon; obj:cert-pl-phishing.favicon-mmh3 (text, "base64") | Text |
| H04 | Favicon content hash (MD5 / SHA-256 of bytes) | `^[0-9a-f]{32}$` / `^[0-9a-f]{64}$` | lowercase | 32/64 | MD5/SHA-256 | Censys, ZoomEye, urlscan `hash:` | H (exact file) | type:md5/sha256 or obj:favicon.favicon(attachment) | File/Artifact hashes |
| H05 | Shodan `http.html_hash` | `^-?[0-9]{1,10}$` (i32) | signed decimal | | Shodan numeric hash | Shodan only | H (exact page), but vendor-scoped | none | none |
| H06 | Shodan `http.headers_hash` | as H05 | | | | Shodan | M | none | none |
| H07 | Shodan `http.title_hash`, `robots_hash`, `securitytxt_hash`, `sitemap_hash`, `server_hash`, `components_hash`, `dom_hash`, banner `hash` | as H05 | | | | Shodan | M-H (each vendor-scoped) | none | none |
| H08 | MISP dom-hash | `^[0-9a-f]{32}$` | lowercase | 32 | truncated SHA-256 of `\|`-joined tag names | own crawler (Lookyloo style) | M (kit templates) | type:dom-hash (l.156); obj:dom-hash; obj:url.dom-hash | Text |
| H09 | urlscan DOM SHA-256 | `^[0-9a-f]{64}$` | lowercase | 64 | SHA-256 of serialised DOM | urlscan `dom.hash` | H (exact DOM), vendor-scoped | none | none |
| H10 | urlscan text SHA-256 | `^[0-9a-f]{64}$` | lowercase | 64 | SHA-256 of visible text | urlscan `text.hash` | M-H | none | none |
| H11 | HTTP response body / resource SHA-256 | `^[0-9a-f]{64}$` | lowercase | 64 | SHA-256 of body | urlscan `hash`, own crawls | H | type:sha256 | Artifact.hashes |
| H12 | HTTP ETag | `^(?:W/)?"[\x21\x23-\x7E\x80-\xFF]*"$` (RFC 9110 entity-tag) | as-is | var | Apache default encodes inode-size-mtime in hex (Obs) | response headers | M (identical file deployed to many servers) | text | Text |
| H13 | Screenshot perceptual hash (pHash / dHash) | provider specific (cert.pl stores base64 DCT pHash) | | | similarity, Hamming distance | crawler screenshots | M (similarity only) | obj:cert-pl-phishing.phash-dct-base64(text) | none |
| H14 | Community ID | `^1:[A-Za-z0-9+/]{27}=$` | case-sensitive | 30 | base64(SHA-1) | Zeek, Suricata, Arkime | X for infrastructure (correlates one flow across sensors) | type:community-id (l.152) | none |

---

## 10. Family I: TCP, OS, DHCP and NTP fingerprints

| ID | Identifier | Validation regex | Example | Canonical | Extraction | Pivot | MISP | OpenCTI | Evidence |
|---|---|---|---|---|---|---|---|---|---|
| I01 | JA4T (client SYN) | `^[0-9]{1,5}_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)_[0-9]{1,5}_[0-9]{1,2}$` | `64240_2-1-3-1-1-4_1460_8` (Windows 11) | as-is | passive SYN | L-M (OS / VPN / scanner stack) | obj:ja4-plus | none | Doc [W:ja4-img-t] [W:ja4-repo] |
| I02 | JA4TS (server SYN-ACK) | same as I01 | `65535_2-1-3-1-1-4_1460_8` (format) | as-is | passive / active | L-M | obj:ja4-plus | none | Doc [W:ja4-img-t] |
| I03 | JA4TScan (active, with retransmission timings) | `^[0-9]{1,5}_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)_[0-9]{1,5}_[0-9]{1,2}_[0-9R-]+$` | `28960_2-4-8-1-3_1460_3_1-4-8-16` (Epson printer) | as-is | active SYN scan | M (device / OS identification) | obj:ja4-plus | none | Doc [W:ja4-repo] |
| I04 | MuonFP | `^[0-9]{1,5}:(?:[0-9]{1,3}(?:-[0-9]{1,3})*)?:[0-9]{1,5}:[0-9]{1,2}$` | `26847:2-4-8-1-3:1460:8` | as-is | passive SYN / SYN-ACK | L-M (same information as JA4T, open licence) | obj:muonfp.muonfp-fingerprint(text) l.32-36 | none | Src MISP object |
| I05 | p0f signature | p0f `ver:ittl:olen:mss:wsize,scale:olayout:quirks:pclass` | | as-is | passive | L-M | text | none | Doc (p0f v3) |
| I06 | JA4D (DHCPv4) | `^[a-z]{5}[0-9]{4}[in][dn]_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)$` | `reqst1500in_55-57-61-51-12_1-121-3-6-15-108-114-119-252` (iPhone); `disco0000in_61-12-60-55_1-3-6-15-31-33-43-44-46-47-119-121-249-252` (Windows 11) | as-is | LAN DHCP capture | L for CTI (device type inside a network), useful in DFIR | none | none | Doc [W:ja4-img-d] [W:ja4-repo] |
| I07 | JA4D6 (DHCPv6) | `^[a-z]{5}[0-9]{4}[in][dn]_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)_(?:[0-9]{1,3}(?:-[0-9]{1,3})*\|00)$` | `solct0014nn_8-1-3-39-16-6_17-23-24-39` (Windows 11) | as-is | LAN | L | none | none | Doc [W:ja4-img-d6] |
| I08 | JA4N (NTP) | format not published in sources reviewed | | | | L | none | none | Doc (listed) [W:ja4-repo] |

---

## 11. Family J: binary similarity and structural hashes

Facts: MISP types and validation in section 1.1. OpenCTI supports only the `hashes` keys MD5, SHA-1, SHA-256, SHA-512, SHA3-256, SHA3-512, SSDEEP, SDHASH, TLSH, LZJD on File/Artifact/X509 (registration l.54-77); none of imphash, pehash, authentihash, vhash, impfuzzy, telfhash, Rich hash, cdhash. STIX allows custom hash keys `^[a-zA-Z0-9_-]{3,250}$` (hashes-type.json l.9) and `imphash` only inside `windows-pebinary-ext` (file.json l.266-268). TLSH default output is `T1` + 70 hex (72 chars), legacy 70 hex via `-old`, default 128 buckets and 1-byte checksum, minimum 50 bytes of input, distance 0 = near identical [W:tlsh]. Telfhash = TLSH over the ELF symbol list (or call destinations when stripped), README examples are 70 lowercase hex, CLI prints `-` and the module returns `None` when no symbols [W:telfhash]. VirusTotal file object exposes `tlsh`, `permhash`, `vhash` ("in-house similarity clustering algorithm value, based on a simple structural feature hash") and `main_icon.{raw_md5,dhash}` [W:vt-files].

| ID | Identifier | Validation regex | Case / canonical | Length | Algorithm (summary) | Comparison | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|---|---|
| J01 | Imphash | `^[0-9a-f]{32}$` | lowercase | 32 | MD5 of comma-joined `dll.function` (lowercased, `.dll`/`.ocx`/`.sys` stripped, ordinals resolved for ws2_32/wsock32/oleaut32, else `ordN`) as implemented in pefile | equality | M (packers and stubs collide; H for bespoke import sets) | type:imphash (l.372); obj:pe.imphash l.78 | none (Text) |
| J02 | Impfuzzy (JPCERT) | `^[0-9]+:[A-Za-z0-9/+]+:[A-Za-z0-9/+]+$` | case-sensitive | var | ssdeep over the import list | similarity | M | type:impfuzzy (l.384); obj:pe.impfuzzy l.73 | none |
| J03 | PEhash (Wicherski 2009) | `^[0-9a-f]{40}$` | lowercase | 40 | SHA-1 over selected PE header and section structure fields | equality | M (polymorphic families) | type:pehash (l.380) | none |
| J04 | Authentihash | `^[0-9a-f]{64}$` (SHA-256); Authenticode may also use SHA-1 (40) | lowercase | 64 | hash of the PE image excluding CheckSum, the Certificate Table directory entry and the attribute certificate table | equality | H (same code re-signed or with modified signature; used by driver blocklists) | type:authentihash (l.360, 64 hex only); obj:pe.authentihash | none |
| J05 | VirusTotal vhash | `^[0-9A-Za-z!.]+$` (opaque; MISP accepts `^.+$`) | as-is | var | proprietary structural feature hash, file-type specific [W:vt-files] | equality (clustering) | M (VT ecosystem only) | type:vhash (l.364) | none |
| J06 | Rich header hash | `^[0-9a-f]{32}$` | lowercase | 32 | MD5 of the decoded Rich header; tools differ (with or without padding, XOR key, DanS marker) | equality per variant | H (build environment), but forgeable (false flags) | obj:pe.richpe(md5) l.184 | none |
| J07 | TLSH | `^T1[0-9A-F]{70}$` canonical; accept `^(?:[Tt]1)?[0-9A-Fa-f]{70}$` | uppercase hex with `T1` (REC); STIX needs the 70-char form | 72 | locality-sensitive hash, 128 buckets | distance (lower = closer) | M | type:tlsh (l.424) | hashes.TLSH (STIX pattern `^[a-zA-Z0-9]{70}$`) |
| J08 | ssdeep | `^[0-9]{1,10}:[A-Za-z0-9/+]{1,64}:[A-Za-z0-9/+]{1,32}$` | case-sensitive | var | context-triggered piecewise hash; block size = 3*2^n | similarity (0-100) | M | type:ssdeep (l.368; regex AVT l.738) | hashes.SSDEEP (STIX `^[a-zA-Z0-9/+:.]{1,128}$`) |
| J09 | sdhash | `^sdbf(?:-dd)?:[0-9]{2}:.+$` | as-is | large (KB) | similarity digest bloom filters | similarity | L-M (rarely shared) | none | hashes.SDHASH |
| J10 | LZJD | opaque | | | Lempel-Ziv Jaccard distance digest | similarity | L-M | none | hashes.LZJD |
| J11 | Telfhash | `^(?:[Tt]1)?[0-9a-f]{70}$` | lowercase, `t1` stripped (REC) | 70 | TLSH over sorted ELF symbols | distance | M (IoT botnets) | type:telfhash (l.376) | none |
| J12 | Permhash (Mandiant) | `^[0-9a-f]{64}$` | lowercase | 64 | SHA-256 of the permission list in CRX/APK manifests | equality | M (extension and app families) | none | none |
| J13 | Icon dHash (VirusTotal `main_icon.dhash`) | `^[0-9a-f]{16}$` | lowercase | 16 (64-bit, Obs) | difference hash of embedded icon | Hamming distance | M | none | none |
| J14 | Apple CDHash | `^[0-9a-f]{40}$` (truncated); full `^[0-9a-f]{64}$` | lowercase | 40 / 64 | SHA-1 or SHA-256 (truncated to 20 bytes) of the Code Directory | equality | H (signed Mach-O identity; used for notarisation revocation) | type:cdhash (l.428, `^[0-9a-f]{40,}$`) | none |

---

## 12. Family K: build and code-signing provenance identifiers

| ID | Identifier | Validation regex | Canonical | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|
| K01 | PDB path | single line | as-is (case-preserving; Windows paths compare case-insensitively) | PE debug directory (CodeView RSDS) | H (developer usernames, project names) | type:pdb (describeTypes l.20); obj:pe.pdb l.155 | Text |
| K02 | PDB GUID + age (RSDS signature) | `^[0-9A-F]{32}[0-9A-F]{1,8}$` (symbol-server key: GUID without dashes + age in hex) | uppercase | PE debug directory | H (same build) | none | none |
| K03 | .NET TypeLib ID (assembly `GuidAttribute`) | UUID | lowercase | .NET metadata | H (same Visual Studio project across builds; public research by Brian Wallace 2015) | none | none |
| K04 | .NET MVID (module version ID) | UUID | lowercase | .NET metadata `#GUID` heap | H (same compilation) | none | none |
| K05 | .NET TypeRefHash (G DATA) | `^[0-9a-f]{64}$` | lowercase | SHA-256 over sorted TypeRef namespace+name | M-H (obfuscation-resistant family clustering) | none | none |
| K06 | ELF GNU build-id | `^[0-9a-f]{16,64}$` (commonly 40) | lowercase | `.note.gnu.build-id` (NT_GNU_BUILD_ID) | H (same build) | none | none |
| K07 | Go build ID | `^[A-Za-z0-9_-]{20}(?:/[A-Za-z0-9_-]{20}){1,3}$` (Obs) | as-is | `go tool buildid`, `.note.go.buildid` | H | none | none |
| K08 | Mach-O LC_UUID | UUID (uppercase in Apple tooling) | uppercase | Mach-O load command | H | none | none |
| K09 | Apple Developer Team ID | `^[A-Z0-9]{10}$` | uppercase | code signature (`TeamIdentifier`), Developer ID certificate OU | H (developer account; revocation unit) | none | none |
| K10 | Android signing certificate digest | SHA-256 `^[0-9a-f]{64}$`, SHA-1 `^[0-9a-f]{40}$` | lowercase, no colons | APK v1/v2/v3 signature block | H (developer key; same signer across many package names) | obj:android-app.certificate(sha1), certificate-sha256(sha256) l.15 | X509-Certificate.hashes |
| K11 | Authenticode signer certificate (thumbprint, serial + issuer) | see F10-F13 | | PE security directory | H (stolen or abused code-signing certificates) | obj:authenticode-signerinfo; obj:x509 | X509-Certificate |

---

## 13. Family L: C2 framework and remote-access identifiers

Facts (Cobalt Strike):

- Didier Stevens' `1768.py` config table: `0x0007: 'publickey'`, `0x0025: 'license-id'` (lines 1999 and 2029 of `1768.py` in DidierStevensSuite); the licence ID is read as a 4-byte big-endian unsigned integer (`struct.unpack('>I', remainder)`, line 1645). The public key is a DER `SubjectPublicKeyInfo` for RSA-1024 that starts with `30 81 9F 30 0D 06 09 2A 86 48 86 F7 0D 01 01 01 05 00 03 81 8D 00 30 81 89 02 81` (line 2440), stored in a 256-byte padded config slot [W:1768].
- Watermark: present since Cobalt Strike 3.10; "should be unique per Cobalt Strike installation, as the license server issues this"; cracked builds patch it to fixed values, so a watermark can be shared by unrelated operators (NCC Group, Elastic) [W:cs-watermark]. Elastic recommends clustering on watermark together with other fields such as the public key [W:cs-watermark].
- Shodan exposes `cobalt_strike_beacon.{x86,x64}.watermark` (integer) and, in example data, `server.publickey_md5` [W:shodan-cs].
- MISP `cs-beacon-config` has `watermark` (text, l.182) and `license-id` (text, l.131) but no public-key attribute; requiredOneOf includes `watermark`.

| ID | Identifier | Validation regex | Canonical | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|
| L01 | Cobalt Strike watermark (licence ID) | `^[0-9]{1,10}$` and value <= 4294967295 | u32 decimal | beacon config (setting 0x25), memory dumps, Shodan / Censys beacon parsers | H for genuine licences, L for known cracked values (needs a maintained list) | obj:cs-beacon-config.watermark(text), license-id(text) | Text |
| L02 | Cobalt Strike team-server public key | DER hex `^30819f300d06092a864886f70d010101050003818d0030818902818100[0-9a-f]{256}0203010001$` (Obs, RSA-1024, e=65537); hash forms `^[0-9a-f]{32}$` (MD5) / `^[0-9a-f]{64}$` (SHA-256) | store DER bytes trimmed of padding; identity = SHA-256 of DER; also expose MD5 for Shodan compatibility | beacon config setting 0x07 | H (one key pair per team server; survives domain and IP rotation) | none (only config hashes) | Cryptographic-Key (value) |
| L03 | Other C2 framework default fingerprints (Sliver, Havoc, Mythic, Brute Ratel, Metasploit) | no own identifier syntax; expressed as JA4X / JARM / JA4S / X.509 values (F-family) | | scans | M | obj:jarm.scope `Malicious - C2` | Text |
| L04 | AnyDesk ID | `^[0-9]{9,10}$` | decimal | ransom / tech-support scam notes, logs (`ad.trace`), RMM abuse | H | obj:rmm.guid(text) | Text |
| L05 | TeamViewer ID | `^[0-9]{9,10}$` | decimal | as L04 (`TeamViewer*_Logfile.log`) | H | obj:rmm (text) | Text |
| L06 | RMM agent / deployment ID and API key (Atera, ScreenConnect, etc.) | vendor specific (GUIDs, tokens) | | installer MSI properties, configs | H (operator tenant) | obj:rmm.guid, rmm.api-key (text) | Text / Credential |

---

## 14. Family M: cryptographic identity keys and messenger key IDs

| ID | Identifier | Validation regex | Canonical | Length | Checksum / derivation | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|---|
| M01 | OpenPGP short key ID | `^[0-9A-F]{8}$` | uppercase, no `0x` | 8 | low 32 bits of v4 fingerprint | L (trivially collidable; do not use as identity) | obj:pgp-key.key-id(text) ("Short IDs can collide", l.45) | Text |
| M02 | OpenPGP long key ID | `^[0-9A-F]{16}$` | uppercase | 16 | v4: low-order 64 bits of fingerprint; v6: high-order 64 bits (RFC 9580) | M-H | obj:pgp-key.key-id, obj:pgp-meta.key-id | Text |
| M03 | OpenPGP v4 fingerprint | `^[0-9A-F]{40}$` | uppercase, no spaces or `0x` (as MISP pgp-key l.24-25) | 40 | SHA-1 over v4 public-key packet | H | obj:pgp-key.fingerprint(text) | Cryptographic-Key |
| M04 | OpenPGP v6 (RFC 9580) / v5 (LibrePGP) fingerprint | `^[0-9A-F]{64}$` | uppercase | 64 | SHA-256 over the public-key packet | H | obj:pgp-key.fingerprint(text) | Cryptographic-Key |
| M05 | OpenPGP public key block | ASCII armour `-----BEGIN PGP PUBLIC KEY BLOCK-----` | normalised armour; identity = M03/M04 | var | | H (ransom notes, leak sites, Keybase) | type:pgp-public-key (l.176) | Cryptographic-Key (detector l.153) |
| M06 | Tox ID | `^[0-9A-F]{76}$` | uppercase | 76 | 32-byte public key + 4-byte nospam + 2-byte checksum (XOR of the 2-byte chunks of key+nospam) | H (ransomware negotiation contact) | text | Text |
| M07 | Session messenger ID | `^05[0-9a-f]{64}$` | lowercase | 66 | `05` prefix + X25519 public key | H (ransomware contact) | text | Text |
| M08 | SSH public key | see G07 | | | | H | see G07 | SSH-Key |

---

## 15. Family N: telecom and device identifiers

Facts: 3GPP TS 23.003 defines IMEI = TAC (8 digits) + SNR (6) + CD (1, Luhn over the 14 digits; sent as `0` "spare" by the handset), IMEISV = TAC + SNR + SVN (2 digits, no check digit); IMSI = MCC (3) + MNC (2 or 3, depends on MCC) + MSIN, total not more than 15 digits [W:3gpp-23003]. ITU-T E.118 ICCID: starts with MII `89`, country code + issuer identifier, individual account number, Luhn check digit per ISO/IEC 7812-1 Annex B; 19 digits per the standard, 20 common in practice, up to 22 seen [W:e118]. GSMA SGP.29 EID: 32 digits, check digits computed so that the 32-digit number mod 97 = 1; new EIDs must not start with `89` (legacy SGP.02 EIDs do) [W:eid]. OpenCTI accepts IMEI 15-16, ICCID 18-22, IMSI 14-15 digits with no checksum (1.3). MISP keeps these in the `phone` object as text.

### 15.1 Syntax

| ID | Identifier | Validation regex | Canonical | Length | Checksum | Derived components |
|---|---|---|---|---|---|---|
| N01 | IMEI | `^[0-9]{15}$` (accept 14 and compute CD) | 15 digits, CD recomputed | 15 | Luhn over 15 digits (CD computed over first 14) | TAC (8) gives make/model via GSMA TAC database; SNR |
| N02 | IMEISV | `^[0-9]{16}$` | 16 digits | 16 | none | IMEI = first 14 + computed Luhn CD; SVN |
| N03 | TAC (Type Allocation Code) | `^[0-9]{8}$` | | 8 | none | device model (not unique per device) |
| N04 | MEID (3GPP2) | hex `^[0-9A-F]{14}$`, optional CD `^[0-9A-F]{15}$`; decimal form `^[0-9]{18}$` | uppercase hex | 14 (+1) | optional Luhn-like base-16 check digit | pESN = `80` + low 24 bits of SHA-1(MEID) |
| N05 | ESN (legacy CDMA) | hex `^[0-9A-F]{8}$` or decimal `^[0-9]{11}$` | uppercase hex | 8 / 11 | none | manufacturer code |
| N06 | IMSI | `^[0-9]{6,15}$` (practically 14-15) | digits | <=15 | none | PLMN = MCC + MNC (MNC length needs an E.212 table lookup) |
| N07 | PLMN ID (MCC+MNC) | `^[0-9]{3}[0-9]{2,3}$` | | 5-6 | none | country + operator |
| N08 | TMSI / GUTI / P-TMSI | TMSI `^[0-9A-Fa-f]{8}$` | | 8 hex | none | transient (reallocated by the network) |
| N09 | ICCID | `^89[0-9]{16,20}$` (allow trailing `F` filler on input, strip) | digits only | 18-22 | Luhn (when the last digit is a check digit) | country code, issuer identifier |
| N10 | EID (eUICC) | `^[0-9]{32}$` | digits | 32 | ISO 7064 MOD 97-10 (number mod 97 = 1) | EUM identification |
| N11 | MSISDN / E.164 phone number | `^\+[1-9][0-9]{1,14}$` | E.164 with `+`, no spaces | 2-15 digits | none (validity needs numbering-plan metadata, e.g. libphonenumber) | country calling code, national destination code |
| N12 | MAC address (EUI-48) | `^[0-9a-f]{2}(?::[0-9a-f]{2}){5}$`; accept `-`, `.` (Cisco `xxxx.xxxx.xxxx`), bare 12 hex, uppercase | lowercase colon form (STIX mac-addr l.26, MISP AVT l.217-220) | 17 | none | OUI (24 bits), I/G bit (multicast, LSB of first octet), U/L bit (locally administered, second LSB) |
| N13 | OUI / MA-L assignment (derived) | `^[0-9a-f]{2}(?::[0-9a-f]{2}){2}$` | lowercase | 8 | none | vendor via IEEE registry |
| N14 | EUI-64 | `^[0-9a-f]{2}(?::[0-9a-f]{2}){7}$` | lowercase colon form | 23 | none | if bytes 4-5 are `ff:fe`, MAC-derived; IPv6 modified EUI-64 IID flips the U/L bit |
| N15 | Bluetooth device address (BD_ADDR) | same as N12 | lowercase colon form | 17 | none | NAP (16) / UAP (8) / LAP (24); for BLE random addresses the two MSBs give the subtype: `11` static, `01` resolvable private (rotates), `00` non-resolvable |
| N16 | Wi-Fi BSSID | same as N12 | | 17 | none | AP radio MAC |
| N17 | Wi-Fi SSID | 0-32 octets, arbitrary bytes | store bytes; render UTF-8 when valid | <=32 bytes | none | case-sensitive |

### 15.2 Context

| ID | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|
| N01-N03 | stalkerware / mobile malware C2 registrations (device fingerprint posted to C2), forensic extractions, leaked panels | H (device) | obj:phone.imei(text) l.30; obj:uav / remote-controller imei | SCO:IMEI (value; 15-16 digits) |
| N04-N05 | legacy CDMA devices | M | text | Text |
| N06-N08 | IMSI catchers, SS7/Diameter attack logs, mobile malware | H for IMSI (subscriber, PII); X for TMSI/GUTI | obj:phone.imsi l.36, obj:imsi-catcher.imsi; obj:phone.tmsi/guti | SCO:IMSI (14-15 digits) |
| N09-N10 | SIM-swap fraud, mobile malware, device forensics | H | obj:phone.iccid(text) l.25 | SCO:ICCID (18-22 digits); EID none |
| N11 | WHOIS, scam pages, messaging accounts, SMS phishing senders, malware SMS C2 | H | type:phone-number (l.336), type:whois-registrant-phone (l.512); obj:phone-number, obj:phone.msisdn | SCO:Phone-Number |
| N12-N13 | DHCP / ARP logs, LNK files (machine MAC in DROID UUIDv1), IPv6 SLAAC addresses, malware host fingerprints, Office metadata | H for globally administered MACs; L for randomised (locally administered) MACs | type:mac-address (l.580); obj:device.MAC-address | SCO:Mac-Addr |
| N14 | IPv6 interface identifiers | M | type:mac-eui-64 (l.584, only ff:fe form) | none (STIX mac-addr cannot hold it) |
| N15 | Bluetooth / BLE scans, IoT forensics | M (public or static only) | type:mac-address (no BT type) | Mac-Addr (no BT type) |
| N16-N17 | Wi-Fi geolocation (WiGLE), malware Wi-Fi survey modules | M (BSSID gives location) | obj:wifi-connection.bssid (mac-address) l.21, ssid (text) l.139 | Mac-Addr (BSSID); Text (SSID) |

---

## 16. Family O: account handles and platform IDs

Design fact for the whole family: a **handle** (mutable, often case-insensitive, re-registrable) is a different observable from the platform's **numeric ID** (immutable). Both are needed; the handle is only valid with an observation time.

### 16.1 Syntax

| ID | Identifier | Validation regex | Case / canonical | Length | Decoding / notes | Evidence |
|---|---|---|---|---|---|---|
| O01 | Telegram username | `^[A-Za-z][A-Za-z0-9_]{3,31}$` | case-insensitive; canonical lowercase without `@` | 5-32 (4-char collectible handles reported) | bots end with `bot` | Obs (5-char minimum per Telegram bug tracker; collectible usernames via Fragment [W:tg-fragment]); MISP telegram-bot l.24-27 |
| O02 | Telegram user / bot ID | `^[0-9]{1,16}$` | u64 decimal | | at most 52 significant bits | Doc [W:tg-botapi] |
| O03 | Telegram chat / channel ID | Bot API `^-?[0-9]{1,16}$`, channels/supergroups `^-100[0-9]{9,13}$` | REC: store (peer kind, positive id) | | Bot API adds `-100` to MTProto channel IDs (Obs) | Doc (signed 64-bit) [W:tg-botapi] |
| O04 | Telegram invite link hash | `^[A-Za-z0-9_-]{16,24}$` from `t.me/+<hash>` or `t.me/joinchat/<hash>` | case-sensitive | | | Obs |
| O05 | Discord username (2023+ system) | `^[a-z0-9_.]{2,32}$` plus "no two consecutive periods" (code check) | case-insensitive, forced lowercase | 2-32 | | Doc [W:discord-usernames] |
| O06 | Discord legacy tag | `^.{2,32}#[0-9]{4}$` | as-is | | historical | Obs |
| O07 | Discord snowflake (user, guild, channel, message, application, webhook) | `^[0-9]{17,20}$` | u64 decimal; REC: separate types per object kind | 17-20 | timestamp ms = (id >> 22) + 1420070400000; worker = (id & 0x3E0000) >> 17; process = (id & 0x1F000) >> 12; increment = id & 0xFFF | Doc [W:discord-ref] |
| O08 | Discord invite code | `^[A-Za-z0-9-]{2,32}$` from `discord(?:\.gg\|(?:app)?\.com/invite)/<code>` | case-sensitive (vanity codes lowercase) | | resolves to guild ID | Obs |
| O09 | GitHub username / organisation login | `^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*$` and length <= 39 | case-insensitive; canonical lowercase | 1-39 | no leading/trailing or double hyphen | Doc (GHES: max 39, alphanumeric and dashes, no leading/trailing or consecutive dashes) [W:gh-usernames] |
| O10 | GitHub numeric user / org / repo ID; GraphQL node_id | `^[0-9]{1,12}$`; node_id `^[A-Za-z0-9_=-]{4,64}$` | decimal | | stable across renames; noreply email `<id>+<login>@users.noreply.github.com` | Obs; MISP github-user.id l.35, node_id l.52 |
| O11 | GitHub repository full name | `^[A-Za-z0-9]+(?:-[A-Za-z0-9]+)*/[A-Za-z0-9._-]{1,100}$` (repo name not `.` or `..`) | case-insensitive; canonical lowercase `owner/name` | | | Obs; MISP github-repo.full-name l.48 |
| O12 | GitHub gist ID | `^[0-9a-f]{20}$\|^[0-9a-f]{32}$` | lowercase | 20/32 | | Obs (dead-drop resolvers) |
| O13 | GitLab username / namespace path | `^[A-Za-z0-9_.][A-Za-z0-9_.-]{1,254}$` (must not end in `.git` or `.atom`; code check) | case-insensitive | 2-255 | | Obs (lenient) |
| O14 | X (Twitter) handle | `^[A-Za-z0-9_]{1,15}$` | case-insensitive; canonical lowercase without `@` | 1-15 (new accounts have a higher minimum) | | Obs (help page not retrievable, 403); MISP twitter-account.name "without the @" l.103 |
| O15 | X (Twitter) numeric user ID | `^[0-9]{1,20}$` | u64 decimal | | snowflake IDs: timestamp ms = (id >> 22) + 1288834974657 (Twitter epoch); early accounts have small sequential IDs | Obs; MISP twitter-account.id "Numeric account id" l.62 |
| O16 | Mastodon / ActivityPub handle | username `^[A-Za-z0-9_]+(?:[.-]+[A-Za-z0-9_]+)*$` (remote), local `^[A-Za-z0-9_]+$` max 30; domain IDNA | canonical `acct:user@domain` lowercase | | stable identity = actor URI (`https://domain/users/user`) via WebFinger | Src Mastodon `app/models/account.rb` l.74 (`USERNAME_RE = /[a-z0-9_]+([.-]+[a-z0-9_]+)*/i`), l.84-85 (`USERNAME_LENGTH_LIMIT = 30`), l.137 [W:mastodon] |
| O17 | Facebook numeric profile / page ID | `^[0-9]{5,20}$` | decimal | | stable; vanity name separate | Obs; MISP facebook-account.account-id(text) |
| O18 | Instagram numeric ID and username | ID `^[0-9]{5,20}$`; username `^[A-Za-z0-9._]{1,30}$` | username case-insensitive | | | Obs; MISP instagram-account |
| O19 | YouTube channel ID / handle | ID `^UC[A-Za-z0-9_-]{22}$`; handle `^@[A-Za-z0-9._-]{3,30}$` | ID case-sensitive; handle case-insensitive | 24 | | Obs; MISP youtube-channel.channel-id |
| O20 | TikTok username and numeric user ID | username `^[A-Za-z0-9._]{2,24}$`; ID `^[0-9]{6,20}$` | username case-insensitive | | | Obs |
| O21 | Reddit username and account fullname | `^[A-Za-z0-9_-]{3,20}$`; fullname `^t2_[a-z0-9]{1,12}$` | case-insensitive | | fullname = `t2_` + base36 ID | Obs; MISP reddit-account |
| O22 | XMPP / Jabber JID | `localpart@domainpart[/resource]` (RFC 7622) | bare JID lowercase (PRECIS) without resource | <=3071 | | Doc RFC 7622; MISP type:jabber-id (l.632, email-like check AVT l.456-461) |
| O23 | Matrix user ID | `^@[a-z0-9._=/+-]+:[A-Za-z0-9.-]+(?::[0-9]{1,5})?$` | lowercase localpart | <=255 | | Obs (Matrix spec user identifier grammar) |
| O24 | Signal username | `^[A-Za-z][A-Za-z0-9_]{2,31}\.[0-9]{2,9}$` | case-insensitive | | discriminator digits mandatory | Obs |
| O25 | Keybase username | `^[a-z0-9_]{2,16}$` | lowercase | | links proofs (Twitter, GitHub, PGP) | Obs; MISP keybase-account |
| O26 | VK ID / screen name | ID `^(?:id)?[0-9]{1,12}$`; screen name `^[a-z0-9_.]{3,32}$` | lowercase | | | Obs |

### 16.2 Context

| ID | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|
| O01-O04 | `t.me/<name>` links in pages and malware, scam ads, channel posts, Bot API traffic | H | obj:telegram-account.username / id; obj:telegram-bot.username; obj:chat | User-Account (no `telegram` in account_type_ov) or Persona or Text |
| O05-O08 | Discord invites in lures, stealer configs, webhook payloads, CDN URLs (channel IDs) | H | obj:chat (protocol discord); text | User-Account / Text |
| O09-O12 | malware hosting on GitHub, dead-drop resolvers, commit metadata, noreply emails | H (numeric ID H, login M) | type:github-username / github-repository / github-organisation (l.620-628); obj:github-user, obj:github-repo | User-Account / Text |
| O13 | as above | M-H | obj:gitlab-user | User-Account / Text |
| O14-O15 | social profiles, influence operations, scam promotion | H (ID), M (handle) | type:twitter-id (l.636); obj:twitter-account | User-Account (`account_type` = twitter, vocabulary l.22) |
| O16 | influence operations, fediverse C2 or scam promotion | M-H | text | User-Account / Text |
| O17-O21, O26 | social media investigations | M-H | obj:facebook-account, instagram-account, youtube-channel, reddit-account (all text) | User-Account (`facebook` only in vocabulary) / Text |
| O22-O25 | ransom notes and actor contact details, forum profiles | H | type:jabber-id; obj:keybase-account; text | User-Account / Text |

---

## 17. Family P: mobile apps, browser extensions and software packages

| ID | Identifier | Validation regex | Case / canonical | Length | Derivation / checksum | Extraction | Pivot | MISP | OpenCTI | Evidence |
|---|---|---|---|---|---|---|---|---|---|---|
| P01 | Android application ID (package name) | `^[A-Za-z][A-Za-z0-9_]*(?:\.[A-Za-z][A-Za-z0-9_]*)+$` | case-sensitive (Java package) | var | identity should be (package, signing cert K10) because the same package name can be re-signed by another key | AndroidManifest, Play URL `play.google.com/store/apps/details?id=<pkg>`, `market://` | H | type:mobile-application-id (l.744); obj:android-app.appid l.3; obj:apk.package-name l.47 | Software (name) / Text | Doc: "at least two segments", "Each segment must start with a letter", "[a-zA-Z0-9_]" [W:android-appid] |
| P02 | iOS / macOS bundle ID | `^[A-Za-z0-9-]+(?:\.[A-Za-z0-9-]+)+$` | store as observed; compare case-insensitively (Apple docs conflict) | var | | Info.plist `CFBundleIdentifier`, provisioning profiles, MDM | H | type:mobile-application-id | Software / Text | Doc: letters, digits, hyphen, period; glossary says case-insensitive, Xcode help and CFBundle API say case-sensitive [W:apple-bundle] |
| P03 | Apple App Store ID | `^[0-9]{6,12}$` (URL form `id[0-9]+`) | decimal | | | `apps.apple.com/.../id<n>` | M-H | text | Text | Obs |
| P04 | Chrome Web Store extension ID | `^[a-p]{32}$` | lowercase | 32 | first 128 bits of SHA-256(DER public key), each hex nibble 0-f mapped to a-p | `chrome-extension://<id>/`, CRX3 header, store URL, enterprise policies | H (bound to the signing key) | type:chrome-extension-id (l.752; AVT l.642-643); obj:browser-extension.id l.70 | Software / Text | Src MISP; derivation [W:crx-id] |
| P05 | Microsoft Edge Add-ons extension ID | `^[a-p]{32}$` | lowercase | 32 | same derivation | `microsoftedge.microsoft.com/addons/detail/<slug>/<id>` | H | type:edge-extension-id (l.756; AVT l.644-645) | Software / Text | Src MISP |
| P06 | Firefox add-on ID (gecko ID) | GUID form `^\{[0-9A-Fa-f]{8}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{4}-[0-9A-Fa-f]{12}\}$` or email-like `^[A-Za-z0-9._%+-]*@[A-Za-z0-9.-]+$` | as declared (`browser_specific_settings.gecko.id`) | | | manifest.json, XPI | H | obj:browser-extension.id (AMO slug or ID) | Software / Text | Obs |
| P07 | Firefox AMO slug / numeric add-on ID | slug `^[a-z0-9-]{1,100}$`; ID `^[0-9]{1,10}$` | lowercase | | | `addons.mozilla.org/.../addon/<slug>/` | M | obj:browser-extension.id | Text | Src MISP l.70 |
| P08 | VS Code / Open VSX extension ID | `^[A-Za-z0-9][A-Za-z0-9-]*\.[A-Za-z0-9][A-Za-z0-9-]*$` (`publisher.name`) | case-insensitive (lowercase) | | | marketplace URL `itemName=`, `extensions.json` | H (supply chain) | text | Software / Text | Obs |
| P09 | npm package name | `^(?:@[a-z0-9-~][a-z0-9-._~]*/)?[a-z0-9-~][a-z0-9-._~]*$` (<=214 chars) | lowercase | | | package.json, lockfiles | H (supply chain) | obj:malicious-package-report.package-name | Software / Text | Obs (npm naming rules) |
| P10 | PyPI project name | `^[A-Za-z0-9](?:[A-Za-z0-9._-]*[A-Za-z0-9])?$` | normalise per PEP 503: lowercase, runs of `-_.` to `-` | | | requirements, wheels | H | text | Software / Text | Doc (PEP 503 normalisation) |
| P11 | Package URL (purl) | `^pkg:[a-z][a-z0-9.+-]*/.+$` | per purl spec (type lowercase, name normalised per type) | | | SBOMs | H (cross-ecosystem canonical) | text | Software.cpe / swid only | Doc (package-url spec) |

---

## 18. Family Q: cloud, SaaS tenant and credential identifiers

| ID | Identifier | Validation regex | Case / canonical | Length | Checksum / derivation | Extraction | Pivot | MISP | OpenCTI | Evidence |
|---|---|---|---|---|---|---|---|---|---|---|
| Q01 | Entra ID (Azure AD) application (client) ID | `^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$` | lowercase GUID | 36 | none | OAuth consent-phishing URLs (`client_id=`), sign-in logs, malicious multi-tenant apps | H | type:azure-application-id (l.748, no GUID validation) | Text | Src MISP |
| Q02 | Entra ID tenant ID | GUID (as Q01) | lowercase | 36 | none | `login.microsoftonline.com/<tenant>/...`, OpenID configuration of a domain, Teams webhook URLs (D09), JWT `tid` | H (organisation) | text | Text | Obs |
| Q03 | Azure subscription ID | GUID | lowercase | 36 | none | ARM resource IDs `/subscriptions/<id>/` | H | text | Text | Obs |
| Q04 | AWS access key ID | `^(?:AKIA\|ASIA\|ABIA\|ACCA)[A-Z2-7]{16}$` (gitleaks also `A3T[A-Z0-9]`) | uppercase | 20 | encodes the account ID (public research, not verified here) | malware configs, phishing kits (S3 exfil), leaked repos | H | obj:credential (text) | Credential / Text | Src gitleaks `aws-access-token` [W:gitleaks]; Doc prefixes [W:aws-iam] |
| Q05 | AWS IAM unique ID | `^(?:AIDA\|AROA\|AGPA\|AIPA\|ANPA\|ANVA\|APKA\|ASCA)[A-Z0-9]{16,}$` | uppercase | ~21 | prefix = resource type (AIDA user, AROA role, AGPA group, AIPA instance profile, ANPA managed policy, ANVA policy version, APKA public key, ASCA certificate) | CloudTrail, policies | M-H | text | Text | Doc [W:aws-iam] |
| Q06 | AWS account ID | `^[0-9]{12}$` | 12 digits (keep leading zeros) | 12 | none | ARNs `arn:aws:...:<account>:...`, S3 bucket policies | H | text | Text | Doc [W:aws-iam] |
| Q07 | AWS ARN | `^arn:aws(?:-[a-z]+)*:[a-z0-9-]+:[a-z0-9-]*:[0-9]{12}?:.+$` | as-is | | contains Q06 | logs, policies | H | text | Text | Doc [W:aws-iam] |
| Q08 | GCP project ID | `^[a-z][a-z0-9-]{4,28}[a-z0-9]$` | lowercase | 6-30 | none | Firebase configs (C02), API URLs, service account emails | H | text | Text | Obs |
| Q09 | GCP service account email | `^[a-z][a-z0-9-]{4,28}[a-z0-9]@[a-z][a-z0-9-]{4,28}[a-z0-9]\.iam\.gserviceaccount\.com$` | lowercase | | contains Q08 | leaked JSON keys | H | type:email | Email-Addr | Obs |
| Q10 | Prefixed secret tokens | GitHub `^gh[pousr]_[0-9A-Za-z]{36}$`, `^github_pat_[0-9A-Za-z_]{82}$`; GitLab `^glpat-[0-9A-Za-z_-]{20}$`; Slack `^xox[baprs]-[0-9A-Za-z-]{10,}$`; Stripe `^(?:sk\|rk)_(?:test\|live\|prod)_[0-9A-Za-z]{10,99}$`; OpenAI `^sk-(?:proj\|svcacct\|admin)-[A-Za-z0-9_-]+$` | case-sensitive | | some embed workspace or user IDs (Slack) | leaks, malware, kits | H (but secret) | obj:credential.password / type api-key | Credential | Src gitleaks rules [W:gitleaks] |

---

## 19. Family R: hosting, dead-drop and anonymity-network identifiers

| ID | Identifier | Validation regex | Canonical | Checksum | Extraction | Pivot | MISP | OpenCTI |
|---|---|---|---|---|---|---|---|---|
| R01 | Pastebin paste key | `^[A-Za-z0-9]{8}$` | case-sensitive | none | `pastebin.com/raw/<key>` in loaders | M-H | obj:paste | Text / Url |
| R02 | Discord CDN attachment URL | `^https://(?:cdn\|media)\.discordapp\.(?:com\|net)/attachments/[0-9]{17,20}/[0-9]{17,20}/[^/?#]+` | host lowercase; signed query params (`ex`, `is`, `hm`) not part of identity | snowflakes (channel ID gives the uploader's channel) | payload hosting in loaders | H (channel ID) | url | Url |
| R03 | Google Drive / Docs file ID | `^[A-Za-z0-9_-]{25,44}$` | case-sensitive | none | `drive.google.com/file/d/<id>/`, `uc?id=` | M-H | text | Text |
| R04 | IPFS CID | v0 `^Qm[1-9A-HJ-NP-Za-km-z]{44}$`; v1 base32 `^b[a-z2-7]{58,}$` | canonical CIDv1 base32 lowercase | multihash self-describing | IPFS gateway phishing URLs (`/ipfs/<cid>`, `<cid>.ipfs.<gateway>`) | H (content-addressed) | text | Text |
| R05 | Tor onion service v3 | `^[a-z2-7]{56}\.onion$` | lowercase | last 3 bytes of the base32-decoded 35 bytes: 2-byte checksum = first 2 bytes of SHA3-256(".onion checksum" + pubkey + version) and version byte `0x03` (Tor rend-spec-v3) | leak sites, ransom notes | H | type:onion-address (l.776; accepts v2 16-char legacy) | Domain-Name / Text |
| R06 | I2P b32 address | `^[a-z2-7]{52}\.b32\.i2p$` | lowercase | none | | M | text | Domain-Name / Text |
| R07 | Blockchain-hosted C2 contract (EtherHiding style) | see E05 | lowercase | EIP-55 | JS loaders calling BSC / Ethereum RPC | H | text | Cryptocurrency-Wallet / Text |

---

## 20. Checksum, decoding and derivation algorithms (reference)

| Algorithm | Used by | Definition (summary) | Source |
|---|---|---|---|
| Luhn (mod 10) | IMEI (N01), ICCID (N09), payment card (E16) | From the rightmost digit (the check digit) moving left, double every second digit, subtract 9 from results above 9, sum all digits; valid if sum mod 10 = 0. IMEI: CD computed over the 14 digits TAC+SNR; IMEISV has no CD. | 3GPP TS 23.003 annex B [W:3gpp-23003]; ISO/IEC 7812-1 annex B [W:e118] |
| Luhn over hex digits | MEID check digit (N04) | Luhn variant applied to base-16 digits | 3GPP2 (not verified here) |
| ISO 7064 MOD 97-10 | IBAN (E14), EID (N10) | IBAN: move the first 4 characters to the end, map A..Z to 10..35, interpret as integer, valid if mod 97 = 1. EID: 32-digit integer mod 97 = 1; check digits = 98 - (number with last two digits zeroed mod 97). | ISO 13616; GSMA SGP.29 section 10 and pySim implementation [W:eid] |
| Base58Check | Bitcoin legacy (E03), Litecoin, Dogecoin, TRON (E07) | alphabet `123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`; payload = version byte(s) + data; checksum = first 4 bytes of SHA-256(SHA-256(payload)) | Bitcoin docs |
| Ripple Base58 | XRP (E11) | alphabet `rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz`, double SHA-256 checksum | XRPL docs |
| Bech32 / Bech32m | Bitcoin SegWit (E04), Litecoin `ltc1` | BCH code over 5-bit groups with HRP expansion; final polymod constant 1 (Bech32, BIP-173, witness v0) or `0x2bc830a3` (Bech32m, BIP-350, witness v1+); max 90 chars; mixed case invalid | BIP-173, BIP-350 |
| CashAddr | Bitcoin Cash (E10) | 40-bit BCH checksum with prefix `bitcoincash` | Bitcoin Cash spec |
| EIP-55 | EVM addresses (E05, R07) | hash = keccak256(lowercase hex address without `0x`); uppercase the i-th letter iff the i-th nibble of hash >= 8; an all-lowercase or all-uppercase address carries no checksum | EIP-55 |
| Monero base58 | Monero (E06) | data split in 8-byte blocks, each encoded to 11 chars (last block shorter); checksum = first 4 bytes of Keccak-256(network byte + keys [+ payment id]) | Monero docs |
| Onion v3 | R05 | 32-byte ed25519 pubkey + 2-byte checksum + version `0x03`, base32 lowercase (56 chars); checksum = SHA3-256(".onion checksum" + pubkey + version)[0..2] | Tor rend-spec-v3 |
| Tox ID checksum | M06 | 2-byte checksum = XOR of all 2-byte chunks of (public key + nospam) | Tox spec |
| Snowflake decode | Discord (O07, D05, R02) | timestamp_ms = (id >> 22) + 1420070400000; worker = (id & 0x3E0000) >> 17; process = (id & 0x1F000) >> 12; increment = id & 0xFFF | Discord developer reference [W:discord-ref] |
| Snowflake decode | X / Twitter (O15) | timestamp_ms = (id >> 22) + 1288834974657 | Twitter snowflake (Obs) |
| Chrome extension ID | P04, P05 | SHA-256 of the DER SubjectPublicKeyInfo, take first 16 bytes, hex-encode, map `0-9a-f` to `a-p` | [W:crx-id] |
| Favicon mmh3 | H03 | `mmh3.hash(base64.encodebytes(data))` (seed 0, signed result); `encodebytes` inserts `\n` every 76 output chars and at the end | [W:favicon] |
| JA3 / JA3S | F01-F03 | MD5 of the decimal comma/hyphen string | [W:ja3] |
| JA4+ truncated hash | F04-F07, H01 | first 12 hex chars of SHA-256 of the comma-joined list | [W:ja4-spec] |
| HHHash | H02 | `hhh:1:` + hex SHA-256 of header names joined by `:` in response order | [W:hhhash] |
| SSH fingerprints | G05, G06 | SHA-256 of the key blob, base64 without padding, prefix `SHA256:`; MD5 of the key blob as colon-separated hex | OpenSSH; MISP AVT l.745-757 |
| OpenPGP fingerprint and key ID | M02-M04 | v4: SHA-1 over 0x99 + 2-byte length + key packet body, key ID = low 64 bits; v6: SHA-256 over 0x9B + 4-byte length + body, key ID = high 64 bits | RFC 9580 section 5.5.4 |
| X.509 fingerprints | F10-F12, F14 | hash over the DER certificate; SPKI pin = SHA-256 over DER SubjectPublicKeyInfo | RFC 5280, RFC 7469 |
| IMEISV to IMEI | N02 to N01 | drop the 2 SVN digits, append Luhn CD over TAC+SNR | 3GPP TS 23.003 |
| SLAAC IPv6 to MAC | N12, N14 | if interface identifier bytes 3-4 are `ff:fe`, remove them and flip bit `0x02` of the first byte | RFC 4291 appendix A |
| Telegram bot token to bot ID | D01 to D02 | integer before `:` | Bot API [W:tg-botapi] |
| Discord bot token to user ID | D06 | base64url-decode the first dot-separated segment | Obs |
| Slack webhook to workspace ID | D07 to D08 | path segment starting with `T` | TruffleHog regex [W:th-slackwebhook] |
| Teams webhook to tenant ID | D09 to Q02 | GUID after `@` in the `webhookb2` path | TruffleHog regex [W:th-teams] |
| UA tracking ID to GA account | A01 to A02 | drop the trailing `-N` | Bellingcat [W:bellingcat-2017] |
| Sentry DSN components | A33 to A34 | named groups `key`, `org`, `project` of block A33 | [W:sentry-dsn] |
| AWS access key ID to account ID | Q04 to Q06 | offline base32 decoding technique published by researchers | public research, not verified here |

---

## 21. Recommendations for Fennec (`REC`, not source facts)

### 21.1 Four kinds of "pivot observables", with different domain contracts

| Kind | Definition | Equality / search | Examples | Domain contract |
|---|---|---|---|---|
| Identifier | Assigned by a platform or standard; names an account, tenant, device, app or resource | exact match on canonical value | A01-A38, B, C, D, E, K, L01, L04-L06, N, O, P, Q, R | `parse` + canonical form + typed components + optional checksum |
| Fingerprint | Deterministic digest computed from an observed artefact (handshake, certificate, key, page, binary) | exact match on canonical value | F01-F07, F09-F16, G01-G07, H01-H12, I01-I07, J01, J03-J06, J12, J14, L02, M01-M04 | as Identifier, plus `algorithm` and (when relevant) `algorithm_version` / producer profile (HHHash crawler, Shodan namespace) |
| Similarity digest | Digest designed for distance comparison | distance or score, not equality | J02, J07-J11, J13, H13 | separate trait with `distance(&self, other: &Self) -> u32` or `score`; cannot be a uniqueness key for pivoting by itself; needs dedicated indexing |
| Measurement / transient | Values that describe one session or are reallocated | none (attribute of a sighting) | F08 (JA4L/JA4LS), G04 (JA4SSH), H14 (Community ID), N08 (TMSI/GUTI), I06-I07 (JA4D, DHCP device types, low CTI value) | not an aggregate; value object on an observation / network-session entity |

### 21.2 Type determination

- Never infer the type from the value alone: 32 hex (MD5, JA3, JA3S, HASSH, HASSHServer, imphash, Rich hash, Mixpanel, Amplitude, Baidu, Cloudflare token, Sentry key, MISP dom-hash, X.509 MD5) and pure digits (Meta pixel, Facebook app, Yandex, Hotjar, LinkedIn, Pinterest, UET, HubSpot, AnyDesk, TeamViewer, Telegram and Discord IDs, X IDs, CS watermark) are syntactically ambiguous (section 1.5 item 10; MISP itself offers 4 candidate types for 32 hex, ComplexTypeTool l.35).
- Provide one **extractor** per type that carries the context pattern (columns "Typical extraction" above). Free-text import may propose candidate types (as MISP does) but the analyst or the source connector must choose.

### 21.3 Proposed aggregates and priority

P0 (high pivot value, frequent in reporting, or has a MISP/OpenCTI counterpart to round-trip):

| Family | Aggregates |
|---|---|
| Web analytics and ads | `GoogleAnalyticsTrackingId` (UA) with derived `GoogleAnalyticsAccountId`, `Ga4MeasurementId`, `GoogleTagManagerContainerId`, `GoogleAdsConversionId`, `AdSensePublisherId`, `MetaPixelId`, `YandexMetricaCounterId`, `BaiduTongjiSiteKey`, `HotjarSiteId`, `MicrosoftClarityProjectId`, `BingUetTagId`, `TikTokPixelId`, `SentryDsn` with derived `SentryOrganizationId`, `SentryProjectId`, `SentryPublicKey` |
| Verification tokens | `GoogleSiteVerificationToken`, `BingSiteVerificationToken`, `YandexSiteVerificationToken`, `Microsoft365DomainVerificationToken` |
| Exfiltration endpoints | `TelegramBotToken` (secret) with derived `TelegramBotId`, `TelegramChatId`, `DiscordWebhookUrl` (secret) with derived `DiscordWebhookId`, `SlackWebhookUrl` (secret) with derived `SlackWorkspaceId`, `TeamsWebhookUrl` (secret) with derived `EntraTenantId` |
| TLS | `Ja3Fingerprint`, `Ja3sFingerprint`, `Ja4Fingerprint`, `Ja4sFingerprint`, `Ja4xFingerprint`, `JarmFingerprint`, X.509 fingerprints as value objects of an `X509Certificate` aggregate plus standalone `X509SpkiSha256` |
| SSH | `HasshFingerprint`, `HasshServerFingerprint`, `SshHostKeyFingerprint` (algorithm SHA256 or MD5 as enum inside one type, because both describe the same key) or two types if strictly one-algorithm-per-type is preferred, `SshPublicKey` |
| HTTP content | `FaviconMmh3`, `HhHash`, `MispDomHash`, `Ja4hFingerprint` |
| Binary | `Imphash`, `Authentihash`, `PeHash`, `RichHeaderHash`, `CdHash`, `Vhash`; similarity: `Tlsh`, `Ssdeep`, `Telfhash`, `Impfuzzy` |
| C2 | `CobaltStrikeWatermark`, `CobaltStrikePublicKey` |
| Keys | `PgpFingerprint` (v4 / v6 variants), `PgpKeyId` (long only) |
| Telecom | `Imei`, `Imeisv`, `Imsi`, `Iccid`, `PhoneNumber` (E.164), `MacAddress` (EUI-48), `Eui64` |
| Accounts | `TelegramUsername`, `TelegramUserId`, `DiscordUsername`, `DiscordUserId`, `DiscordGuildId`, `DiscordInviteCode`, `GitHubUsername`, `GitHubUserId`, `GitHubRepository`, `XHandle`, `XUserId`, `MastodonHandle` |
| Apps and extensions | `AndroidPackageName`, `AppleBundleId`, `ChromeExtensionId`, `EdgeExtensionId`, `FirefoxAddonId` |
| Cloud | `EntraApplicationId`, `EntraTenantId`, `AwsAccessKeyId` (secret-adjacent), `AwsAccountId` |
| Other | `OnionV3Address`; cryptocurrency addresses per chain if not owned by a sibling design |

P1: remaining rows of families A-C, D10-D14, F01/F05 raw strings, G03, G08, H04-H12, I01-I05, J09-J13, K01-K11, L04-L06, M06-M07, N04-N07, N09-N17, O rest, P rest, Q rest, R rest.

P2 (value objects on observations, not aggregates): F08, G04, H14, I06-I08, N08.

### 21.4 Value object rules

1. `parse(&str) -> Result<T, ValidationError>` = trim, normalise (case, separators, prefixes such as `ca-pub-` to `pub-`, `0x`, colons), validate (regex, length, numeric range, checksum), then build the canonical form. Keep the raw input on the observation for provenance, not on the value object.
2. Expose typed components instead of re-parsing strings: `GoogleAnalyticsTrackingId::account()`, `Imei::tac()`, `Imsi::plmn()`, `Iccid::issuer_prefix()`, `MacAddress::{oui(), is_multicast(), is_locally_administered()}`, `DiscordSnowflake::created_at()`, `SentryDsn::{org_id(), project_id(), public_key()}`, `TelegramBotToken::bot_id()`, `Ja4Fingerprint::{transport(), tls_version(), sni(), cipher_count(), extension_count(), alpn(), cipher_hash(), extension_hash()}`, `JarmFingerprint::is_null()`, `FaviconMmh3::compute(bytes)`.
3. Native numeric canonical types where the domain is numeric: `FaviconMmh3(i32)` (accept unsigned input and convert), `CobaltStrikeWatermark(u32)` (PostgreSQL `BIGINT`), snowflakes `u64` (PostgreSQL `BIGINT` holds every snowflake whose top bit is unset; `NUMERIC(20)` if full u64 must be guaranteed), Telegram IDs `i64`; keep `AwsAccountId` as a 12-character string (leading zeros are significant).
4. Case policy per type (from the tables): lowercase canonical for case-insensitive handles and hex digests; preserve case for base58, base64, tokens, bundle IDs (but compare bundle IDs case-insensitively).
5. Use `[0-9]` not `\d` in Rust regexes; implement non-regular rules (Discord consecutive periods, GitHub hyphens, GitLab suffixes, checksums, numeric ranges) in code.
6. `Obs` regexes must be lenient and versioned: vendors change formats (GA4 length, TikTok, Clarity). Provide a "strict" and a "lenient" validation mode, with ingest defaulting to lenient plus a data-quality flag.
7. Secret-bearing values (`TelegramBotToken`, Discord webhook and bot tokens, Slack and Teams webhook URLs, `AwsAccessKeyId` with its secret if present, GitHub, Stripe and other tokens, Sentry DSN secret part): redact in `Debug`/`Display`, never log, store encrypted or hashed according to policy, and always emit the non-secret derived identifier as its own observable so pivoting does not require exposing the secret.
8. Personal data: phone numbers, IMSI, IMEI, ICCID, EID, MAC addresses, handles and numeric account IDs are personal data under GDPR; mark them so retention and sharing policies can apply.

### 21.5 Derived observables and relationships (domain events)

| Source observable | Derived observable(s) | Suggested relationship |
|---|---|---|
| `GoogleAnalyticsTrackingId` | `GoogleAnalyticsAccountId` | `part-of` |
| `AdMob app ID` (A10) | `AdSensePublisherId` | `part-of` |
| `GoogleTagManagerContainerId` | IDs found in the container JS (A01, A03, A06, A11) | `contains` (observed at time t) |
| `SentryDsn` | `SentryOrganizationId`, `SentryProjectId`, `SentryPublicKey`, self-hosted host (Hostname) | `part-of` / `resolves-to` |
| `TelegramBotToken` | `TelegramBotId`; with `chat_id` usage, `TelegramChatId` | `part-of`, `communicates-with` |
| `DiscordWebhookUrl` | `DiscordWebhookId` (creation time from snowflake) | `part-of` |
| `SlackWebhookUrl` | `SlackWorkspaceId` | `part-of` |
| `TeamsWebhookUrl` | `EntraTenantId` | `part-of` |
| `AwsAccessKeyId` | `AwsAccountId` | `belongs-to` |
| `X509Certificate` | SHA-256 / SHA-1 / MD5 fingerprints, `X509SpkiSha256`, serial (scoped by issuer), SKI, AKI, JA4X | attributes or `has` |
| `SshPublicKey` | `SshHostKeyFingerprint` (SHA256, MD5) | `has` |
| `PgpPublicKey` | `PgpFingerprint`, `PgpKeyId` | `has` |
| `CobaltStrikeBeaconConfig` (file-derived) | `CobaltStrikeWatermark`, `CobaltStrikePublicKey`, C2 URLs | `has` |
| `Imeisv` | `Imei` | `part-of` |
| `Imei` | TAC (device model) | attribute |
| `Imsi` | PLMN (operator) | attribute |
| IPv6 address with modified EUI-64 IID | `MacAddress` | `derived-from` (OpenCTI already models IPv6-Addr to Mac-Addr, `database/stix.ts` l.1192) |
| `ChromeExtensionId` | signing public key (Cryptographic-Key) | `derived-from` |
| `AndroidPackageName` | signing certificate digest (K10) | `signed-by` |

### 21.6 Interoperability mapping (export / import)

| Fennec aggregate | MISP | OpenCTI | STIX 2.1 |
|---|---|---|---|
| `Ja3Fingerprint` | `ja3-fingerprint-md5` / obj `ja3` | `Text` | custom (no core property) |
| `Ja3sFingerprint` | obj `ja3s.ja3s-fingerprint-md5` (type `ja3-fingerprint-md5`) | `Text` | custom |
| `Ja4*` | obj `ja4-plus` (`ja4-fingerprint` + `ja4-type`) | `Text` | custom |
| `JarmFingerprint` | `jarm-fingerprint` / obj `jarm` | `Text` | custom |
| `HasshFingerprint` / `HasshServerFingerprint` | `hassh-md5` / `hasshserver-md5` | `Text` | custom |
| `SshHostKeyFingerprint`, `SshPublicKey` | `ssh-fingerprint`; obj `passive-ssh`, `ssh-authorized-keys` | `SSH-Key` (identity `fingerprint_sha256`) | custom (OpenCTI `ssh-key` custom SCO in pycti, `constants.py` l.736-763) |
| X.509 fingerprints, serial, SKI/AKI | `x509-fingerprint-*`; obj `x509` | `X509-Certificate` | `x509-certificate` (`hashes`, `serial_number`, extensions) |
| `FaviconMmh3` | `favicon-mmh3` / obj `favicon` | `Text` | custom |
| `MispDomHash` | `dom-hash` | `Text` | custom |
| `HhHash` | obj `hhhash` | `Text` | custom |
| `Imphash` | `imphash` / obj `pe.imphash` | none (`Text`) | `file` `windows-pebinary-ext.imphash` |
| `Authentihash`, `PeHash`, `Vhash`, `CdHash`, `Telfhash`, `Impfuzzy` | dedicated MISP types | none (`Text`) | custom hash keys in `hashes` (allowed by `hashes-type.json` l.9) |
| `Tlsh`, `Ssdeep` | `tlsh`, `ssdeep` | `hashes.TLSH`, `hashes.SSDEEP` | `hashes.TLSH` (70-char form only), `hashes.SSDEEP` |
| Web analytics IDs (family A) | obj `tracking-id` (`id` + `tracker`) | `Text` (never `Tracking-Number`) | custom |
| Verification tokens (family B) | `text` | `Text` | custom |
| `TelegramBotToken` and derived | obj `telegram-bot` | `Text` or `Credential` | custom |
| `CobaltStrikeWatermark` | obj `cs-beacon-config.watermark` | `Text` | custom |
| `PgpFingerprint` / `PgpKeyId` | obj `pgp-key` | `Cryptographic-Key` (value) | custom |
| `Imei`, `Imsi`, `Iccid` | obj `phone` | `IMEI`, `IMSI`, `ICCID` (OpenCTI custom SCOs `imei`, `imsi`, `iccid`) | custom (`imei`, `imsi`, `iccid` as pycti defines them) |
| `PhoneNumber` | `phone-number` / obj `phone-number` | `Phone-Number` | custom (`phone-number`, OpenCTI) |
| `MacAddress` | `mac-address` | `Mac-Addr` | `mac-addr` |
| `Eui64` | `mac-eui-64` (MAC-derived only) | none | none |
| Handles and platform IDs | `github-*`, `twitter-id`, `jabber-id`, objs `telegram-account`, `twitter-account`, `github-user`, `gitlab-user`, ... | `User-Account` (`account_type` open vocabulary; only `facebook` and `twitter` predefined) or `Persona` | `user-account` (`account_type`, `user_id`, `account_login`) |
| `ChromeExtensionId`, `EdgeExtensionId` | `chrome-extension-id`, `edge-extension-id` | `Text` / `Software` | custom |
| `AndroidPackageName`, `AppleBundleId` | `mobile-application-id`, obj `android-app.appid` | `Text` / `Software` | custom |
| `EntraApplicationId` | `azure-application-id` | `Text` | custom |
| Cryptocurrency addresses | `btc`, `xmr`, `dash`, obj `coin-address` | `Cryptocurrency-Wallet` | custom (`cryptocurrency-wallet`, OpenCTI) |
| `OnionV3Address` | `onion-address` | `Domain-Name` / `Text` | `domain-name` |

"custom" = no STIX 2.1 core object or property; options are an OpenCTI-compatible `Text` SCO (lossy: the Fennec type must travel as label or description) or a STIX 2.1 custom SCO declared through an `extension-definition` of type `new-sco` (STIX 2.1 section 7.3) for Fennec-to-Fennec exchange. pycti only maps the SCO types listed in `client-python/pycti/utils/opencti_stix2_utils.py` l.115-154, so a new custom SCO type will not be mapped by OpenCTI tooling without a converter (inference from that list; ingestion behaviour not tested).

---

## 22. Open questions

1. Should SHA-256 and MD5 SSH host key fingerprints (and X.509 SHA-1/SHA-256/MD5) be one aggregate per algorithm (strict "one kind per type") or one aggregate per key/certificate with algorithm-typed value objects?
2. How will similarity digests (TLSH, ssdeep, telfhash, impfuzzy, sdhash, dHash) be indexed in PostgreSQL (TLSH bucketing, ssdeep chunk n-grams, pg extensions)? Equality indexes alone give little value.
3. Vendor-scoped hashes (Shodan `html_hash`, `headers_hash`, urlscan `dom.hash`, Censys body hashes): model as distinct aggregates per vendor, or as one fingerprint type with a mandatory `producer` value object?
4. Policy for secret-bearing observables: store the secret at all, store only a keyed hash plus derived public IDs, or store encrypted with access control?
5. Which cracked or leaked Cobalt Strike watermark values should be flagged as low-uniqueness, and who maintains that list?
6. Is a JA4+ licence review needed (FoxIO License 1.1 restricts monetisation for all methods except JA4)? Storing values produced by third parties may be unaffected, but computing them inside Fennec may not be.
7. For handles (Telegram, X, GitHub, Discord), should the aggregate identity be the handle string with validity intervals, or should handles be value objects attached to the stable numeric-ID aggregate?
8. Do we need the long tail of verification tokens and analytics vendors as first-class types, or a closed set plus an extension process (strict no-generic rule implies a new type per vendor)?
9. Sibling research may already define cryptocurrency, banking, IP, domain and URL types; families E14-E17 and R should be reconciled with it.

---

## 23. External sources (fetched 2026-10-25)

| Key | URL | Used for |
|---|---|---|
| ja4-repo | https://github.com/FoxIO-LLC/ja4 | JA4+ method list, licences, published example values |
| ja4-spec | https://raw.githubusercontent.com/FoxIO-LLC/ja4/main/technical_details/JA4.md | JA4 structure, version codes, GREASE, ALPN, raw forms |
| ja4h-md | https://github.com/FoxIO-LLC/ja4/blob/main/technical_details/JA4H.md | JA4H header count rule |
| ja4-img-s / -h / -x / -ssh / -t / -l / -d / -d6 | https://github.com/FoxIO-LLC/ja4/tree/main/technical_details (JA4S.png, JA4H.png, JA4X.png, JA4SSH.png, JA4T.png, JA4L.png, JA4D.png, JA4D6.png) | field layouts of each JA4+ method |
| ja3 | https://github.com/salesforce/ja3 | JA3/JA3S construction, archive notice |
| jarm | https://github.com/salesforce/jarm | JARM structure and example |
| hassh | https://github.com/salesforce/hassh | HASSH construction, archive notice |
| chrome-permute | https://www.fastly.com/blog/a-first-look-at-chromes-tls-clienthello-permutation-in-the-wild ; https://www.peakhour.io/blog/tls-extension-randomisation/ | Chrome ClientHello extension permutation |
| hhhash | https://pypi.org/project/hhhash ; https://rud.is/b/2023/07/09/new-r-package-hhhash/ | HHHash algorithm and format |
| favicon | https://isc.sans.edu/diary/33110 ; https://www.iblue.team/general-notes-1/favicon-hashing-and-hunting-with-shodan.md | Shodan favicon mmh3 computation |
| shodan-root / shodan-http / shodan-ssl / shodan-ssh / shodan-cs | https://datapedia.shodan.io/ (property pages `http`, `ssl`, `ssh`, `cobalt_strike_beacon`) | Shodan property names and types |
| urlscan-search | https://urlscan.io/docs/search/ | urlscan search fields (`hash`, `dom.hash`, `text.hash`, `content.identifiers`) |
| dt-params | https://docs.domaintools.com/api/iris/investigate/search/base-parameters/ | DomainTools tracking-code and SSL pivot parameters |
| dt-fall2023 | https://www.domaintools.com/blog/whats-new-with-iris-investigate-and-enrich-fall-2023 | tracker capture added September 2023 |
| bellingcat-2015 | https://www.bellingcat.com/resources/2015/07/23/unveiling-hidden-connections-with-google-analytics-ids/ | analytics and AdSense pivot method |
| bellingcat-2017 | https://www.bellingcat.com/news/africa/2017/08/04/guptaleaks-google-analytics/ | UA `-N` suffix and account linkage |
| hackertarget | https://hackertarget.com/reverse-analytics-search/ | reverse analytics lookup and planted-ID caveat |
| ga-ua | https://support.google.com/analytics/answer/9310895 | UA property ID format |
| google-tag-ids | https://support.google.com/analytics/answer/12002338 ; https://support.google.com/google-ads/answer/14855153 | G-, AW-, DC-, GT-, GTM- tag IDs |
| adsense | https://support.google.com/adsense/answer/105516 | AdSense publisher ID format |
| google-txt | https://support.google.com/a/answer/2716802 | google-site-verification TXT length (68 / 43) |
| fb-dv | https://help.kliken.com/en/articles/9406060-how-to-verify-your-domain-for-facebook-ads (and similar guides) | Meta domain verification (unique per domain claim) |
| yandex-metrica | https://yandex.com/support/metrica/code/counter-initialize.html | Yandex Metrica `ym(ID, "init")` |
| clarity | https://learn.microsoft.com/en-us/clarity/google-tag-manager ; https://docs.tealium.com/client-side-tags/microsoft-clarity-tag/ | Clarity project ID |
| uet | https://help.ads.microsoft.com/apex/index/3/en/56688 ; https://help.cimcloud.com/help-center/microsoft-universal-event-tracking-uet-installation/ | UET `ti` tag ID |
| hotjar | https://help.hotjar.com/hc/en-us/articles/36820044421393-What-is-the-Hotjar-Tracking-Code | hjid / hjsv |
| baidu | https://www.trackingplan.com/blog/baidu-tracking | Baidu `hm.js?<key>` |
| tiktok | https://ads.tiktok.com/help/article/how-to-create-and-access-tiktok-pixel-id | TikTok pixel (no published format) |
| apify-pixel | https://apify.com/trovevault/ad-pixel-id-extractor | third-party pixel ID examples (Obs) |
| cf-wa | https://developers.cloudflare.com/web-analytics/faq/ | Cloudflare Web Analytics beacon token |
| sentry-dsn | https://forum.sentry.io/t/organization-subdomains-in-dsns/9360 ; https://www.mintlify.com/getsentry/sentry/concepts/projects | DSN structure `o<org>.ingest.sentry.io/<project>` |
| gitleaks | https://raw.githubusercontent.com/gitleaks/gitleaks/master/config/gitleaks.toml | AWS, GCP, GitHub, GitLab, Slack, Telegram, New Relic, Stripe, OpenAI regexes |
| th-* | https://github.com/trufflesecurity/trufflehog/tree/main/pkg/detectors (discordwebhook, discordbottoken, slackwebhook, microsoftteamswebhook, telegrambottoken, mixpanel, amplitudeapikey, segmentapikey, ngrok) | detector regexes |
| tg-botapi | https://core.telegram.org/bots/api | bot token example, 52-bit IDs |
| tg-fragment | https://core.telegram.org/api/fragment ; https://bugs.telegram.org/c/10735 | collectible usernames, 5-char minimum discussion |
| discord-ref | https://docs.discord.com/developers/reference | snowflake layout and epoch |
| discord-usernames | https://support.discord.com/hc/articles/12620128861463 | new username rules |
| gh-usernames | https://docs.github.com/en/enterprise-server@3.8/admin/identity-and-access-management/iam-configuration-reference/username-considerations-for-external-authentication | 39-char limit and dash rules |
| mastodon | https://github.com/mastodon/mastodon/blob/main/app/models/account.rb | USERNAME_RE, length limit |
| android-appid | https://developer.android.com/build/configure-app-module | applicationId naming rules |
| apple-bundle | https://developer.apple.com/help/glossary/bundle-id ; https://help.apple.com/xcode/mac/current/en.lproj/dev9b66ae7df.html | bundle ID characters and case-sensitivity conflict |
| crx-id | https://www.plasmo.com/blog/posts/how-to-create-a-consistent-id-for-your-chrome-extension ; https://siderite.dev/blog/compute-google-chrome-extension-ids-in.html | extension ID derivation |
| aws-iam | https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html | IAM unique ID prefixes, ARN format |
| 3gpp-23003 | https://itecspec.com/3gpp/23.003/s/6.2.1 ; https://www.itecspec.com/3gpp/23.003/s/2.2 | IMEI, IMEISV, IMSI structure |
| e118 | https://en.wikipedia.org/wiki/E.118 ; https://www.npmjs.com/package/e118-iin-list | ICCID structure and Luhn |
| eid | https://www.gsma.com/solutions-and-impact/technologies/esim/wp-content/uploads/2024/03/SGP.29-v1.1.pdf ; https://gitea.osmocom.org/sim-card/pysim/src/branch/master/pySim/euicc.py | EID length and mod-97 check digits |
| tlsh | https://github.com/trendmicro/tlsh | TLSH digest format |
| telfhash | https://github.com/trendmicro/telfhash | telfhash output |
| vt-files | https://docs.virustotal.com/reference/files | VirusTotal tlsh, permhash, vhash, main_icon.dhash |
| 1768 | https://github.com/DidierStevens/DidierStevensSuite/blob/master/1768.py | Cobalt Strike config IDs 0x07 and 0x25, licence ID decoding, public key prefix |
| cs-watermark | https://www.nccgroup.com/research/mining-data-from-cobalt-strike-beacons/ ; https://www.elastic.co/security-labs/extracting-cobalt-strike-beacon-configurations | watermark semantics and cracked builds |
