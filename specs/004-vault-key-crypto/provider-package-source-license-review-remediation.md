# Specification 004P — P003/P004 Review Remediation

## Purpose and controlling status

This file is the forward-only successor evidence for the independent review of `provider-package-source-license-closure.md` on PR #35. It preserves the original resolver and closure artifacts as historical evidence while correcting their defects without pretending the earlier runs or statements never existed.

For the P003/P004 candidate closure, this file controls where it conflicts with:

- `provider-package-source-license-closure.md` exact research-run metadata;
- `provider-resolution-evidence.md` package-count prose;
- `provider-provenance-selection.md` package-count prose;
- `provider-selection-challenge.md` package-count prose.

The package tables and exact package identities remain unchanged except that the correct external-package count is 41.

This remediation still does not adopt dependency bytes, change canonical `Cargo.toml` or `Cargo.lock`, write provenance registry entries, regenerate deterministic SBOM/notices, or authorize Specification 004B implementation.

## Independent review finding

PR #35 independent CodeRabbit review response `5563056163` identified two blocking evidence defects and one count-consistency repair:

1. `fallible-streaming-iterator 0.1.9` used a recorded revision override without a fail-closed upstream tag-resolution check in the cited collector.
2. SQLCipher / embedded SQLite / OpenSSL native identities and license evidence were manually inspected but were not re-verified fail-closed by the cited collector.
3. predecessor documents still contained active prose claiming 40 external packages even though the exact package table contains 41.

The review otherwise confirmed that:

- the package table contains 41 distinct external packages;
- the 41 Cargo package MIT adoption branches are supported by package metadata and concrete packaged-license evidence;
- the `r-efi 6.0.0` `AUTHORS` MIT exception is explicit;
- P005/P006 remain pending and 004B remains blocked.

The two evidence defects were treated as blocking. PR #35 was not merged on the earlier green CI/R3 evidence.

## Final fail-closed research execution

```text
CANONICAL_BASE_AT_RESEARCH = b2eed59b582fdb783613e84974ac97dba4bb14e2
RESEARCH_BRANCH = research/004p-p003-source-license-closure
FINAL_REMEDIATION_RESEARCH_HEAD = 0401abf6641dc25fcefc93fc40724cfd3fde21fc
FINAL_REMEDIATION_WORKFLOW_RUN = 34067945881
FINAL_REMEDIATION_JOB = 101579927959
FINAL_REMEDIATION_JOB_CONCLUSION = SUCCESS
RUNNER_OS = Ubuntu 24.04.4 LTS
RUNNER_IMAGE = ubuntu-24.04 / 20260831.293.1
RUSTC = 1.98.1 (48a229cea 2026-09-01)
CARGO = 1.98.1 (797e8a9bc 2026-08-05)
OPENSSL_RUST_USE_NASM = 0
EXTERNAL_REGISTRY_PACKAGES = 41
PROVENANCE_GAPS = 0
```

Run `34067945881` supersedes run `34066909709` as the controlling research execution for P003/P004 closure qualification. The earlier successful run remains historical evidence but is insufficient for closure because it did not fail-closed verify the exceptional tag mapping and complete native source/license relationships.

## Blocker 1 remediation — exceptional VCS tag resolution

The published `fallible-streaming-iterator 0.1.9` crate does not contain `.cargo_vcs_info.json`.

The remediation collector now executes a live immutable ref check before accepting the exception:

```text
UPSTREAM_REPOSITORY = https://github.com/sfackler/fallible-streaming-iterator
UPSTREAM_TAG = refs/tags/v0.1.9
EXPECTED_REVISION = 9217bc5e381b54b4ef4c38959488a5dc993b7b81
OBSERVED_REVISION = 9217bc5e381b54b4ef4c38959488a5dc993b7b81
CHECK = git ls-remote --refs
FAIL_CLOSED_ON_MISMATCH = YES
```

The verified revision is exported into the package collector. The package collector then rejects the exception if the verified value is absent or differs from the expected immutable revision.

