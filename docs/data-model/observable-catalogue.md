# Observable catalogue

Every observable kind Fennec models or plans to model. Each row becomes one
module in `libs/kernel/src/<module>/`, built as described in
[observable-kinds.md](observable-kinds.md), and gets its own page under [kinds/](kinds/README.md) when it is
implemented.

Kinds are delivered in **waves**. A wave starts once the previous one is
merged, and within a wave kinds are delivered one module (one pull request)
at a time. The **Status** column is updated in the pull request that
delivers the kind.

Column legend:

- **Module**: the `kernel` module and the snake case stem of every type in it.
- **STIX type**: the type prefix of the identifier (see
  [identity.md](identity.md#stix-type-names)). `x-fennec-*` types are
  Fennec-only.
- **OpenCTI** and **MISP**: the counterpart on import and export. `Text` and
  `text` mean the platform has no typed home for the kind and the mapping is
  lossy.
- **Canonical form**: what the value object stores and what feeds the
  identifier.
- **Column**: the PostgreSQL type of the natural key.
- **Id**: the identity regime, `A` (value-identified, UUID v5) or `B`
  (evolving, UUID v4).
- **PD**: marks kinds that are personal data.
- **Status**: `planned`, `in memory` (the module exists with the in-memory
  repository), or `postgres` (the PostgreSQL repository and table exist too).
  Implemented kinds link to their page.

Grammars quoted below are anchored Rust `regex` patterns. Grammars marked
*observed* come from real-world samples because the vendor publishes no
format; their sources are in
[../research/pivot-identifiers.md](../research/pivot-identifiers.md).

## Wave 1: network and host values

| Module | STIX type | OpenCTI | MISP | Canonical form | Column | Id | PD | Status |
|---|---|---|---|---|---|---|---|---|
| `ipv4_address` | `ipv4-addr` | IPv4-Addr | `ip-src`, `ip-dst` | Dotted quad, no leading zeros, no `/32` | `inet`, `CHECK (family = 4 AND masklen = 32)` | A | | [postgres](kinds/ipv4-address.md) |
| `ipv4_network` | `ipv4-addr` | IPv4-Addr | `ip-src`, `ip-dst` (CIDR) | Network address with zero host bits, prefix `/0` to `/31` | `cidr`, `CHECK (family = 4 AND masklen < 32)` | A | | [postgres](kinds/ipv4-network.md) |
| `ipv6_address` | `ipv6-addr` | IPv6-Addr | `ip-src`, `ip-dst` | RFC 5952 lowercase, no `/128` | `inet`, `CHECK (family = 6 AND masklen = 128)` | A | | [postgres](kinds/ipv6-address.md) |
| `ipv6_network` | `ipv6-addr` | IPv6-Addr | `ip-src`, `ip-dst` (CIDR) | RFC 5952 network address, prefix `/0` to `/127` | `cidr`, `CHECK (family = 6 AND masklen < 128)` | A | | planned |
| `domain_name` | `domain-name` | Domain-Name | `domain` | Lowercase A-label, no trailing dot, at least two labels | `text COLLATE "C"` | A | | [postgres](kinds/domain-name.md) |
| `hostname` | `hostname` | Hostname | `hostname` | Lowercase A-label, no trailing dot, one or more labels | `text COLLATE "C"` | A | | [postgres](kinds/hostname.md) |
| `url` | `url` | Url | `url`, `uri`, `link` | WHATWG serialisation | `text COLLATE "C"` plus a `bytea` SHA-256 unique key | A | | planned |
| `email_address` | `email-addr` | Email-Addr | `email`, `email-src`, `email-dst` | Lowercase NFC local part, domain as A-label | `text COLLATE "C"` | A | PD | [postgres](kinds/email-address.md) |
| `mac_address` | `mac-addr` | Mac-Addr | `mac-address` | EUI-48, lowercase, colon separated | `macaddr` | A | PD | [postgres](kinds/mac-address.md) |
| `autonomous_system` | `autonomous-system` | Autonomous-System | `AS` | Number `0..=4294967295` (the `AS` prefix and asdot notation are accepted on input); optional name and RIR | `bigint` | A | | planned |

## Wave 2: web tracking and site ownership identifiers

Values embedded in web pages and DNS records that tie sites to one operator.
None has a STIX type, MISP stores them as free text in the `tracking-id`
object, and OpenCTI can only hold them as `Text`.

| Module | STIX type | Grammar (input) | Canonical form | Column | Id | Status |
|---|---|---|---|---|---|---|
| `google_analytics_tracking_id` | `x-fennec-google-analytics-tracking-id` | `^UA-[0-9]{4,10}-[0-9]{1,4}$` (case-insensitive prefix) | `UA-<account>-<property>`; exposes the account number and the property index | `text COLLATE "C"` | A | planned |
| `google_analytics_account_id` | `x-fennec-google-analytics-account-id` | `^UA-[0-9]{4,10}$` | `UA-<account>` (derived from a tracking id, or seen alone) | `text COLLATE "C"` | A | planned |
| `google_analytics_measurement_id` | `x-fennec-google-analytics-measurement-id` | `^G-[A-Z0-9]{6,12}$` (length observed) | Uppercase | `text COLLATE "C"` | A | planned |
| `google_tag_id` | `x-fennec-google-tag-id` | `^GT-[A-Z0-9]{6,12}$` (length observed) | Uppercase | `text COLLATE "C"` | A | planned |
| `google_tag_manager_container_id` | `x-fennec-google-tag-manager-container-id` | `^GTM-[A-Z0-9]{4,10}$` | Uppercase | `text COLLATE "C"` | A | planned |
| `google_ads_conversion_id` | `x-fennec-google-ads-conversion-id` | `^AW-[0-9]{6,12}$`; a `/label` suffix is accepted and dropped | `AW-<digits>` | `text COLLATE "C"` | A | planned |
| `google_adsense_publisher_id` | `x-fennec-google-adsense-publisher-id` | `^(ca-\|partner-)?pub-[0-9]{16}$` | `pub-<16 digits>` | `text COLLATE "C"` | A | planned |
| `meta_pixel_id` | `x-fennec-meta-pixel-id` | `^[0-9]{15,16}$` (observed) | Decimal | `bigint` | A | planned |
| `facebook_app_id` | `x-fennec-facebook-app-id` | `^[0-9]{14,17}$` (observed) | Decimal | `bigint` | A | planned |
| `yandex_metrica_counter_id` | `x-fennec-yandex-metrica-counter-id` | `^[0-9]{5,10}$` | Decimal | `bigint` | A | planned |
| `microsoft_clarity_project_id` | `x-fennec-microsoft-clarity-project-id` | `^[a-z0-9]{10}$` (observed) | Lowercase | `text COLLATE "C"` | A | planned |
| `microsoft_uet_tag_id` | `x-fennec-microsoft-uet-tag-id` | `^[0-9]{6,10}$` | Decimal | `bigint` | A | planned |
| `hotjar_site_id` | `x-fennec-hotjar-site-id` | `^[0-9]{5,9}$` | Decimal | `bigint` | A | planned |
| `baidu_tongji_site_key` | `x-fennec-baidu-tongji-site-key` | `^[0-9a-f]{32}$` | Lowercase hex | `bytea` (16 bytes) | A | planned |
| `tiktok_pixel_id` | `x-fennec-tiktok-pixel-id` | `^[A-Z0-9]{20}$` (observed) | Uppercase | `text COLLATE "C"` | A | planned |
| `linkedin_insight_partner_id` | `x-fennec-linkedin-insight-partner-id` | `^[0-9]{4,9}$` (observed) | Decimal | `bigint` | A | planned |
| `google_site_verification_token` | `x-fennec-google-site-verification-token` | `^[A-Za-z0-9_-]{43}$` | As received (case-sensitive) | `text COLLATE "C"` | A | planned |
| `bing_site_verification_token` | `x-fennec-bing-site-verification-token` | `^[0-9A-Fa-f]{32}$` | Uppercase hex | `bytea` (16 bytes) | A | planned |
| `yandex_site_verification_token` | `x-fennec-yandex-site-verification-token` | `^[0-9a-f]{16}$` | Lowercase hex | `bytea` (8 bytes) | A | planned |
| `microsoft_domain_verification_token` | `x-fennec-microsoft-domain-verification-token` | `^MS=ms[0-9]{8}$` | `MS=ms<8 digits>` | `text COLLATE "C"` | A | planned |
| `facebook_domain_verification_token` | `x-fennec-facebook-domain-verification-token` | `^[a-z0-9]{20,40}$` (observed) | Lowercase | `text COLLATE "C"` | A | planned |

## Wave 3: files and binary fingerprints

| Module | STIX type | OpenCTI | MISP | Canonical form | Column | Id | Status |
|---|---|---|---|---|---|---|---|
| `file` | `file` | StixFile | `md5`, `sha1`, `sha256`, `sha512`, `sha3-*`, `ssdeep`, `tlsh`, `filename`, `size-in-bytes`, object `file` | Typed hashes (MD5, SHA-1, SHA-256, SHA-512, SHA3-256, SHA3-512 as lowercase hex; ssdeep; TLSH), name, size, MIME type | One `bytea` column per digest with a partial unique index each | B | planned |
| `imphash` | `x-fennec-imphash` | Text | `imphash` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `authentihash` | `x-fennec-authentihash` | Text | `authentihash` | Lowercase hex, 64 (SHA-256) | `bytea` (32 bytes) | A | planned |
| `pehash` | `x-fennec-pehash` | Text | `pehash` | Lowercase hex, 40 | `bytea` (20 bytes) | A | planned |
| `rich_header_hash` | `x-fennec-rich-header-hash` | Text | object `pe` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `apple_cdhash` | `x-fennec-apple-cdhash` | Text | `cdhash` | Lowercase hex, 40 (truncated) or 64 | `bytea` | A | planned |
| `telfhash` | `x-fennec-telfhash` | Text | `telfhash` | Lowercase hex, 70, `t1` prefix stripped | `bytea` (35 bytes) | A | planned |
| `impfuzzy` | `x-fennec-impfuzzy` | Text | `impfuzzy` | `<blocksize>:<hash1>:<hash2>`, case-sensitive | `text COLLATE "C"` | A | planned |
| `vhash` | `x-fennec-vhash` | Text | `vhash` | As received, `^[0-9A-Za-z!.]+$` (observed, opaque) | `text COLLATE "C"` | A | planned |
| `elf_build_id` | `x-fennec-elf-build-id` | Text | none | Lowercase hex, 16 to 64 | `bytea` | A | planned |
| `pdb_path` | `x-fennec-pdb-path` | Text | `pdb` | Single line, case preserved | `text COLLATE "C"` | A | planned |

## Wave 4: network fingerprints

| Module | STIX type | OpenCTI | MISP | Canonical form | Column | Id | Status |
|---|---|---|---|---|---|---|---|
| `ja3_fingerprint` | `x-fennec-ja3-fingerprint` | Text | `ja3-fingerprint-md5` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `ja3s_fingerprint` | `x-fennec-ja3s-fingerprint` | Text | object `ja3s` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `ja4_fingerprint` | `x-fennec-ja4-fingerprint` | Text | object `ja4-plus` | `a_b_c` sections lowercase; exposes transport, TLS version, SNI flag, cipher and extension counts, ALPN and both hashes | `text COLLATE "C"` | A | planned |
| `ja4s_fingerprint` | `x-fennec-ja4s-fingerprint` | Text | object `ja4-plus` | Lowercase | `text COLLATE "C"` | A | planned |
| `ja4h_fingerprint` | `x-fennec-ja4h-fingerprint` | Text | object `ja4-plus` | Lowercase | `text COLLATE "C"` | A | planned |
| `ja4x_fingerprint` | `x-fennec-ja4x-fingerprint` | Text | object `ja4-plus` | `^[0-9a-f]{12}_[0-9a-f]{12}_[0-9a-f]{12}$` | `text COLLATE "C"` | A | planned |
| `ja4t_fingerprint` | `x-fennec-ja4t-fingerprint` | Text | object `ja4-plus` | Decimal sections joined by `_` | `text COLLATE "C"` | A | planned |
| `jarm_fingerprint` | `x-fennec-jarm-fingerprint` | Text | `jarm-fingerprint` | Lowercase hex, 62; exposes `is_null()` for the all-zero value | `bytea` (31 bytes) | A | planned |
| `hassh_fingerprint` | `x-fennec-hassh-fingerprint` | Text | `hassh-md5` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `hassh_server_fingerprint` | `x-fennec-hassh-server-fingerprint` | Text | `hasshserver-md5` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `ssh_key` | `ssh-key` | SSH-Key | `ssh-fingerprint` | Fingerprint `SHA256:<base64 without padding>` (contributing property `fingerprint_sha256`, the format OpenCTI producers use); the public key, when known, is a descriptive property | `bytea` (32 bytes) | A | planned |
| `ssh_key_md5_fingerprint` | `x-fennec-ssh-key-md5-fingerprint` | Text | `ssh-fingerprint` | Lowercase colon-separated hex, 16 octets (legacy OpenSSH form) | `bytea` (16 bytes) | A | planned |
| `favicon_mmh3` | `x-fennec-favicon-mmh3` | Text | `favicon-mmh3` | Signed 32-bit decimal (Shodan convention); unsigned input converted | `integer` | A | planned |
| `hhhash` | `x-fennec-hhhash` | Text | object `hhhash` | `hhh:1:<64 lowercase hex>` | `text COLLATE "C"` | A | planned |
| `misp_dom_hash` | `x-fennec-misp-dom-hash` | Text | `dom-hash` | Lowercase hex, 32 | `bytea` (16 bytes) | A | planned |
| `x509_spki_sha256` | `x-fennec-x509-spki-sha256` | Text | `text` plus tag | Lowercase hex, 64 (base64 pins accepted) | `bytea` (32 bytes) | A | planned |

## Wave 5: the remaining STIX 2.1 cyber-observables

| Module | STIX type | OpenCTI | MISP | Identifying properties | Id | Status |
|---|---|---|---|---|---|---|
| `mutex` | `mutex` | Mutex | `mutex` | `name`, case preserved | A | planned |
| `directory` | `directory` | Directory | object `directory` | `path`, case preserved | A | planned |
| `software` | `software` | Software | object `software`, `cpe` | `name`, `cpe` (2.3), `swid`, `vendor`, `version` | B | planned |
| `user_account` | `user-account` | User-Account | object `user-account` | `account_type` (open vocabulary: the STIX values plus any lowercase hyphenated name), `user_id`, `account_login` | B | planned |
| `windows_registry_key` | `windows-registry-key` | Windows-Registry-Key | `regkey`, `regkey\|value` | `key` (hive canonicalised, case preserved), `values` | B | planned |
| `x509_certificate` | `x509-certificate` | X509-Certificate | `x509-fingerprint-*`, object `x509` | `hashes` (one), `serial_number`; subject, issuer and validity as properties | B | planned |
| `email_message` | `email-message` | Email-Message | object `email` | `from_ref`, `subject`, `body` | B | planned |
| `network_traffic` | `network-traffic` | Network-Traffic | objects `network-connection`, `netflow` | `start`, `end`, `src_ref`, `dst_ref`, ports, `protocols`, extensions | B | planned |
| `process` | `process` | Process | object `process` | none (UUID v4 per the specification) | B | planned |
| `artifact` | `artifact` | Artifact | `malware-sample`, `attachment` | `hashes` (one), `payload_bin` | B | planned |

## Wave 6: telecom, financial and cryptocurrency identifiers

| Module | STIX type | OpenCTI | MISP | Canonical form | Column | Id | PD | Status |
|---|---|---|---|---|---|---|---|---|
| `phone_number` | `phone-number` | Phone-Number | `phone-number` | E.164 with `+`, no separators | `text COLLATE "C"` | A | PD | planned |
| `imei` | `imei` | IMEI | object `phone` | 15 digits; exposes the TAC and whether the check digit is valid | `bigint` | A | PD | planned |
| `imeisv` | `x-fennec-imeisv` | Text | `text` plus tag | 16 digits; derives the IMEI (a 16-digit OpenCTI `imei` imports as this kind) | `bigint` | A | PD | planned |
| `imsi` | `imsi` | IMSI | object `phone` | 6 to 15 digits; exposes MCC and MNC (OpenCTI only accepts 14 or 15 digits, so shorter values do not export) | `text COLLATE "C"` | A | PD | planned |
| `iccid` | `iccid` | ICCID | object `phone` | `89` plus 16 to 20 digits, filler `F` stripped | `text COLLATE "C"` | A | PD | planned |
| `bitcoin_address` | `cryptocurrency-wallet` | Cryptocurrency-Wallet | `btc` | Base58Check (case preserved) or Bech32/Bech32m (lowercase), checksum enforced | `text COLLATE "C"` | A | | planned |
| `ethereum_address` | `cryptocurrency-wallet` | Cryptocurrency-Wallet | object `coin-address` | Lowercase `0x` plus 40 hex; a mixed-case input must pass EIP-55 | `bytea` (20 bytes) | A | | planned |
| `monero_address` | `cryptocurrency-wallet` | Cryptocurrency-Wallet | `xmr` | Monero Base58, case preserved, checksum enforced | `text COLLATE "C"` | A | | planned |
| `tron_address` | `cryptocurrency-wallet` | Cryptocurrency-Wallet | object `coin-address` | Base58Check with `T` prefix | `text COLLATE "C"` | A | | planned |
| `litecoin_address` | `cryptocurrency-wallet` | Cryptocurrency-Wallet | object `coin-address` | Base58Check with `L` or `M` prefix, or Bech32 (`ltc1`); legacy `3` P2SH addresses are excluded because they are indistinguishable from Bitcoin's | `text COLLATE "C"` | A | | planned |
| `iban` | `bank-account` | Bank-Account (`iban`) | `iban` | Uppercase, no spaces; exposes the country and whether mod 97 is valid; contributing property `iban`, as in OpenCTI | `text COLLATE "C"` | A | PD | planned |
| `bic` | `x-fennec-bic` | Text (OpenCTI's `bic` is only an attribute of a Bank-Account keyed by IBAN) | `bic` | Uppercase 8 or 11; the `XXX` branch folds to 8 | `text COLLATE "C"` | A | | planned |

Cryptocurrency addresses are one kind per chain because each chain has its
own encoding and checksum. They all export to OpenCTI's
`cryptocurrency-wallet`, whose identifier hashes only the value, so the
identifiers match OpenCTI's. The chain cannot be recovered from that type, so
an imported `cryptocurrency-wallet` (or a MISP `coin-address`) needs an
explicit chain from the producer (MISP's `symbol` attribute); without one it
is rejected as untyped.

## Wave 7: accounts on platforms

| Module | STIX type | MISP | Canonical form | Column | Id | PD | Status |
|---|---|---|---|---|---|---|---|
| `telegram_username` | `x-fennec-telegram-username` | object `telegram-account` | Lowercase, no `@`, `^[a-z][a-z0-9_]{3,31}$` | `text COLLATE "C"` | A | PD | planned |
| `telegram_user_id` | `x-fennec-telegram-user-id` | object `telegram-account` | Positive integer (users and bots) | `bigint` | A | PD | planned |
| `telegram_chat_id` | `x-fennec-telegram-chat-id` | object `telegram-account` | Bot API signed integer (`-100...` for channels and supergroups) | `bigint` | A | | planned |
| `discord_username` | `x-fennec-discord-username` | none | Lowercase `^[a-z0-9_.]{2,32}$`, no consecutive periods | `text COLLATE "C"` | A | PD | planned |
| `discord_user_id` | `x-fennec-discord-user-id` | none | Snowflake; exposes its creation time | `bigint` | A | PD | planned |
| `discord_guild_id` | `x-fennec-discord-guild-id` | none | Snowflake | `bigint` | A | | planned |
| `discord_webhook_id` | `x-fennec-discord-webhook-id` | none | Snowflake (the non-secret part of a webhook URL) | `bigint` | A | | planned |
| `discord_invite_code` | `x-fennec-discord-invite-code` | none | Case-sensitive code | `text COLLATE "C"` | A | | planned |
| `github_username` | `x-fennec-github-username` | `github-username` | Lowercase login, at most 39 characters | `text COLLATE "C"` | A | PD | planned |
| `github_user_id` | `x-fennec-github-user-id` | object `github-user` | Positive integer | `bigint` | A | PD | planned |
| `github_repository` | `x-fennec-github-repository` | `github-repository` | Lowercase `owner/name` | `text COLLATE "C"` | A | | planned |
| `x_username` | `x-fennec-x-username` | object `twitter-account` | Lowercase, no `@`, `^[a-z0-9_]{1,15}$` | `text COLLATE "C"` | A | PD | planned |
| `x_user_id` | `x-fennec-x-user-id` | object `twitter-account` | Positive integer | `bigint` | A | PD | planned |
| `mastodon_account` | `x-fennec-mastodon-account` | none | Lowercase `user@domain`, domain as A-label | `text COLLATE "C"` | A | PD | planned |
| `xmpp_address` | `x-fennec-xmpp-address` | `jabber-id` | Bare JID, lowercase, no resource | `text COLLATE "C"` | A | PD | planned |
| `slack_workspace_id` | `x-fennec-slack-workspace-id` | none | Uppercase `T...` | `text COLLATE "C"` | A | | planned |

Account handles and numeric account ids are separate kinds: handles can be
renamed and re-registered, numeric ids cannot. A relationship (or a sighting
with a validity window) ties a handle to the id it was seen with.

## Wave 8: applications, cloud, C2 tooling, keys and anonymity networks

| Module | STIX type | MISP | Canonical form | Column | Id | Status |
|---|---|---|---|---|---|---|
| `android_package_name` | `x-fennec-android-package-name` | `mobile-application-id` | Case-sensitive dotted Java package name | `text COLLATE "C"` | A | planned |
| `apple_bundle_id` | `x-fennec-apple-bundle-id` | `mobile-application-id` | Dotted identifier, lowercase (Apple treats bundle ids case-insensitively; the original spelling is not kept) | `text COLLATE "C"` | A | planned |
| `chrome_extension_id` | `x-fennec-chrome-extension-id` | `chrome-extension-id` | `^[a-p]{32}$` | `text COLLATE "C"` | A | planned |
| `edge_extension_id` | `x-fennec-edge-extension-id` | `edge-extension-id` | `^[a-p]{32}$` | `text COLLATE "C"` | A | planned |
| `firefox_addon_id` | `x-fennec-firefox-addon-id` | object `browser-extension` | Braced lowercase GUID or email-like gecko id | `text COLLATE "C"` | A | planned |
| `package_url` | `x-fennec-package-url` | none | purl, type lowercase, name normalised per type | `text COLLATE "C"` | A | planned |
| `entra_tenant_id` | `x-fennec-entra-tenant-id` | none | Lowercase GUID | `uuid` | A | planned |
| `entra_application_id` | `x-fennec-entra-application-id` | `azure-application-id` | Lowercase GUID | `uuid` | A | planned |
| `aws_account_id` | `x-fennec-aws-account-id` | none | 12 digits, leading zeros kept | `text COLLATE "C"` | A | planned |
| `aws_access_key_id` | `x-fennec-aws-access-key-id` | none | `AKIA` or `ASIA` plus 16 uppercase; derives the account id | `text COLLATE "C"` | A | planned |
| `cobalt_strike_watermark` | `x-fennec-cobalt-strike-watermark` | object `cs-beacon-config` | Unsigned 32-bit integer | `bigint` | A | planned |
| `cobalt_strike_public_key` | `x-fennec-cobalt-strike-public-key` | object `cs-beacon-config` | SHA-256 of the DER key, lowercase hex | `bytea` (32 bytes) | A | planned |
| `anydesk_id` | `x-fennec-anydesk-id` | none | 9 or 10 digits | `bigint` | A | planned |
| `teamviewer_id` | `x-fennec-teamviewer-id` | none | 9 or 10 digits | `bigint` | A | planned |
| `pgp_fingerprint` | `x-fennec-pgp-fingerprint` | object `pgp-key` | Uppercase hex, 40 (v4) or 64 (v5 and v6) | `bytea` | A | planned |
| `user_agent` | `user-agent` | `user-agent` | As received, single line | `text COLLATE "C"` plus a `bytea` SHA-256 unique key | A | planned |
| `onion_address` | `x-fennec-onion-address` | `onion-address` | Lowercase v3 `^[a-z2-7]{56}\.onion$`, checksum enforced | `text COLLATE "C"` | A | planned |
| `i2p_address` | `x-fennec-i2p-address` | none | Lowercase `^[a-z2-7]{52}\.b32\.i2p$` | `text COLLATE "C"` | A | planned |
| `ipfs_cid` | `x-fennec-ipfs-cid` | none | CIDv1 base32 lowercase (v0 converted) | `text COLLATE "C"` | A | planned |

None of the wave 7 and wave 8 kinds except `user_agent` has an OpenCTI type;
they export to OpenCTI as labelled `text` (see
[interoperability.md](interoperability.md#kinds-without-an-opencti-type)).
MISP's `mobile-application-id` does not say whether it is an Android package
or an Apple bundle id: the importer takes the platform from the enclosing
object template (`android-app`, `apk`) or reports the value as untyped.

## Deliberately not observables

| Values | Why | Where they will live |
|---|---|---|
| MISP `text`, `comment`, `other`, `hex`, `anonymised`, `float`, `integer`, `counter`, `boolean`, `datetime`, `uuid`; OpenCTI `Text` | Catch-alls: they carry no kind, so they cannot be validated, canonicalised or pivoted on | Nowhere; an importer reports them as untyped |
| JA4L, JA4LS, JA4SSH, Community ID, TMSI, GUTI, JA4D | Measurements of one session or reallocated radio identities | Value objects on the sighting or network session that measured them |
| Bot tokens, webhook URLs with tokens, API keys, private keys, passwords, card numbers with CVV, OpenCTI Credential and Payment-Card | Secrets need encryption and access control that do not exist yet | A future access-controlled context; their non-secret derived ids are observables now |
| YARA, Sigma, Snort, Suricata, STIX patterns | Detection logic, not something that is seen | A future indicator and detection-rule context |
| CVE and other vulnerability ids, threat actor and malware names, ATT&CK ids | Knowledge objects (STIX SDOs), not observables | Future SDO modules |
| Score, labels, markings, confidence, description, author | Assessments about an observable | Future assessment aggregates that reference observables |
| OpenCTI Persona, Media-Content, Tracking-Number, AI-Prompt | Identity, content and logistics data with their own lifecycles | Re-evaluated after wave 8 |
