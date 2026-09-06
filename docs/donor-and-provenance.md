# Himsat Donor and Provenance Plan

## Purpose

Himsat will learn aggressively from strong external systems and reuse strong implementations when that is the technically superior path, while keeping the codebase legally clean, independently testable, and architecturally coherent.

The rule is:

> **Permission makes reuse eligible; exact path + exact revision + exact controlling terms + Himsat reason + Himsat tests make reuse adoptable.**

## Founder source-use authorization

The founder/user has explicitly stated that Himsat has permission to use the code/material from every external source referenced anywhere in the repository as of canonical snapshot `97535af240aabf130153fce7718f9ca2eb3f85bf`.

The durable authorization record is:

```text
governance/provenance/source-use-authorization.md
```

This supersedes earlier planning assumptions that a covered source had to remain `REFERENCE_ONLY` solely because no additional permission was known.

It does **not** waive exact provenance, notice, embedded-third-party, model/data/asset, security, testing, or active-spec authority requirements. A source may be permission-eligible and still be unsuitable for a particular Himsat subsystem.

## Donor classes

### `COPY`

Use selected source files or closely adapt implementation after provenance review.

Use when:

- Himsat has compatible public-license rights or an exact permission grant covering the intended use;
- code materially reduces risk/time;
- Himsat can isolate the imported behavior behind its own contract;
- copying is better than a dependency because Himsat requires portability, patching, deterministic ownership, or tight platform integration.

### `ADAPT`

Use a donor implementation as a close implementation basis while reshaping it to Himsat-owned interfaces, invariants, tests, and lifecycle rules.

Use when the donor solves the hard platform/algorithmic part but importing its architecture wholesale would be worse than a bounded transplant.

### `DEPEND`

Use an upstream crate/library/package at a pinned/reproducible version.

Use when:

- upstream is mature;
- dependency boundary is stable;
- Himsat does not need to own the internals;
- security/update cost is lower than vendoring;
- provenance and native/transitive closure can be qualified.

### `VENDOR`

Store an immutable source snapshot in-tree or in a controlled build artifact.

Use only when:

- deterministic offline build/distribution requires it;
- upstream packaging is unsuitable;
- controlling rights/notices permit it;
- update ownership is explicitly accepted.

### `REFERENCE`

Study behavior/architecture/research but write Himsat's implementation independently.

Use when:

- source bytes are not actually available;
- copying would import unnecessary architecture or risk;
- the relevant permission scope cannot yet be pinned;
- a source is useful mainly as competitive/product/research evidence;
- a smaller Himsat-native implementation is clearly superior.

`REFERENCE` is therefore an engineering/provenance choice, not the automatic status of a covered source merely because its ordinary public license is restrictive.

## Permission and license posture

### Generally straightforward public-license candidates

Subject to exact-path review:

- MIT;
- Apache-2.0;
- BSD-2-Clause;
- BSD-3-Clause;
- ISC;
- Zlib;
- public-domain/CC0 code/data where provenance is clear.

For these sources, prefer the durable public license as the adoption basis even when founder permission also exists.

### Exact-scope review required

- MPL-2.0;
- LGPL and linking questions;
- GPL/AGPL/SSPL;
- dual/multi-license packages;
- source-available/custom licenses;
- non-commercial or field-of-use licenses;
- unknown/no-license material;
- code with generated/vendor subtrees under different terms;
- academic datasets and model weights;
- fonts/assets with separate redistribution requirements;
- any source relying on the founder permission rather than a compatible public license.

A covered source in this group is **not automatically forbidden**. Before adoption, the provenance record must identify the exact permission scope needed for modification, redistribution, sublicensing, commercial distribution, and any other intended Himsat use.

If that exact permission cannot be demonstrated for a specific path or embedded component, that path remains `REFERENCE`/excluded until separately cleared.

## Provenance record requirements

Every copied/closely adapted/vendor/dependency donor entry records, as applicable:

```text
name
source_repository
source_revision
tag/release if applicable
source_path(s)
source_license/public terms
separate_permission_basis if relied upon
permission_scope if relied upon
model/data/asset license if separate
copyright/notice requirements
destination_path(s)
adoption_mode = copy | adapt | depend | vendor | reference
adaptation_description
why_reuse_beats_native
Himsat behavior tests
reviewer confirmation
```

Machine adoption records live under `governance/provenance/` and must match the exact canonical bytes/lock/native closure.

## Current candidate registry

