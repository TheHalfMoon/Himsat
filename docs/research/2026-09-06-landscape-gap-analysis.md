# Himsat Landscape and Gap Analysis

**Research date:** 2026-09-06  
**Purpose:** identify missing product, platform, architecture, reliability, security, interoperability, and donor-plan requirements before Himsat implementation begins.

This document is a planning research record. Product proposals are not implementation claims. External product behavior can change after the research date; live sources must be rechecked before comparative claims or donor adoption.

## Executive conclusion

The original concept — a better local alternative to Otter built from Meetily — is directionally strong but incomplete. The category has moved from “meeting transcription” to **continuous conversation context + live assistance + workflow action + cross-source memory**. The strongest Himsat opportunity is not feature-count parity; it is combining modern meeting intelligence with an architecture competitors generally do not optimize for:

> **user-controlled local capture + local multimodal evidence + temporal memory + professional document intelligence + local agents + portable context + accountless/private sync.**

The largest gaps found in the earlier plan were:

1. audio enhancement/AEC and capture-quality diagnostics were under-specified;
2. OS/version capability boundaries, especially mobile system audio and phone calls, needed stronger versioning/fail-closed behavior;
3. background recording was described as a feature rather than a reliability program covering interruptions, routes, power, storage, thermal state, crash recovery, and updates;
4. accessibility was not treated as a release-critical capability;
5. document intelligence needed a full ingestion/intermediate-representation architecture, not only “talk to PDF”;
6. plugin/agent extensibility needed a sandbox and capability broker;
7. model/package supply-chain integrity and air-gapped model installation were missing;
8. collaboration needed encrypted selective sharing and accountless group/device sync, not only backup connectors;
9. import/migration from incumbent meeting tools needed to be a first-class adoption feature;
10. output portability needed separate human-publishing and machine/LLM-context products;
11. continuous-memory ideas needed an explicit privacy-safe rolling-buffer design instead of accidental always-on surveillance;
12. compliance/consent UX needed to be separated from unverifiable legal-compliance marketing claims;
13. project-level memory needed temporal state, contradiction/change tracking, and source provenance across meetings **and documents**;
14. the donor plan needed path-level licensing and model-license gates because multiple attractive projects are mixed/custom licensed.

## Competitive signals

### Circleback

Sources:

- https://circleback.ai/releases/new-plans
- https://circleback.ai/releases/ask-circleback-to-do-things-in-your-apps
- https://circleback.ai/releases/notes-capture-whats-shared-on-screen
- https://support.circleback.ai/en/articles/10460573-getting-started-with-automations
- https://support.circleback.ai/en/articles/13249081-circleback-mcp
- https://support.circleback.ai/en/articles/10460578-record-meetings-with-the-desktop-app

Observed signals:

- unlimited meeting notes/action items are becoming commodity entry features;
- assistant search now spans meetings plus connected email/context;
- integrations are not only export targets: the assistant can search and take actions in external apps;
- automations use conditions plus chained actions;
- screen-shared information is captured into searchable meeting context;
- desktop capture can be botless and records microphone plus computer audio;
- MCP/API/CLI access makes meeting context a reusable platform surface;
- Circleback's MCP is centrally hosted, which creates a differentiation opportunity for a local capability-scoped Himsat MCP.

**Himsat implication:** transcript + summary is insufficient. Himsat needs cross-source search, multimodal context, local action workflows, and reusable local developer/agent surfaces.

### Otter

Sources:

- https://otter.ai/blog/never-miss-the-conversation-introducing-automatic-recording-in-otter-desktop
- https://otter.ai/blog/otter-ai-introduces-live-assist-the-first-live-coaching-agent-for-every-call
- https://help.otter.ai/hc/en-us/articles/35973988280215-Otter-Desktop-App-Mac-Windows

Observed signals:

- automatic meeting recording based on microphone/meeting activity is now a mainstream expectation;
- desktop capture, headphones support, meeting reminders, and recording-state ergonomics matter as much as transcription models;
- live assistance/coaching grounded in playbooks, SOPs, prior meetings, and resources is moving intelligence into the meeting itself.

