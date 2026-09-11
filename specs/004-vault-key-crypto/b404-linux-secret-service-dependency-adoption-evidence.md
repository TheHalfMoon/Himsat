# B404 Linux Secret Service Dependency Adoption Evidence

## Scope

This leaf stages only the dependency/provenance closure required for the B404 Linux Secret Service adapter. It does not implement the adapter, does not qualify a Linux Secret Service provider, does not close B404, and does not satisfy Q009.

## Canonical authority

```text
B403_B404_RECONCILIATION_MERGE = a0478b5dceeef6a596d2b90aaea22936b4751b33
POSTMERGE_CI = 34630209865 / SUCCESS / attempt 1 / push
POSTMERGE_R3 = 34630209923 / SUCCESS / attempt 1 / push
B404_AUTHORITY = OPEN
Q009 = UNSATISFIED
```

## Selected client

The selected direct Linux dependency is `secret-service 5.2.0` with `default-features = false` and `rt-tokio-crypto-rust`. The runtime adapter is required to use the library's Secret Service Diffie-Hellman encrypted session rather than the plaintext session mode.

The `rt-async-io-crypto-rust` candidate was rejected because it added 92 new external Cargo package identities versus 83 for the Tokio runtime feature set. The `dbus-secret-service 4.1.0` candidate was also rejected. Its non-vendored mode depends on an unpinned system `libdbus-1`; its vendored mode embeds D-Bus 1.14.4 at `8501a73dfe923ad273229b7c45925d4abe4cca7e`, whose selectable upstream licenses are AFL-2.1 or GPL-2.0-or-later. GPL is denied by repository provenance policy and AFL-2.1 is not an allowed disposition. No policy weakening is authorized by B404.

## Exact dependency delta

```text
CANONICAL_EXTERNAL_PACKAGES = 78
B404_NEW_EXTERNAL_PACKAGES = 83
B404_CANDIDATE_EXTERNAL_PACKAGES = 161
DIRECT_PACKAGE = secret-service 5.2.0
DIRECT_FEATURES = rt-tokio-crypto-rust
PLAINTEXT_SECRET_SERVICE_SESSION = NOT_AUTHORIZED
SYSTEM_LIBDBUS_FALLBACK = NOT_AUTHORIZED
```

Every new Cargo identity is pinned by exact package version/checksum, immutable upstream revision, source path, and selected MIT license evidence in `tools/004p_dependency_closure.py` and `governance/provenance/registry.json`. Generated notices and SBOM bytes are derived from that registry.

## Security and capability non-claims

This adoption does not prove that a Secret Service provider exists, is unlocked, persists across restart, enforces stronger-than-session isolation, requires user presence, is hardware backed, or is available in production. B404 runtime qualification must prove the exact provider path separately and must report `SAME_USER_SESSION` unless stronger semantics are positively proven. Lookup attributes remain limited to fixed Himsat application/service identifiers plus an opaque random protector identifier; VaultId, logical IDs, key generation, filesystem paths, and secret content are forbidden from attributes.

B405-B406, B501-B506, Specification 005, release/FIPS/compliance claims, and Q009 remain outside this dependency leaf.
