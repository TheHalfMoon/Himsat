# Himsat Execution Master Plan

## Purpose

This document is the durable dependency-ordered continuation plan for Himsat. It converts the product vision into a sequence of bounded program candidates while preserving SpecGrain's rule that roadmap entries are **not automatically Grains**.

`specs/CURRENT.md` owns the active frontier. Live GitHub/repository truth overrides this plan when they disagree.

## Canonical reading order

Before changing the repository:

1. `AGENTS.md`;
2. `specs/CURRENT.md`;
3. `.specify/memory/constitution.md`;
4. this file;
5. active `spec.md`, `plan.md`, and `tasks.md`;
6. `governance/provenance/source-use-authorization.md`;
7. referenced research/architecture/security/donor/qualification artifacts;
8. live GitHub state, exact branch/head/diff/checks/reviews.

## Delivery method

Himsat combines the methods of `TheHalfMoon/SpecGrain` and `TheHalfMoon/Diffcipline`.

### SpecGrain planning rules

Each successor is shaped only when live evidence establishes a bounded need. Candidate units below may be split, reordered, or rejected during shaping.

A selected implementation leaf must make explicit:

- outcome;
- `scope_in`;
- `scope_out`;
- dependencies;
- acceptance conditions;
- risk level;
- recovery path;
- context budget/footprint;
- change surface;
- evidence requirements;
- minimality rationale;
- safety/security implications.

Do not call a roadmap candidate `GRAIN` unless native SpecGrain state/readiness actually establishes it.

### Diffcipline finish-line rules

For every implementation/reconciliation unit:

**Think → Challenge → Minimize → Change → Prove**

- exact diff over narrative;
- exact executed verification over self-report;
- `NOT RUN` is never `PASS`;
- higher-risk work requires stronger negative/adversarial evidence;
- no merge/complete/qualified claim without exact evidence.

## Program gates

### Gate A — Planning foundation

No donor adoption/product implementation until Specification 000 is reviewed and the first implementation successor is shaped.

### Gate B — Provenance before donors

No non-trivial copied/closely adapted/vendor/dependency donor material before machine-readable donor/provenance policy is established and exact source revision/path plus controlling public terms or separate permission basis are recorded. Founder permission makes covered sources eligible for evaluation; it does not waive the adoption boundary.

### Gate C — Durable source before intelligence

No product should depend on AI extraction before crash-safe source preservation and source lineage exist.

### Gate D — Evidence before memory

Durable decisions/commitments/claims require a source-aware evidence model before cross-session memory is treated as canonical user memory.

### Gate E — Platform qualification before support claim

A platform is not “supported” because it compiles. User-visible background/capture lifecycle, interruptions, recovery, and device-specific tests are required.

### Gate F — External action after capability security

No connector automation/external writes before capability policy, prompt-injection boundaries, payload minimization, approval/dry-run, and audit provenance are established.

### Gate G — Release after qualification closure

No 1.0 or comparative superiority claim before platform/reliability/privacy/accessibility/benchmark/security/release evidence closes.

## Source-reuse authorization and selection rule

The founder/user has recorded source-use permission for every external source referenced anywhere in the repository at or before the exact snapshot named in `governance/provenance/source-use-authorization.md`.

This changes the planning default for covered sources:

```text
OLD_CONSERVATIVE_DEFAULT = REIMPLEMENT_WHEN_PERMISSION_UNCERTAIN
NEW_DEFAULT = COMPARE_REFERENCE_COPY_ADAPT_DEPEND_VENDOR_NATIVE_ON_ENGINEERING_MERIT
AUTOMATIC_ADOPTION = NO
ACTIVE_UNIT_AUTHORITY_CHANGED = NO
```

When a source becomes relevant, the selected unit must compare the best available donor implementations and a Himsat-native alternative. Prefer the smallest reliable long-term boundary; do not bulk-copy simply because permission exists. Exact path/revision, permission/license scope, embedded third-party material, models/assets, notices, Himsat behavior tests, and risk-appropriate qualification remain mandatory.

