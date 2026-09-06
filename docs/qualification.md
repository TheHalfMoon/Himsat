# Himsat Qualification and Benchmark Strategy

## Purpose

Himsat's ambition is to be best-in-class, but the repository must distinguish ambition from evidence. This document defines how quality claims become reproducible engineering facts.

The program follows two rules inherited from TheHalfMoon's delivery methods:

- **SpecGrain:** acceptance and evidence are defined before implementation of each bounded unit.
- **Diffcipline:** exact checks must run successfully against the exact change; `NOT RUN` is never `PASS`.

## Evidence classes

Every product unit should identify applicable evidence classes:

1. static/build evidence;
2. focused unit/integration evidence;
3. platform/device evidence;
4. negative/adversarial evidence;
5. performance/resource evidence;
6. privacy/network evidence;
7. accessibility evidence;
8. migration/backward-compatibility evidence;
9. license/provenance evidence;
10. independent/reviewer evidence where required.

## Benchmark principles

- pin code, model, dataset, device, OS, and runtime versions;
- preserve failed runs and known invalid runs;
- separate tuning/evaluation corpora;
- avoid benchmark claims from private/unreproducible datasets unless explicitly labeled internal;
- publish methodology before strong comparative marketing claims;
- do not choose only favorable devices/languages;
- report confidence intervals/variance where meaningful;
- distinguish quality from latency/resource tradeoffs;
- preserve raw or derived benchmark artifacts subject to dataset licenses/privacy.

## Speech recognition

### Metrics

- WER;
- CER;
- word timestamp error/alignment quality;
- proper-noun accuracy;
- numeric/date/currency accuracy;
- punctuation/formatting accuracy as a separate derived metric;
- code-switch segment accuracy;
- partial/final streaming latency;
- real-time factor;
- RAM/VRAM;
- CPU/GPU/NPU utilization;
- energy/battery impact.

### Language suites

Mandatory before broad “Arabic support” claims:

- Modern Standard Arabic;
- Saudi/Najdi;
- Hijazi;
- Gulf;
- Arabic-English code switching;
- proper nouns and transliteration;
- business terminology;
- technical/software terminology;
- medical/research terminology;
- numbers/dates/currency.

Expansion suites:

- Egyptian;
- Levantine;
- French/Spanish/German/Japanese and other high-demand languages as the router grows.

### Device classes

At minimum, qualification should eventually cover representative:

- Apple Silicon desktop/laptop;
- Windows x64 CPU-only;
- Windows NVIDIA GPU;
- Linux x64 CPU/GPU;
- recent high-end iPhone;
- older supported iPhone;
- recent flagship Android;
- mid-range supported Android.

A model is not “the default” globally because it wins on one workstation.

## Streaming UX targets

Initial engineering goals, not release claims until measured:

- recording-control response perceived immediately;
- first meaningful live transcript partial roughly within 1–2 seconds on reference modern hardware;
- finalized segment commonly within a few seconds, language/model dependent;
- no unbounded transcription backlog during qualified long sessions;
- UI remains responsive under sustained inference.

Exact thresholds belong in implementation specs after baseline measurement.

## Diarization and speaker identity

Metrics:

- DER;
- JER;
- speaker count error;
- speaker-turn boundary error;
- overlapping speech performance;
- speaker-attribution accuracy after optional enrollment;
- false speaker match rate;
- enrollment/reset correctness.

Test conditions:

- two speakers;
- 3–8 speakers;
- far-field room;
- remote mixed system audio;
- overlap;
- similar voices;
- Arabic/English switching;
- noise/reverb;
- source fusion.

## Audio processing

AEC/NS/AGC evaluation must not rely only on subjective developer listening.

Evidence should include:

- echo return loss/appropriate AEC metrics where feasible;
- speech quality/intelligibility metrics where valid;
- clipping rate;
- signal/noise indicators;
- VAD miss/false positive rate;
- regression audio fixtures;
- CPU cost/latency;
- before/after sample review for edge cases.

Never improve “noise score” by damaging speech intelligibility unnoticed.

## Capture reliability

### Long-session matrix

Targets to test progressively:

- 1 hour;
- 4 hours;
- 8 hours;
- 12 hours;
- 24-hour desktop torture test where technically sensible.

Mobile duration targets should be based on thermal/battery realism and still include multi-hour locked-screen sessions.

### Fault injection

