# Specification 000 Plan — Foundation Planning

## Strategy

Build the planning foundation in evidence-first layers instead of starting implementation from the first donor repository.

## Workstream 1 — Establish authority

Create:

- project constitution;
- canonical reading order;
- current program state;
- explicit non-authority for product code/donor adoption;
- SpecGrain and Diffcipline integration rules.

Expected result: future agents cannot interpret roadmap text as blanket implementation authority.

## Workstream 2 — Challenge the product thesis

Research current competitors, mobile/desktop OS capture rules, modern local speech/document stacks, plugin sandboxing, sync/publishing technologies, and donor licensing.

Questions challenged:

- Is “local Otter” enough? — no; the category now includes live assistance, actions, cross-source memory, and multimodal context.
- Can a phone record every app/call? — no; capability is OS/version/source-policy dependent.
- Is transcription the hard part? — not alone; capture health, acoustic preprocessing, background reliability, recovery, and storage are equally foundational.
- Is PDF chat enough? — no; Himsat needs a document IR, hostile-parser boundary, region citations, and portable export.
- Can all public donor code be copied? — no; current licensing is mixed across attractive projects.
- Does local mean secure/private automatically? — no; local plugins/models/docs/connectors/device sync add serious threats.
- Is a beautiful PDF the best AI interchange? — no; human publishing and LLM context need distinct outputs.

## Workstream 3 — Define product capability map

Ensure the product plan includes:

- Record, Meeting, Memory, Watch/Media, Read/Documents, Ask, Publish/Transform, Act;
- desktop/mobile/wearable/background capture;
- Himsat Bridge and later capture fusion;
- Recall Buffer as visible opt-in only;
- live/quality STT, diarization, speaker identity;
- multimodal timeline/Lens;
- temporal memory/evidence;
- Himsat Brief/Live;
- document intelligence;
- professional PDF/DOCX;
- open AI Context Pack;
- local agents/flows/plugin sandbox;
- connectors/privacy firewall;
- P2P/selective sharing;
- migration;
- daily intelligence;
- accessibility and Arabic/RTL.

## Workstream 4 — Define architecture boundaries

Record:

- Rust shared core;
- native Apple/Android services/UI;
- event/evidence core;
- durable chunked source storage;
- capture lifecycle and health;
- model registry and worker isolation;
- document IR and hostile import boundary;
- hybrid search and temporal memory;
- publishing IR/context pack;
- capability broker/plugin sandbox;
- Bridge vs durable sync separation;
- unresolved ADRs for future experiments.

## Workstream 5 — Define donor/provenance control

Classify donors as `COPY`, `DEPEND`, `VENDOR`, or `REFERENCE`.

Record examples where appealing code must stay reference-only by default due current licensing. Explicitly separate code, model, dataset, font, template, and asset terms.

The implementation successor must automate this policy before the first major donor transplant.

## Workstream 6 — Define security/platform truth

Threat-model:

- vault theft;
- malicious plugin/model/parser;
- prompt injection;
- connector exfiltration;
- compromised storage;
- pairing/sync attacks;
- crash/storage/power failures;
- biometric speaker data;
- update/build compromise.

Document OS-specific mobile capture limitations and the background reliability matrix.

## Workstream 7 — Define qualification

Specify metrics and evidence for:

- ASR/Arabic/code switching;
- diarization;
- audio processing;
- streaming latency;
- long-session reliability/recovery;
- battery/thermal;
- search/retrieval;
- evidence correctness;
- temporal memory;
- document extraction;
- PDF/DOCX/context pack;
- P2P/security/privacy;
- accessibility;
- migration.

Map high-risk Himsat changes to Diffcipline-style R2/R3 rigor.

## Workstream 8 — Build dependency-ordered candidate program

Create the 000–040 program map, but label it provisional. Explain which gates prevent premature product work and which candidate features remain intentionally unassigned until evidence selects them.

## Reconciliation

Before proposing merge:

1. compare the planning branch against `main`;
2. verify exact expected paths only;
3. inspect that no source/workflow/dependency/model was introduced;
4. inspect internal consistency between README/CURRENT/constitution/master plan;
5. review current external claims that materially affect platform/donor safety;
6. record checks as PASS/FAIL/NOT RUN accurately;
7. leave PR unmerged unless the exact planning head is qualified under repository policy available at that time.

## Post-closeout successor

The first implementation-era successor should establish repository/toolchain/delivery-control foundations. The donor provenance automation unit follows before donor source adoption. This sequence is deliberate: product code must not precede the mechanism used to prove where it came from.
