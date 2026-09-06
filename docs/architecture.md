# Himsat Architecture

## Architectural objective

Build a Rust-first, local-first platform in which capture, evidence, memory, document understanding, publishing, sync, and agent permissions are explicit modules rather than one desktop application process.

The architecture must support desktop, iOS/iPadOS/watchOS, Android, and future device surfaces without pretending platform audio/screen APIs are identical.

## Top-level shape

```text
                              ┌──────────────────────┐
                              │     Himsat Core      │
                              │        Rust          │
                              └──────────┬───────────┘
                                         │
             ┌───────────────────────────┼───────────────────────────┐
             │                           │                           │
             ▼                           ▼                           ▼
      Desktop application          Apple applications         Android application
      Tauri + React/TS             Swift / SwiftUI            Kotlin / Compose
      native Rust capture          native Apple services      native Android services
             │                           │                           │
             └───────────────────────────┼───────────────────────────┘
                                         ▼
                                  shared local vault
```

Native UI/platform code is not a failure of portability. It is the correct boundary for background services, audio sessions, lock-screen controls, notifications, secure storage, watch integration, and capture permissions.

## Planned repository structure

```text
himsat/
├── apps/
│   ├── desktop/
│   ├── ios/
│   ├── android/
│   ├── watchos/
│   └── cli/
├── crates/
│   ├── himsat-core/
│   ├── himsat-runtime/
│   ├── himsat-events/
│   ├── himsat-policy/
│   ├── himsat-capabilities/
│   ├── himsat-audio/
│   ├── himsat-audio-processing/
│   ├── himsat-capture-macos/
│   ├── himsat-capture-windows/
│   ├── himsat-capture-linux/
│   ├── himsat-stt/
│   ├── himsat-diarization/
│   ├── himsat-speakers/
│   ├── himsat-models/
│   ├── himsat-vault/
│   ├── himsat-db/
│   ├── himsat-search/
│   ├── himsat-memory/
│   ├── himsat-evidence/
│   ├── himsat-documents/
│   ├── himsat-document-ir/
│   ├── himsat-ocr/
│   ├── himsat-vision/
│   ├── himsat-intelligence/
│   ├── himsat-transform/
│   ├── himsat-publish/
│   ├── himsat-context-pack/
│   ├── himsat-bridge/
│   ├── himsat-sync/
│   ├── himsat-connectors/
│   ├── himsat-agents/
│   ├── himsat-flows/
│   ├── himsat-mcp/
│   └── himsat-plugin-host/
├── packages/
│   ├── ui/
│   ├── editor/
│   ├── schemas/
│   └── design-tokens/
├── models/
│   ├── registry/
│   └── licenses/
├── specs/
├── docs/
├── benchmarks/
├── governance/
└── security/
```

This is a target map, not an instruction to create empty crates before their Grains are selected.

## Event-sourced evidence core

The core should model durable facts/events instead of a single mutable transcript blob.

Candidate event types:

```text
SessionCreated
CaptureSourceAttached
CaptureSourceDetached
AudioChunkDurable
CaptureHealthChanged
SpeechDetected
TranscriptPartialProduced
TranscriptSegmentFinalized
TranscriptSegmentSuperseded
SpeakerClusterObserved
SpeakerIdentityAssigned
ManualNoteCreated
BookmarkCreated
ScreenFrameCaptured
DocumentLinked
DecisionCandidateProduced
CommitmentCandidateProduced
ClaimProduced
EvidenceLinked
UserCorrectionApplied
ActionProposed
ActionApproved
ActionExecuted
RetentionApplied
ExportCreated
SyncReplicaUpdated
```

Benefits:

- crash recovery;
- provenance;
- exact source lineage;
- temporal reconstruction;
- sync/reconciliation;
- auditability;
- non-destructive transcript refinement.

Event sourcing must not become an excuse for unbounded storage. Compaction/materialized views are allowed when the evidence contract remains intact.

