# B502 Restore State-Transition Candidate Evidence

## Scope

B502 implements only explicit restore freshness/state-transition semantics above the canonical B501 manifest and freshness foundations. It does not implement the B505 portable-backup container/provider transport, cross-generation VRK migration, filesystem publication, provider upload/download, or final Specification 004 closeout.

The canonical authority prerequisite is the B501/B502 reconciliation merge `b6109abd6c3fc6a849306b3f280d4dd0beb4a431` with tree `90b5ac315335470990d0c0df307527c79b540609`. Its original-attempt push CI `34674875652` and R3 `34674875624` both completed `SUCCESS` on attempt 1; Windows Rust job `103502824766` also completed `SUCCESS`.

## Existing-anchor older-backup restore

The B502 entry point authenticates and canonically parses the supplied manifest envelope before any restore-state decision. It then requires the authenticated `VaultId` to match the trusted protected anchor and requires the backup epoch to be strictly less than the trusted anchor epoch.

After explicit caller proof that the user confirmed intentional older-backup recovery, B502 computes exactly `trusted_anchor.highest_epoch + 1`. Overflow is fail closed. It republishes the authenticated backup manifest semantics at that exact new epoch with `previous_manifest_hash = trusted_anchor.manifest_hash` and a fresh B203 manifest nonce.

The returned publication binds the exact expected old anchor and exact new anchor. B502 itself does not mutate protected state. A storage coordinator must write/fsync the returned candidate, reread/authenticate it, verify the complete referenced object set, and only then invoke the separately qualified protected compare-and-advance operation.

## Nonce and key-generation discipline

Before fresh republication, B502 extracts the manifest nonce reservation from the exact envelope that has already authenticated successfully. The caller must hydrate the supplied B203 ledger from retained authenticated canonical manifest history, excluding only the exact backup candidate being restored. If the source reservation is already present, B502 treats that as a collision with retained history and fails closed. Otherwise the source reservation is passed as an extra forbidden reservation to fresh nonce generation without being inserted into the persistent in-process ledger. This preserves retry semantics while ensuring a new candidate cannot reuse the authenticated source nonce.

Both the authenticated envelope key generation and the authenticated manifest plaintext `active_key_generation` must equal the current active key generation supplied by the caller. Either mismatch returns `KeyGenerationMismatch` before republication. This prevents an envelope derived under the current generation from smuggling a different retained/staged manifest generation into restore publication, and prevents B502 from pretending that an older recovery-wrapped VRK can be made current merely by advancing freshness. Cross-generation restore after a completed VRK rotation remains a B503 integration boundary and must preserve copy/verify/re-encryption semantics before this leaf can be treated as sufficient for that case.

The source manifest inventory and rotation fields remain authenticated canonical semantics. B502 does not weaken the existing manifest validator or invent a new rotation-state rule.

## Fresh-device restore genesis

Fresh-device restore authenticates the complete supplied manifest before checking the genesis state. The authenticated `VaultId` must equal the expected vault identity. Genesis is accepted only when an existing protected freshness record is explicitly `UNINITIALIZED`; `PRESENT` returns `AnchorAlreadyInitialized`. A missing provider item is not representable as genesis by this layer and remains a typed provider failure.

The accepted anchor uses the authenticated backup's non-zero epoch and `SHA-256(exact accepted manifest envelope bytes)`. The result records `global_newestness_proven = false`. The API requires an explicit `FreshDeviceNewestnessRiskAccepted::ACCEPTED` token so the call site cannot use an implicit/default acceptance path.

## Regression coverage

The B502 unit suite freezes these invariants:

- authentication wins over wrong-vault, already-present, and later restore-state decisions;
- older backup content is republished exactly one epoch above the trusted anchor with a fresh envelope hash and anchor linkage;
- equal/future epochs, wrong vault identity, epoch overflow, envelope-generation mismatch, and manifest-active-generation mismatch fail closed;
- the authenticated source manifest nonce is excluded from fresh generation without poisoning retry state, while collision with retained history fails closed;
- fresh-device restore accepts authenticated non-zero epochs only from explicit `UNINITIALIZED` and records inability to prove global newestness;
- protected `PRESENT` and wrong-vault fresh-device attempts fail closed.

## Non-claims and successor boundary

B502 does not claim that provider-visible backup metadata satisfies B505, that old-generation ciphertext is re-encrypted under a later VRK, that a returned publication has been durably fsynced, or that a protected anchor has already advanced. B503 must reconcile the cross-generation rotation boundary. B505 remains responsible for opaque portable backup packaging/transport and provider-visible metadata qualification.

No dependency, lockfile, provider, workflow, provenance registry, SBOM, notice, or donor-code adoption is introduced by this leaf.

Q009 remains `UNSATISFIED`. Repository-owner review, ChatGPT work, CI/R3, CodeRabbit, and this evidence do not substitute for the required genuinely independent substantive crypto/security review of the exact final Specification 004 implementation revision.