The final record therefore uses:

```text
vcs_evidence = verified-upstream-tag-v0.1.9
source_revision = 9217bc5e381b54b4ef4c38959488a5dc993b7b81
source_path = .
package_checksum = 7360491ce676a36bf9bb3c56c1aa791658183a54d2744120f27285738d90465a
selected_license = MIT
license_evidence = LICENSE-MIT sha256:8dcec5569a9be5b0e086c80faed6f1aefa670af0ec29cecc2f714303096887e0
```

This closes review blocker 1.

## Blocker 2 remediation — native SQLCipher / SQLite / OpenSSL verification

The remediation collector now obtains and checks the exact upstream/native repositories and fails on any identity, tag, gitlink, blob, version, source-id, or license-marker mismatch.

### SQLCipher release source and license

```text
SQLCIPHER_REPOSITORY = https://github.com/sqlcipher/sqlcipher
SQLCIPHER_TAG = v4.14.0
SQLCIPHER_TAG_OBJECT = 46bb08ec73b2caa84b6945a19c9e435fff446dcd
SQLCIPHER_SOURCE_REVISION = 778ab890cfc30c3631212dcceb0295498abdcd3e
SQLCIPHER_LICENSE_FILE = LICENSE.md
SQLCIPHER_LICENSE_GIT_BLOB = 3f71443161b6ff424ca4f335e910469f46ed4c44
SQLCIPHER_LICENSE = BSD-3-Clause
SQLITE_LICENSE_FILE = SQLITE_LICENSE.md
SQLITE_LICENSE_GIT_BLOB = 4029dc9e7f25d12adfa0165542d702208276913a
SQLITE_IMPLEMENTATION_DISPOSITION = PUBLIC_DOMAIN
```

The collector verifies both the annotated tag object and its peeled commit, verifies the exact license blobs, and checks the controlling redistribution/no-endorsement and public-domain license markers.

### `libsqlite3-sys` vendored SQLCipher amalgamation and SQLite identity

```text
LIBSQLITE3_SYS_PACKAGE = 0.38.2
LIBSQLITE3_SYS_SOURCE_REVISION = e88f112bef7899234a497baed5cc3c3d553deeb8
SQLCIPHER_GENERATOR_VERSION = 4.14.0
VENDORED_SQLITE3_H_GIT_BLOB = d1a3f170510e551a47eeb91f04addd1387bc79ab
VENDORED_SQLITE3_C_GIT_BLOB = 9a9d2f2034328eea3a595303ea6e8da9ffc36214
UPGRADE_SQLCIPHER_SCRIPT_GIT_BLOB = a96143d098a652fae59d21d6a6cd017e0438b5e7
SQLITE_VERSION = 3.51.3
SQLITE_SOURCE_ID = 737ae4a34738ffa0c3ff7f9bb18df914dd1cad163f28fd6b6e114a344fe6alt1
```

The collector fetches the exact `libsqlite3-sys` source revision, verifies the three immutable blobs, checks that the generator remains pinned to SQLCipher `4.14.0`, and checks the exact SQLite version and source id from the vendored header.

### OpenSSL wrapper-to-upstream source relationship and license

```text
OPENSSL_SRC_PACKAGE = 300.6.1+3.6.3
OPENSSL_SRC_WRAPPER_REVISION = 64c38cc48205400720199476aff8a780f98d167d
OPENSSL_SUBMODULE_PATH = openssl
OPENSSL_SUBMODULE_REVISION = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f
OPENSSL_UPSTREAM_REPOSITORY = https://github.com/openssl/openssl
OPENSSL_UPSTREAM_TAG = openssl-3.6.3
OPENSSL_UPSTREAM_TAG_OBJECT = e5c234b0c471a676ae6141d2f157df61eb293477
OPENSSL_UPSTREAM_REVISION = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f
OPENSSL_LICENSE_FILE = LICENSE.txt
OPENSSL_LICENSE_GIT_BLOB = 49cc83d2ee29d13453188217f0e4edd70c7f842f
OPENSSL_LICENSE = Apache-2.0
```

