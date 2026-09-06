# Himsat Donor Integration Master Plan

Date: 2026-09-06
Status: planning supplement; `specs/CURRENT.md` remains active authority

## Purpose

Translate the founder's broad source-code permission into a repeatable engineering process that maximizes reuse without turning Himsat into a mixed-architecture copy of its donors.

The canonical product direction does not change:

> Himsat is a Rust-first, local-first conversation intelligence and private memory platform for meetings, dictation, media, and documents across desktop/mobile/wearables.

The new permission changes how aggressively Himsat may reuse exact source paths when doing so wins on quality, reliability, speed, and maintenance.

## Core rule

**Copy evidence and hard-won failure knowledge, not just code.**

Every donor-consuming SpecGrain leaf executes this sequence:

```text
1. REVERIFY
   exact upstream repo/head/path/rightsholder/public license/special permission

2. DECOMPOSE
   project-owned code vs third-party/vendor/generated/model/asset material

3. CONTRACT FIRST
   define the Himsat-owned interface/events/errors/health/lifecycle

4. ARBITRATE
   compare all serious donor/native candidates with one scorecard

5. SELECT
   choose copy/depend/vendor/reference and smallest useful source surface

6. TRANSPLANT EVIDENCE
   preserve useful tests, regression cases, capability probes, error semantics

7. ADAPT
   make Himsat architecture authoritative; remove telemetry/cloud/global-state assumptions

8. PROVE
   focused + adversarial + platform + performance + privacy tests

9. RECORD
   provenance/SBOM/notices/source mapping/upstream divergence

10. TRACK
    assign upstream security/fix watch ownership
```

## Donor scorecard

A subsystem candidate receives measured evidence, not subjective preference.

| Dimension | Questions |
| --- | --- |
| Correctness | Does it handle known platform/API edge cases and malformed input? |
| Reliability | Crash recovery, lifecycle recovery, device changes, restart behavior? |
| Latency | p50/p95 live latency on reference hardware? |
| Resource cost | CPU, RAM, VRAM, disk, battery, thermal load? |
| Platform fidelity | Uses supported native APIs correctly and exposes real capability limits? |
| Privacy | Any telemetry, hidden egress, cloud fallback, sensitive logs? |
| Security | Native unsafe surface, hostile-input exposure, privilege, sandboxability? |
| Maintainability | Size, complexity, test quality, active upstream maintenance? |
| Packaging | System libs, installers, model downloads, binary size? |
| Accessibility | Does the behavior degrade safely for screen readers/keyboard/RTL/live captions? |
| Himsat fit | Can it sit behind a Himsat-owned contract without dragging donor architecture? |
| Rights/provenance | Exact source ownership, public license, special permission, foreign-content boundary? |

A lower feature-count implementation may win if it is more reliable, maintainable, secure, and portable.

## Updated dependency-ordered product plan

Existing Specifications 000–040 remain the base plan. The following refinements should be reconciled into the canonical master plan when the active security unit permits plan reconciliation.

### 006 — Capture abstraction + Capture Health

Add explicit contracts for:

- self-process/system-audio exclusion;
- source ownership and external-vs-Himsat mic activity;
- capture capability probe;
- silence vs broken-capture distinction;
- PCM continuity/discontinuity and wall-clock drift;
- backpressure and dropped-frame accounting;
- audio-route/source generation identity;
- health warning/event protocol suitable for native helper sidecars.

Donor arbitration: OpenWhispr, Meetily, Anarlog, direct native/Rust implementation.

### 007/008/009 — platform capture

#### macOS

Compare OpenWhispr CoreAudio process-tap helper with Meetily/Anarlog/native implementation. Qualify CoreAudio/ScreenCaptureKit differences, self-capture exclusion, permissions, aggregate-device cleanup, sleep/wake, route changes, Intel/Apple Silicon baselines.

#### Windows

