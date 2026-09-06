# Himsat Landscape and Gap Analysis

**Research date:** 2026-09-06  
**Purpose:** identify missing product, platform, architecture, reliability, security, interoperability, and donor-plan requirements before Himsat implementation begins.

This document is a planning research record. Product proposals are not implementation claims. External product behavior, operating-system APIs, repository ownership, and licenses can change; live sources must be rechecked before comparative claims or donor adoption.

## Executive conclusion

The original concept — a better local alternative to Otter built from Meetily — is directionally strong but incomplete. The category has moved from “meeting transcription” toward **continuous conversation context + live assistance + workflow action + cross-source memory**.

The strongest Himsat opportunity is not feature-count parity. It is combining modern meeting intelligence with an architecture competitors generally do not optimize for:

> **user-controlled local capture + local multimodal evidence + temporal memory + professional document intelligence + local agents + portable context + accountless/private sync.**

The largest gaps found in the earlier plan were:

1. audio enhancement/AEC and capture-quality diagnostics were under-specified;
2. OS/version capability boundaries, especially mobile system audio and phone calls, needed stronger versioning and fail-closed behavior;
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
14. the donor plan needed path-level licensing and model-license gates because attractive projects can contain mixed licenses or can change licensing over time.

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

- meeting notes/action items are becoming commodity entry features;
- assistant search spans meetings plus connected context;
- integrations are not only export targets: assistants can search and take actions in external apps;
- automations use conditions plus chained actions;
- screen-shared information can become searchable meeting context;
- desktop capture can be botless and capture microphone plus computer audio;
- MCP/API/CLI access makes meeting context a reusable platform surface;
- Circleback's MCP is hosted, creating room for a local capability-scoped Himsat MCP.

**Himsat implication:** transcript + summary is insufficient. Himsat needs cross-source search, multimodal context, local action workflows, and reusable local developer/agent surfaces.

### Otter

Sources:

- https://otter.ai/blog/never-miss-the-conversation-introducing-automatic-recording-in-otter-desktop
- https://otter.ai/blog/otter-ai-introduces-live-assist-the-first-live-coaching-agent-for-every-call
- https://help.otter.ai/hc/en-us/articles/35973988280215-Otter-Desktop-App-Mac-Windows

Observed signals:

- automatic meeting recording based on microphone/meeting activity is a mainstream expectation;
- desktop capture, headphones support, meeting reminders, and recording-state ergonomics matter as much as transcription models;
- live assistance/coaching grounded in playbooks, prior meetings, and resources moves intelligence into the meeting itself.

**Himsat implication:** local live assistance and meeting detection belong in the roadmap, but must remain opt-in, evidence-aware, and platform-qualified.

### Granola

Sources:

- https://www.granola.ai/updates
- https://www.granola.ai/integrations
- https://docs.granola.ai/help-center/ios/getting-started
- https://docs.granola.ai/help-center/ios/phone-calls
- https://docs.granola.ai/help-center/ios/taking-notes

Observed signals:

- mobile, Android, Apple Watch, briefs, team context, MCP, integrations, people/company views, and saved prompt recipes expand the product beyond meeting notes;
- human notes are intentionally useful guidance for AI enhancement;
- mobile audio behavior exposes platform limits: ordinary iOS applications cannot simply assume arbitrary other-app virtual meeting/call audio is available.

**Himsat implication:** Note Gravity stays important; mobile UI must be native-quality; virtual-meeting/system-audio/call claims require OS-specific capability detection.

### Fireflies

Source:

- https://guide.fireflies.ai/articles/2679406774-live-assist-on-the-fireflies-desktop-app-real-time-notes-and-suggestions

Observed signals:

- live notes, transcript, Q&A, custom skills, meeting prep, instant summaries, and suggestions converge in a floating desktop surface.

**Himsat implication:** a local floating Live surface should expose transcript, bookmarks, questions, decisions, contradictions, and user-selected playbook prompts without requiring vendor-cloud processing.

### Plaud / dedicated capture hardware

Source:

- https://support.plaud.ai/hc/en-us/articles/53771056805785-Does-Plaud-NotePin-S-work-offline

Observed signal:

- dedicated hardware can record/store offline while advanced processing may still require connectivity.

**Himsat implication:** local intelligence is a meaningful differentiator. Generic recorder-file import and external microphones are higher-priority than proprietary hardware.

### Continuous-memory category

Reference:

- https://www.limitless.ai/

Observed signal:

- continuous personal-memory products create both demand and platform/provider risk; portability and graceful exit matter.

**Himsat implication:** continuous-memory capability must be explicitly enabled, visibly active, locally encrypted, retention-controlled, exportable, and independent of a vendor service remaining online.

## Platform research and consequences

### Apple background audio and recording intents

