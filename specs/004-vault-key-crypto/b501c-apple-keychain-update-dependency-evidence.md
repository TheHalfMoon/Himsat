# B501C Apple Keychain update dependency exposure evidence

## Scope

This candidate exposes `core-foundation 0.10.1` as a direct macOS dependency only so the already-adopted `security-framework 3.7.0` safe `SecItemUpdate` wrapper can receive `CFData` for update-only Keychain value mutation.

No new external package identity is introduced. `core-foundation 0.10.1` is already present in `Cargo.lock`, the provenance registry, generated SBOM, and `THIRD_PARTY_NOTICES.md` through the canonical Apple provider closure.

## Security boundary

This leaf adds no freshness-anchor runtime behavior and makes no atomicity, CAS, rollback-resistance, or crash old-or-new claim. It does not authorize unsafe FFI or add-or-update password APIs as a substitute for update-only semantics.

The direct dependency remains pinned exactly to `0.10.1` with default features disabled. Provider counts, package checksum, source revision, selected license, notices, and generated provenance identities remain unchanged.

B501 Apple freshness-anchor runtime remains a separate later leaf requiring exact provider/native qualification. Q009 remains UNSATISFIED.