The detailed reuse decision matrix and current donor registry are maintained in `docs/donor-and-provenance.md`.

## Provisional dependency graph

```text
000 Foundation planning
  |
  v
001 Repository + delivery-control foundation
  |
  +--> 002 Provenance/license/SBOM machinery
  |
  +--> 003 Core schemas + event/evidence identities
          |
          +--> 004 Vault/key/crypto design + storage foundation
          |      |
          |      +--> 005 Crash-safe media journal/chunk store
          |              |
          |              +--> 006 Capture abstraction + Capture Health
          |                      |
          |                      +--> 007 macOS capture
          |                      +--> 008 Windows capture
          |                      +--> 009 Linux capture
          |                      +--> 010 Audio processing AEC/NS/AGC
          |
          +--> 011 Model registry + speech-engine contract
                 |
                 +--> 012 Live transcription
                 +--> 013 Quality-pass transcription/alignment
                 +--> 014 Diarization + speaker vault

007/008/009 + 012/014
  -> 015 Desktop background runtime + meeting detection

004 + 006 + 011
  -> 016 Apple mobile foundation/background recording
  -> 017 Android mobile foundation/foreground-service recording

007/008/009 + 016/017 + 004
  -> 018 Himsat Bridge live P2P session protocol

005 + 011
  -> 019 Audio/video import pipeline

003 + 004
  -> 020 Document IR + hostile-import boundary
       -> 021 PDF intelligence
       -> 022 Office/document formats
       -> 023 OCR + visual understanding

003 + 020 + 012/013
  -> 024 Hybrid local search
       -> 025 Evidence engine
            -> 026 Temporal memory graph
                 -> 027 Intelligence + Himsat Brief

020 + 025
  -> 028 Semantic publishing IR + professional PDF
       -> 029 DOCX + open Himsat Context Pack

015 + 025 + 027
  -> 030 Himsat Live

025 + 004
  -> 031 Local API/CLI/MCP + permission broker
       -> 032 Wasm plugin sandbox
       -> 033 Himsat Flows/action proposal runtime
            -> 034 Connectors + privacy firewall

004 + 018
  -> 035 Durable P2P sync + selective sharing

016 + 018
  -> 036 Apple Watch/wearable surface

019 + 020 + 025
  -> 037 Bring Your Memory migration adapters

all shipped surfaces
  -> 038 Accessibility + RTL/i18n reconciliation
  -> 039 Security/fuzz/supply-chain/update hardening
  -> 040 Public release qualification
```

Cross-cutting accessibility/security/RTL requirements apply inside earlier units; Specifications 038–039 are reconciliation/closure candidates, not permission to defer obvious requirements until the end.

## Candidate unit register

### 000 — Foundation planning

**Outcome:** establish product, architecture, platform, donor, security, qualification, and program truth.  
**Risk:** R1 planning/governance.  
**No product code.**

### 001 — Repository and delivery-control foundation

**Outcome:** create the real Rust workspace/app skeleton only after toolchain decisions are explicit; wire repository-local SpecGrain/Diffcipline usage, CI basics, formatting/lint/test scaffolding, contribution/security/license placeholders appropriate to live decisions.  
**Scope-out:** donor code, audio implementation, models, product feature claims.  
**Evidence:** clean multi-platform-capable workspace checks, deterministic policy files, exact CI proof.

### 002 — Provenance, license, SBOM machinery

**Outcome:** machine-readable donor/model/asset registry and CI policy for unknown/incompatible adoption, including separate permission evidence where an adoption does not rely solely on a compatible public license.  
**Scope-out:** copying donor product behavior.  
**Risk:** R2 governance/supply chain.  
**Evidence:** positive/negative fixtures for allowed/manual/denied licenses, separate-permission cases as needed, generated notice closure, source-digest checks.

### 003 — Core identities, schemas, events