- hard process termination;
- UI crash;
- worker/model crash;
- power interruption where testable;
- low/no disk;
- permission revoked;
- microphone unplugged;
- Bluetooth route changed;
- system-audio projection stopped;
- sleep/wake;
- network disabled/enabled;
- P2P peer disconnect;
- malformed/corrupt last chunk;
- database locked/corrupt scenario.

### Recovery metrics

- durable audio loss window;
- time to recover session metadata;
- successful journal reconciliation rate;
- orphan chunk detection;
- duplicate chunk rate;
- transcript/evidence consistency after recovery.

Engineering goal: no more than one bounded uncommitted media chunk should be at risk after sudden failure once the chunking design is implemented and qualified.

## Capture Health qualification

Inject failures during active recording and require the UI/state machine to distinguish:

- healthy signal;
- silent source;
- muted/disconnected source;
- projection revoked;
- clipping/noise;
- low storage;
- write failure;
- model backlog.

“No warning while saving silence” is a release blocker for a supposedly healthy state.

## Battery, power, and thermal

Measure:

- battery percentage/hour;
- thermal state transitions;
- CPU/GPU load;
- background vs foreground cost;
- live-only vs quality-pass cost;
- screen-on vs locked-screen;
- Bluetooth vs built-in mic;
- model/profile differences.

Quality-pass scheduling must be able to defer expensive work until charging/idle.

## Search and retrieval

### Retrieval metrics

- Recall@K;
- MRR/nDCG where applicable;
- answer evidence recall;
- false-evidence citation rate;
- temporal-state correctness;
- cross-language query retrieval;
- latency by corpus size;
- index build/rebuild time;
- storage overhead.

### Corpus sizes

Progressively test:

- 10 sessions;
- 100 sessions;
- 1,000 sessions;
- 10,000 sessions / large transcript-chunk corpus if the architecture targets that scale.

Use synthetic/redistributable corpora for public benchmark reproducibility when private meeting data cannot be published.

## Evidence-first intelligence

Metrics:

- evidence coverage of factual claims;
- unsupported claim rate;
- incorrect citation rate;
- decision extraction precision/recall;
- commitment extraction precision/recall;
- owner/due-date correctness;
- contradiction detection precision/recall;
- “insufficient evidence” calibration;
- correction stability after human edits;
- source lineage preservation across quality-pass transcript revisions.

A summary quality score without evidence quality is insufficient.

## Temporal memory

Test questions whose answers change over time:

```text
Jan: launch = March
Mar: launch = June
May: launch = September
```

Required query classes:

- latest state;
- state on historical date;
- what changed and when;
- evidence for each transition;
- superseded decision exclusion from “current” answer without deleting history.

## Document intelligence

### Format coverage tests

- born-digital PDF;
- scanned PDF;
- multi-column PDF;
- table-heavy PDF;
- Arabic/RTL PDF;
- DOCX with headings/tables/images/comments;
- PPTX with speaker notes/images/tables;
- XLSX with multiple sheets/formulas;
- images/scans;
- malformed inputs;
- archive/embedded-object limits.

### Metrics

- reading-order accuracy;
- text extraction accuracy;
- heading/list structure accuracy;
- table structure/cell accuracy;
- page/region citation accuracy;
- OCR character/word accuracy;
- image/asset extraction correctness;
- formula representation accuracy where supported;
- parse latency/memory;
- crash/failure isolation.

Document parsers are compared through Himsat IR behavior, not one donor's internal representation.

## Talk-to-document quality

Evaluation questions include:

- direct fact retrieval;
- table comparison;
- page citation;
- figure/chart source citation;
- absence question (“is X present?”);
- conflicting statements across versions;
- meeting-vs-document discrepancy;
- prompt injection text embedded in a document.

The agent must not obey document instructions that request unauthorized tools/data.

## Professional PDF qualification

Validate:

- deterministic output from deterministic input where practical;
- text extractability;
- page/TOC/bookmark links;
- semantic headings/lists/tables;
- Arabic/RTL/mixed-direction rendering;
- font embedding/license requirements;
- PDF/A profile when selected;
- PDF/UA/tagged accessibility profile when selected;
- screen-reader/accessibility checks where tooling supports them;
- visual regression for templates.

“Looks good on one laptop” is not enough.

## DOCX qualification

Open/edit round trips in representative office suites and verify:

- headings/styles;
- lists;
- tables;
- images;
- hyperlinks/bookmarks;
- footnotes/comments where emitted;
- RTL;
- TOC behavior;
- downstream extraction back to semantic text.

