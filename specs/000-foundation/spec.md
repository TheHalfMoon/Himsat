# Specification 000 — Foundation Planning

## Status

```text
repository planning status: ACTIVE
native SpecGrain lifecycle state: NOT YET ESTABLISHED
implementation authority: NONE
```

This specification is written using SpecGrain concepts but is not represented as a native `GRAIN` unless the actual SpecGrain state/tooling later proves that transition.

## Title

Establish the canonical Himsat foundation plan

## Outcome

The repository contains one internally consistent, evidence-aware planning foundation that defines what Himsat is, what it will not claim, how it will be built, how donor code is governed, how sensitive platform/security boundaries are handled, how quality is proven, and how implementation is decomposed before product code begins.

## Scope in

- product thesis and feature map;
- gap/landscape research current to 2026-09-06;
- Rust-first/native-platform architecture;
- background recording/capture capability boundaries;
- audio quality/reliability architecture;
- document intelligence and portable AI context architecture;
- evidence/memory/agent/sync architecture;
- donor/provenance/license rules;
- security/privacy/platform capability plan;
- benchmark/qualification strategy;
- dependency-ordered execution master plan;
- repository agent rules/constitution/current-state authority.

## Scope out

- Rust workspace/product scaffolding;
- mobile/desktop application code;
- donor code copying or dependency adoption;
- model downloads/weights;
- CI/toolchain implementation;
- encryption implementation;
- any claim that a platform is already supported;
- any claim that Himsat beats a competitor;
- any release/version publication;
- native SpecGrain lifecycle mutation;
- Diffcipline PASS claim without actual configured execution.

## Dependencies

None inside Himsat: this is the repository's initial planning unit.

Methodology references:

- `TheHalfMoon/SpecGrain` current repository truth;
- `TheHalfMoon/Diffcipline` current repository truth.

External factual research references are recorded in `docs/research/2026-09-06-landscape-gap-analysis.md`.

## Risk

**Planning risk: R1**, with high downstream architectural influence.

The change itself is documentation/governance, but incorrect claims about mobile capture, licensing, cryptography, or donor eligibility could cause later R3 product mistakes. Therefore review must challenge claims even though no runtime behavior changes.

## Acceptance conditions

1. `README.md` defines Himsat without claiming a shipped product.
2. `AGENTS.md` binds future work to canonical reading order, SpecGrain discipline, Diffcipline proof rules, donor provenance, and platform truth.
3. `.specify/memory/constitution.md` records durable product/security/proof principles.
4. `specs/CURRENT.md` states that implementation and donor adoption are not yet authorized.
5. gap analysis distinguishes observed external behavior from proposed Himsat design.
6. product plan covers desktop/mobile/background capture, Himsat Bridge, media/document intelligence, memory/evidence, publishing, agents, connectors/sync, accessibility, Arabic/RTL, migration, and privacy modes.
7. architecture defines shared Rust core plus native Apple/Android responsibilities and versioned platform adapters.
8. donor plan uses exact-path/revision/license provenance and marks mixed/restrictive examples correctly as non-default donors.
9. security/platform plan explicitly rejects stealth recording and universal mobile call/system-audio claims.
10. qualification plan defines reliability, speech, diarization, retrieval, document, publishing, accessibility, privacy/network, and security evidence.
11. execution master plan is dependency ordered and clearly says roadmap candidates are not automatic Grains or blanket authority.
12. no product source, donor source, model, binary, workflow, or dependency manifest is introduced by this specification.
13. exact planning diff is reviewed before merge.
14. residual architectural unknowns remain explicit instead of being invented as settled decisions.

## Change surface

Expected paths:

```text
README.md
AGENTS.md
.specify/memory/constitution.md
specs/CURRENT.md
specs/000-foundation/spec.md
specs/000-foundation/plan.md
specs/000-foundation/tasks.md
docs/research/2026-09-06-landscape-gap-analysis.md
docs/product-plan.md
docs/architecture.md
docs/donor-and-provenance.md
docs/security-and-platform.md
docs/qualification.md
docs/execution-master-plan.md
```

No other path is required for Specification 000.

## Evidence requirements

Before merge/closeout:

- exact diff/path list against `main`;
- inspection that all changed paths are planning/governance only;
- internal-link/path consistency review;
- spot re-verification of time-sensitive external platform/product/license claims;
- confirmation that restrictive/mixed donors are not described as adopted code;
- confirmation that current branch/head has not moved after review;
- any available repository checks/CI recorded accurately;
- unavailable/not-run checks explicitly remain NOT RUN rather than PASS.

## Recovery

Because this unit contains planning only, recovery is a normal revert of the bounded planning commit/PR. The bootstrap `main` commit remains a minimal repository starting point. No user data, schema, donor history, model cache, or release artifact requires migration.

## Context budget and selection

The specification intentionally uses several focused planning documents rather than one unbounded master prompt. Future implementation agents should select only the active spec plus the architecture/security/donor context relevant to their bounded change.

## Minimality rationale

A smaller plan that only says “fork Meetily and build Otter locally” is insufficient because it omits platform constraints, donor licensing, hostile documents/plugins/models, data recovery, evidence architecture, mobile lifecycle, accessibility, publishing interoperability, and measurable qualification. The selected document set is the minimum foundation that separates those high-risk concerns while still deferring implementation decisions that require experiments.

## Safety status

No direct user-data processing is introduced. The plan explicitly constrains recording visibility, biometric handling, external actions, prompt injection, connector egress, and unsupported capture behavior.

## Closeout boundary

Closing Specification 000 authorizes only successor shaping. It does **not** itself authorize copying Meetily/Anarlog or implementing the entire roadmap.
