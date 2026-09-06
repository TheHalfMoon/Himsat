# Himsat Donor Integration Master Plan

Date: 2026-09-06
Status: planning supplement; `specs/CURRENT.md` remains active authority

## Goal

Use the founder's broad source-code permission aggressively **without** turning Himsat into a bundle of donor architectures.

Himsat remains Rust-first and local-first. Donor code wins only when it improves measurable product quality, reliability, security, speed, portability, or maintainability.

## Mandatory donor-consuming sequence

```text
REVERIFY exact repo/revision/path/rights
  -> DECOMPOSE project-owned vs foreign material
  -> DEFINE Himsat-owned contract first
  -> ARBITRATE donor/native alternatives
  -> SELECT smallest copy/depend/worker/reimplement surface
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
- Himsat architectural fit;
- exact rights/provenance/foreign-content boundary.

Do not keep multiple donor stacks for the same job without measured reason.

## Canonical-plan refinements to reconcile later

Existing Specifications 000–040 remain authoritative. Reconcile these refinements after the active Spec004 security unit permits broader plan updates.

### 006 — Capture abstraction + Capture Health

Add contracts for:

- self-process/system-audio exclusion;
- external-vs-Himsat capture ownership;
- capability probes;
- silence vs failed capture;
- PCM continuity/drift;
- dropped-frame/backpressure/storage-pressure accounting;
- route/source generations;
- structured health/lifecycle events.

Arbitrate OpenWhispr vs Meetily vs Anarlog vs direct native/Rust.

### 007/008/009 — Platform capture

- macOS: compare OpenWhispr CoreAudio process tap with Meetily/Anarlog/native APIs; qualify permissions, aggregate cleanup, self-capture exclusion, sleep/wake, route changes, Intel/Apple-Silicon behavior.
- Windows: treat OpenWhispr WASAPI process-loopback exclusion helper as a leading candidate; qualify parent death, silent-vs-broken detection, output-device independence, device changes, Bluetooth/USB.
- Linux: compare OpenWhispr PipeWire helper with Meetily/Anarlog/direct PipeWire; qualify Wayland/distro fallbacks and dropped-buffer accounting.

### 010 — Audio processing

Add mic/system de-duplication to AEC/NS/AGC:

- render-bleed detection;
- lag/correlation evidence;
- double-talk preservation;
- risky-segment holdback/retract;
- transcript duplicate suppression only as a second line.

Donors: OpenWhispr regression knowledge, Sonora, WebRTC AudioProcessing.

### 011 — Model Registry + Resource Governor

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

Admission requires exact code+weight rights, benchmark, platform/degradation policy, CI smoke test, steward, and deprecation trigger.

Resource Governor tracks RAM/VRAM, accelerator, battery/charging, thermal state, disk, and resident models/workers. Hidden model residency is a bug.

### 012 — Live transcription

Use one versioned Himsat speech-session protocol across desktop, Bridge, CLI, future WebSocket/API/MCP. Do not duplicate model loading per interface.

### 014 — Diarization + Speaker Vault

Add regressions for overlapping speech, unexpected speaker counts, user-locked names followed by refined clustering, worker crash/restart, and strict separation of diarization cluster vs identity vs biometric profile.

### New bounded candidate — Desktop Dictation & Safe Output Control

Place after stable desktop capture/live STT and before desktop runtime closure.

Required:

- global hotkey / hold / toggle;
- capture exact destination at session start;
- session-bound output identity;
- stale result cannot paste into a newer target;
- clipboard lease/snapshot/restore without overwriting new user clipboard data;
- copy-only fallback when destination cannot be proven (notably some Wayland paths);
- optional translation/cleanup after authoritative raw transcript;
- explicit cancellation and visible recording state.

Donors: OpenWhispr and VoiceStudio special-permission source plus native APIs.

### 015 — Desktop background runtime + meeting detection

Add launch-at-login/start-hidden, tray/menu lifecycle, recording survival after window close, process detection as context rather than proof, event-driven mic activity, cooldown, bounded auto-end, restart recovery window, and stale-session rejection.

### 018 — Himsat Bridge

Keep encrypted P2P capture/control and leave room for optional user-owned compute offload:

```text
phone capture -> paired user desktop GPU -> encrypted results -> phone
```

No mandatory remote compute or Himsat cloud.

### 019 — Media import

Local translation/read-aloud/TTS/video dubbing may be post-core transforms informed by VoiceStudio. They are not 1.0 blockers.

### 020–023 — Documents

Arbitrate Xberg/tokimo/Docling/MarkItDown/MinerU/Marker/native parsers using hostile fixtures, deterministic IR, tables/equations/reading-order fidelity, OCR handoff, Office coverage, resource cost, and cross-platform packaging. Risky parsers run isolated.

### 031 — Local API/CLI/MCP

Reuse the same local speech/context control plane. Default endpoints loopback-only; validate browser origins; remote Bridge access requires authenticated encrypted pairing; mic/recording start remains privileged.

### Cross-cutting diagnostics

From 006 onward, build scrubbed self-check/support evidence for permissions, capture routes/capabilities, model/runtime digests/residency, RAM/VRAM/disk/thermal state, worker crashes, provenance version, Network Lock, and audio health. Exclude transcripts/audio/documents/speaker profiles unless the user explicitly opts in.

## Copy modes

### A — Small source transplant

Use for focused native helpers/pure algorithms. Preserve source map, useful donor tests/regressions, Himsat wrapper, notices, and divergence record.

### B — Dependency

Use for mature maintained libraries. Pin version/source/checksum; keep SBOM/dependency closure and replaceable abstraction.

### C — Isolated worker

Use for Python/C++/model-heavy/crash-prone/hostile-parser code. Require versioned IPC, limits, crash recovery, and untrusted-input boundary.

### D — Behavioral reimplementation

Use when donor UI/global architecture creates more debt than copied code saves—even if permission exists.

## Upstream tracking

Every copied path records:

```text
donor_repo + donor_revision + donor_paths
Himsat_destination + first Himsat commit
last upstream review
known security/platform fixes
intentional divergence
sync owner
```

Never auto-merge upstream. Forward-port only reviewed fixes behind Himsat tests.

## Ready-to-copy gate

A donor-consuming leaf is ready only when:

1. active SpecGrain authority exists;
2. rights/provenance decomposition is complete;
3. special-permission machine support exists if needed;
4. alternatives are arbitrated;
5. Himsat contract exists first;
6. exact source path/revision is pinned;
7. foreign material is excluded/authorized;
8. donor tests/regressions are inventoried;
9. copy/depend/worker/reimplement mode is selected;
10. isolation/security decision is explicit;
11. qualification/benchmark plan exists;
12. upstream tracking owner exists.

Otherwise: `DONOR_NOT_READY`.

## "Best app" metric

Do not optimize donor count or feature count. Optimize:

```text
privacy + reliability + evidence correctness + capture quality + local intelligence quality + UX + portability + user ownership
```
