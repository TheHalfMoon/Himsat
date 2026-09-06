# Himsat Donor Integration Master Plan

Date: 2026-09-06
Reconciled: 2026-09-07
Status: canonical planning supplement; `specs/CURRENT.md` remains active implementation authority

## Goal

Use the founder's broad source-use permission aggressively without turning Himsat into a bundle of donor architectures.

Himsat remains Rust-first, local-first, evidence-first, and Himsat-contract-first. Donor code wins only when it improves measurable product quality, reliability, security, speed, portability, accessibility, or maintainability.

The founder authorization is recorded in `governance/provenance/source-use-authorization.md`. The canonical roadmap integration is in `docs/execution-master-plan.md`.

## Mandatory donor-consuming sequence

```text
REVERIFY exact repo/revision/path/rights
  -> DECOMPOSE project-owned vs foreign material
  -> DEFINE Himsat-owned contract first
  -> ARBITRATE donor/native alternatives
  -> SELECT smallest copy/adapt/depend/vendor/worker/reimplement surface
  -> TRANSPLANT useful tests/regressions/capability probes
  -> ADAPT to Himsat architecture
  -> PROVE platform/security/privacy/performance behavior
  -> RECORD provenance/SBOM/notices/divergence
  -> TRACK upstream fixes/security
```

Permission changes the rights gate; it does not waive any other step.

## Donor scorecard

Compare competing implementations on:

- correctness and known regression coverage;
- crash/lifecycle recovery;
- latency and CPU/RAM/VRAM/battery/thermal cost;
- native API fidelity and capability detection;
- privacy/telemetry/network behavior;
- security/unsafe/native/hostile-input surface;
- platform packaging and binary size;
- test quality and upstream maintenance;
- accessibility/degradation behavior;
- Arabic/RTL/code-switch quality when relevant;
- Himsat architectural fit;
- exact rights/provenance/foreign-content boundary.

Do not keep multiple donor stacks for the same job without measured reason.

## Source-derived refinements

These refinements are inputs to the existing 000–040 dependency graph. They do not create current implementation authority while Specification 004 is active.

### 006 — Capture abstraction + Capture Health

Add explicit contracts for:

- self-process/system-audio exclusion;
- external-vs-Himsat capture ownership;
- capability probes;
- silence vs failed capture;
- PCM continuity and wall-clock drift;
- dropped-frame/backpressure/storage-pressure accounting;
- route/source generations;
- structured health/lifecycle events.

Arbitrate OpenWhispr, Meetily, Anarlog, OpenSuperWhisper where relevant, and direct native/Rust implementations.

### 007/008/009 — Platform capture

- macOS: compare OpenWhispr CoreAudio process-tap patterns, OpenSuperWhisper recorder/control patterns, Meetily, Anarlog, and native APIs; qualify permissions, aggregate cleanup, self-capture exclusion, sleep/wake, route changes, Intel/Apple-Silicon behavior.
- Windows: treat OpenWhispr WASAPI process-loopback exclusion as a leading candidate; qualify parent death, silence-vs-broken detection, output-device independence, device changes, Bluetooth/USB.
- Linux: compare OpenWhispr PipeWire patterns with Meetily/Anarlog/direct PipeWire; qualify Wayland/distro fallbacks and dropped-buffer accounting.

### 010 — Audio processing and duplicate-speech control

Add mic/system de-duplication to AEC/NS/AGC:

- render-bleed detection;
- lag/correlation evidence;
- double-talk preservation;
- risky-segment holdback/retract;
- transcript duplicate suppression only as a second line.

Donors include OpenWhispr regression knowledge, Sonora, WebRTC AudioProcessing, and any stronger covered source at shaping time.

### 011 — Model Registry + Voice Model Router + Resource Governor

Every engine/model owns a named job such as:

```text
best offline quality
best live latency
best Arabic/code-switch
lowest resource use
best Apple/Android path
best diarization
compatibility fallback
crash-isolated fallback
```

Admission requires exact code + weight rights, immutable identity/digest, benchmark, platform/degradation policy, CI smoke test, steward, and deprecation trigger.

The Voice Model Router exposes locality, language, streaming, diarization, resource, provenance, and Network Lock compatibility.

The Resource Governor tracks RAM/VRAM or unified memory, accelerator capability, battery/charging, thermal state, free disk, and resident models/workers. Hidden model residency is a product bug.

Use OpenSuperWhisper and VoiceStudio as orchestration/UX references or selective donors when exact source/provenance permits; use whisper.cpp, sherpa-onnx, Argmax, Moonshine, Vibe/Sona, and other covered engines according to measured job fit.

### 012 — One speech-session data plane

Use one versioned Himsat speech-session protocol across desktop UI, dictation, Bridge, CLI, local API/WebSocket, and future MCP clients. Do not duplicate model loading or permission behavior per surface.

Meeting transcription and dictation may share the speech core while keeping user-facing retention, consent, target-output, and evidence semantics distinct.

### 014 — Diarization + Speaker Vault

Add regressions for:

- overlapping speech;
- unexpected speaker counts;
- user-locked names followed by refined clustering;
- worker crash/restart;
- strict separation of diarization cluster, named identity, and biometric profile.