**Outcome:** minimal Himsat-owned IDs/types for Session, Source, Event, Artifact, EvidenceRef, model/parser identity, schema versioning.  
**Scope-out:** storage encryption and domain-specific intelligence.  
**Risk:** R2 because schema becomes foundational.  
**Recovery:** versioned migrations or pre-release schema reset according to stage.

### 004 — Vault/key/crypto architecture

**Outcome:** reviewed local vault/key hierarchy and encrypted structured/blob storage foundation.  
**Risk:** R3.  
**Required:** threat model, OS key storage adapters, backup/recovery/deletion semantics, independent crypto/security review.  
**Scope-out:** inventing custom cryptographic primitives.

### 005 — Crash-safe media journal and chunk store

**Outcome:** bounded durable media chunks and recoverable session journal independent of UI/model workers.  
**Risk:** R3 data integrity.  
**Evidence:** forced termination/power-like fault injection, corruption/orphan/duplicate reconciliation, bounded loss proof.

### 006 — Capture abstraction and Capture Health

**Outcome:** stable source/capture lifecycle contract with meaningful health telemetry.  
**Scope:** source attached/detached, signal, silence, clipping, route state, durable checkpoint, backpressure, storage pressure.  
**Risk:** R2.

### 007 — macOS capture

**Outcome:** qualified microphone + authorized system/screen-audio pathways using current supported Apple APIs.  
**Donor review:** Meetily/Anarlog/OpenSuperWhisper/native API patterns and any stronger covered source available at shaping time.  
**Risk:** R3 recording lifecycle.  
**Evidence:** route changes, permissions, sleep/wake, long-session matrix.

### 008 — Windows capture

**Outcome:** qualified microphone + WASAPI/system-audio path.  
**Risk:** R3.  
**Evidence:** device changes, exclusive/shared conflicts, Bluetooth/USB, long sessions.

### 009 — Linux capture

**Outcome:** PipeWire-first qualified microphone/system pathway with justified fallbacks.  
**Risk:** R3.  
**Evidence:** Wayland/PipeWire target matrix and degradation behavior.

### 010 — Audio processing

**Outcome:** AEC/NS/AGC/limiting/resampling pipeline behind swappable contract.  
**Donor candidates:** Sonora/WebRTC AudioProcessing plus any stronger covered implementation identified at shaping time.  
**Risk:** R2.  
**Evidence:** objective/perceptual fixtures, CPU/latency, speech-damage regression.

### 011 — Model registry and speech-engine contract

**Outcome:** model identity/license-or-permission/digest/resource manifest, offline side-load path, engine abstraction and explicit Voice Model Router exposing locality, capability, resource, and Network Lock compatibility.  
**Donor candidates:** whisper.cpp, sherpa-onnx, Argmax OSS, Moonshine/Parakeet experiments, OpenSuperWhisper engine/routing patterns, and other covered engines after exact comparison.  
**Risk:** R2 supply chain/native execution.

### 012 — Live transcription

**Outcome:** low-latency streaming transcript revisions with backpressure/cancellation and source time mapping, suitable for both meeting/live surfaces and later dictation-grade fast paths without conflating their retention/consent UX.  
**Risk:** R2.  
**Evidence:** latency/resource/device/language benchmarks.

### 013 — Quality-pass transcription and alignment

**Outcome:** non-destructive high-accuracy revision linked to live/source evidence.  
**Evidence:** alignment accuracy, revision lineage, recovery after worker crash.

### 014 — Diarization and speaker vault

**Outcome:** diarization plus optional encrypted speaker identity/correction/deletion.  
**Risk:** R3 biometric/privacy.  
**Evidence:** DER/JER, false match, overlap, opt-in/reset/privacy tests.

### 015 — Desktop runtime, controls, meeting detection

**Outcome:** close-window-to-tray/menu runtime, launch-ready behavior, floating controls, opt-in meeting detection, session recovery, and a reusable Input Control Layer for global shortcuts/push-to-talk/mouse controls where supported.  
**Donor review:** Meetily, Anarlog, OpenSuperWhisper, native platform APIs, and any stronger covered implementation available at shaping time.  
**Scope-out:** hidden recording.  
**Risk:** R3 due persistent recording state.