## Session model

A `Session` owns:

- session identity and metadata;
- authorized capture sources;
- source clocks/offsets;
- durable media chunks;
- transcript versions;
- user notes/bookmarks;
- visual/document context;
- derived intelligence;
- retention policy;
- encryption domain;
- provenance.

A `Project/Space` groups sessions and documents without changing source ownership.

## Audio architecture

```text
sources
  ├── microphone
  ├── desktop system audio
  ├── mobile playback/screen audio where OS policy permits
  ├── imported media
  └── Himsat Bridge peer source
        │
        ▼
source adapters
        │
        ▼
clocking + resampling + channel alignment
        │
        ▼
AEC / NS / AGC / limiter / health metrics
        │
        ├──────────────► crash-safe encrypted media chunks
        │
        ▼
VAD / segmentation
        │
        ▼
streaming STT
        │
        ▼
live transcript + diarization candidates
```

The raw/original source should be preserved according to retention policy so later quality passes are possible. Processed audio must not silently replace source evidence without lineage.

## Capture reliability state machine

A recording session is not a boolean.

Candidate states:

```text
IDLE
PREPARING
RECORDING_HEALTHY
RECORDING_DEGRADED
INTERRUPTED
RECOVERING
STOPPING
FINALIZING
COMPLETED
FAILED_RECOVERABLE
FAILED_TERMINAL
```

Each platform adapter emits typed reasons such as route change, permission revoked, projection stopped, source silent, disk pressure, process interruption, or thermal/resource pressure.

The UI must distinguish “recording indicator is on” from “meaningful source audio is reaching durable storage.”

## Durable media format

Do not design the vault around one giant WAV file.

Direction:

- bounded chunks;
- monotonically ordered source sequence;
- authenticated encryption per chunk or safe envelope;
- checksum/digest;
- timestamp range;
- codec/sample format metadata;
- source/device ID;
- previous/chain reference if a tamper-evident chain is selected;
- journal/checkpoint acknowledgement only after durable write.

Loss target should be at most one bounded uncommitted chunk after a sudden crash; the exact chunk duration is a benchmark/implementation decision.

## Speech engine abstraction

Candidate interface concepts:

```rust
trait SpeechEngine {
    fn identity(&self) -> ModelIdentity;
    fn capabilities(&self) -> SpeechCapabilities;
    fn resource_profile(&self) -> ResourceProfile;
    fn transcribe_batch(&self, input: AudioInput) -> Result<TranscriptArtifact>;
    fn transcribe_stream(&self, input: AudioStream) -> Result<TranscriptStream>;
}
```

Capabilities include language set, streaming, word timestamps, contextual biasing, hardware backends, memory estimate, and license/model identity.

The product chooses engines via a model router; callers depend on Himsat contracts.

## Two-pass transcript lineage

```text
source audio
   ├── live pass -> transcript revision L1
   └── quality pass -> transcript revision Q1
                          │
                          ├── alignment
                          ├── diarization refinement
                          ├── punctuation/formatting
                          └── correction
```

Do not destroy L1 when Q1 exists. Mark the active materialized transcript while retaining revision provenance.

## Model runtime isolation

Some models/runtime libraries are high-risk/native/GPU-heavy. Support two execution modes:

- in-process when mature and safe enough;
- isolated local worker with a narrow IPC contract when crash/leak/resource isolation is valuable.

A model crash must not corrupt the vault or lose durable audio.

## Model registry

Each model package/manifest records:

```text
id
version
source URI
license identifier/text reference
weights digest
engine requirements
supported languages
task types
architectures
accelerators
RAM/VRAM/disk estimate
quality tier
signature/trust source
```

A code license never implies a compatible weight license.

Offline/air-gapped installations need an export/import model-bundle path.

## Document architecture

### Himsat Document IR

Parser adapters output a Himsat-owned IR.

Candidate object graph:

