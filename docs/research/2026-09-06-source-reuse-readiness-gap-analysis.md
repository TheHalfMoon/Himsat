# Himsat Source-Reuse Readiness and Gap Analysis

Date: 2026-09-06
Status: planning/governance research; no donor adoption authority

## Executive conclusion

The founder's special source-code permissions materially expand the set of code Himsat may consider, but **the best Himsat is still not built by copying everything**. The correct strategy is to turn donor code into a controlled engineering supply chain:

```text
permission + provenance
  -> exact subsystem inventory
  -> Himsat-owned contract
  -> cross-donor arbitration
  -> smallest transplant/dependency
  -> source test/evidence transplant
  -> Himsat adaptation
  -> adversarial/platform benchmarks
  -> upstream-drift ownership
```

Permission removes one major legal/rights blocker. It does not remove third-party ownership, security, quality, architecture, privacy, platform, or maintenance risk.

## Rights-model change

Before the founder attestation, publicly restrictive projects defaulted to `REFERENCE_ONLY`. After the attestation, Himsat may evaluate project-owned source code from those projects under `founder_attested_special_permission` while preserving the truth about the public license.

Examples of changed planning posture:

- VoiceStudio: public AGPL-3.0 remains a fact, but project-owned source becomes a `COPY_SELECTIVE_SPECIAL_PERMISSION` candidate rather than reference-only if the founder's grant covers the exact path.
- current Screenpipe: its public competing-product restriction remains a fact, but project-owned source can become a special-permission candidate where the grant covers the exact path.
- custom/non-commercial source projects such as document parsers can be considered path-by-path under special permission instead of rejected solely because of the public license.
- mixed-license repositories such as Anarlog may allow specifically granted project-owned commercial/enterprise paths, but foreign/vendor/generated material remains separately controlled.

No behavioral-reference-only proprietary product becomes a code donor merely because it was discussed. Source must actually exist and be within the founder-attested permission scope.

## New cross-cutting gates

### Gate R1 — rights decomposition

For every donor path, record:

- public license;
- special-permission basis;
- exact rightsholder/grantor when known;
- whether the path contains third-party/imported/generated code;
- what is explicitly excluded.

Do not flatten these facts into one fake "Himsat-compatible license" label.

### Gate R2 — donor arbitration before copy

When two or more donors solve the same problem, Himsat must compare them before adoption. The comparison uses measurable criteria:

```text
correctness
platform API fidelity
reliability/failure recovery
latency
CPU/RAM/VRAM
battery/thermal cost
binary size
security blast radius
privacy/network behavior
accessibility
code complexity
native dependency burden
test quality
maintenance activity
upstream security-fix cost
Himsat architectural fit
```

The goal is one primary implementation behind a Himsat contract, not parallel donor stacks that permanently duplicate behavior.

### Gate R3 — test/evidence transplant

Copying source without the donor's failure knowledge is incomplete. For every reused subsystem, inventory and selectively preserve:

- focused tests;
- regression tests for known bugs;
- fixtures that can be legally redistributed;
- platform capability probes;
- failure codes and recovery semantics;
- benchmark methodology.

Where donor fixtures contain third-party/private content, recreate synthetic fixtures instead of copying them.

### Gate R4 — upstream-drift ownership

Every copied/closely adapted donor path gets an upstream tracking record:

```text
source_revision
last_reviewed_upstream_revision
security/advisory watch owner
sync policy = none | manual-review | selective-forward-port
known divergence
```

Never blindly merge upstream. Security fixes are reviewed against Himsat's adapted behavior and tests.

### Gate R5 — architecture decontamination

A donor implementation may enter Himsat only behind a Himsat-owned interface/event/error model. Donor UI, global state, telemetry, cloud assumptions, package-manager conventions, database schema, or app lifecycle must not silently become Himsat architecture.

### Gate R6 — runtime isolation decision

Native/model/parser code is classified by failure blast radius:

- safe in-process library;
- isolated worker/sidecar;
- sandboxed plugin;
- import-only conversion worker.

Model runtimes, document parsers, codecs, and experimental native libraries should default toward isolation when crashes or hostile input could take down a recording session.

## Donor arbitration map

### Desktop capture and meeting runtime

Primary candidates to compare:

- OpenWhispr;
- Meetily;
- Anarlog;
- platform-native APIs directly.

#### macOS

OpenWhispr provides a concrete CoreAudio process-tap/aggregate-device helper. Meetily and Anarlog provide independent local-capture patterns. Himsat should benchmark API correctness, self-audio exclusion, route changes, sleep/wake, Intel/Apple-Silicon behavior, permissions, long-session drift, and failure recovery before choosing the transplant surface.

