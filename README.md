# Himsat

**Remember everything. Privately.**

Himsat is being designed as an open-source, local-first Conversation Intelligence OS for desktop, mobile, and wearable devices. It captures user-authorized conversations and media, understands them on-device, builds evidence-linked memory, works with documents, and turns that context into useful briefs, reports, actions, and portable AI context without requiring a Himsat cloud.

## Status

Himsat is in **foundation planning**. This repository does not yet claim a working product, qualified capture pipeline, supported platform, benchmark result, or completed donor integration.

The current planning program deliberately separates product ambition from implementation proof. A feature is not considered delivered until the relevant exact revision has passed its declared checks and evidence gates.

## Product direction

Himsat is intended to support, subject to platform capability and explicit user permission:

- local microphone and desktop system-audio capture;
- user-started background recording that continues while the app UI is hidden or the phone is locked, within operating-system rules;
- in-person meetings, lectures, interviews, voice notes, media watching, and imported audio/video;
- desktop-to-phone local capture through **Himsat Bridge**;
- live and high-accuracy two-pass transcription;
- local speaker diarization and optional speaker identity;
- multimodal meeting context from audio, user notes, selected screen/window content, slides, and documents;
- evidence-first summaries, decisions, commitments, risks, unresolved questions, and temporal change tracking;
- local search and cross-meeting/project memory;
- PDF, DOCX, PPTX, XLSX, Markdown, images, and other document intelligence;
- professional accessible PDF and DOCX publishing;
- a portable AI Context Pack using Markdown plus structured evidence/manifest data;
- local MCP/API/CLI access with capability-scoped permissions;
- local automation and agent workflows with explicit approval for external writes;
- encrypted local vaults, optional encrypted user-owned storage, and accountless device-to-device sync;
- Arabic-first quality work alongside broad multilingual support.

## Non-negotiable design principles

1. **No Himsat cloud dependency.** Core capture, storage, transcription, search, memory, and intelligence must be usable without a Himsat-hosted service.
2. **Local-first by construction.** Networking is a capability granted only to explicit connector/sync/update surfaces.
3. **User-visible capture.** Himsat must not be designed as a stealth recorder or as a mechanism to bypass operating-system privacy indicators or protected-content controls.
4. **Evidence before AI confidence.** Factual outputs that are expected to be grounded in user sources must link to source evidence or state that evidence is insufficient.
5. **Data ownership and portability.** The user owns the files, keys, models, exports, and memory.
6. **Platform truth over marketing.** Mobile system-audio and phone-call capture capabilities differ by OS/version/app policy; Himsat must detect and report actual capability instead of promising impossible behavior.
7. **Open provenance.** Donor code, models, datasets, assets, and closely adapted material require license-aware provenance.
8. **Proof before done.** Repository claims follow SpecGrain and Diffcipline principles: bounded work, explicit evidence, exact-change verification, and no invented PASS state.

## Canonical reading order

Before changing product direction or implementation:

1. `AGENTS.md`
2. `specs/CURRENT.md`
3. `.specify/memory/constitution.md`
4. `docs/execution-master-plan.md`
5. the active specification's `spec.md`, `plan.md`, and `tasks.md`
6. referenced architecture, research, donor, security, and qualification documents
7. live GitHub state and exact changed revision

Live repository/GitHub truth overrides stale chat summaries and stale planning text when they disagree.

## Planning methodology

Himsat adopts two project-local methods owned by TheHalfMoon:

- **SpecGrain** — recursively refine work until a bounded Grain has explicit scope, dependencies, recovery, context, acceptance, and evidence requirements.
- **Diffcipline** — Think → Challenge → Minimize → Change → Prove; repository facts and executed verification outrank agent self-report.

Planning artifacts in this repository do **not** automatically imply native SpecGrain lifecycle promotion. A document describing a candidate Grain is not proof that SpecGrain has declared it `GRAIN`.

## Repository map

The planned repository shape is documented in `docs/architecture.md`. The most important planning artifacts are:

- `docs/research/2026-09-06-landscape-gap-analysis.md` — researched competitive/donor landscape and gaps discovered before implementation;
- `docs/product-plan.md` — complete product capability map and user experience targets;
- `docs/architecture.md` — Rust-first core, native mobile shells, data/event model, intelligence and document architecture;
- `docs/donor-and-provenance.md` — donor classes, license rules, and current candidate registry;
- `docs/security-and-platform.md` — threat model, capture constraints, privacy and platform rules;
- `docs/qualification.md` — benchmark and release-proof strategy;
- `docs/execution-master-plan.md` — dependency-ordered canonical program sequence;
- `specs/000-foundation/` — initial bounded planning specification.

## License direction

No final Himsat licensing declaration is implied by the planning state. The current direction is to use a permissive license for Himsat-owned code (Apache-2.0 is preferred for new core code because of its explicit patent grant) while preserving the original licenses and notices of donor material. This must be finalized as a bounded governance unit before donor code is incorporated.