Sources:

- https://developer.apple.com/documentation/avfaudio/avaudiosession/category-swift.struct/playandrecord
- https://developer.apple.com/documentation/appintents/audiorecordingintent
- https://developer.apple.com/documentation/appintents/widgets-live-activities-and-controls

Planning facts rechecked on 2026-09-06:

- correct audio-session/background configuration supports ongoing recording behavior under supported conditions;
- microphone permission remains explicit;
- sessions are still subject to interruptions, routes, lifecycle, power, and OS policy;
- when `AudioRecordingIntent` is used, Apple requires the corresponding visible Live Activity behavior while recording; the API is not a stealth background-capture mechanism.

**Gap added:** Himsat needs a formal Apple recording lifecycle state machine, Live Activity/system-control integration where used, and interruption reconciliation tests.

### Apple ScreenCaptureKit

Sources:

- https://developer.apple.com/documentation/screencapturekit
- https://developer.apple.com/documentation/screencapturekit/capturing-screen-content-on-ios

Planning facts rechecked on 2026-09-06:

- ScreenCaptureKit provides fine-grained user-authorized screen/audio capture surfaces;
- permission and system selection/control are part of the model;
- the current iOS screen-capture sample referenced by Apple requires iOS 27 or later and is not a baseline for all supported phones.

**Gap added:** selected-screen/window/app capture must be runtime/version-gated. New/future APIs are enhancement tracks, not minimum mobile requirements.

### Android background microphone

Sources:

- https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start
- https://developer.android.com/about/versions/14/changes/fgs-types-required

Planning facts rechecked on 2026-09-06:

- ongoing microphone capture uses a microphone foreground service in the normal background-recording architecture;
- Android 12+ restricts starting foreground services from the background;
- Android 14+ applies stricter while-in-use microphone permission/start constraints;
- user-visible foreground-service state is part of the platform contract.

**Gap added:** Android needs a foreground-service lifecycle, notification/quick-action UX, process-death/recovery rules, and version/device tests.

### Android playback audio and phone calls

Sources:

- https://developer.android.com/media/platform/av-capture
- https://developer.android.com/reference/android/media/AudioPlaybackCaptureConfiguration
- https://developer.android.com/reference/android/media/MediaRecorder.AudioSource
- https://developer.android.com/media/platform/sharing-audio-input

Planning facts:

- playback capture requires MediaProjection approval and depends on source-app usage/capture policy;
- ordinary third-party applications do not generally receive privileged cellular call uplink/downlink capture authority;
- concurrent audio input/privacy-sensitive app priority can cause silence or degraded capture.

**Gap added:** Himsat must expose source health/capability diagnostics and must never promise universal “record every phone call/app” behavior.

## Open-source donor and architecture research

### Meetily

Sources:

- https://github.com/Zackriya-Solutions/meetily
- public Community repository license and architecture documents

Observed:

- Community source is MIT;
- Rust/Tauri capture/transcription foundations are useful;
- the legacy Python backend is not the desired Himsat architecture;
- separate Pro behavior must not be assumed to exist in the Community source.

Role: **primary historical selective donor candidate**, not final Himsat architecture.

### Anarlog

Sources:

- https://github.com/fastrepl/anarlog
- https://github.com/fastrepl/anarlog/blob/main/LICENSING.md

Observed:

- community layer is MIT;
- repository spans desktop, mobile, watch, CLI/MCP, plugins, and Rust crates for capture/transcription/diarization/storage;
- `enterprise/**` is commercially licensed and must not enter the permissive Himsat core without a separate decision/license.

Role: **selective community donor/reference with path-level license gates**.

### Argmax OSS / WhisperKit / SpeakerKit direction

Source:

- https://github.com/argmaxinc/argmax-oss-swift

Observed:

- MIT on-device Speech AI for Apple platforms with Apple-native acceleration paths.

Role: **Apple speech/diarization candidate**, especially where CoreML/native integration is stronger than forcing one generic runtime.

### Moonshine

Source:

- https://github.com/moonshine-ai/moonshine

Observed:

- low-latency speech stack targeted at edge/voice-agent scenarios;
- source code is MIT;
- model licenses must be checked per exact model/revision, including documented legacy exceptions.

Role: **experimental low-latency speech donor/dependency candidate**.

### Sonora / WebRTC audio processing

Sources:

- https://github.com/dignifiedquire/sonora
- https://github.com/tonarino/webrtc-audio-processing

Observed:

- echo cancellation, noise suppression, AGC, VAD, and resampling are reusable subsystem candidates;
- Sonora is Rust-oriented and permissively licensed.

**Major gap discovered:** earlier plans relied too heavily on capture and STT while under-specifying acoustic preprocessing. Add a dedicated audio-processing boundary with AEC/NS/AGC and reproducible quality tests.