### 016 — Apple mobile foundation/background recording

**Outcome:** native SwiftUI + Rust-core integration; user-started microphone session survives ordinary background/lock states; Live Activity/recording intent integration where supported.  
**Scope-out:** universal other-app audio/call recording.  
**Risk:** R3.  
**Evidence:** OS-version/device/interruption/route/thermal/battery matrix.

### 017 — Android mobile foundation/background recording

**Outcome:** Compose + Rust-core integration; microphone foreground-service lifecycle and visible notification controls.  
**Scope-out:** privileged cellular call capture.  
**Risk:** R3.  
**Evidence:** Android-version/device/process-death/background-start/concurrent-input matrix.

### 018 — Himsat Bridge

**Outcome:** authenticated encrypted phone↔desktop live session protocol with source provenance, clock sync, reconnect, direct/LAN policy.  
**Risk:** R3 trust/network.  
**Scope-out:** durable cross-vault sync/CRDT and hidden public relay.

### 019 — Audio/video import

**Outcome:** safe cancellable media import, demux/decode, timeline/chapter hooks, crash isolation, and recoverable file-transcription queue behavior where shaped.  
**Donor review:** covered media/transcription sources including OpenSuperWhisper queue UX/implementation patterns where useful.  
**Risk:** R2 hostile media parsing.  
**Evidence:** malformed/large/media-format matrix and fuzzing as justified.

### 020 — Document IR and hostile-import boundary

**Outcome:** Himsat-owned document representation and resource-limited parser adapter contract.  
**Risk:** R3 untrusted parser surface.  
**Scope-out:** universal high-quality format support in first unit.

### 021 — PDF intelligence

**Outcome:** born-digital/scanned PDF text/layout/page/region/table/asset extraction into Document IR.  
**Evidence:** multi-column/table/RTL/scanned/malformed PDF suite and citation accuracy.

### 022 — Office and common document formats

**Outcome:** DOCX/PPTX/XLSX/Markdown/HTML/text adapters with semantic preservation and embedded asset handling.  
**Risk:** R2 parser complexity.

### 023 — OCR and visual understanding

**Outcome:** local OCR plus optional local vision adapter for selected page/frame interpretation with provenance.  
**Scope-out:** always-on blanket screen surveillance.  
**Evidence:** OCR multilingual/RTL/layout and model-resource tests.

### 024 — Hybrid local search

**Outcome:** lexical + semantic + filtered retrieval behind rebuildable indexes.  
**Candidates:** FTS5, sqlite-vec abstraction, fastembed-rs, optional reranker.  
**Evidence:** Recall@K/MRR/nDCG/latency/scale/cross-language.

### 025 — Evidence engine

**Outcome:** source-aware evidence refs/resolution for audio ranges, transcript ranges, document regions, visual frames, notes, artifacts.  
**Risk:** R2 correctness foundation.  
**Acceptance:** derived factual claim can resolve exact source or explicitly remain unsupported.

### 026 — Temporal memory graph

**Outcome:** revisioned evidence-linked Persons/Projects/Decisions/Commitments/Questions/Risks and historical state.  
**Features enabled:** Decision Drift, Commitment Ledger, Conversation Time Machine, What Changed, contradiction graph.  
**Risk:** R2 high semantic impact.

### 027 — Intelligence and Himsat Brief

**Outcome:** structured extraction and pre/post-session brief templates over evidence/memory; no one-shot opaque summary authority.  
**Evidence:** decision/commitment precision/recall, unsupported claim/citation rate.

### 028 — Semantic publishing + professional PDF

**Outcome:** format-neutral PublishDocument plus Typst-backed professional/RTL/accessibility-aware PDF profiles.  
**Evidence:** PDF/A/PDF-UA target validation where selected, text/TOC/bookmark/RTL/accessibility/visual regression.