**Himsat implication:** local live assistance and meeting detection belong in the core roadmap, but must remain opt-in and evidence-aware.

### Granola

Sources:

- https://www.granola.ai/updates
- https://www.granola.ai/integrations
- https://docs.granola.ai/help-center/ios/getting-started
- https://docs.granola.ai/help-center/ios/phone-calls
- https://docs.granola.ai/help-center/ios/taking-notes

Observed signals:

- mobile, Android, Apple Watch, briefs, team context, MCP, integrations, people/company views, and saved prompt “recipes” have expanded the product beyond meeting notes;
- human notes are intentionally used to guide AI enhancement;
- iPhone behavior exposes an important platform truth: ordinary iOS apps cannot simply capture arbitrary other-app virtual meeting audio; Granola uses the phone microphone for in-person capture and a built-in outbound-calling mechanism for phone-call notes;
- inbound call transcription remains constrained in Granola's documented iOS behavior.

**Himsat implication:** Note Gravity stays important; mobile UI must be native-quality; virtual-meeting/system-audio claims require OS-specific capability detection; phone-call support must not be generalized from desktop audio capture.

### Fireflies

Source:

- https://guide.fireflies.ai/articles/2679406774-live-assist-on-the-fireflies-desktop-app-real-time-notes-and-suggestions

Observed signals:

- live notes, live transcript, real-time Q&A, custom skills, meeting prep, instant summary, and sales suggestions are converging in a floating desktop surface.

**Himsat implication:** a local floating Live surface should expose transcript, bookmarks, open questions, decisions, contradictions, and user-selected playbook prompts without sending the meeting to a vendor cloud.

### Plaud / dedicated capture hardware

Source:

- https://support.plaud.ai/hc/en-us/articles/53771056805785-Does-Plaud-NotePin-S-work-offline

Observed signal:

- dedicated hardware can record/store offline, but AI processing commonly requires cloud connectivity.

**Himsat implication:** local intelligence is a genuine differentiator. Himsat should support imported hardware-recorder files and generic external microphones before considering proprietary hardware.

### Limitless/Rewind category

Source:

- https://www.limitless.ai/

Observed signal:

- continuous personal-memory products create both demand and long-term platform/provider risk; data export and graceful exit matter.

**Himsat implication:** continuous-memory capability must be portable, explicitly enabled, visibly active, locally encrypted, retention-controlled, and never dependent on a vendor service continuing to exist.

## Platform research and consequences

### Apple background audio and recording intents

Sources:

- https://developer.apple.com/documentation/avfaudio/avaudiosession/category-swift.struct/playandrecord
- https://developer.apple.com/documentation/appintents/audiorecordingintent
- https://developer.apple.com/documentation/appintents/widgets-live-activities-and-controls

Key facts:

- correct AVAudioSession/background configuration supports ongoing audio while the screen is locked/backgrounded;
- the user must grant recording permission;
- recording sessions remain subject to interruptions and route/lifecycle events;
- `AudioRecordingIntent` makes recording actions available to system surfaces and requires visible Live Activity behavior where the API specifies it.

**Gap added:** Himsat needs a formal Apple recording lifecycle state machine, Live Activity/lock-screen/control-center/action-button integration, and interruption reconciliation tests.

### Apple ScreenCaptureKit

Sources:

- https://developer.apple.com/documentation/screencapturekit
- https://developer.apple.com/documentation/screencapturekit/capturing-screen-content-on-ios

Key facts:

- ScreenCaptureKit provides fine-grained screen/audio capture with a system content-sharing picker;
- user permission and visible system controls are expected;
- current documentation includes newer iOS capabilities whose sample requires iOS 27 or later.

**Gap added:** selected-screen/window capture must be version-gated. Future/beta iOS APIs are an enhancement track, not a minimum supported mobile requirement.

### Android background microphone

Sources:

- https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start
- https://developer.android.com/about/versions/14/changes/fgs-types-required

Key facts:

- ongoing microphone capture can use a microphone foreground service;
- modern Android restricts starting such services from the background;
- user-visible foreground-service notification/state is part of the platform contract.

