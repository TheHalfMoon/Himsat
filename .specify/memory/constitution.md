# Himsat Constitution

## I. User sovereignty

The user owns the captured media, transcripts, notes, documents, memory, models, exports, and cryptographic keys. Core Himsat behavior must not depend on a Himsat-hosted account or mandatory Himsat cloud service.

## II. Local-first means architectural, not cosmetic

Capture, persistence, transcription, search, evidence resolution, memory, and the default intelligence path must be able to run locally. Networking is an explicit capability owned by narrow connector, sync, update, or optional extension surfaces. No hidden remote fallback is permitted.

## III. Visible, permission-respecting capture

Himsat is not a stealth-recorder project. It must respect operating-system microphone/screen indicators, permission prompts, protected-content controls, and app-store/platform rules. User-started background recording is a core goal; covert recording and privacy-indicator bypass are not.

## IV. Evidence before narrative

When Himsat produces a factual claim from meetings, media, or documents, the system should preserve and expose the source evidence: transcript/audio timestamp, document page/region, screen frame, user note, or other source object. If evidence is insufficient, the product must be able to say so instead of manufacturing certainty.

## V. Provenance is product integrity

Every non-trivial copied or closely adapted donor artifact requires source, immutable revision, source path, controlling license, destination path, adaptation description, and notice handling. Code, model weights, datasets, icons, fonts, sample media, and templates are reviewed independently. Public source visibility is not a license grant.

## VI. Rust-first portable core; native where the OS demands it

Correctness-sensitive shared behavior should live in a portable Rust core when practical. Platform-native shells and services are first-class rather than second-class wrappers:

- desktop: Tauri/Rust plus web UI where appropriate;
- Apple platforms: Swift/SwiftUI for OS integration, with Rust core reuse;
- Android: Kotlin/Compose for OS integration, with Rust core reuse;
- watch/wearable surfaces: native UI and lifecycle integration.

A language choice is subordinate to reliability, OS capability, accessibility, and maintainability.

## VII. Capture reliability outranks feature count

A six-hour recording that disappears is a failed product regardless of AI quality. Long-session survival, crash recovery, route changes, interruptions, sleep/wake, thermal pressure, low battery, low storage, and database integrity are release-critical engineering surfaces.

## VIII. Portable memory and open interchange

Himsat must provide documented, non-proprietary export paths. Human publishing formats and machine/LLM context formats are separate concerns. At minimum, the architecture must support high-quality human documents and simple semantic exports such as Markdown plus structured evidence/manifest data.

## IX. Agents receive capabilities, not ambient authority

Local MCP/API/CLI/plugin/automation surfaces must be capability-scoped. Reading a meeting does not imply reading raw audio or speaker biometrics. Reading does not imply writing. External writes normally require an explicit approval or an explicitly configured policy. Plugin execution must be sandboxed or otherwise constrained proportional to risk.

## X. Accessibility and internationalization are core quality

Live captions, keyboard access, screen-reader compatibility, contrast, semantic document structure, RTL layouts, and accessible publishing are not optional polish. Arabic quality, Arabic-English code switching, and RTL publishing are first-class benchmarks; broad multilingual support remains a product goal.

## XI. Platform truth over universal promises

Capabilities are versioned and device-specific. Himsat must maintain a capability matrix and runtime detection. A feature available on desktop or a future OS beta must not be marketed as universally available on all phones. Unsupported capture paths must fail clearly and safely.

## XII. Proof before done

Himsat adopts SpecGrain and Diffcipline principles:

- work is recursively refined until bounded;
- acceptance and evidence requirements precede implementation;
- dependencies are explicit;
- risk changes verification rigor;
- exact repository state outranks agent narrative;
- `NOT RUN` is never `PASS`;
- benchmark/superiority claims require reproducible evidence;
- negative results remain part of the evidence record.

No document, agent output, or checklist alone can establish completion.

## XIII. Security defaults must survive offline operation

Encryption, key management, access controls, model integrity, plugin integrity, and local auditability must not require a network connection. Optional cloud-storage connectors store only what the user authorizes, and encrypted user-owned backup/sync should be supported without giving the storage provider Himsat vault keys.

## XIV. Minimize surveillance surfaces

Features that can become invasive require explicit opt-in, narrow scope, and visible state. Screen context should prefer selected/share-screen content over blanket capture. Speaker identity is opt-in. Retention is user-controlled. Emotion/personality inference and hidden biometric profiling are outside the default product direction.

## XV. Best is a benchmark target, not a slogan

Himsat may aspire to be the best product in its category, but repository claims must be precise. Speech accuracy, diarization, latency, retrieval, battery use, document extraction, evidence coverage, crash recovery, privacy, and publishing quality each require independent measurement. No single metric proves universal superiority.
