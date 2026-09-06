# Specification 001 Toolchain Baseline Research

**Date:** 2026-09-06  
**Canonical Himsat base:** `3f6687b34530e55098f7854042b12adfa607f394`

## Purpose

Record the external/tooling facts used to shape Specification 001 so future implementation can distinguish deliberate choices from stale assumptions.

## Rust

Official source:

- https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/

Observed on 2026-09-06:

- Rust `1.98.1` is the current stable point release;
- it was published on 2026-09-03;
- it fixes a vtable-generation miscompilation in Rust 1.98.0 that could produce undefined behavior.

Decision for Specification 001:

> Pin Rust `1.98.1`, not `1.98.0` and not an unpinned `stable` channel.

The pin is a reproducibility choice for this foundation, not a promise never to upgrade. Future upgrades require their own exact verification.

## Node.js

Official source:

- https://nodejs.org/en/download

Observed on 2026-09-06:

- Node.js `24.20.0` is the latest LTS shown by the official download surface;
- Node.js `26.8.1` is Current.

Decision for Specification 001:

> Do not add or pin Node.js yet.

Rationale: Specification 001 has no JavaScript/TypeScript/Tauri frontend implementation. Adding Node because Himsat may need it later would create an unused toolchain and verification surface. The first specification that actually introduces a JS/TS app shell must recheck and pin the then-current qualified LTS/package manager.

## SpecGrain

Repository:

- https://github.com/TheHalfMoon/SpecGrain

Exact source observed:

`faddebccb4f4b1dd71bf06b1ce7e3d7b367178ed`

Relevant current-source behavior:

- repository-local deterministic store;
- `init`, `draft`, `shape`, `refine`, `grain`, `check`, `next`, `packet`, `scan`, and `prove` CLI surfaces;
- native pre-Grain lifecycle `DRAFT -> SHAPED -> REFINING -> GRAIN`;
- evidence/readiness rules that distinguish tool state from narrative claims.

The historical published v0.3.0 release does not contain every current pre-Grain CLI surface. Himsat therefore must pin the exact source revision it validates against rather than saying merely “latest SpecGrain.”

Decision for Specification 001:

> Commit a minimal `.specgrain` project store in readiness `report` mode, keep it free of invented Grain state, and require `specgrain check` using the exact source pin.

## Diffcipline

Repository:

- https://github.com/TheHalfMoon/Diffcipline

Immutable v1 source/release commit recorded by Diffcipline:

`5cb1c77340b75649f6168e0e8f66479ea047ea96`

Relevant contract:

- `Think -> Challenge -> Minimize -> Change -> Prove`;
- exact diff/scope inspection;
- R0-R3 risk-scaled verification;
- `PASS`, `REVIEW`, `FAIL` with `NOT RUN` never represented as PASS;
- machine-readable proof and GitHub Action support.

Decision for Specification 001:

> Pin Diffcipline to the immutable v1 identity and configure an R2 repository policy whose verification commands execute on the actual Himsat workspace.

## Current Himsat repository enforcement state

Observed during Specification 000 closeout:

- no repository rulesets returned;
- `main` branch metadata reported `protected=false`;
- required status-check enforcement was off;
- no Himsat GitHub Actions workflow existed;
- no Diffcipline policy existed;
- no native `.specgrain` state existed;
- no Rust/Node/dependency manifest existed.

Decision for Specification 001:

> Repository-file CI and delivery policy must become the first executable proof boundary. Organization/branch administration that is not available through the authorized repository interface remains an explicit residual governance item, not a fabricated enforcement claim.

## Minimality conclusion

Specification 001 should introduce:

- Rust 1.98.1;
- one dependency-free Rust crate;
- Cargo workspace verification;
- SpecGrain state/check;
- Diffcipline policy/proof;
- one least-privilege CI workflow;
- Apache-2.0 Himsat-owned source licensing;
- minimum contribution/security files.

It should **not** introduce:

- Node/Tauri/React;
- audio/media/AI/database/network dependencies;
- donor source;
- models;
- app packaging/release machinery.

This keeps the foundation small enough to prove before the first product or donor boundary is crossed.
