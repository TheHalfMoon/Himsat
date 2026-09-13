# B505B Round 5 Self-Audit Remediation Evidence

## Status

This document records a forward-only security-semantic remediation discovered after the initial Round 5 contract became canonical at `52544ad82f2f2071c470183d6788b5334a19a5d3` and before any qualifying independent human Round 5 review was obtained.

```text
INITIAL_ROUND5_CANONICAL_SHA = 52544ad82f2f2071c470183d6788b5334a19a5d3
INITIAL_ROUND5_ACCEPTED_HEAD = dcbcbc1e2f15a3bfa370e6d861b9b1c68fc127d7
INITIAL_ROUND5_POSTMERGE_CI = 34740286573_SUCCESS_ATTEMPT_1
INITIAL_ROUND5_POSTMERGE_R3 = 34740286572_SUCCESS_ATTEMPT_1
INITIAL_REVIEW_ONLY_PR = 124
INITIAL_HUMAN_REVIEW = NONE
B505_IMPLEMENTATION_AUTHORITY = NONE
```

The initial review target is superseded by this remediation. PR #124 must not be used to reopen B505 implementation authority even if a later review is posted against its older SHA.

## Finding B505B-F001 -- authenticated source storage identity was omitted

The canonical B501 inventory authenticates an opaque `storage_id` for every stored object, and Round 4 requires a `GENERIC_ARTIFACT_BLOB` verifier to resolve the stored object using the authenticated manifest storage identifier before accepting its complete envelope length/hash and context.
The initial Round 5 encrypted index carried only `logical_id`. That can identify a blob semantically, but it does not preserve the authenticated B501 storage-resolution identity required for exact one-to-one source-object closure.

Remediation:

- add encrypted `source_storage_id` to every payload record;
- require all-zero `source_storage_id` only for the manifest payload;
- copy the exact authenticated B501 `storage_id` for structured-store and blob payloads;
- after B501 authentication on restore, require exact one-to-one storage-ID/logical-ID mapping with no missing, duplicate, extra, or mismatched payload.

`source_storage_id` remains inside the encrypted index and therefore does not enlarge the provider-visible metadata surface.

## Finding B505B-F002 -- Round 5 capacity did not cover canonical B501 inventory

B501 permits up to 262,144 authenticated inventory objects. A portable backup adds one manifest payload, so a complete representation may require 262,145 payload records before chunking.

The initial Round 5 `payload_count <= 131072` and `data_object_count <= 131072` could reject otherwise canonical vault state without an explicit compatibility contract.

Remediation raises the bounded v1 capacities to:

```text
payload_count <= 262_145
data_object_count / global chunk count <= 1_048_576
index plaintext <= 67_108_864 bytes (64 MiB)
index ciphertext+tag <= 67_108_880 bytes
```
The fixed payload record is 82 bytes after adding `source_storage_id`; the fixed chunk record is 24 bytes. At the simultaneous maxima, fixed record bytes are 46,661,714 before the small fixed index header, so the 64 MiB index-plaintext cap remains a concrete parse/allocation bound.

## Finding B505B-F003 -- unstable or multi-generation source state was not rejected

The Round 5 recovery bootstrap carries recovery material for one active VRK generation. Round 3 and B503 allow authenticated manifests with `STAGED`/`RETAINED` generations while full rotation is in progress.

The initial Round 5 snapshot protocol did not explicitly reject `rotation_phase != NONE` or inventory objects whose `object_key_generation` differs from `active_key_generation`. Such a set could preserve valid inner ciphertext that a fresh-device restore cannot decrypt from the single recovered active VRK.

Remediation requires, before any provider-visible publication and again after restore manifest authentication:

```text
rotation_phase == NONE
rotation_target_generation == 0
for every B501 inventory object:
    object_key_generation == active_key_generation
```

Violation fails `BackupStateNotStable`. B505 must not export a portable set requiring an additional VRK absent from its recovery bootstrap.

## Finding B505B-F004 -- provider-key hierarchy was not filesystem-safe

The initial descriptor key was exactly `hex_lower(BackupSetId)`, while data objects were children beneath `hex_lower(BackupSetId)/...`. Filesystem-style providers cannot represent the same path as both a file and a directory.