**Gap added:** Android needs a foreground-service lifecycle, notification/quick-action UX, restart/recovery rules, and tests for process death/background start restrictions.

### Android system/playback audio and phone calls

Sources:

- https://developer.android.com/media/platform/av-capture
- https://developer.android.com/reference/android/media/AudioPlaybackCaptureConfiguration
- https://developer.android.com/reference/android/media/MediaRecorder.AudioSource
- https://developer.android.com/media/platform/sharing-audio-input

Key facts:

- playback capture requires MediaProjection approval and depends on source-app usage/capture policy;
- a third-party application does not generally receive privileged `CAPTURE_AUDIO_OUTPUT` access to cellular voice-call uplink/downlink;
- concurrent audio-input behavior can produce silence depending on privacy-sensitive/app-priority conditions.

**Gap added:** Himsat must expose source health/capability diagnostics and must never promise universal “record every phone call/app” behavior.

## Open-source donor and architecture research

### Meetily

- https://github.com/Zackriya-Solutions/meetily
- public Community code is MIT;
- useful Rust/Tauri audio/transcription foundation;
- legacy Python backend is not the desired future architecture.

Role: **primary historical capture donor candidate**, not final Himsat architecture.

### Anarlog

- https://github.com/fastrepl/anarlog
- https://github.com/fastrepl/anarlog/blob/main/LICENSING.md

Observed:

- community layer is MIT;
- repository includes desktop, mobile, watch, CLI/MCP, plugins, and Rust crates for capture/transcription/diarization/storage;
- `enterprise/**` is commercially licensed and must not enter a permissive Himsat core.

Role: **selective community donor/reference** with path-level license gates.

### Argmax OSS / WhisperKit / SpeakerKit direction

- https://github.com/argmaxinc/argmax-oss-swift

Observed:

- MIT on-device Speech AI for Apple platforms, with WhisperKit/SpeakerKit/TTSKit direction and Apple-native acceleration.

Role: **Apple speech/diarization candidate**, especially where native CoreML/Apple integration beats forcing a generic Rust backend.

### Moonshine

- https://github.com/moonshine-ai/moonshine

Observed:

- very-low-latency speech stack targeted at voice agents/edge devices;
- code is MIT; current models are generally MIT with explicitly documented legacy non-English exceptions.

Role: **experimental low-latency speech donor/dependency candidate**. Model licenses must be checked per model.

### Sonora / WebRTC audio processing

- https://github.com/dignifiedquire/sonora
- https://github.com/tonarino/webrtc-audio-processing

Observed:

- echo cancellation, noise suppression, AGC, VAD, resampling;
- Sonora is pure Rust, BSD-3-Clause, and targets desktop plus Android/iOS.

**Major gap discovered:** earlier Himsat plans relied too heavily on capture and STT while under-specifying acoustic preprocessing. Add `himsat-audio-processing` with AEC/NS/AGC and reproducible quality tests.

### Document ingestion candidates

#### Kreuzberg

- current project: https://github.com/kreuzberg-dev/kreuzberg
- MIT LTS history: https://github.com/kreuzberg-dev/kreuzberg-lts

Observed:

- excellent Rust-core multi-format architecture and broad format coverage;
- current line uses Elastic License 2.0 and is not a default permissive donor for Himsat;
- v4 LTS is MIT but is a legacy line with limited support horizon.

Role: **current code REFERENCE_ONLY**; historical MIT LTS can only be considered after exact snapshot/provenance/security review.

#### tokimo-package-fileparser

- https://github.com/tokimo-lab/tokimo-package-fileparser

Observed:

- pure-Rust unified PDF/DOCX/XLSX/PPTX/text/archive to Markdown extraction;
- embedded images/media/objects extraction;
- MIT OR Apache-2.0.

Role: **strong permissive document-parser donor/dependency candidate** subject to maturity/security testing.

#### MarkItDown

- https://github.com/microsoft/markitdown

Observed:

- MIT file-to-Markdown conversion with useful security guidance for untrusted inputs;
- Python-centric.

Role: **behavior/reference and selective algorithm donor**, not preferred runtime dependency for the Rust core.

#### Docling

