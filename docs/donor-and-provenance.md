# Himsat Donor and Provenance Plan

## Purpose

Himsat will learn aggressively from strong open-source systems while keeping the codebase legally clean, independently testable, and architecturally coherent.

The rule is simple:

> **Public source is not enough. Exact path + exact revision + exact license + Himsat reason + Himsat tests are required before adoption.**

## Donor classes

### `COPY`

Use selected source files or closely adapt implementation after provenance review.

Use when:

- license is compatible;
- code materially reduces risk/time;
- Himsat can isolate the imported behavior behind its own contract;
- copying is better than a dependency because Himsat requires portability, patching, or tight platform integration.

### `DEPEND`

Use an upstream crate/library/package at a pinned/reproducible version.

Use when:

- upstream is mature;
- dependency boundary is stable;
- Himsat does not need to own the internals;
- security/update cost is lower than vendoring.

### `VENDOR`

Store an immutable source snapshot in-tree or in a controlled build artifact.

Use only when:

- deterministic offline build/distribution requires it;
- upstream packaging is unsuitable;
- license and notices permit it;
- update ownership is explicitly accepted.

### `REFERENCE`

Study behavior/architecture/research but write Himsat's implementation independently.

Mandatory for:

- proprietary products;
- source-available/custom-restricted projects by default;
- incompatible copyleft material unless governance explicitly decides otherwise;
- projects whose code license/model license cannot safely enter the permissive core.

## Default license posture

### Generally acceptable candidates

Subject to exact-path review:

- MIT;
- Apache-2.0;
- BSD-2-Clause;
- BSD-3-Clause;
- ISC;
- Zlib;
- public-domain/CC0 code/data where provenance is clear.

### Manual review

- MPL-2.0;
- LGPL and linking questions;
- dual/multi-license packages;
- code with generated/vendor subtrees under different terms;
- academic datasets and model weights;
- fonts/assets with separate redistribution requirements.

### `REFERENCE_ONLY` by default

- GPL/AGPL/SSPL;
- non-commercial licenses;
- source-available licenses;
- custom licenses with field-of-use/revenue/competition restrictions;
- unknown/no-license material.

A future legal/governance decision may alter this, but no coding agent may assume permission.

## Provenance record requirements

Every copied/closely adapted donor entry records:

```text
name
source_repository
source_revision
tag/release if applicable
source_path(s)
source_license
model/data/asset license if separate
copyright/notice requirements
destination_path(s)
adoption_mode = copy | depend | vendor | reference
adaptation_description
why_reuse_beats_native
Himsat behavior tests
reviewer confirmation
```

Records should live under `governance/provenance/` once implementation starts.

## Current candidate registry

This is a research registry, not adoption authority. Exact immutable revisions must be pinned in the implementation Grain that first consumes a donor.