### 029 — DOCX + Himsat Context Pack

**Outcome:** clean semantic DOCX and open AI-oriented Markdown/JSONL bundle with deterministic manifests/evidence.  
**Evidence:** office-suite round trips, schema validation, source digest closure, multi-LLM ingestion smoke tests.

### 030 — Himsat Live

**Outcome:** low-latency floating/mobile live surface for transcript, evidence, questions, decisions, relevant project/doc context, opt-in playbooks.  
**Scope-out:** unsupported emotion/personality truth scoring.

### 031 — Local API/CLI/MCP + permission broker

**Outcome:** capability-scoped local developer/agent interface; read/write/sensitive scopes separate. Its capability model is also the foundation for a later Context Capability Broker used by context-aware dictation/modes.  
**Risk:** R3 authorization.  
**Evidence:** capability negative tests, prompt-injection/tool tests, no ambient raw-audio/biometric access.

### 032 — Plugin sandbox

**Outcome:** Wasm-based or equivalently constrained extension runtime with resource/network/filesystem/vault limits and package integrity.  
**Candidates:** Extism/Wasmtime.  
**Risk:** R3 untrusted code.

### 033 — Himsat Flows

**Outcome:** local deterministic trigger/condition/action schema, AI extraction as candidate input, dry-run/replay, action proposal evidence.  
**Risk:** R3 once actions can mutate state.

### 034 — Connectors + privacy firewall

**Outcome:** least-privilege optional storage/workflow connectors, minimized payload preview, approval, local egress audit.  
**Initial candidates:** Drive/Box and calendar/contact foundations only if separately shaped.  
**Scope-out:** connector breadth race before privacy contract is proven.

### 035 — Durable P2P sync and selective sharing

**Outcome:** accountless encrypted device replicas and scoped sharing with revocation/resume/conflict semantics.  
**Candidate transport:** Iroh behind Himsat relay policy.  
**Risk:** R3 key/trust/data consistency.

### 036 — Watch/wearable

**Outcome:** native watch start/stop/bookmark/remote-control and bounded independent recording/transfer where platform permits.  
**Risk:** R2/3 lifecycle/privacy.

### 037 — Bring Your Memory

**Outcome:** import adapters with explicit preserved/transformed/unsupported migration reports.  
**Targets selected based on available export formats at implementation time.  
**Rule:** silent source loss is failure.

### 038 — Accessibility, RTL, internationalization reconciliation

**Outcome:** cross-surface closure for keyboard/screen-reader/live-caption/RTL/mixed-direction/accessible publish behavior and language UX.  
**Not permission to defer accessibility earlier.**

### 039 — Security, fuzzing, supply chain, updates

**Outcome:** threat-model reconciliation, parser/media fuzzing, plugin/model integrity, SBOM/license/permission closure, release signing/update verification, deletion/backup/security evidence.  
**Risk:** R3.

### 040 — Public release qualification

**Outcome:** bounded release spec based on actual supported surfaces, not roadmap aspiration.  
**Expected proof:** signed/attested artifacts, platform matrix, long-session reliability, network/privacy, speech/diarization/retrieval/document/publish/accessibility benchmarks, backup/export/migration, security closeout, exact release-source preservation.

## Source-derived roadmap overlays

The Superwhisper/OpenSuperWhisper research and the broader covered donor set expose product capabilities that should be carried into future shaping without prematurely creating new fixed-number units.

### Universal Dictation Surface

**Dependencies:** 011 + 012 + 015.  
**Outcome candidate:** system-wide private voice-to-text/voice-to-polished-text with explicit recording state, deterministic target insertion/fallback, and retention/consent semantics distinct from meetings.  
**Source inputs:** Superwhisper product behavior, OpenSuperWhisper implementation, Meetily/Anarlog/native platform mechanisms, and any stronger covered source available at shaping time.

### Terminology System