- https://github.com/docling-project/docling
- https://github.com/DS4SD/docling-core

Observed:

- MIT code, rich document intermediate representation, advanced PDF/layout/table/formula/image understanding;
- model licenses are separate.

Role: **document-IR and quality reference; potential optional local worker** if Rust-native paths cannot initially reach required extraction quality.

#### MinerU / Marker

- https://github.com/opendatalab/MinerU
- Marker project variants/references

Observed:

- technically strong document parsing but current licensing/model restrictions are not a clean fit for a permissive Himsat core.

Role: **REFERENCE_ONLY by default**.

### Plugin sandbox candidates

- https://github.com/extism/extism — BSD-3-Clause, Wasm-oriented plugin framework with host-controlled capabilities and runtime limits;
- https://github.com/bytecodealliance/wasmtime — Apache-2.0 WebAssembly runtime.

**Gap added:** Himsat plugins should not be arbitrary in-process dynamic code with ambient filesystem/network access. A Wasm capability sandbox is the preferred direction for third-party extensions.

### Local storage/search/sync/publishing candidates

- SQLCipher: https://github.com/sqlcipher/sqlcipher — BSD-style community license, encrypted SQLite;
- sqlite-vec: https://github.com/asg017/sqlite-vec — MIT/Apache-2.0, pre-v1 vector extension;
- fastembed-rs: https://github.com/Anush008/fastembed-rs — Apache-2.0 local embeddings/reranking;
- Iroh: https://github.com/n0-computer/iroh — MIT/Apache-2.0 P2P QUIC/NAT traversal;
- Typst: https://github.com/typst/typst — professional PDF generation with Tagged PDF, PDF/A and PDF/UA support;
- docx-rs: https://github.com/bokuweb/docx-rs — MIT DOCX read/write.

**Gaps added:**

- distinguish LAN/direct P2P from relay-assisted sync; strict Network Lock must not silently use public relays;
- publish accessible PDF/UA/PDF/A profiles, not only visually attractive PDF;
- document export must preserve semantic structure for Word/LLM reuse;
- sqlite-vec pre-v1 status requires an abstraction and migration strategy.

## Feature gaps added to the canonical plan

### 1. Capture Health

A recording UI must continuously know whether capture is actually healthy:

- source present/removed;
- silence vs expected signal;
- muted mic;
- system-audio source lost;
- Bluetooth route switch;
- clipping;
- excessive noise;
- low storage;
- thermal throttling;
- recorder lag/backpressure.

The product should warn during the meeting instead of discovering an empty recording afterward.

### 2. Audio Quality Pipeline

Add:

- AEC3/echo cancellation;
- noise suppression;
- AGC/limiting;
- resampling/channel alignment;
- optional dereverberation/beamforming research;
- objective and perceptual before/after benchmark fixtures.

### 3. Recall Buffer

An optional, visible, user-enabled encrypted rolling buffer can solve “I forgot to press save” without pretending recording was never active.

Proposed behavior:

- explicit mode with OS recording indicator;
- short bounded duration (for example 2/5/10 minutes configurable);
- preferably memory-backed where practical, encrypted spill only when required;
- overwritten automatically;
- becomes durable only on user action;
- disabled by default.

### 4. Capture Fusion / Himsat Bridge

Himsat Bridge becomes a first-class protocol, not just a convenience:

- phone controls session;
- desktop provides clean system audio;
- multiple devices can contribute timestamped sources;
- clocks are synchronized and drift measured;
- fusion preserves original sources and lineage;
- consensus reconstruction is a later high-risk Grain.

### 5. Accessibility

Add release gates for:

- live captions;
- screen reader semantics;
- complete keyboard control on desktop;
- scalable text/high contrast/reduced motion where appropriate;
- RTL and mixed RTL/LTR correctness;
- PDF/UA and tagged PDF publishing;
- caption/subtitle exports (`VTT`, `SRT`);
- optional local translation/caption language layer.

### 6. Translation and multilingual memory

Store original transcript as evidence and treat translations as derived artifacts. Search should support queries in one language retrieving evidence in another without replacing source truth.

### 7. Document Intermediate Representation

