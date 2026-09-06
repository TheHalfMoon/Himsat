# Himsat Research — Superwhisper and OpenSuperWhisper

**Research date:** 2026-09-07  
**Purpose:** evaluate Superwhisper and OpenSuperWhisper as product/UX/reference sources for future Himsat dictation and transcription surfaces without changing the active Specification 004 authority boundary.

## Governance status

```text
ACTIVE_CANONICAL_MAIN = c054566e9015ac7d05c575b090a7ecfe7bd8e418
ACTIVE_SPECIFICATION = 004-vault-key-crypto
CURRENT_AUTHORITY = 004P_PROVIDER_PROVENANCE
PRODUCT_FEATURE_AUTHORITY = NONE
DONOR_CODE_ADOPTION_AUTHORITY = NONE
SUPERWHISPER_ROLE = PRODUCT_REFERENCE_ONLY
OPENSUPERWHISPER_ROLE = REFERENCE_AND_FUTURE_SELECTIVE_DONOR_CANDIDATE
OPENSUPERWHISPER_CODE_ADOPTED = NO
```

This record does not authorize implementation, dependency adoption, source copying, model adoption, workflow changes, or release claims. Any later use of OpenSuperWhisper code or dependencies must pass the canonical provenance/license/security boundary independently.

## Sources reviewed

### Superwhisper

Current public product/documentation sources reviewed on 2026-09-07:

- https://superwhisper.com/
- https://superwhisper.com/models
- https://superwhisper.com/windows
- https://superwhisper.com/ios
- https://superwhisper.com/download
- https://superwhisper.com/changelog
- https://superwhisper.com/docs/modes/modes
- https://superwhisper.com/docs/modes/switching-modes
- https://superwhisper.com/docs/modes/super
- https://superwhisper.com/docs/modes/custom
- https://superwhisper.com/docs/get-started/interface-vocabulary
- https://superwhisper.com/docs/get-started/settings-shortcuts
- https://superwhisper.com/docs/common-issues/context
- https://superwhisper.com/docs/get-started/windows

Observed current product versions at the time of research:

```text
MACOS_VERSION = 2.18.3
WINDOWS_VERSION = 1.6.5
MACOS_MINIMUM = 13.3
WINDOWS_MINIMUM = Windows 10
IOS_MINIMUM = iOS 18
```

These are time-sensitive product facts and must be rechecked before any future compatibility claim.

### OpenSuperWhisper

Repository:

```text
REPOSITORY = https://github.com/Starmel/OpenSuperWhisper
DEFAULT_BRANCH = master
REVIEWED_MASTER_SHA = bef6bc0421d0c010e8f2fb4288c0d74978c8b964
LICENSE = MIT
PLATFORM = macOS Apple Silicon/ARM64
```

Primary reviewed repository files:

- `Readme.md`
- `LICENSE`
- `.gitmodules`
- `Package.resolved`
- `OpenSuperWhisper/AudioRecorder.swift`
- `OpenSuperWhisper/MicrophoneService.swift`
- `OpenSuperWhisper/ModifierKeyMonitor.swift`
- `OpenSuperWhisper/MouseButtonMonitor.swift`
- `OpenSuperWhisper/FileDropHandler.swift`
- `OpenSuperWhisper/Engines/*`
- `OpenSuperWhisper/Indicator/*`
- project/test/build configuration

The repository README reports two transcription engines: Whisper through `whisper.cpp` and Parakeet through FluidAudio. It supports global keyboard shortcuts, single-modifier shortcuts, mouse-button triggers, hold-to-record, drag/drop file transcription queues, microphone selection, language auto-detection, and Asian-language autocorrection.

## Competitive/product signals from Superwhisper

### 1. Dictation is a first-class product surface, not merely a transcription input

Superwhisper treats voice input as a system-wide writing surface that can place polished text into the user's current app. This is materially different from a meeting-transcript-only model.

**Himsat implication:** future roadmap should include a distinct `Universal Dictation Surface` rather than forcing dictation through meeting/session UX.

Minimum future requirements should include:

- system-wide trigger from any supported application;
- explicit recording state and cancel semantics;
- push-to-talk and toggle recording;
- cursor-targeted insertion with clipboard-safe fallback;
- deterministic recovery when target focus changes or paste fails;
- local-only mode compatible with Network Lock;
- no silent capture of application context.

### 2. Declarative modes are an important abstraction

Superwhisper modes combine:

- language;
- voice model;
- optional language model;
- audio behavior;
- application/site auto-activation;
- context sources;
- prompt/instruction behavior;
- shortcut/deep-link activation.