```text
Document
  -> Section/Page/Slide/Sheet
      -> Block
          -> Heading
          -> Paragraph
          -> List
          -> Table -> Row -> Cell
          -> Figure/Image/Chart
          -> Formula
          -> Code
          -> Annotation
```

Every object can carry:

- source digest;
- original file/page/region;
- bounding box;
- text/extracted representation;
- OCR/parser confidence;
- parser/model identity;
- relationships;
- embedded asset references.

This IR becomes the bridge between parsers, search, memory, citations, and publishing.

### Untrusted document boundary

Importers must protect against:

- zip/archive bombs;
- path traversal;
- malicious embedded objects;
- parser crashes;
- oversized images/pages;
- recursive archives;
- huge spreadsheets;
- malformed PDFs/OOXML;
- external links/resources;
- prompt injection embedded in content.

Document text is data, never agent instruction authority.

## Search architecture

Use layered retrieval:

```text
query
  -> scope filter
  -> FTS/lexical candidates
  -> semantic/vector candidates
  -> graph/temporal candidates
  -> local reranker
  -> evidence resolver
  -> answer synthesis
```

Potential components:

- SQLite/SQLCipher for canonical structured data;
- FTS5 for lexical retrieval;
- sqlite-vec behind a Himsat vector interface while it is pre-v1;
- fastembed-rs or another local embedding adapter;
- optional Tantivy only after scale measurements justify a second index.

Indexes are rebuildable derived state, not canonical truth.

## Memory architecture

Materialized memory objects are derived from evidence and revisioned.

A memory fact should contain:

```text
fact_id
fact_type
value
valid_from / valid_to where applicable
confidence
source evidence refs
extractor/model/template identity
supersedes / contradicted_by links
human correction state
```

Temporal queries reconstruct source-backed historical state rather than overwriting old decisions.

## Evidence engine

`EvidenceRef` must be source-type aware.

Examples:

- audio range;
- transcript segment/word range;
- document page + bounding region;
- screen frame + region;
- manual note;
- imported artifact hash.

Claims contain evidence references and generation provenance.

No evidence resolver should turn a semantic similarity hit into a statement that the source literally asserted the answer.

## Intelligence architecture

The local intelligence layer should use a structured pipeline rather than one giant summary prompt:

```text
normalized evidence
  -> entity/topic extraction
  -> candidate decisions/commitments/questions/risks
  -> evidence validation
  -> temporal reconciliation
  -> view/template generation
```

LLM-generated candidates can be probabilistic; acceptance into durable memory remains schema-validated and evidence-linked.

## Local inference

Current candidate direction:

- mistral.rs as a Rust-native local inference candidate;
- llama.cpp as a broad GGUF/runtime compatibility candidate;
- platform-native runtimes when they materially improve device performance;
- small specialized ONNX/Candle-style models for embeddings/classifiers where justified.

No runtime is canonical until benchmarked on the target device matrix.

## Publishing architecture

Create a semantic `PublishDocument` IR independent of output format.

```text
Memory/Document evidence
      -> Transform template
      -> PublishDocument
          ├── Typst -> PDF/PDF-A/PDF-UA target
          ├── docx-rs -> DOCX
          ├── renderer -> Markdown/HTML
          └── subtitle renderer -> VTT/SRT
```

Human publishing must preserve semantics/accessibility rather than encoding everything as positioned text.

## Portable AI Context Pack

A separate export path builds a deterministic machine-oriented bundle:

```text
manifest.json
content.md
evidence.jsonl
entities.jsonl
relations.jsonl
assets/
```

The manifest records schema version, source set, source digests, generation time, included/excluded surfaces, ordering rules, and optional token-budget profile.

The pack is not an opaque archive required by Himsat. Each essential textual file remains directly readable.

## Himsat Bridge and sync

### Bridge

Low-latency local session protocol for live source contribution/control.

Requirements:

- mutual authentication;
- explicit pairing/session approval;
- clock sync;
- stream backpressure;
- encryption;
- source provenance;
- reconnect;
- direct/LAN preference.