### Document ingestion candidates

#### Xberg / Kreuzberg lineage

Live repository truth rechecked on 2026-09-06:

- the former `kreuzberg-dev/kreuzberg` repository resolves to `https://github.com/xberg-io/xberg`;
- current Xberg v1 workspace metadata declares `license = "MIT"`;
- the Xberg changelog explicitly distinguishes the current MIT v1 line from Kreuzberg 4.8/4.9 releases that used Elastic License 2.0;
- the separate `kreuzberg-dev/kreuzberg-lts` v4 line is a historical MIT candidate;
- exact path-level third-party/vendored/model/font/plugin/asset terms still require review.

Sources:

- https://github.com/xberg-io/xberg
- https://github.com/xberg-io/xberg/blob/main/Cargo.toml
- https://github.com/xberg-io/xberg/blob/main/CHANGELOG.md
- https://github.com/kreuzberg-dev/kreuzberg-lts

Role: **current Xberg is a permissive experiment/dependency/selective-copy candidate, not `REFERENCE_ONLY` solely because of historical Kreuzberg licensing**. Historical releases must be treated by exact revision and controlling license.

#### tokimo-package-fileparser

Source:

- https://github.com/tokimo-lab/tokimo-package-fileparser

Observed:

- Rust-oriented unified PDF/DOCX/XLSX/PPTX/text/archive extraction;
- embedded asset extraction;
- permissive MIT OR Apache-2.0 licensing at the project level.

Role: **strong permissive document-parser donor/dependency candidate subject to maturity/security tests**.

#### MarkItDown

Source:

- https://github.com/microsoft/markitdown

Observed:

- MIT file-to-Markdown conversion;
- useful security guidance for untrusted inputs;
- Python-centric runtime.

Role: **behavior/reference and selective algorithm donor**, not preferred mandatory core runtime.

#### Docling

Sources:

- https://github.com/docling-project/docling
- https://github.com/DS4SD/docling-core

Observed:

- MIT code with rich document representation and advanced PDF/layout/table/formula/image understanding;
- models remain separate supply-chain/license objects.

Role: **document-IR/quality reference and possible optional worker if Rust-native paths cannot initially reach required quality**.

#### MinerU / Marker variants

References:

- https://github.com/opendatalab/MinerU
- current Marker variants/repositories at implementation time

Observed:

- technically strong document parsing;
- licensing/model/commercial constraints are not a clean default fit for a permissive Himsat core.

Role: **REFERENCE_ONLY by default until exact terms are independently cleared**.

### Plugin sandbox candidates

Sources:

- https://github.com/extism/extism
- https://github.com/bytecodealliance/wasmtime

Observed:

- Wasm provides a practical way to give third-party extensions bounded host capabilities rather than ambient filesystem/network access.

**Gap added:** Himsat plugins should use a capability-constrained sandbox direction with memory/time/fuel/resource controls.

### Local storage/search/sync/publishing candidates

Candidates:

- SQLCipher — https://github.com/sqlcipher/sqlcipher
- sqlite-vec — https://github.com/asg017/sqlite-vec
- fastembed-rs — https://github.com/Anush008/fastembed-rs
- Iroh — https://github.com/n0-computer/iroh
- Typst — https://github.com/typst/typst
- docx-rs — https://github.com/bokuweb/docx-rs

Planning consequences:

- separate database encryption, blob encryption, key management, and backup semantics;
- hide vector search behind an abstraction because pre-v1 APIs can change;
- distinguish direct/LAN P2P from relay-assisted connectivity so Network Lock cannot silently use public relays;
- publish accessible/semantic PDF and DOCX rather than treating visual appearance as sufficient;
- treat fonts/templates/assets as separately licensed objects.

### Screenpipe licensing boundary

Source:

- https://github.com/screenpipe/screenpipe

Live licensing rechecked on 2026-09-06:

- current Screenpipe source uses a commercial/source-available license that restricts using current code to build a competing product without the required commercial permission;
- versions previously released under MIT retain their historical MIT license.

Role: **current code REFERENCE_ONLY under the default permissive-core plan**. An older permissive snapshot may only be considered after exact commit/path/license/security review.

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

- echo cancellation;
- noise suppression;
- AGC/limiting;
- resampling/channel alignment;
- optional dereverberation/beamforming research;
- objective and perceptual before/after benchmark fixtures.

### 3. Recall Buffer

An optional, visible, user-enabled encrypted rolling buffer can solve “I forgot to save that moment” without pretending recording was inactive.

Required direction:

- explicit mode with OS recording indicator;
- short bounded configurable duration;
- overwritten automatically;
- becomes durable only on user action;
- disabled by default;
- retention/storage behavior is visible.