Create a Himsat-owned document IR rather than coupling memory directly to one parser. It must represent:

- pages/slides/sheets/sections;
- paragraphs/lists/headings;
- tables and cells;
- images/figures/charts;
- formulas;
- annotations/comments;
- bounding regions/page coordinates;
- source hashes;
- extracted attachments/media;
- OCR confidence;
- parser/model provenance.

This allows parser replacement without rewriting the memory system.

### 8. Portable AI Context Pack

Human PDF/DOCX is not the same output as optimal LLM context. Define an open Himsat Context Pack containing at least:

```text
manifest.json
content.md
evidence.jsonl
entities.jsonl
relations.jsonl
assets/
```

Optional profiles can generate bounded context for different token budgets while preserving source IDs and excluded-material declarations.

### 9. Model/Plugin Supply Chain

Every executable model/plugin pack requires:

- immutable identity/version;
- source/license;
- cryptographic digest;
- supported architectures/accelerators;
- memory/disk estimates;
- capability declaration;
- signature/trust policy;
- offline side-load path for air-gapped installations;
- explicit model-data license separate from engine code.

### 10. Privacy Firewall and selective disclosure

Before connector/write/export/share operations:

- detect/mark sensitive content where possible;
- preview exactly what leaves the vault;
- support “summary only”, “actions only”, “selected evidence”, “no audio”, “redacted transcript”;
- record egress provenance locally;
- never give a connector broader data access simply because OAuth allowed it.

### 11. Project/time memory

Beyond meeting search, Himsat needs:

- Projects/Spaces;
- People/Organizations;
- Decision Drift;
- Commitment Ledger;
- unresolved questions;
- contradictions;
- “what changed since?”;
- “what did we believe on date X?” temporal reconstruction;
- document-vs-meeting discrepancy detection;
- evidence lineage for every derived state.

### 12. Migration / Bring Your Memory

Support import adapters for available exports from:

- Otter;
- Circleback;
- Granola;
- Zoom/Teams/Meet artifacts;
- voice memo apps;
- generic audio/video/folder trees;
- Meetily/Anarlog when formats are known.

Migration quality must be measured: timestamps, speakers, notes, attachments, and evidence should not silently disappear.

### 13. Safe local agents and automation

Add:

- local MCP;
- local CLI/API;
- permission broker;
- read/write capability separation;
- action preview;
- explicit approval by default for external writes;
- Wasm plugin sandbox;
- prompt-injection-aware provenance labels (document/transcript content is untrusted data, not system instruction);
- automation dry-run and replay/evidence.

### 14. Daily/weekly private intelligence

A local scheduler can produce:

- today's meeting prep;
- daily decisions/actions digest;
- overdue commitments;
- project changes since last week;
- unresolved questions;
- documents added/changed;
- storage/model health notices.

This extends Himsat beyond active meeting time without creating a cloud service.

### 15. Accessibility-safe professional publishing

“Himsat Publish” should support templates for executive reports, minutes, research briefs, SOPs, PRDs, decision records, study guides, and technical specs while preserving semantic headings/tables/lists, citations, source links, RTL, and accessibility.

## Explicitly rejected or deferred directions

The following are not default product goals:

- stealth/hidden recording or privacy-indicator bypass;
- generalized emotion/personality scoring presented as truth;
- covert biometric speaker profiling;
- automatic external writes with no policy/approval;
- mandatory hosted Himsat identity or storage;
- cloud transcription or cloud LLM fallback hidden behind “local” branding;
- a claim that Himsat is legally compliant in every jurisdiction;
- a claim that every mobile application/phone call can be captured;
- copying public source that is not permissively licensed;
- copying donor architecture wholesale when a smaller native abstraction is better.

## Research conclusion

Himsat's strongest defensible product shape is:

> A private multimodal memory and action layer that records user-authorized conversations reliably, understands speech and documents locally, preserves evidence, reconstructs decisions and commitments over time, publishes human-quality and AI-ready outputs, and exposes the memory to local agents without requiring a vendor cloud.

The execution program must therefore optimize first for **provenance, reliability, platform truth, evidence, and data architecture**, then for feature breadth.