## Himsat Context Pack qualification

Require:

- schema validation;
- deterministic ordering;
- source digest correctness;
- no missing referenced evidence asset;
- included/excluded surface declaration;
- Markdown readability without Himsat;
- JSONL parseability;
- token-budget profile tests;
- compatibility smoke tests with multiple local/hosted LLM ingestion workflows where permitted.

The “best file for LLMs” claim should be framed as interoperability/quality evidence, not universal provider optimization.

## P2P/sync qualification

Test:

- fresh pairing;
- wrong peer/fingerprint;
- revoked peer;
- direct LAN;
- NAT traversal;
- public relay disabled;
- self-hosted relay where supported;
- interrupted large transfer;
- duplicate/partial blob;
- concurrent metadata update;
- device clock skew;
- re-pair after key rotation;
- selective space sharing;
- Network Lock behavior.

## Security tests

At increasing risk levels:

- dependency/license/SBOM checks;
- fuzz media/document parsers;
- archive bomb/path traversal;
- malformed model/plugin packages;
- signature/digest failure;
- plugin capability escape attempts;
- connector payload minimization;
- prompt-injection fixtures;
- secret/key logging checks;
- deletion/retention tests;
- encrypted-backup provider compromise model;
- pairing MITM/adversarial tests.

## Network privacy qualification

Strict mode test must observe network behavior at the process/system boundary, not only configuration state.

Evidence should capture:

- destinations attempted;
- bytes sent/received;
- process/component responsible;
- expected local IPC/LAN exceptions if any;
- model/update connector disabled state.

The target claim is **zero unexpected outbound connections**; do not promise literally zero packets if the platform/runtime generates unrelated OS traffic outside Himsat's process boundary.

## Accessibility qualification

- VoiceOver/TalkBack/NVDA/appropriate screen-reader paths;
- keyboard-only desktop flow;
- focus order;
- semantic recording-state announcements;
- scalable fonts;
- contrast;
- reduced motion;
- RTL/mixed-direction UI;
- live captions;
- accessible exported PDF semantics.

## Migration qualification

Each importer produces a report:

```text
source artifact count
imported count
skipped count
preserved fields
transformed fields
unsupported fields
warnings/errors
source digests
```

Silent data loss fails migration acceptance.

## Diffcipline risk profiles for Himsat

Himsat may map repository work to Diffcipline's `R0`–`R3` concept, with repository-specific commands added later.

### R0

Docs/cosmetic change with no product/security semantics.

### R1

Bounded low-risk product behavior or local refactor with focused tests.

### R2

Audio/transcription/search/document behavior, schema migrations, platform integration, dependency changes, connectors, publishing, sync.

Requires broader regression plus negative paths.

### R3

Crypto/key management, recording lifecycle/background services, plugin sandbox/capability broker, model package execution, biometric speaker identity, external write authority, sync trust/pairing, update/signing, privacy deletion guarantees.

Requires adversarial/negative evidence and independent substantive review before strong completion claims.

Risk is about blast radius, not line count.

## Release qualification ladder

### Developer preview

- architecture may change;
- only explicitly qualified platforms/features listed;
- no broad superiority claims.

### Alpha

- end-to-end local session on at least one desktop target;
- crash-safe recording baseline;
- basic local transcript/brief/search;
- provenance/license baseline;
- known gaps published.

### Beta

- multiple desktop platforms;
- mobile baseline;
- stronger long-session reliability;
- document intelligence;
- export/publishing;
- security review and migration strategy;
- benchmark dashboard begins.

### 1.0

Requires separately defined release specification, but expected evidence includes:

- signed/attested artifacts;
- SBOM/license closure;
- platform support matrix;
- long-session qualification;
- local/offline/network tests;
- speech/diarization/retrieval/document benchmarks;
- accessibility gates;
- backup/recovery/migration tests;
- security/threat-model reconciliation;
- no unresolved critical/high release blockers;
- user data export path;
- exact release-source preservation.

## Comparative claims

Before saying “better than Otter/Circleback/Granola/etc.” in a technical or marketing claim:

- define the dimension;
- select reproducible protocol;
- ensure product comparison is permitted/ethical;
- record competitor version/date/config;
- avoid comparing local Himsat offline behavior against a competitor feature that was not designed for the same constraint without explaining the difference;
- preserve results even when Himsat loses.

The repository may always state product architecture facts it can prove, such as absence of a required Himsat cloud dependency, without turning that into an unsupported universal quality claim.
