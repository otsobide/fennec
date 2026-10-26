# Observable kinds

One page per implemented observable kind: its exact grammar, canonical form,
identity, events, HTTP API and storage. Planned kinds are listed in
[../observable-catalogue.md](../observable-catalogue.md); a kind gets its page
in the pull request that implements it.

| Kind | Module | STIX type | Persistence |
|---|---|---|---|
| [Domain name](domain-name.md) | `domain_name` | `domain-name` | In memory and PostgreSQL |
| [Email address](email-address.md) | `email_address` | `email-addr` | In memory and PostgreSQL |
| [Hostname](hostname.md) | `hostname` | `hostname` | In memory and PostgreSQL |
| [IPv4 address](ipv4-address.md) | `ipv4_address` | `ipv4-addr` | In memory and PostgreSQL |
| [IPv6 address](ipv6-address.md) | `ipv6_address` | `ipv6-addr` | In memory and PostgreSQL |
| [MAC address](mac-address.md) | `mac_address` | `mac-addr` | In memory and PostgreSQL |