**Himsat implication:** future dictation/assistant configuration should be modeled as a versioned declarative profile rather than scattered UI preferences.

A future Himsat mode contract should separate:

```text
capture policy
transcription provider/model
language/translation policy
context capability grants
post-processing instructions
output target/format
activation rules
network policy
retention policy
```

Profiles must be exportable, diffable, and provenance-aware.

### 3. Context awareness needs a capability broker

Superwhisper documents three context classes:

- application context;
- selected text;
- clipboard context.

The product uses accessibility APIs to collect contextual information for context-aware rewriting.

**Himsat implication:** context-aware dictation is valuable, but Himsat should not expose ambient desktop data to a model implicitly.

A future Himsat `Context Capability Broker` should require per-mode grants and expose independently switchable capabilities such as:

```text
active_app_identity
window_title
focused_field_text
selected_text
clipboard_text
screen_region
meeting_context
project_memory
```

Each capability must have:

- explicit user-visible state;
- bounded maximum payload;
- local/redacted/cloud policy;
- audit/log metadata without secret payload leakage;
- fail-closed behavior when the OS capability is unavailable.

### 4. Vocabulary and deterministic replacements should be separate systems

Superwhisper distinguishes model vocabulary hints from deterministic post-transcription replacements.

This distinction is strong and should be preserved.

**Himsat implication:** future terminology support should have two independent layers:

1. `VocabularyHint` — probabilistic model hinting/boosting;
2. `ReplacementRule` — deterministic post-transcription rewrite.

Replacement rules should be testable, portable, scoped, and reversible. Medical/professional terminology must never be injected into unrelated contexts by a single global unbounded dictionary.

### 5. Input ergonomics are product-critical

Observed Superwhisper/OpenSuperWhisper interaction patterns include:

- configurable global shortcuts;
- single left/right modifier triggers;
- push-to-talk;
- mouse buttons;
- mode cycling;
- deep links;
- menu-bar control;
- recording indicator/window.

**Himsat implication:** input control is a reliability/accessibility subsystem, not decorative UX.

Future qualification should include:

- shortcut collision detection;
- left/right modifier distinction where available;
- press-vs-hold state-machine tests;
- key-repeat suppression;
- accessibility permissions;
- mouse device capability variance;
- target-focus loss;
- sleep/wake and app-relaunch behavior;
- clear visual/audible recording state.

### 6. Local/cloud model routing should be explicit and visible

Superwhisper supports local and cloud transcription/LLM models and BYOK-style configurations. Its public product material emphasizes that local models can keep microphone audio on-device while cloud paths send data through a proxy/provider path.

**Himsat implication:** Himsat should expose model execution locality as a first-class policy, not infer it from provider names.

Future model-route state should distinguish at least:

```text
LOCAL_OFFLINE
LOCAL_NETWORKED
REMOTE_FIRST_PARTY_PROXY
REMOTE_DIRECT_BYOK
REMOTE_ENTERPRISE_ENDPOINT
```

The UI should show which path is active before recording/processing begins. Network Lock must make remote paths impossible rather than merely discouraged.

No Superwhisper privacy/compliance marketing claim is inherited by Himsat. Himsat must qualify its own behavior.

### 7. Meeting and dictation surfaces should share engines but not UX semantics

Superwhisper combines dictation, meeting recording, speaker separation, system audio, file transcription, and AI post-processing in one product.

**Himsat implication:** shared capture/STT/model infrastructure is useful, but Himsat should preserve separate product contracts:

```text
Quick Dictation
Meeting Session
File Transcription
Recall/Continuous Memory
Live Assistant
```

Each surface has different consent, retention, interruption, context, speaker, and output requirements.

### 8. Cross-platform parity should be capability-based, not marketing-based

Superwhisper's Windows documentation explicitly reports feature gaps relative to macOS, including current limitations around FileSync, full speaker separation, model library/favorites, local language models, mouse shortcuts, microphone-volume automation, and clipboard restoration.

**Himsat implication:** do not promise one feature matrix merely because the app exists on multiple platforms. Maintain a capability table with exact OS/version/runtime evidence.

## OpenSuperWhisper donor/reference analysis

### Useful architecture/reference surfaces

OpenSuperWhisper is especially useful as a macOS implementation reference for:

- AVFoundation microphone capture lifecycle;
- enumerating/selecting input devices;
- global modifier-key monitoring;
- mouse-button recording triggers;
- press/hold recording state;
- floating recording indicators;
- drag-and-drop file ingestion with queue processing;
- engine abstraction between Whisper and Parakeet;
- model download/selection UX;
- menu-bar/background-app behavior;
- language availability tied to engine/model capability.