### Durable sync

Separate from live Bridge. Candidate direction uses content-addressed encrypted blobs and local replica metadata. Iroh is a candidate transport, but Himsat must control relay policy:

- strict LAN/direct mode;
- user-approved public relay mode if ever shipped;
- self-hosted relay option;
- no silent public relay under Network Lock.

## Vault and crypto architecture

High-level requirements:

- encrypted structured store;
- encrypted media/asset blobs;
- OS secure key storage when available;
- optional passphrase-derived recovery key;
- key separation per vault/space as justified;
- authenticated encryption;
- zeroization of sensitive transient key material where practical;
- explicit backup/recovery design;
- no connector ever receives plaintext vault master keys.

The exact algorithm/scheme requires a dedicated cryptographic design/review Grain; this planning document intentionally does not invent a final construction.

## Capability and network architecture

Core crates should not receive unrestricted networking by default.

Capability classes may include:

```text
vault.read
vault.write
recording.control
raw_audio.read
transcript.read
document.read
speaker_biometrics.read
network.connector:<id>
network.sync
filesystem.export
external.write:<connector>
```

A central policy broker evaluates actor + resource + action + scope.

## Plugin architecture

Preferred direction:

```text
third-party plugin WASM
        │
        ▼
Himsat plugin host
        │
        ├── typed host functions
        ├── resource/time/memory limits
        ├── explicit network grants
        └── explicit vault grants
```

Extism/Wasmtime are donor/dependency candidates. Do not expose arbitrary shell execution as a general plugin primitive.

## Platform adapter boundaries

### macOS

Research/implementation surface:

- CoreAudio / native audio APIs;
- ScreenCaptureKit for authorized system/screen audio/content;
- menu bar/background lifecycle;
- Keychain;
- Metal/CoreML.

### Windows

- WASAPI;
- Windows Graphics Capture or appropriate native capture surface;
- tray/background lifecycle;
- Windows secure credential/DPAPI-class key protection;
- CUDA/Vulkan/DirectML options where engines support them.

### Linux

- PipeWire first for modern audio/screen pathways;
- PulseAudio/ALSA fallback only when required;
- Wayland/X11 boundaries;
- Secret Service or explicit secure fallback;
- CUDA/Vulkan/CPU runtimes.

### iOS/iPadOS/watchOS

- Swift/SwiftUI;
- AVAudioSession/AVAudioEngine;
- background audio recording modes;
- Live Activities / App Intents / AudioRecordingIntent where supported;
- Keychain/Secure Enclave appropriate use;
- version-gated ScreenCaptureKit capabilities;
- watch companion/independent capture paths.

### Android

- Kotlin/Compose;
- AudioRecord;
- microphone foreground service;
- MediaProjection/AudioPlaybackCapture where allowed;
- notification/quick actions;
- Android Keystore;
- WorkManager for deferred non-recording jobs;
- vendor/device audio behavior qualification.

## UI architecture principles

- one obvious recording state;
- capture health visible without opening diagnostics;
- evidence jump is a primary interaction;
- local/offline/network state visible;
- sensitive capability requests are contextual;
- no modal maze before one-tap record;
- keyboard/accessibility semantics built with each surface;
- Arabic/RTL layouts tested continuously, not patched at release.

## Architecture decision gates still required

The following are intentionally unresolved until dedicated specs:

- final Himsat-owned license;
- exact encryption/key hierarchy;
- exact event persistence format;
- exact desktop frontend state framework;
- exact Swift↔Rust and Kotlin↔Rust FFI mechanism;
- primary mobile STT model/runtime per device class;
- vector-store extension choice under long-term compatibility constraints;
- P2P sync/CRDT/event reconciliation details;
- plugin host/runtime choice;
- document parser selection/combination;
- final update/signing/reproducible-build distribution system.

These unknowns are tracked as future bounded decisions, not silently assumed.
