# Specification 005 — Crash-Safe Media Journal and Chunk Store

```text
LIFECYCLE = SHAPING
RISK = R3_DATA_INTEGRITY
DEPENDS_ON = 003_CLOSED_CANONICAL, 004_CLOSED_CANONICAL
IMPLEMENTATION_AUTHORITY = NONE_UNTIL_SHAPING_QUALIFIES
```

## Problem

Capture (006+) will produce hours of audio that must survive process death, power loss, and OS pressure without depending on UI/model workers. Without a reviewed crash-safe media foundation, a six-hour recording that disappears after a crash is a failed product regardless of later AI quality (constitution VII). Specification 005 establishes the smallest durable media-chunk and session-journal architecture that later capture units can consume.

## Canonical dependency state

Specification 003 is `CLOSED_CANONICAL` (session/source/event/object identities). Specification 004 is `CLOSED_CANONICAL` at closeout merge `c5255f6a60044edbdd7562fe1f295ce59e14c5a0` with post-merge CI `35091125997` and R3 `35091125863` SUCCESS. Shaping starts from that exact canonical `main`.

Consumable 004 contracts (no reinvention, no rewriting):

- B201 HKDF purpose-key derivation for media/session key domains;
- B202 bounded-blob XChaCha20-Poly1305 envelope with the 64 MiB plaintext ceiling (chunk plaintext must fit; chunking below the ceiling is a 005 decision);
- B203 OS-CSPRNG nonce lifecycle (fresh nonce per chunk, no reuse);
- B204 recovery envelope (media keys remain recoverable through the vault recovery path, never escrowed elsewhere);
- B501A authenticated manifest codec and inventory discipline (media chunks join the authenticated inventory through an additive kind, never by rewriting reviewed boundaries);
- B501 freshness discipline (session epochs advance under the existing anchor rules; no new anchor semantics);
- B503 rotation (media keys rotate with the vault hierarchy; no parallel key regime);
- B505 backup (chunks are restorable content under the existing backup/restore semantics);
- B506 deletion (chunks and journal records fall under the existing blob/temporary/canonical surface families; detached-backup and physical-erasure limits unchanged).

## Scope in

- fixed-maximum-size media chunk framing with exact byte layout and per-chunk B202-compatible authenticated encryption under a new 005-owned object purpose (additive to B202, never a rewrite of the v1 boundary);
- media/session key domains derived from the 004 hierarchy (no new KDF, no custom primitive);
- append-only session journal with fsync discipline: session open/close markers, chunk-commit records, truncation-safe tail;
- crash-recovery reconciliation: journal scan, orphan/duplication resolution against the authenticated manifest inventory, bounded-loss quantification (at most the un-fsynced tail window);
- negative/adversarial evidence: kill-style fault injection, corruption/orphan/duplicate fixtures, wrong-key/transplant chunk rejection, opaque chunk filenames per 004 metadata rules.

## Scope out

- capture sources, background services, permissions, indicators (006+);
- codecs, DSP, AEC/NS/AGC, transcription, models (010+);
- sync/pairing/relay, connectors, plugins, network egress;
- custom cryptographic primitives or new KDFs;
- rewriting any reviewed 004 envelope, manifest, rotation, backup, or deletion boundary;
- physical secure erase or universal metadata-secrecy claims;
- release/FIPS/compliance claims without separate qualification.

## Donor evaluation posture

Recording-manager/journaling/file-queue patterns in `Zackriya-Solutions/meetily`, `fastrepl/anarlog` (non-enterprise paths), and `Starmel/OpenSuperWhisper` are recorded planning inputs (see `docs/donor-and-provenance.md`). Implementation leaves must COMPARE selective COPY/ADAPT against a Himsat-native alternative on engineering merit with exact revision/path/permission/provenance evidence before any adoption. Shaping adopts nothing.

## Acceptance criteria

Specification 005 shaping is complete when this spec/plan/tasks packet is exact-head qualified (CI + R3 SUCCESS), reconciled, expected-head merged, and post-merge qualified. Implementation leaves are re-bounded only after shaping qualifies; no implementation authority opens in this unit.