Treat OpenWhispr WASAPI process-loopback exclusion helper as a leading donor candidate. Require capability probe, default-device independence, self-process-tree exclusion, silence-vs-failed-capture detection, device changes, sleep/wake, Bluetooth/USB, and helper parent-death cleanup.

#### Linux

Compare OpenWhispr PipeWire native loopback with Meetily/Anarlog/direct PipeWire. Require Wayland/distro capability matrix, dropped-buffer accounting, portal/fallback behavior, and no silent downgrade that violates capture promises.

### 010 — Audio processing

Add explicit **Mic/System De-duplication** alongside AEC/NS/AGC:

- render-bleed detector;
- cross-correlation/lag evidence;
- double-talk preservation;
- risky-segment holdback/retract;
- transcript-level duplicate suppression as a second line, not a replacement for signal processing.

Donors: OpenWhispr regression patterns, Sonora, WebRTC AudioProcessing.

### 011 — Model Registry + Resource Governor

Extend the registry with a VoiceStudio-inspired engine job model.

Each engine/model must own a named job and prove why it exists:

```text
best offline quality
best live latency
best Arabic/code-switch
lowest memory/CPU
best Apple path
best Android path
best diarization
compatibility fallback
crash-isolated fallback
```

Admission requires exact code+weight permission, benchmark, platform/degradation policy, CI smoke test, steward/owner, and deprecation rule.

Add Resource Governor inputs:

- free/total RAM;
- VRAM/unified memory;
- CPU/GPU/NPU capability;
- battery/charging;
- thermal state;
- free disk;
- currently resident models/workers.

No hidden resident model memory.

### 012 — Live transcription

Adopt one versioned Himsat speech-session protocol across desktop, Bridge, CLI, future WebSocket/API/MCP. Donor architecture must not create separate model-loading stacks for each surface.

### 014 — Diarization + Speaker Vault

Add donor-derived regression cases:

- more remote speakers than expected;
- overlapping speakers;
- user-locked speaker names followed by refined clustering;
- speaker-count changes without collapsing remote voices;
- voice fingerprint worker crash/restart;
- explicit separation of cluster, identity, and biometric profile.

### 015A — Desktop Dictation & Safe Output Control (new bounded candidate)

Insert after stable desktop capture + live STT and before desktop runtime is considered complete.

Outcome: use the same Himsat local speech stack as system-wide dictation.

Required:

- global hotkey / push-to-talk / toggle;
- exact destination capture at session start;
- session-bound destination through final output;
- stale result cannot paste into a newer app/window;
- clipboard lease/snapshot/restore that does not overwrite user clipboard changes;
- Wayland/unsupported-platform copy-only fail-safe when destination identity cannot be guaranteed;
- optional translation/cleanup transform after authoritative raw transcript;
- explicit cancel and user-visible recording state.

Donors: OpenWhispr + VoiceStudio special-permission source; native platform implementation.

### 015 — Desktop background runtime + meeting detection

Expand outcome:

- launch at login / start hidden;
- tray/menu-bar lifecycle;
- user-started session survives window close;
- process detection as context, not proof;
- event-driven mic activity where possible;
- false-positive cooldown;
- meeting auto-end with bounded evidence;
- restart recovery window after automatic end;
- session ownership so stale detector events cannot control a new recording.

### 018 — Himsat Bridge

Keep live encrypted P2P session control/audio transport. Add a future-compatible capability for **user-owned compute offload**:

```text
phone capture -> paired desktop GPU worker -> encrypted results -> phone
```

Do not make remote compute mandatory; Network Lock and local-device ownership remain explicit.

### 019 — Media import

Add optional post-core transforms, not 1.0 blockers:

- local translation;
- local read-aloud/TTS;
- later video dubbing using VoiceStudio knowledge where beneficial.

### 020–023 — Document ingestion