| Source | Current classification | Candidate use | Important constraint |
| --- | --- | --- | --- |
| `Zackriya-Solutions/meetily` | COPY selective | desktop audio, recording, Whisper/Parakeet/Tauri patterns | Community MIT; do not assume separate Pro code is included |
| `fastrepl/anarlog` | COPY selective / REFERENCE | modular Rust crates, local session/storage, mobile/watch/CLI/MCP patterns | `enterprise/**` is commercial; exact path license controls |
| `ggml-org/whisper.cpp` | DEPEND/VENDOR candidate | portable Whisper backend | preserve license/notices; model licenses separately |
| `k2-fsa/sherpa-onnx` | DEPEND candidate | mobile/offline ASR/VAD/diarization/speaker features | model licenses separately |
| `argmaxinc/argmax-oss-swift` | DEPEND/COPY selective | Apple-native speech/diarization/TTS acceleration | MIT code + third-party notices; weights separately |
| `moonshine-ai/moonshine` | EXPERIMENT/DEPEND | low-latency edge speech | code MIT; legacy non-English model exceptions require per-model review |
| `dignifiedquire/sonora` | DEPEND/COPY selective | pure-Rust AEC/NS/AGC/audio DSP | BSD-3-Clause notices; benchmark maturity before defaulting |
| `tonarino/webrtc-audio-processing` | DEPEND alternative | battle-tested WebRTC AudioProcessing wrapper | C++/system build complexity; BSD-3-Clause wrapper |
| `thewh1teagle/vibe` / related local engine work | REFERENCE/COPY selective | media import, model lifecycle, transcription job isolation | exact current repository/license/path must be reverified before adoption |
| `EricLBuehler/mistral.rs` | DEPEND candidate | Rust-native local LLM inference | benchmark device/model support before choosing as primary |
| `ggml-org/llama.cpp` | DEPEND/VENDOR candidate | broad GGUF local inference compatibility | model licenses separate |
| `Anush008/fastembed-rs` | DEPEND candidate | local embeddings/reranking | Apache-2.0; model licenses/cache integrity separately |
| `sqlcipher/sqlcipher` | DEPEND/VENDOR candidate | encrypted SQLite | BSD-style community license; integration/export-control/security review |
| `asg017/sqlite-vec` | DEPEND candidate behind abstraction | local vector search | pre-v1 API stability; MIT/Apache-2.0 |
| `n0-computer/iroh` | DEPEND candidate | authenticated P2P/QUIC transport | public relay behavior must not violate Network Lock; MIT/Apache-2.0 |
| `extism/extism` | DEPEND candidate | Wasm plugin host/capability sandbox | BSD-3-Clause |
| `bytecodealliance/wasmtime` | DEPEND alternative | lower-level Wasm runtime | Apache-2.0 |
| `typst/typst` | DEPEND candidate | professional PDF/PDF-A/PDF-UA publishing | verify redistribution/fonts/templates separately |
| `bokuweb/docx-rs` | DEPEND candidate | semantic DOCX generation/parsing | MIT; OOXML coverage incomplete, test target templates |
| `tokimo-lab/tokimo-package-fileparser` | EXPERIMENT/DEPEND/COPY candidate | pure-Rust PDF/Office to Markdown/assets | MIT OR Apache-2.0; maturity/security fuzzing required |
| `microsoft/markitdown` | REFERENCE/COPY selective | conversion UX/Markdown normalization/security guidance | MIT; Python runtime not preferred for core |
| `docling-project/docling` / `DS4SD/docling-core` | REFERENCE/optional worker | document IR/layout/table/formula quality | MIT code, individual models separate; Python-heavy |
| `xberg-io/xberg` (formerly `kreuzberg-dev/kreuzberg`) | EXPERIMENT/DEPEND/COPY candidate | broad Rust-first document extraction/IR/parser architecture | Xberg v1 workspace declares MIT; exact third-party, vendored, model, asset, and plugin-path licenses still control |
| `kreuzberg-dev/kreuzberg-lts` | HISTORICAL_CANDIDATE | legacy v4 document extraction research | current LTS line is MIT; earlier Kreuzberg 4.8/4.9 releases used ELv2; exact snapshot/security horizon required |
| `opendatalab/MinerU` | REFERENCE_ONLY by default | document quality/IR/benchmark ideas | custom license with additional conditions |
| Marker variants | REFERENCE_ONLY by default | document parsing quality ideas | model/commercial/copyleft restrictions |
| current Screenpipe | REFERENCE_ONLY by default | screen/context architecture ideas | current licensing restricts competing-product reuse; historical snapshots require exact review |

## Proprietary behavioral references

The following can inform product behavior but are **not code donors**:

- Circleback;
- Otter.ai;
- Granola;
- Fireflies;
- Fathom;
- Krisp;
- Plaud;
- Limitless and similar personal-memory products;
- operating-system first-party apps and APIs.

When an idea is generic, cite the product/research influence in architecture/research notes if useful, but do not create false code provenance.

## Meetily adoption boundary

Meetily is the likely first historical donor because the user has permission and the Community code is MIT. Still, the Himsat adoption must be selective.

Likely useful surfaces:

- device discovery;
- mic/system audio capture;
- audio pipeline/mixing;
- VAD;
- recording manager/journaling ideas;
- Whisper/Parakeet integration;
- Tauri platform patterns;
- tests/fixtures that are legally reusable.

Do not blindly inherit:

- archived Python/FastAPI backend;
- telemetry/analytics integrations;
- cloud-provider-first summary configuration;
- obsolete backup files;
- branding/assets without asset-level review;
- architectural coupling that prevents a shared Rust core;
- any Pro/Enterprise code that is not actually present under compatible terms.