### 4. Capture Fusion / Himsat Bridge

Himsat Bridge is a first-class protocol:

- phone can control a session;
- desktop can contribute clean system audio;
- multiple devices can contribute timestamped sources;
- clocks/drift are measured;
- reconnection preserves lineage;
- original sources remain available;
- consensus reconstruction is a later separately shaped high-risk capability.

### 5. Accessibility

Add release gates for:

- live captions;
- screen-reader semantics;
- complete keyboard control on desktop;
- scalable text/high contrast/reduced motion where appropriate;
- RTL and mixed RTL/LTR correctness;
- PDF/UA/tagged PDF targets where selected;
- VTT/SRT caption export;
- optional local translation while preserving source-language evidence.

### 6. Translation and multilingual memory

Store original transcript as evidence and translations as derived artifacts. Search should support a query in one language retrieving evidence in another without overwriting source truth.

### 7. Document Intermediate Representation

Create a Himsat-owned document IR rather than coupling memory directly to one parser. It should represent:

- pages/slides/sheets/sections;
- paragraphs/lists/headings;
- tables/cells;
- images/figures/charts;
- formulas;
- comments/annotations;
- bounding regions/page coordinates;
- source hashes;
- attachments/media;
- OCR confidence;
- parser/model provenance.

### 8. Portable AI Context Pack

Human PDF/DOCX is not the same output as optimal LLM context. Define an open pack containing at least:

```text
manifest.json
content.md
evidence.jsonl
entities.jsonl
relations.jsonl
assets/
```

Profiles may create bounded context for token budgets while preserving source IDs and excluded-material declarations.

### 9. Model/Plugin Supply Chain

Every executable model/plugin pack needs:

- immutable identity/version;
- source/license;
- cryptographic digest;
- supported architectures/accelerators;
- resource estimates;
- capability declaration;
- signature/trust policy;
- offline side-load path;
- explicit model/data/asset license separate from engine code.

### 10. Privacy Firewall and selective disclosure

Before connector/write/export/share operations:

- preview what leaves the vault;
- minimize payloads;
- support summary-only/actions-only/selected-evidence/no-audio/redacted-transcript modes;
- record egress provenance locally;
- never treat broad OAuth scope as permission to export every local object.

### 11. Project/time memory

Beyond meeting search, Himsat needs:

- Projects/Spaces;
- People/Organizations;
- Decision Drift;
- Commitment Ledger;
- unresolved questions;
- contradictions;
- “what changed since?”;
- “what did we believe on date X?” reconstruction;
- document-vs-meeting discrepancy detection;
- evidence lineage for derived state.

### 12. Migration / Bring Your Memory

Potential adapters include available exports from:

- Otter;
- Circleback;
- Granola;
- Zoom/Teams/Meet artifacts;
- voice memo apps;
- generic audio/video/folder trees;
- Meetily/Anarlog when formats are known.

Migration quality must be measured. Timestamps, speakers, notes, attachments, and evidence must not silently disappear.

### 13. Safe local agents and automation

Add:

- local MCP;
- local CLI/API;
- permission broker;
- read/write capability separation;
- action preview;
- explicit approval by default for external writes;
- Wasm plugin sandbox;
- prompt-injection-aware provenance labels;
- automation dry-run/replay/evidence.

### 14. Daily/weekly private intelligence

A local scheduler can produce:

- meeting prep;
- decisions/actions digest;
- overdue commitments;
- project changes;
- unresolved questions;
- documents added/changed;
- storage/model health notices.

### 15. Accessibility-safe professional publishing

Himsat Publish should support executive reports, minutes, research briefs, SOPs, PRDs, decision records, study guides, and technical specs while preserving semantic headings/tables/lists, citations, source links, RTL, and accessibility.

## Explicitly rejected or deferred directions

The following are not default product goals:

- stealth/hidden recording or privacy-indicator bypass;
- generalized emotion/personality scoring presented as truth;
- covert biometric speaker profiling;
- automatic external writes with no policy/approval;
- mandatory hosted Himsat identity/storage;
- hidden cloud STT/LLM fallback behind “local” branding;
- blanket legal-compliance claims;
- claims that every mobile application/phone call can be captured;
- copying public source that is not licensed for the intended use;
- copying donor architecture wholesale when a smaller Himsat-native abstraction is lower risk.

## Research conclusion

Himsat's strongest defensible product shape is:

> A private multimodal memory and action layer that records user-authorized conversations reliably, understands speech and documents locally, preserves evidence, reconstructs decisions and commitments over time, publishes human-quality and AI-ready outputs, and exposes memory to local agents without requiring a vendor cloud.

The execution program should optimize first for **provenance, reliability, platform truth, evidence, and data architecture**, then for feature breadth.