**Dependencies:** 003 + 011 + 012, with 024/025 integration where evidence-aware project context is needed.  
**Outcome candidate:** separate probabilistic `VocabularyHint` and deterministic `ReplacementRule` layers with project/mode scope, reversible rules, import/export, and benchmark evidence.

### Mode Engine

**Dependencies:** dictation/live surface + 031 capability foundations.  
**Outcome candidate:** versioned/exportable declarative profiles covering capture policy, model route, language, context capabilities, post-processing instructions, output target, activation rules, network policy, and retention policy.

### Context Capability Broker

**Dependencies:** 031 plus the user-visible surface consuming context.  
**Outcome candidate:** explicit per-mode grants for selected text, focused field, clipboard, app/window identity, screen region, meeting context, and project memory, with payload bounds and no hidden ambient context.

### Voice Model Router

**Home:** 011.  
**Outcome candidate:** queryable model/provider routing by locality, language, streaming, diarization, resource, provenance, and Network Lock compatibility.

### Input Control Layer

**Home:** 015.  
**Outcome candidate:** reliable keyboard chords, left/right modifiers where supported, push-to-talk, mouse buttons, tray/menu actions, deep-link/automation invocation, collision handling, and visible recording state.

These overlays are canonical planning inputs, not current implementation authority. Their exact Grain/unit boundaries are shaped only when their dependencies and active frontier permit implementation.

## Program features not assigned a fixed unit yet

These require evidence before selection and may become children of existing units or new successors:

- Recall Buffer;
- Consensus Capture multi-device source fusion;
- translated live captions;
- local TTS/voice playback;
- camera document scanner;
- Wear OS;
- self-hosted relay administration;
- collaborative shared notes/CRDT;
- optional local LAN web client;
- domain packs (sales/research/education/healthcare) that do not compromise general privacy/security;
- hardware recorder integrations;
- Apple/new-OS advanced screen/audio capture APIs;
- local meeting room/server appliance.

No implementation authority exists merely because a feature appears here.

## Cross-spec quality rules

1. **Accessibility:** every new user-visible surface includes accessibility acceptance, not only Spec 038.
2. **Arabic/RTL:** every text/layout/search/publish surface considers RTL and Unicode/tokenization early.
3. **Privacy:** new captured data types require retention/export/delete analysis.
4. **Evidence:** new AI/memory object types define source lineage before release.
5. **Offline:** core behavior cannot acquire an undeclared network dependency.
6. **Provenance:** donor/model/assets need exact records before merge; covered sources may rely on the founder permission only when exact permission scope is recorded and embedded third-party material is independently qualified.
7. **Recovery:** persistent mutations define recovery/rollback.
8. **Migration:** schema changes define compatibility/migration or explicitly document pre-release reset authority.
9. **Resource use:** model/capture changes measure memory/CPU/battery/thermal effects.
10. **Negative evidence:** known failures remain documented.

## Branch, CI, review, and merge discipline

For every selected implementation unit:

1. reverify canonical `main` and active authority;
2. shape one bounded unit against live evidence;
3. create bounded branch;
4. implement without unrelated cleanup;
5. run focused verification;
6. run risk-appropriate broader verification;
7. generate exact evidence artifacts;
8. open/update PR;
9. verify exact base/head/scope/checks/reviews/comments/threads/mergeability;
10. do not treat absent/skipped/neutral/billing-blocked review as approval;
11. merge only after exact qualified head remains unchanged, using expected-head protection where available;
12. verify canonical post-merge state/checks;
13. reconcile spec/tasks/evidence without inventing recursive meta-closeout work;
14. re-read successor authority.

No force-push or rebase of shared project history is authorized by this plan.

## Program continuation rule

Continue only through **genuinely selected and authorized** dependency-ordered work. The existence of this long plan or the founder source-use authorization is not blanket authority to implement all 040 units at once.

If fresh live evidence changes architecture, licensing/permission scope, platform APIs, or donor suitability, shape the smallest corrective successor and update canonical planning through a reviewed change. Repository truth wins over this document.