This is a research/planning registry, not adoption authority. Founder source-use permission is recorded globally for all sources already referenced in the repository; exact immutable revisions and exact permission/license scope must still be pinned in the implementation leaf that consumes material.

| Source | Current engineering classification | Candidate use | Important constraint |
| --- | --- | --- | --- |
| `Zackriya-Solutions/meetily` | COPY/ADAPT selective | desktop audio, recording, Whisper/Parakeet/Tauri patterns | Community MIT is straightforward; separately governed paths still require exact-path evidence |
| `fastrepl/anarlog` | COPY/ADAPT selective | modular Rust crates, local session/storage, mobile/watch/CLI/MCP patterns | Community paths are easier; `enterprise/**` may be evaluated under the recorded permission only after exact permission scope and embedded-third-party provenance are proven |
| `ggml-org/whisper.cpp` | DEPEND/VENDOR/COPY candidate | portable Whisper backend | preserve exact license/notices; model licenses separately |
| `k2-fsa/sherpa-onnx` | DEPEND/COPY candidate | mobile/offline ASR/VAD/diarization/speaker features | model licenses separately |
| `argmaxinc/argmax-oss-swift` | DEPEND/COPY/ADAPT selective | Apple-native speech/diarization/TTS acceleration | code + third-party notices; weights separately |
| `moonshine-ai/moonshine` | EXPERIMENT/DEPEND/COPY | low-latency edge speech | exact model and language rights separately |
| `dignifiedquire/sonora` | DEPEND/COPY selective | pure-Rust AEC/NS/AGC/audio DSP | benchmark maturity before defaulting |
| `tonarino/webrtc-audio-processing` | DEPEND/COPY alternative | battle-tested WebRTC AudioProcessing wrapper | C++/system build complexity and native closure |
| `thewh1teagle/vibe` / related local engine work | REFERENCE/COPY/ADAPT selective | media import, model lifecycle, transcription job isolation | exact current repository/revision/path must be reverified before adoption |
| `EricLBuehler/mistral.rs` | DEPEND/COPY candidate | Rust-native local LLM inference | benchmark device/model support before choosing as primary |
| `ggml-org/llama.cpp` | DEPEND/VENDOR/COPY candidate | broad GGUF local inference compatibility | model licenses separate |
| `Anush008/fastembed-rs` | DEPEND/COPY candidate | local embeddings/reranking | model licenses/cache integrity separately |
| `sqlcipher/sqlcipher` | DEPEND/VENDOR/COPY candidate | encrypted SQLite | exact SQLCipher/native provider identity and notices remain mandatory |
| `asg017/sqlite-vec` | DEPEND/COPY candidate behind abstraction | local vector search | pre-v1 API stability and reproducible package identity |
| `n0-computer/iroh` | DEPEND/COPY candidate | authenticated P2P/QUIC transport | public relay behavior must not violate Network Lock |
| `extism/extism` | DEPEND/COPY candidate | Wasm plugin host/capability sandbox | sandbox boundary still requires Himsat qualification |
| `bytecodealliance/wasmtime` | DEPEND/COPY alternative | lower-level Wasm runtime | large security/update surface; exact feature closure required |
| `typst/typst` | DEPEND/COPY candidate | professional PDF/PDF-A/PDF-UA publishing | fonts/templates/assets separately |
| `bokuweb/docx-rs` | DEPEND/COPY candidate | semantic DOCX generation/parsing | OOXML coverage incomplete; test target templates |
| `tokimo-lab/tokimo-package-fileparser` | EXPERIMENT/DEPEND/COPY candidate | pure-Rust PDF/Office to Markdown/assets | maturity/security fuzzing required |
| `microsoft/markitdown` | REFERENCE/COPY/ADAPT selective | conversion UX/Markdown normalization/security guidance | Python runtime not preferred for core; reuse only where it improves the architecture |
| `docling-project/docling` / `DS4SD/docling-core` | REFERENCE/worker/COPY selective | document IR/layout/table/formula quality | Python-heavy; individual models/assets separately |
| `xberg-io/xberg` | EXPERIMENT/DEPEND/COPY/ADAPT candidate | broad Rust-first document extraction/IR/parser architecture | exact third-party, vendored, model, asset, and plugin-path provenance still controls |
| `kreuzberg-dev/kreuzberg-lts` | HISTORICAL/COPY candidate | legacy v4 document extraction research/implementation | pin exact historical snapshot and security horizon; do not mix eras silently |
| `opendatalab/MinerU` | REFERENCE/COPY candidate under exact permission | document quality/IR/parser ideas or selective implementation | public custom terms do not by themselves govern a separately permitted transplant; exact permission scope/models/assets still required |
| Marker variants | REFERENCE/COPY candidate under exact permission | document parsing quality or selective implementation | exact code/model/asset rights must be separated |
| current Screenpipe | REFERENCE/COPY candidate under exact permission | screen/context architecture or selective implementation | current public restrictions still matter unless exact separate permission scope for the selected paths is recorded |
| `Starmel/OpenSuperWhisper` | COPY/ADAPT/REFERENCE candidate | macOS dictation, global controls, recorder state, engine abstraction, file queue, model UX | reviewed top-level MIT source is promising; submodules/packages/models must be independently closed |
| Superwhisper product/docs | PRODUCT_REFERENCE; source donor only if source material is actually available | system-wide dictation UX, modes, context, vocabulary, routing, parity | permission does not create unavailable source bytes; product behavior may be used as design input now |