### Desktop Dictation & Safe Output Control

This is a bounded overlay under 011 + 012 + 015, not an independent current Grain.

Required behavior:

- global hotkey / hold / toggle and supported mouse controls;
- capture exact destination identity at session start where the platform permits it;
- carry session/destination identity through partial/final output;
- stale result cannot paste into a newer target;
- clipboard lease/snapshot/restore must not overwrite clipboard content created by the user during transcription;
- copy-only fallback when destination identity cannot be proven, including constrained Wayland paths;
- optional translation/cleanup after an authoritative raw transcript;
- explicit cancellation and visible recording state;
- modes, vocabulary hints, deterministic replacements, and context access remain capability-scoped.

Primary source inputs: OpenWhispr, OpenSuperWhisper, VoiceStudio, Superwhisper product behavior, native platform APIs, and any stronger covered source at shaping time.

### 015 — Desktop runtime + Input Control Layer

Add:

- launch-at-login/start-hidden where supported;
- tray/menu lifecycle;
- recording survival after window close;
- process detection as context rather than proof;
- event-driven mic activity where possible;
- false-positive cooldown;
- bounded meeting auto-end;
- restart recovery window;
- stale-session rejection;
- global shortcut/push-to-talk/mouse/deep-link invocation behind one input-control contract.

### 018 — Himsat Bridge and user-owned compute

Keep encrypted P2P capture/control and leave room for optional user-owned compute offload:

```text
phone capture -> paired user desktop GPU -> encrypted results -> phone
```

Remote compute is optional, authenticated, capability-scoped, user-visible, and never a mandatory Himsat cloud dependency.

### 019 — Media import and speech transforms

Use a recoverable file-transcription queue. Local translation/read-aloud/TTS/video dubbing may be post-core transforms informed by VoiceStudio/OpenSuperWhisper and other covered sources. They are not 1.0 blockers unless later evidence changes prioritization.

### 020–023 — Documents

Arbitrate Xberg, tokimo, Docling, MarkItDown, MinerU, Marker-family sources, and native parsers using hostile fixtures, deterministic IR, tables/equations/reading-order fidelity, OCR handoff, Office coverage, resource cost, and cross-platform packaging. Risky parsers run isolated.

### 031 — Local API/CLI/MCP + Context Capability Broker

Reuse the same local speech/context control plane. Default local endpoints are loopback-only; browser-capable endpoints validate Origin; remote Bridge APIs require authenticated encrypted pairing; mic/recording start remains privileged.

The later Context Capability Broker must expose explicit grants for selected text, focused field, clipboard, app/window identity, screen region, meeting context, and project memory. No mode receives hidden ambient context.

### Cross-cutting diagnostics

From 006 onward, build scrubbed self-check/support evidence for:

- permission/capability probes;
- capture routes and health;
- model/runtime digests and residency;
- RAM/VRAM/disk/thermal state;
- worker crashes/restarts;
- provenance/SBOM version;
- Network Lock and outbound connection state;
- scrubbed errors.

Exclude transcripts, audio, documents, speaker profiles, and equivalent user content unless the user explicitly includes them.

## Reuse modes by risk

### A — Small source transplant

Use for focused native helpers or pure algorithms. Preserve exact source map, useful donor tests/regressions, Himsat wrapper, notices, and divergence record.

### B — Dependency

Use for mature maintained libraries. Pin version/source/checksum; keep complete dependency/native closure and a replaceable Himsat abstraction.

### C — Isolated worker

Use for Python/C++/model-heavy/crash-prone/hostile-parser code. Require versioned IPC, resource limits, crash recovery, and an untrusted-input boundary.

### D — Vendored snapshot

Use only when deterministic/offline build or patch ownership justifies the maintenance burden. Record source digest, upstream revision, notices, and update policy.

### E — Behavioral reimplementation

Use when donor UI/global architecture creates more debt than copied code saves, even when permission exists.

## Upstream tracking

Every copied or closely adapted path records:

```text
donor_repo + donor_revision + donor_paths
Himsat_destination + first_Himsat_commit
last_reviewed_upstream_revision
known_security_or_platform_fixes
intentional_divergence
sync_policy
sync_owner
```

Never auto-merge upstream. Forward-port reviewed fixes behind Himsat tests.

## Ready-to-copy gate

A donor-consuming leaf is ready only when:

1. active SpecGrain authority exists;
2. rights/provenance decomposition is complete;
3. separate-permission machine support exists if that donor path needs it;
4. alternatives are arbitrated;
5. Himsat contract exists first;
6. exact source path/revision is pinned;
7. foreign material is excluded or independently authorized;
8. donor tests/regressions are inventoried;
9. copy/adapt/depend/vendor/worker/reimplement mode is selected;
10. isolation/security decision is explicit;
11. qualification/benchmark plan exists;
12. upstream tracking owner exists.

Otherwise: `DONOR_NOT_READY`.

## Optimization target

Do not optimize donor count or feature count. Optimize:

```text
privacy
+ reliability
+ evidence correctness
+ capture quality
+ local intelligence quality
+ UX
+ accessibility
+ Arabic/RTL quality
+ portability
+ user ownership
+ maintainability
```