These are reference candidates only under current authority.

### Exact top-level repository license

The reviewed OpenSuperWhisper repository root license is MIT.

That does not automatically clear every incorporated dependency, submodule, downloaded model, binary artifact, font, asset, or model weight.

### Known dependency/provenance surfaces requiring independent review

Repository `.gitmodules` declares:

```text
libwhisper/whisper.cpp -> https://github.com/ggerganov/whisper.cpp
asian-autocorrect      -> https://github.com/huacnlee/autocorrect
```

The checked root `Package.resolved` includes at least:

```text
GRDB.swift 6.29.3 @ 2cf6c756e1e5ef6901ebae16576a7e4e4b834622
KeyboardShortcuts 1.17.0 @ ac12762853126cf2e7ad63a6a58e1c9f58c6a0ee
```

The project also references FluidAudio/Parakeet behavior and downloadable Whisper/model assets. Exact source/package/model identities must be resolved independently if any adoption is later considered.

### Donor restrictions for Himsat

Before any OpenSuperWhisper source is copied or adapted:

1. freeze exact upstream commit and exact file paths;
2. resolve all relevant submodules/package revisions;
3. verify source license and notice obligations per path;
4. distinguish source-code license from model-weight license;
5. review security/privacy behavior of the candidate path;
6. compare against Himsat's existing architecture and avoid duplicating a stronger native abstraction;
7. register exact adopted artifacts in Himsat provenance machinery before bytes enter canonical source;
8. preserve attribution/notices as required;
9. independently test rather than treating donor tests as Himsat qualification.

## New roadmap candidates discovered

These are planning candidates only. They do not modify current authority.

### Candidate — Universal Dictation Surface

Goal: system-wide private voice-to-text and voice-to-polished-text in any supported text field.

Key subunits:

- global trigger service;
- explicit recorder state machine;
- local STT fast path;
- optional context broker;
- deterministic output insertion;
- history/retention policy;
- mode selection;
- accessibility/focus integration;
- platform parity matrix.

### Candidate — Mode Engine

Goal: versioned, exportable, portable voice-processing profiles.

Required properties:

- declarative schema;
- deterministic inheritance/versioning;
- app/site activation rules;
- capability grants;
- local/cloud route constraints;
- prompt/version provenance;
- import/export;
- no hidden ambient context.

### Candidate — Terminology System

Goal: improve domain accuracy without uncontrolled prompt growth.

Components:

- vocabulary hints;
- deterministic replacements;
- per-project/per-mode scopes;
- import/export;
- conflict handling;
- benchmark evidence;
- privacy-safe storage.

### Candidate — Input Control Layer

Goal: reliable low-friction voice activation.

Inputs:

- keyboard chords;
- left/right single modifiers;
- push-to-talk;
- mouse buttons;
- menu-bar/system-tray actions;
- deep links/automation invocation;
- accessibility switches where appropriate.

### Candidate — Voice Model Router

Goal: explicit provider/locality choice with capability and privacy metadata.

The router should make model locality, language support, streaming support, diarization support, expected resource use, license/provenance state, and Network Lock compatibility queryable before selection.

## Priority implications for Himsat

The new sources strengthen, rather than replace, the existing Himsat direction.

Recommended future priority after the canonical storage/capture foundations exist:

1. shared capture/model infrastructure;
2. reliable meeting recording;
3. universal dictation surface;
4. vocabulary/replacement system;
5. declarative modes + context capability broker;
6. local-first model router;
7. file transcription queue;
8. speaker separation and meeting-specific AI;
9. automation/deep-link/agent surfaces.

The distinguishing Himsat principle should remain:

> convenience must not require ambient data access, vendor lock-in, or hidden cloud processing.

## Final disposition

```text
SUPERWHISPER_COMPETITIVE_SIGNAL = STRONG
OPENSUPERWHISPER_REFERENCE_VALUE = STRONG
OPENSUPERWHISPER_TOP_LEVEL_LICENSE = MIT
OPENSUPERWHISPER_DONOR_ELIGIBILITY = NOT_YET_QUALIFIED
NEW_PRODUCT_CANDIDATES_RECORDED = YES
CURRENT_SPEC_004_AUTHORITY_CHANGED = NO
CURRENT_004P_DEPENDENCY_SELECTION_CHANGED = NO
DONOR_CODE_ADOPTED = NO
```

These sources should be revisited when Himsat reaches the dictation/input-control/model-routing implementation frontier. At that point, live upstream truth and Himsat's canonical governance must be re-read before any code or dependency adoption.