The collector verifies the `openssl-src-rs` exact source revision, exact `openssl` gitlink/submodule revision and repository URL, the upstream OpenSSL annotated tag object and peeled commit, the exact license blob, and Apache-2.0 license markers.

This closes review blocker 2.

## Package-count correction and predecessor deprecation

Exact Cargo evidence is:

```text
CARGO_MESSAGE = Locking 41 packages to latest Rust 1.98.1 compatible versions
EXTERNAL_METADATA_PACKAGES = 41
EXTERNAL_LOCKFILE_PACKAGES = 41
LOCAL_RESEARCH_PACKAGE = himsat-p003-resolver
LOCAL_RESEARCH_PACKAGE_IS_ADDITIONAL_TO_EXTERNAL_COUNT = YES
```

The local research package is not included in Cargo's reported 41 registry packages. The exact external package table has always contained 41 rows.

Therefore the following predecessor statements are historical mistakes and are explicitly **non-controlling** after this remediation becomes canonical:

- `provider-resolution-evidence.md`: “One package is the local research resolver itself. The resulting external lockfile closure contains 40 packages.”
- `provider-resolution-evidence.md`: any remaining-work reference to “40 external lockfile packages”.
- `provider-provenance-selection.md`: “all 40 external package versions/checksums”.
- `provider-provenance-selection.md`: “isolated 40-package candidate closure”.
- `provider-selection-challenge.md`: “40 external lockfile packages”.

Those documents remain preserved as historical decision/research records. For every P003-P006 decision, registry entry set, canonical lockfile check, SBOM/notices generation, and later review, the controlling count is:

```text
EXTERNAL_CARGO_PACKAGE_COUNT = 41
```

Any tool, PR, plan, review, or implementation that uses 40 as the selected external package count is stale and must fail reconciliation.

## License-policy disposition retained

The final remediation run does not change the previously collected package-license table:

```text
CARGO_PACKAGE_SELECTED_LICENSE = MIT for all 41 packages
MIT_POLICY_DISPOSITION = allow
SQLCIPHER_LICENSE = BSD-3-Clause
SQLCIPHER_POLICY_DISPOSITION = allow
OPENSSL_UPSTREAM_LICENSE = Apache-2.0
OPENSSL_POLICY_DISPOSITION = allow
MANUAL_LICENSE_DECISION_REQUIRED_FOR_SELECTED_CLOSURE = NO
DENIED_LICENSE_PRESENT_IN_SELECTED_CLOSURE = NO
UNKNOWN_LICENSE_PRESENT_IN_SELECTED_CLOSURE = NO
```

The public multi-license expressions remain factual evidence and are not rewritten.

## P003/P004 disposition after canonical merge

The exact candidate graph has now been re-collected with zero gaps and both independent-review blockers have been remediated. This PR head is still not canonical merely because its research and CI evidence succeeds.

If and only if the remediation is independently re-reviewed without unresolved blockers, exact-head CI/R3 succeed, the PR is merged with exact expected-head protection, and the canonical merge passes required post-merge qualification, the resulting canonical disposition is:

```text
P001 = SELECTED_NOT_ADOPTED
P002 = CORRECTED_SELECTED_NOT_ADOPTED
P003_PACKAGE_VERSION_CHECKSUM_DISCOVERY = COMPLETE
P003_SOURCE_REVISION_PATH_CLOSURE = COMPLETE
P004_LICENSE_NOTICE_CLOSURE = COMPLETE
P005_REGISTRY_SBOM_NOTICES_ADOPTION = NEXT_PENDING_UNIT
P006_CANONICAL_LOCKFILE_NATIVE_CLOSURE_PROOF = PENDING
DEPENDENCY_BYTES_ADOPTED = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

Until those canonicalization conditions are met, live canonical `main` remains authoritative and P003/P004 must not be represented as canonically closed.