The registry is intentionally not exhaustive of every nested dependency. Any additional source already referenced elsewhere in the repository is covered by the founder authorization record but must be added/pinned here or in machine provenance when it becomes implementation-relevant.

## Proprietary/source-unavailable behavioral references

Circleback, Otter.ai, Granola, Fireflies, Fathom, Krisp, Plaud, Limitless, Superwhisper, operating-system first-party apps/APIs, and similar product references may inform Himsat behavior.

The founder authorization means these are not barred from source reuse on permission grounds **if source material is actually made available to Himsat**. Until then, they remain product/behavior references because no immutable source bytes exist for Himsat to provenance-pin.

Do not create false code provenance from product observation.

## Meetily adoption boundary

Meetily remains a strong historical donor. Adoption should be selective rather than a monorepo transplant.

Likely useful surfaces:

- device discovery;
- mic/system audio capture;
- audio pipeline/mixing;
- VAD;
- recording manager/journaling ideas;
- Whisper/Parakeet integration;
- Tauri platform patterns;
- tests/fixtures that are rights-cleared and useful.

Do not blindly inherit:

- archived Python/FastAPI backend;
- telemetry/analytics integrations;
- cloud-provider-first summary configuration;
- obsolete backup files;
- branding/assets without asset-level review;
- architectural coupling that prevents a shared Rust core;
- generated/vendor material that is not separately qualified.

The desired migration shape is:

```text
Meetily history/review
   -> exact subsystem provenance
   -> Himsat behavior contract
   -> selective transplant/dependency/adaptation
   -> Himsat tests
   -> Himsat architecture becomes authority
```

## Anarlog adoption boundary

Anarlog is especially useful because its community layer spans desktop/mobile/watch/CLI/MCP plus Rust crates.

Rules:

- community code should be preferred when it solves the need cleanly;
- `enterprise/**` is no longer categorically excluded solely because earlier planning assumed no extra permission; if an enterprise path is useful, freeze the exact path and record the permission scope before copying it;
- confirm selected paths do not import/generated-copy unrelated third-party material outside the permission scope;
- preserve public MIT notices where applicable;
- prefer extracting behavior/contracts over importing the entire monorepo;
- compare equivalent Meetily/Anarlog/other-source implementations and choose the smaller, safer, better-tested Himsat fit instead of maintaining duplicates.

## Xberg / historical Kreuzberg boundary

Live GitHub truth on 2026-09-06 resolved the former `kreuzberg-dev/kreuzberg` repository to `xberg-io/xberg`. Current Xberg v1 workspace licensing and any historical Kreuzberg/Kreuzberg-LTS snapshot must still be treated as distinct source identities.

Founder permission means Himsat may evaluate any covered exact snapshot for COPY/ADAPT/DEPEND/VENDOR use rather than rejecting it solely because an era used ELv2 or another restrictive public license. It does not allow Himsat to mix code from different eras without recording each controlling source identity and permission/license basis.

Third-party/vendor/model/asset material inside any snapshot remains separately qualified.

## Historical-license projects

When a project changes licensing, Himsat may consider any covered snapshot for which the project has sufficient rights. The adoption record must still prove:

1. exact commit/tag and source path;
2. public license at that exact path/era;
3. separate permission basis/scope if the public license is insufficient for intended Himsat use;
4. no accidental mixing with differently governed later/earlier code;
5. security/bug-fix delta against current upstream;
6. notices/trademarks/embedded third-party obligations;
7. why that snapshot is better than a current lower-risk alternative.

