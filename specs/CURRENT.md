# Himsat Current Program State

## Current state

```text
PROGRAM_STATE = SPEC_001_IMPLEMENTATION
ACTIVE_SPECIFICATION = 001-repository-foundation
SPEC_000_DISPOSITION = CLOSED_CANONICAL
SPEC_000_MERGE = 3f6687b34530e55098f7854042b12adfa607f394
SPEC_001_SHAPING_DISPOSITION = CLOSED_CANONICAL
SPEC_001_SHAPING_MERGE = c7fadaebfc44a60f982763bfc20e3a7d41551f4e
NATIVE_SPEC_GRAIN_STATE = TRACKED_REPORT_MODE_PENDING_EXACT_VALIDATION
IMPLEMENTATION_AUTHORITY = SPEC_001_FOUNDATION_ONLY
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
RELEASE_AUTHORITY = NONE
```

Specification 000 established the canonical planning foundation and merged as `3f6687b34530e55098f7854042b12adfa607f394`.

Specification 001 shaping then merged canonically as signed GitHub merge `c7fadaebfc44a60f982763bfc20e3a7d41551f4e`. Live GitHub state therefore closes the shaping gate and authorizes only the bounded Specification 001 repository/delivery-control implementation defined by its canonical `spec.md`, `plan.md`, and `tasks.md`.

This file does not claim that native SpecGrain has promoted any work item to `GRAIN`. The tracked `.specgrain` store introduced by the implementation candidate starts in `report` mode and must be validated by the exact pinned SpecGrain revision before any native lifecycle claim is made.

## Active objective

Implement and qualify the smallest reproducible engineering foundation required before Himsat can safely accept later product work:

- Rust `1.98.1` pinned explicitly;
- dependency-free Rust 2024 workspace/core crate;
- tracked native SpecGrain report-mode state;
- bounded Diffcipline R2 policy/proof;
- least-privilege Linux/macOS/Windows CI;
- negative controls proving the verification path detects failures;
- Apache-2.0 for Himsat-owned source;
- minimal contribution and security guidance.

The active specification is:

```text
specs/001-repository-foundation/spec.md
specs/001-repository-foundation/plan.md
specs/001-repository-foundation/tasks.md
```

Toolchain research:

```text
docs/research/2026-09-06-spec001-toolchain-baseline.md
```

## Exact authority boundary

The current implementation authority permits only Specification 001 foundation work. It does **not** authorize:

- Meetily, Anarlog, Xberg, or any other donor adoption;
- audio capture, STT, diarization, models, or media processing;
- Tauri/React/Node, Swift/Kotlin, mobile/desktop product shells;
- vault/crypto/storage product implementation;
- documents, memory, search, agents, plugins, connectors, or sync;
- release publication or platform-support/superiority claims.

The initial implementation may create Cargo manifests and the lockfile because they are intrinsic to the authorized Rust workspace. The core crate must remain dependency-free. The bootstrap Diffcipline policy may explicitly allow these initial manifest/lockfile additions so the exact foundation diff can be proven; after the foundation is canonical, dependency-manifest handling must be tightened before successor dependency adoption.

## Specification 001 implementation closeout conditions

Specification 001 can close only when:

1. implementation base is the canonical shaping merge `c7fadaebfc44a60f982763bfc20e3a7d41551f4e` or any intervening live change is reconciled;
2. exact changed paths remain within the bounded Specification 001 surface;
3. Rust `1.98.1` is observed on every configured Rust CI host;
4. format, clippy with warnings denied, and locked workspace tests succeed on Linux, macOS, and Windows;
5. the core crate remains dependency-free;
6. tracked SpecGrain state validates using `TheHalfMoon/SpecGrain@faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`;
7. Diffcipline R2 proof executes against the exact base/head rather than reporting configured-but-not-run verification;
8. negative controls demonstrate that formatting, lint/test, malformed SpecGrain state, missing verification, and out-of-scope changes are rejected;
9. workflow permissions remain read-only unless a separately justified need appears;
10. no donor source, model, telemetry, product feature, app-shell dependency, binary, or release artifact is introduced;
11. exact PR head/base, checks, reviews, threads, mergeability, and rulesets are reverified immediately before merge;
12. merge uses expected-head protection;
13. canonical `main` is re-read after merge and applicable post-merge verification is observed;
14. dependency-manifest policy is tightened for successor work before any third-party dependency adoption is authorized.

## Successor rule

Specification 002 — provenance/license/SBOM machinery — remains blocked until Specification 001 closes canonically. Even after Specification 002 is shaped, donor material remains blocked until its exact provenance/license mechanism provides explicit path/revision authority.

Specification 003 and all product implementation remain blocked by their dependency gates.

## Program dependency summary

```text
000 Foundation planning                    CLOSED_CANONICAL
  -> 001 Repository/delivery control       IMPLEMENTATION_ACTIVE
      -> 002 Provenance/license/SBOM        BLOCKED
      -> 003 Core event/schema foundation  BLOCKED
          -> 004 Vault/key architecture    BLOCKED
              -> 005 Crash-safe media journal/chunk store
                  -> 006 Capture abstraction + Capture Health
                      -> 007 macOS capture
                      -> ...
```

The detailed program remains canonical in `docs/execution-master-plan.md`.

## Authority rules

- Live GitHub/repository truth overrides this file if they disagree.
- No force-push/rebase/destructive shared-history rewrite is authorized.
- No donor code may be copied until a future provenance/license unit authorizes the exact material.
- No benchmark superiority claim may be made before reproducible comparative evidence exists.
- No platform capability may be claimed merely because an API exists.
- No specification may be called `CLOSED_CANONICAL` solely because a checklist is complete; exact evidence is required.
- Native SpecGrain lifecycle state must come from actual validated tool state, never from this Markdown label.