#### Windows

OpenWhispr's WASAPI process-loopback helper is a strong candidate because it captures process loopback while excluding the application's process tree and emits explicit capability/lifecycle events. Compare with Meetily and native Rust/Windows API implementation before adoption.

#### Linux

OpenWhispr's PipeWire helper plus capability probe is a strong reference/copy candidate. Compare with Meetily/Anarlog and direct PipeWire Rust bindings; qualify Wayland/PulseAudio fallbacks and distro packaging burden.

### Capture Health and meeting detection

OpenWhispr adds useful failure knowledge that should be represented in Himsat contracts:

- event-driven microphone activity where possible;
- process context is not sufficient proof of a meeting;
- sustained audio evidence and false-positive cooldown;
- explicit external-vs-self microphone ownership where possible;
- meeting auto-end only after bounded evidence;
- restart/recovery window after automatic end;
- detection state must not leak across replaced/restarted sessions.

### Echo/bleed and duplicate speech

OpenWhispr exposes a gap in the original plan: simultaneous mic + system-audio capture can double-count the remote side through speaker bleed.

Himsat should add explicit tests/contracts for:

- system-audio self-exclusion;
- mic/system cross-correlation;
- suspected render bleed;
- double-talk;
- holdback/retract of risky transcript finals;
- duplicate text suppression without deleting genuine overlapping speech.

This belongs across Specifications 006, 010, 012, and 015 rather than being left to diarization alone.

### Dictation and safe desktop output — newly elevated feature

The earlier plan focused on meetings, memory, and documents but under-specified system-wide dictation. VoiceStudio/OpenWhispr show that this can be a major Himsat surface.

Himsat should add a bounded **Desktop Dictation & Output Control** unit after stable desktop capture/live transcription and before final desktop-runtime closure.

Required behavior:

- global hotkey/hold/toggle;
- capture the exact focused destination at session start;
- carry destination/session identity through partial/final events;
- never paste a stale result into a newer target;
- safe clipboard snapshot/lease/restore;
- never overwrite clipboard content created by the user during transcription;
- copy-only safe fallback when the OS/compositor cannot prove the original destination;
- translation dictation as an optional transformation, not a separate capture stack;
- user-visible recording state and cancellation.

VoiceStudio source may be used only under the founder special-permission path; OpenWhispr MIT source is independently permissive.

### Speech control plane and data plane

VoiceStudio demonstrates a useful architecture to implement independently or selectively transplant where permitted:

```text
native capture/output control
        +
versioned ASR data plane
```

Himsat should preserve one local speech session protocol across:

- desktop UI;
- CLI;
- local HTTP/JSON-RPC control;
- WebSocket streaming audio/transcript;
- MCP/agent clients;
- Himsat Bridge remote-edge capture.

The protocol must not force every client to reload models or reimplement platform permissions.

### Model admission and resource governance

VoiceStudio's engine "job map" reveals a maintenance gap in Himsat's Model Registry plan. Himsat should require every speech/LLM/embedding/vision/TTS engine to own a named job such as:

```text
best offline accuracy
lowest live latency
lowest memory/CPU
best Apple-Silicon path
best Arabic/code-switch path
best diarization path
compatibility fallback
crash-isolated fallback
```

Admission requires:

- named job and evidence it beats/fills the incumbent;
- exact source + model-weight permission/license;
- platform support or explicit opt-in/degradation;
- adapter compatibility;
- CI contract smoke test;
- benchmark fixture;
- maintenance owner/steward;
- deprecation trigger when unmaintained or failing.

Add a **Resource Governor** to Model Registry/runtime planning: RAM/VRAM/battery/thermal/free-disk measurements determine which engines may remain warm. A resident model must be visible in diagnostics; hidden memory residency is a product bug.

### Speech-to-text engine stack

Recommended architecture remains Himsat-owned `SpeechEngine`/worker contracts rather than donor-specific APIs.

Candidates:

- whisper.cpp: broad local compatibility;
- sherpa-onnx: mobile/streaming/VAD/diarization ecosystem;
- Vibe/Sona: model/job lifecycle and isolated local worker patterns;
- Meetily/Anarlog/OpenWhispr: integration/failure/test patterns;
- Argmax/Moonshine: device-specific/low-latency candidates;
- VoiceStudio: special-permission source for engine orchestration ideas and broad adapter knowledge.

### Audio DSP

Prefer one swappable contract and benchmark:

- Sonora/Rust-native path;
- WebRTC AudioProcessing wrapper;
- donor-specific AEC/NS/AGC patterns.