Remediation reserves the all-zero 16-byte object identifier exclusively for the descriptor leaf and uses `hex_lower(BackupSetId)/00000000000000000000000000000000`; data-object identifiers remain non-zero. This is a fixed protocol token and reveals no semantic payload identity.

## Finding B505B-F005 -- backup data keys were generation-scoped but not set-scoped

The initial `index_key` and `object_key` derivations were distinct by purpose but identical across repeated backup exports of the same vault generation. Fresh 192-bit nonces made collision risk negligible, but the design can eliminate cross-set same-key nonce reuse entirely.

Remediation appends the raw `BackupSetId` to each HKDF info string. Index and object keys remain purpose-separated and are now also backup-set-specific; fresh OS-CSPRNG nonces remain mandatory.

## Finding B505B-F006 -- backup creation did not prove the recovery envelope wrapped the active VRK

The initial protocol required a current authenticated B204 envelope but did not explicitly require backup creation to obtain the recovery passphrase, authenticate/decrypt that exact envelope, and prove the recovered VRK equals the active unlocked VRK before constructing the outer bootstrap. A stale or semantically divergent recovery envelope could otherwise produce a portable set whose bootstrap opens a different key from the one used to encrypt the backup objects.

Remediation requires local passphrase entry, canonical B204 KDF/authentication, equality with the active unlocked VRK, and bootstrap-key derivation from that same verified Recovery KEK before any provider-visible publication. Wrong-passphrase/tag failures remain uniform; an authenticated envelope yielding a different VRK is `CorruptOrTampered`.

## Finding B505B-F007 -- parser logical-ID uniqueness conflicted with required zero IDs

CodeRabbit inline review comment `3999042756` on PR #126 against predecessor `b89ca01ef8cdc3e3f3669bd738df13bab3d385d2` identified that the Round 5 parser sentence rejected duplicate logical IDs globally even though the required `MANIFEST` and `STRUCTURED_STORE` payload records both use all-zero `logical_id`. Taken literally, the parser rule would reject every conforming index containing both required records.

Remediation scopes uniqueness correctly: duplicate `(payload_kind, logical_id)` records are rejected; duplicate `GENERIC_ARTIFACT_BLOB` logical IDs are rejected; the required MANIFEST and STRUCTURED_STORE zero logical IDs are allowed only because their payload kinds differ.

This is a security-semantic parser-contract correction. Predecessor commit `b89ca01ef8cdc3e3f3669bd738df13bab3d385d2` remains preserved as review evidence and is not amended or rewritten.

## Governance disposition

These findings are security-semantic. They invalidate `52544ad82f2f2071c470183d6788b5334a19a5d3` as the final Round 5 human-review target before any qualifying human review was obtained.
The remediation remains docs/design/evidence/state only. It adds no product code, dependency, provider SDK, provenance adoption, SBOM/notice material, workflow change, donor code, model, dataset, release claim, FIPS claim, or compliance claim.

Before B505 implementation authority can reopen, the remediated exact head must pass local and GitHub qualification, merge with explicit `expected_head_sha`, receive exact post-merge CI/R3 success, and then receive a genuinely independent substantive human crypto/security review of the new exact canonical SHA with no unresolved blocker.

Any review of PR #124 or SHA `52544ad82f2f2071c470183d6788b5334a19a5d3` is historical/superseded evidence only after this remediation becomes canonical.
## Local remediation verification

On the remediation worktree derived exactly from canonical `52544ad82f2f2071c470183d6788b5334a19a5d3`:

- provenance V2 validate: PASS;
- generated-output closure: PASS;
- registered 004P dependency closure: PASS;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS;
- `cargo clippy --workspace --all-targets --locked -- -D warnings`: PASS;
- `cargo test --workspace --all-targets --locked`: PASS;
- provenance adversarial self-test on Windows: NOT PASS, preserving the known `allowed-permissive-copy` fixture digest mismatch rather than rerunning-to-green;
- provenance adversarial self-test on Ubuntu/WSL against the same worktree: `SELF-TEST PASS`.

Fresh exact-commit verification and original-attempt GitHub CI/R3 remain required after the final remediation commit is created and pushed.