The desired migration shape is:

```text
Meetily history/review
   -> exact subsystem provenance
   -> Himsat behavior contract
   -> selective transplant or dependency
   -> Himsat tests
   -> Himsat architecture becomes authority
```

## Anarlog adoption boundary

Anarlog is especially useful because its community layer already spans desktop/mobile/watch/CLI/MCP plus Rust crates.

Rules:

- never copy from `enterprise/**` under the current permissive-core plan;
- confirm no community path imports/generated content from enterprise;
- preserve MIT notices;
- prefer extracting behavior/contracts instead of importing the entire monorepo;
- compare equivalent Meetily/Anarlog implementations and choose the smaller/better-tested Himsat fit instead of maintaining duplicate subsystems.

## Xberg / historical Kreuzberg boundary

Live GitHub truth on 2026-09-06 resolves the former `kreuzberg-dev/kreuzberg` repository to `xberg-io/xberg`. The current Xberg v1 workspace declares `license = "MIT"`; its changelog explicitly distinguishes this from the Kreuzberg 4.8/4.9 ELv2 line.

This makes current Xberg a permissive donor/dependency candidate rather than `REFERENCE_ONLY` solely because of the historical Kreuzberg license. It does **not** grant blanket adoption authority: Himsat must still inspect the exact Xberg revision and source path, including bundled third-party code, plugins, models, fonts, test corpora, native libraries, generated artifacts, and other assets.

The separate `kreuzberg-dev/kreuzberg-lts` v4 line is also a historical candidate; exact release-era licensing and security support must be pinned before use.

## Historical-license projects

When a project changed from permissive to restrictive licensing, Himsat may only consider an older permissive snapshot if all of the following hold:

1. exact commit/tag predates the license change;
2. controlling license at that exact path is recorded;
3. later restricted code is not accidentally cherry-picked;
4. security/bug fixes after the snapshot are independently evaluated rather than copied blindly;
5. notices and trademarks are handled;
6. a native current alternative is not lower risk.

This applies especially to Screenpipe and other projects whose current license is more restrictive than an older permissive snapshot. Historical Kreuzberg releases require exact per-release treatment because the licensing lineage includes both ELv2 and MIT eras; current Xberg v1 must be evaluated separately under its present MIT workspace license.

## Model and dataset policy

Models are independent supply-chain objects.

Before adding a model to the registry:

- exact model repository/version/revision;
- weights digest;
- license;
- training/use restrictions known from published terms;
- supported tasks/languages;
- redistribution right;
- commercial right if the project may be used commercially;
- attribution;
- hardware/runtime requirements;
- benchmark evidence;
- download/sideload integrity.

Never infer model permission from the engine's software license.

## Asset/font/template policy

Professional PDF/DOCX generation creates a hidden licensing surface.

Track:

- fonts and embedding rights;
- icons;
- illustrations;
- screenshots/sample docs;
- PDF/report templates;
- test corpora;
- sample audio/video;
- logos/trademarks.

Use generated/synthetic fixtures where possible for test privacy and redistribution clarity.

## Dependency restraint

A donor dependency must justify:

- capability gained;
- binary/runtime size;
- native/system dependencies;
- platform coverage;
- security history/maintenance;
- offline behavior;
- license;
- update strategy;
- failure isolation;
- whether an abstraction can make future replacement possible.

No dependency is added because “the donor uses it.”

## Provenance automation roadmap

The first implementation-era governance units should create:

- `governance/donors.toml` or equivalent machine-readable registry;
- `governance/provenance/*.toml|json` records;
- license allow/deny/manual-review policy;
- generated `THIRD_PARTY_NOTICES` support;
- model-license manifest validation;
- SBOM generation;
- CI failure on unknown copied/vendor material where practical;
- source digest verification for vendored snapshots.

## Donor merge rule

No donor adoption is complete until:

- provenance record exists;
- exact source revision is immutable;
- license is verified;
- required notices are present;
- destination code follows Himsat architecture;
- focused Himsat behavior tests pass;
- full applicable regression passes;
- Diffcipline proof is based on the exact diff;
- security/platform-specific tests appropriate to the blast radius pass.