## Superwhisper/OpenSuperWhisper roadmap integration

The research record `docs/research/2026-09-07-superwhisper-reference.md` is now an explicit planning input rather than an isolated note.

Its useful concepts map into the existing master-plan structure as follows:

```text
011 Model registry + speech-engine contract
  -> Voice Model Router
     - execution locality
     - language/streaming/diarization capabilities
     - model/license/digest/resource identity
     - Network Lock compatibility

012 Live transcription
  -> dictation-grade low-latency transcript/revision path

015 Desktop background runtime + meeting detection
  -> Input Control Layer
     - global shortcuts
     - single-modifier / push-to-talk state
     - mouse controls where supported
     - tray/menu-bar controls
     - explicit recording indicator

015 + 011 + 012
  -> Universal Dictation Surface candidate
     - system-wide voice-to-text
     - deterministic target insertion/fallback
     - separate retention/consent semantics from meetings

003 + 011 + 012 + 024/025
  -> Terminology System candidate
     - probabilistic vocabulary hints
     - deterministic replacement rules
     - project/mode scopes
     - testable/reversible behavior

031 capability broker foundations + dictation surface
  -> Mode Engine + Context Capability Broker candidate
     - versioned declarative profiles
     - per-capability context grants
     - app/site activation rules
     - local/cloud route policy
     - no hidden ambient context

019 media import
  -> file transcription queue/recovery UX patterns
```

These mappings are roadmap shaping guidance only. They do not renumber the canonical master-plan units or grant current feature implementation authority while Specification 004 is active.

When these frontiers become active, OpenSuperWhisper and every other relevant covered source should be re-compared live. Himsat should reuse the best bounded implementation rather than reflexively rewriting working code or reflexively copying donor architecture.

## Model and dataset policy

Models are independent supply-chain objects.

Before adding a model to the registry:

- exact model repository/version/revision;
- weights digest;
- license or separate permission basis;
- training/use restrictions known from published terms;
- supported tasks/languages;
- redistribution right;
- commercial right if required by intended Himsat distribution;
- attribution;
- hardware/runtime requirements;
- benchmark evidence;
- download/sideload integrity.

Never infer model permission from the engine's software license or from permission to use unrelated source code.

## Asset/font/template policy

Professional PDF/DOCX generation and donor UI reuse create hidden licensing surfaces.

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
- license/permission basis;
- update strategy;
- failure isolation;
- whether an abstraction can make future replacement possible.

No dependency is added merely because a donor uses it or because Himsat has permission to copy it.

## Reuse decision rule

For each implementation frontier, compare all relevant covered sources and the native alternative.

Prefer, in order of engineering value rather than ideology:

1. a mature pinned dependency when it gives the narrowest reliable boundary;
2. a selective copied/adapted implementation when Himsat needs ownership or platform integration;
3. a vendored snapshot when deterministic/offline distribution justifies the maintenance burden;
4. a native implementation when available donor code is larger, riskier, lower quality, or mismatched;
5. reference-only use when source access, permission scope, provenance, or architecture cannot be closed.

The goal is not maximum copied code. The goal is maximum product quality with the smallest defensible long-term maintenance and supply-chain surface.

## Provenance automation roadmap

The repository already contains machine provenance policy/registry/generation machinery. Future implementation-era units should continue strengthening it for source reuse by supporting, as required:

- explicit `separate_permission_basis`/scope evidence for adoptions not relying only on compatible public licenses;
- copied/adapted source records with exact source/destination paths;
- license allow/deny/manual-review policy without treating founder permission as a silent bypass;
- generated `THIRD_PARTY_NOTICES` closure;
- model-license/permission manifest validation;
- SBOM generation;
- CI failure on unknown copied/vendor material where practical;
- source digest verification for vendored snapshots;
- explicit native/embedded component representation for packages that bundle independently licensed code.

## Donor merge rule

No donor adoption is complete until:

- provenance record exists;
- exact source revision is immutable;
- exact public-license or separate-permission basis is recorded;
- required notices are present;
- embedded third-party/model/data/asset material is independently resolved;
- destination code follows Himsat architecture;
- focused Himsat behavior tests pass;
- full applicable regression passes;
- Diffcipline proof is based on the exact diff;
- security/platform-specific tests appropriate to the blast radius pass;
- active SpecGrain authority permits the adoption;
- exact-head CI/review/merge qualification is satisfied.