Do not stack multiple suppressors by default; double processing can damage speech and diarization.

### Diarization and speaker identity

Use separate concepts:

```text
diarization cluster != named speaker identity != biometric voice profile
```

Candidates:

- sherpa-onnx production path;
- pyannote benchmark/reference oracle;
- OpenWhispr speaker-fingerprint and multi-speaker regression patterns;
- Anarlog/Meetily community code where useful.

Speaker profile storage remains opt-in encrypted biometric data with deletion/export controls.

### Local intelligence

Primary candidate direction:

- `mistral.rs` for Rust-native model runtime;
- `llama.cpp` compatibility/fallback;
- isolated workers for model families whose native runtime can crash the app;
- FastEmbed for embeddings behind replaceable contract.

No cloud fallback may silently occur when Network Lock/local-only mode is active.

### Document ingestion

Cross-donor arbitration should compare:

- Xberg current Rust-first parser;
- tokimo fileparser;
- MarkItDown behavior/normalization;
- Docling document IR/layout quality;
- MinerU/Marker special-permission code where useful;
- native PDF/OOXML parsers.

Choose by hostile-input safety, table/layout preservation, OCR handoff, deterministic IR, resource use, and supported formats rather than feature-count marketing.

Document parsers should run in an isolation boundary appropriate to hostile files.

### Search and memory

Initial strategy remains:

- SQLCipher structured store;
- SQLite FTS5 lexical search;
- sqlite-vec semantic search;
- FastEmbed local embeddings;
- Tantivy only when measured corpus scale requires it.

Do not introduce a second canonical database solely because a donor uses it.

### P2P, plugins, and publishing

- Iroh: P2P transport candidate behind Network Lock/policy boundaries;
- Yrs/y-crdt: editorial concurrent state only where CRDT semantics are justified;
- Extism first candidate for plugin sandbox; Wasmtime lower-level fallback;
- Typst for professional PDF/PDF-A/PDF-UA;
- docx-rs for semantic DOCX where coverage qualifies.

## New feature candidates discovered by donor analysis

### Desktop Dictation & Output Control

Elevate to pre-1.0 candidate, not a post-release toy. It makes Himsat useful outside meetings and reuses the same capture/STT core.

### Local Read Aloud / TTS

VoiceStudio's TTS breadth creates an optional future surface:

- read summaries/PDFs aloud locally;
- accessibility speech output;
- local conversational assistant voice;
- translated video/audio dubbing as an optional media transform.

Do **not** put TTS on the critical path to Himsat 1.0. Add it only after capture/memory/document quality is strong and model licensing/resource governance exists.

### Local compute worker

Extend Himsat Bridge eventually so a user-owned desktop GPU can process audio/documents captured on a phone without a Himsat cloud. This must be authenticated, encrypted, capability-scoped, and user-visible.

### Diagnostics and self-check

The donor projects show that local AI/audio apps fail in hardware-specific ways. Himsat should have a first-class diagnostics bundle that can report, without user content by default:

- capture capabilities;
- active device/routes;
- model versions/digests;
- resident RAM/VRAM;
- storage headroom;
- GPU/provider availability;
- Network Lock state;
- worker crashes/restarts;
- permission status;
- audio pipeline health;
- scrubbed error journal.

## Architecture rule after special permission

Even with permission to copy all project-owned source, Himsat remains **Rust-first, not donor-first**.

Preferred language ownership:

- Rust: shared core, event model, vault, storage, memory/search, policy, orchestration, worker supervision, networking, CLI;
- Swift/Objective-C/C as needed: Apple audio/background/watch/native APIs;
- Kotlin/Java as needed: Android audio/services/OS APIs;
- C/C++ only for small native platform/model helpers where it is the best proven implementation;
- TypeScript/React: desktop UI where selected;
- Python: benchmark/reference/isolated worker only when the model/document ecosystem justifies it, never as an accidental mandatory core dependency.

## Readiness checklist before first donor transplant

The plan is donor-ready when all of these are true:

- founder source-reuse authority is recorded;
- provenance machinery can represent special-permission basis without falsifying public license;
- active SpecGrain unit explicitly authorizes the exact donor-consuming leaf;
- donor arbitration table is complete for that subsystem;
- exact source revision/path/foreign-content boundary is known;
- Himsat behavior contract exists before copy;
- tests/fixtures and known regressions are inventoried;
- copy/depend/vendor decision is justified;
- security/runtime isolation decision is explicit;
- upstream tracking owner/policy is defined;
- exact Diffcipline/CI/benchmark plan exists.

Until then, permission is planning authority, not byte-adoption authority.