Require parser arbitration benchmark across Xberg/tokimo/Docling/MarkItDown/MinerU/Marker/native implementations with hostile fixtures and isolated workers for risky parsers.

Evaluation dimensions:

- digital PDF fidelity;
- scanned PDF/OCR handoff;
- tables;
- equations;
- reading order;
- images/captions;
- PPTX/DOCX/XLSX;
- corrupt/archive bomb/path traversal;
- deterministic IR;
- CPU/RAM/time;
- cross-platform packaging.

### 031 — Local API/CLI/MCP

Reuse the one speech/context control plane. Do not build independent agent-only model infrastructure.

Security additions inspired by donor architectures:

- loopback-only by default;
- explicit Origin validation for browser-capable local endpoints;
- short-lived capability/session tokens where appropriate;
- remote Bridge APIs require authenticated encrypted pairing;
- raw mic/recording start remains privileged/approval-gated.

### Cross-cutting Diagnostics & Self-Check

Make diagnostics a first-class platform capability, delivered incrementally from 006 onward.

A scrubbed support bundle may include:

- OS/app/build/version;
- permissions/capability probes;
- capture sources/routes;
- model/runtime/digest/residency;
- RAM/VRAM/disk/thermal state;
- worker crash/restart counters;
- provenance/SBOM versions;
- Network Lock status and outbound connection count;
- audio health/errors;
- redacted logs.

Default diagnostic collection must exclude transcripts/audio/documents/speaker profiles unless the user explicitly includes them.

## Donor-copy modes by risk

### Mode A — direct small-source transplant

Use for small, well-tested platform helpers/algorithms where source ownership is clear and Himsat wants control.

Examples: native capture helper, capability probe, pure policy algorithm.

Requirements: exact source map, source tests/regressions, Himsat wrapper, notices, divergence record.

### Mode B — dependency

Use for mature libraries whose maintenance/security burden is better upstream.

Examples: crypto primitives, inference engines, embedded libraries.

Requirements: pinned version/source/checksum, dependency closure, SBOM, abstraction boundary.

### Mode C — isolated worker

Use when useful code is Python/C++/native, crash-prone, large, model-heavy, or parses hostile input.

Examples: experimental STT/vision/document engines.

Requirements: versioned IPC, resource/time limits, crash restart policy, untrusted-input boundary.

### Mode D — behavioral reimplementation

Use when copying creates more debt than value, even if permission exists.

Examples: donor UI/global app architecture that conflicts with Himsat Rust-first core.

## Upstream tracking ledger

Every copied donor path should create a durable mapping record:

```text
donor_repo
donor_revision
donor_paths
Himsat_destination
Himsat_first_commit
last_upstream_review
known_upstream_security_fixes
intentional_divergence
sync_owner
```

Recommended policy:

- no automatic donor merges;
- monthly/quarterly review depending on risk/activity;
- immediate review for security advisories or platform API breakage;
- forward-port only the relevant fix behind Himsat tests.

## Definition of "ready to copy"

A donor-consuming unit is ready only when:

1. the active SpecGrain leaf authorizes the capability and exact change surface;
2. source rights/provenance decomposition is complete;
3. special-permission machine support exists if public license is otherwise blocked;
4. alternatives have been arbitrated;
5. Himsat contract is written first;
6. exact source paths/revision are pinned;
7. third-party material is excluded or separately authorized;
8. donor tests/regressions are inventoried;
9. copy/depend/worker/reimplement mode is selected;
10. security/runtime isolation is selected;
11. benchmark/negative/platform qualification is defined;
12. upstream tracking owner is defined.

If any item is missing, the correct state is `DONOR_NOT_READY`, regardless of permission.

## Definition of "best app"

Himsat should not measure success by donor count or feature count. The engineering north-star is:

```text
privacy + reliability + evidence correctness + capture quality + local intelligence quality + UX + portability + ownership
```

Each donor must improve at least one of those dimensions without creating unacceptable regression in the others.
