# Specification 004P — P005/P006 Adoption Closure Evidence

## Purpose

This file records the bounded adoption-closure candidate for Specification 004P P005/P006. It does not claim canonical completion before guarded merge and exact post-merge qualification, and it does not authorize Specification 004B implementation by itself.

## Canonical base

```text
BASE_MAIN = 0e6a3422bfe4a18a248d66027f5f538fd68542d3
P003 = CANONICAL_CLOSED
P004 = CANONICAL_CLOSED
P005_P006_BRANCH = feat/004p-p005-p006-adoption
```

The controlling P003/P004 evidence is:

```text
specs/004-vault-key-crypto/provider-package-source-license-closure.md
specs/004-vault-key-crypto/provider-package-source-license-review-remediation.md
```

## Exact adoption generator evidence

A temporary branch-only generator reproduced the selected exact dependency graph, fetched package bytes under Cargo's exact lock identities, re-fetched the native upstream license evidence, generated the machine registry/notices/SBOM, and refused to commit unless the generated working-tree change set contained only the four intended generated artifacts.

The first run attempt failed after the substantive closure checks because Python created an untracked `tools/__pycache__/` directory. That failure was preserved and not bypassed:

```text
WORKFLOW_RUN = 34068958942
FAILED_JOB = 101582656529
FAILURE_CLASS = WORKTREE_HYGIENE
SUBSTANTIVE_RESULTS_BEFORE_FAILURE =
  PROVENANCE_V2_PASS
  GENERATED_OUTPUTS_PASS
  004P_P005_P006_CLOSURE_PASS
GENERATED_COMMIT_WRITTEN = NO
```

The hygiene defect was repaired by ignoring standard Python bytecode/cache output rather than widening the generator's allowed generated artifact set. The rerun then succeeded:

```text
WORKFLOW_RUN = 34068958942
SUCCESS_JOB = 101582863135
GENERATOR_INPUT_HEAD = 501021a385dd0cc22479ea6f3e195afaaa74c5f2
GENERATED_COMMIT = 78f17e475d459fdf36fce7d1bbffbd7a0f247bf9
RUSTC = 1.98.1 (48a229cea 2026-09-01)
CARGO = 1.98.1 (797e8a9bc 2026-08-05)
PYTHON = 3.13.15
RUNNER_OS = Ubuntu 24.04.4 LTS
OPENSSL_RUST_USE_NASM = 0
GENERATOR_RESULT = SUCCESS
```

The successful generator reported:

```text
Locking 41 packages to latest Rust 1.98.1 compatible versions
GENERATED_REGISTRY_ENTRIES = 41
PROVENANCE V2 PASS
GENERATED OUTPUTS PASS
004P P005/P006 CLOSURE PASS
```

The generated commit changed exactly:

```text
Cargo.lock
THIRD_PARTY_NOTICES.md
governance/generated/sbom.json
governance/provenance/registry.json
```

The temporary generator workflow was removed from the feature branch after the generated commit was persisted.

## Direct dependency adoption

The exact direct package pins in `crates/himsat-core/Cargo.toml` are:

```text
chacha20poly1305 = 0.11.0, default-features=false, features=[alloc, zeroize]
hkdf = 0.13.0, default-features=false
sha2 = 0.11.0, default-features=false
argon2 = 0.6.0, default-features=false, features=[alloc, zeroize]
zeroize = 1.9.0
getrandom = 0.4.3, default-features=false
rusqlite = 0.40.1, default-features=false, features=[bundled-sqlcipher-vendored-openssl]
libsqlite3-sys = 0.38.2, default-features=false, features=[bundled-sqlcipher-vendored-openssl]
```

The successful feature-tree evidence proves the selected SQLCipher path activates vendored OpenSSL through `openssl-sys` and `openssl-src`, with the `openssl-src legacy` feature observed in the selected graph.

## Machine adoption boundary

The provenance registry advances from `himsat.provenance-registry/v1` to `himsat.provenance-registry/v2` only to represent independently licensed embedded native components under an adopted Cargo dependency.

The v2 gate preserves the v1 Cargo package adoption rules and adds fail-closed native-component requirements for:

- unique native component identity;
- HTTPS source repository;
- immutable 40- or 64-hex source revision;
- exact source paths;
- exact embedded paths inside the parent dependency source tree;
- an allowed source-license identity;
- non-empty notice text;
- a repository-local evidence reference.

Native components may appear only under an adopted `dependency` entry using `depend` mode. Native IDs are globally unique across the registry. Native components are emitted separately inside deterministic notices and the Himsat SBOM.

The native adversarial fixture matrix rejects at least:

- symbolic/non-immutable revisions;
- manual, denied, and unknown license identities;
- unsafe repository paths;
- non-HTTPS source repositories.

It includes a positive SQLite public-domain case using the explicit policy identity:

```text
LicenseRef-SQLite-Public-Domain
```

This policy addition is narrowly scoped to the already-proven SQLite public-domain implementation notice. It does not relabel a restricted license and does not implement or consume the special-permission model tracked by Issue #14.

## Native adopted closure

The machine closure separately represents:

```text
SQLCipher 4.14.0
  source = https://github.com/sqlcipher/sqlcipher
  revision = 778ab890cfc30c3631212dcceb0295498abdcd3e
  license = BSD-3-Clause

SQLite 3.51.3 embedded amalgamation
  containing source repository = https://github.com/rusqlite/rusqlite
  revision = e88f112bef7899234a497baed5cc3c3d553deeb8
  license = LicenseRef-SQLite-Public-Domain

OpenSSL 3.6.3
  source = https://github.com/openssl/openssl
  revision = aae016bfd52fcad2bc9657c2c782cfdf73b1ed5f
  license = Apache-2.0
```

Their exact blob/tag/gitlink/version/source-id relationships remain controlled by the successful fail-closed P003/P004 remediation evidence. P005/P006 do not weaken or replace that evidence.

## Continuous qualification change

The repository CI no longer asserts a dependency-free workspace. It now fails unless all of the following hold on the exact checked-out revision:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
python tools/provenance_gate.py validate
python tools/provenance_gate.py check-generated
python tools/004p_dependency_closure.py
python tools/provenance_gate.py self-test
```

The CI and R3 workflows pin:

```text
OPENSSL_RUST_USE_NASM = 0
```

The R3 workflow additionally asserts the registered dependency closure before executing the Diffcipline R3 proof.

## Candidate disposition

Before PR qualification and canonical merge:

```text
P005_CANDIDATE = COMPLETE_NOT_CANONICAL
P006_CANDIDATE = COMPLETE_NOT_CANONICAL
DEPENDENCY_BYTES_ADOPTED_ON_MAIN = NO
004B_IMPLEMENTATION_AUTHORITY = BLOCKED
```

P005/P006 may become canonical only after:

1. final exact feature-branch diff reconciliation;
2. exact-head multi-platform CI success;
3. exact-head R3 success;
4. substantive independent review of the exact adoption head with no unresolved blocker;
5. live `main`, comments, reviews, threads, mergeability, and head revalidation;
6. guarded merge with expected-head protection;
7. exact post-merge CI and R3 success on the canonical merge.

Only after those gates close may live canonical truth be used to re-bound the smallest Specification 004B implementation leaf.
