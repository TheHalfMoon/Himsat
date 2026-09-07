# Specification 004B1 B104 Final Evidence

## Disposition

```text
LEAF = B104_KEY_HIERARCHY_SECRET_LIFETIME
DISPOSITION = CANONICAL_CLOSED
IMPLEMENTATION_PR = 46
CANONICAL_BASE = dec7555363c64726bc4835350ddf520fe44ff01a
INITIAL_HEAD = 87455b97547b76855a5d68a4eb196ec4041ba70f
FINAL_HEAD = e3c27cc2806feb2aa5ace7169e0507b9bd970a1c
CANONICAL_MERGE = 6578936f7051b548339f3cf95ca0d41e6629d8bc
```

B104 is closed only for the reviewed portable key-domain and secret-lifetime contract recorded below. It does not close B105, B201 or any later implementation leaf, qualification task, closeout task, or Specification 005.

## Authority entering B104

The B103/B104 reconciliation was completed through PR #44 exact head `f348330a52cec6eb46772529fbcb82fb52eb43d4`. That head passed CI `34161105524` and R3 `34161105501`. Durable PR comment `5575629798` records the actual guarded merge transport with:

```text
EXPECTED_HEAD_SHA = f348330a52cec6eb46772529fbcb82fb52eb43d4
MERGE_METHOD = merge
MERGED = true
MERGE_SHA = 57217a6614c07ac5e8a00d85114dd06eee1a0120
GITHUB_RESULT = Pull Request successfully merged
```

Canonical reconciliation merge `57217a6614c07ac5e8a00d85114dd06eee1a0120` has exact parents `ead22ea8c0b248431a2f8a50264f6acdbc9f7a72` and `f348330a52cec6eb46772529fbcb82fb52eb43d4`, then passed exact push-triggered CI `34161857768` and R3 `34161857755`.

Planning-only PR #45 later merged as `dec7555363c64726bc4835350ddf520fe44ff01a`, with exact parents `57217a6614c07ac5e8a00d85114dd06eee1a0120` and `dba2cc460ec9f380c6ccd1d5cc89e46a0175245b`, then passed post-merge CI `34165435937` and R3 `34165435931`. That docs/provenance authorization planning change did not alter Specification 004 B104 implementation semantics or dependency authority.

The historical P011 expected-head request-body limitation remains preserved. This evidence does not reconstruct or promote unavailable PR #37 transport proof.

## Exact implementation scope

PR #46 final head `e3c27cc2806feb2aa5ace7169e0507b9bd970a1c` changed only:

```text
crates/himsat-core/src/lib.rs
crates/himsat-core/src/vault_keys.rs
```

The implementation adds only the reviewed portable B104 contract:

- canonical purpose-domain identifiers `HIMSAT/004/STRUCTURED/v1 || u64be(key_generation)`, `HIMSAT/004/BLOB/v1 || u64be(key_generation)`, and `HIMSAT/004/MANIFEST/v1 || u64be(key_generation)`;
- the raw 16-byte `VaultId` HKDF-salt contract without executing HKDF;
- owned 32-byte key material that is not `Clone` or `Copy`, uses redacted `Debug`, and zeroizes its owned arrays through the already reviewed `zeroize` dependency;
- VRK, optional Recovery KEK, and purpose-key ownership;
- portable `KeyedHandleCloser` and `PlaintextCache` abstractions;
- terminal teardown reasons for lock, rotation, revocation, and fatal failure;
- teardown ordering that revokes the lease first, closes keyed handles, releases owned VRK/Recovery KEK/purpose-key objects, and discards the session-owned plaintext cache; and
- cleanup continuation after an ordinary handle-close error.

No Cargo manifest or lockfile changed. No new dependency, donor code, native adapter, provenance entry, generated SBOM/notices artifact, or workflow was adopted by B104.

## Preserved negative evidence

Initial B104 head `87455b97547b76855a5d68a4eb196ec4041ba70f` triggered CI `34166444702`. The Windows Rust job failed at formatting. Its lint and tests were skipped and are not PASS. The overall run was later cancelled after the PR head advanced through a forward-only formatting repair; cancellation does not erase the formatting failure or promote skipped work.

The formatting-only repair produced final head `e3c27cc2806feb2aa5ace7169e0507b9bd970a1c`. No force-push, accepted-history rewrite, or retroactive evidence repair is claimed.

## Exact final-head qualification and pre-merge reconciliation

Exact final head `e3c27cc2806feb2aa5ace7169e0507b9bd970a1c` passed:

```text
CI = 34166549041_SUCCESS
R3 = 34166549045_SUCCESS
COMMITS_AHEAD_OF_BASE = 4
COMMITS_BEHIND_BASE = 0
CHANGED_FILES = 2
SUBMITTED_REVIEWS = 0
REVIEW_THREADS = 0
```

Qodo billing-blocked output and CodeRabbit automatic skip output were not counted as independent PASS evidence. Durable PR comment `5576267748` records the final pre-merge reconciliation, exact changed-path boundary, preserved negative evidence, live base, mergeability, and required expected head.

## Expected-head transport and canonical parentage

PR #46 was merged only with:

```text
expected_head_sha = e3c27cc2806feb2aa5ace7169e0507b9bd970a1c
```

Durable PR comment `5576270121` records the actual transport result:

```text
EXPECTED_HEAD_SHA = e3c27cc2806feb2aa5ace7169e0507b9bd970a1c
MERGE_METHOD = merge
MERGED = true
MERGE_SHA = 6578936f7051b548339f3cf95ca0d41e6629d8bc
GITHUB_RESULT = Pull Request successfully merged
```

Canonical merge `6578936f7051b548339f3cf95ca0d41e6629d8bc` has exact parents:

```text
parent 1 = dec7555363c64726bc4835350ddf520fe44ff01a
parent 2 = e3c27cc2806feb2aa5ace7169e0507b9bd970a1c
```

Its tree matches the qualified implementation head. Exact push-triggered post-merge qualification then succeeded:

```text
CI = 34167236097_SUCCESS
R3 = 34167236079_SUCCESS
```

The Windows Rust job in post-merge CI completed formatting, lint, tests, and registered dependency closure successfully. The longer Windows lint runtime is not treated as a failure because the job and workflow reached terminal SUCCESS.

## Scope not claimed

B104 does not claim or implement:

- HKDF-SHA-256 execution or deterministic derivation vectors; those remain B201;
- concrete proof that previously obtained DB/blob handles reject I/O after lock/revocation/failure; that remains B105;
- SQLCipher integration or encrypted structured-store qualification;
- XChaCha20-Poly1305 or Argon2id execution;
- entropy generation or VRK generation;
- native secure-store adapters;
- freshness persistence/genesis/CAS;
- backup/restore/full rotation/deletion;
- Specification 005 media behavior;
- a new dependency or donor-code adoption; or
- complete-memory-erasure guarantees beyond owned buffers, given allocator copies, swap, crash dumps, process teardown, and compromised unlocked-process residual risk.

## B105 rebound boundary

B105 remains blocked until the documentation/evidence-only B104/B105 reconciliation containing this evidence, `specs/004-vault-key-crypto/tasks.md`, and `specs/CURRENT.md` is itself exact-head CI/R3 qualified, fully reconciled, merged with explicit expected-head protection, parentage-proven, and exact post-merge CI/R3 qualified.

Only after that reconciliation becomes canonical may B105 begin from the new exact canonical `main`. B201 and every later leaf remain blocked until their declared predecessors are canonically closed.