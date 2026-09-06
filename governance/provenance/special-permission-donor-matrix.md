# Special-Permission Donor Matrix

Date: 2026-09-06
Status: planning matrix only; machine `registry.json` remains the adoption authority

The founder has attested to special reuse permission for project-owned source code from all source-code projects discussed for Himsat through this date. This table updates **planning eligibility**, not byte-level adoption authority.

`PUBLIC` describes the known/public license posture. `SPECIAL` describes how founder-attested permission changes planning. Exact revision/path/rightsholder/foreign-content review is still mandatory at adoption time.

| Source | PUBLIC truth / prior posture | SPECIAL planning posture | Best Himsat use | Exclusions / cautions |
| --- | --- | --- | --- | --- |
| `Zackriya-Solutions/meetily` | Community MIT | COPY selective, public + special permission | desktop capture, recording lifecycle, STT integration, Tauri patterns | Pro/private code only if exact source is supplied and grant covers it; strip telemetry/cloud assumptions/assets |
| `fastrepl/anarlog` | mixed community MIT + commercial enterprise paths | COPY selective from any exact project-owned path covered by grant | Rust modular architecture, mobile/watch, local sessions, MCP/CLI | verify contributor/vendor/generated ownership per path; do not infer model/asset rights |
| `OpenWhispr/openwhispr` | MIT | COPY selective, public + special permission | macOS/Windows/Linux native capture, meeting detection, background runtime, self-audio exclusion, diarization/runtime regressions | downloaded models/binaries retain independent provenance |
| `debpalash/VoiceStudio` | AGPL-3.0 application | COPY selective under special permission | dictation/output safety, local speech control/data plane, engine registry, diagnostics, model orchestration | public AGPL truth stays recorded; model weights and external engines separately controlled |
| `ggml-org/whisper.cpp` | permissive | DEPEND/VENDOR/COPY candidate | portable Whisper backend | model weights separate |
| `k2-fsa/sherpa-onnx` | permissive code | DEPEND/COPY candidate | streaming ASR, VAD, diarization/speaker/mobile runtime | model licenses separate; native packaging review |
| `argmaxinc/argmax-oss-swift` | MIT code | DEPEND/COPY selective | Apple-native speech acceleration | third-party notices/weights separate |
| `moonshine-ai/moonshine` | MIT code with model-era exceptions | EXPERIMENT/DEPEND/COPY under exact permission | low-latency edge speech | exact model-weight terms remain independent |
| `dignifiedquire/sonora` | BSD-3-Clause | DEPEND/COPY selective | Rust AEC/NS/AGC | maturity/quality benchmark before default |
| `tonarino/webrtc-audio-processing` | BSD-style wrapper | DEPEND/COPY selective | WebRTC audio-processing alternative | C++/system build burden |
| `thewh1teagle/vibe` / local engine descendants | permissive paths require exact recheck | COPY selective / REFERENCE | media import, transcription worker isolation, model lifecycle | pin current canonical repo/path/license before use |
| Sona/local STT server work | permissive paths require exact recheck | COPY selective / DEPEND | isolated local STT worker and model loading | do not import server architecture if in-process Himsat contract is better |
| `pyannote/pyannote-audio` | MIT code | REFERENCE/optional worker/copy selective | diarization benchmark oracle and pipelines | Python/runtime/model weights separate |
| `EricLBuehler/mistral.rs` | MIT | DEPEND/COPY candidate | primary Rust-native local LLM runtime | model weights/runtime provider support separate |
| `ggml-org/llama.cpp` | MIT | DEPEND/VENDOR/COPY candidate | GGUF compatibility fallback | model weights separate |
| `huggingface/candle` | permissive | DEPEND/COPY candidate | small Himsat-owned ML components and experimentation | avoid duplicating full inference stack without need |
| `Anush008/fastembed-rs` | Apache-2.0 | DEPEND/COPY candidate | local embeddings/reranking | embedding model terms/cache integrity separate |
| `sqlcipher/sqlcipher` | BSD-style community source | DEPEND/VENDOR/COPY candidate | encrypted SQLite | crypto/security review, build/provider/export-control handling |
| `asg017/sqlite-vec` | MIT/Apache-2.0 | DEPEND/COPY candidate | embedded semantic search | pre-v1 API/migration risk |
| `quickwit-oss/tantivy` | MIT | DEPEND/COPY candidate later | large-corpus text search | only when benchmarks prove SQLite FTS is insufficient |
| `n0-computer/iroh` | MIT/Apache-2.0 | DEPEND/COPY candidate | authenticated P2P/QUIC transport | relays/network behavior must obey Network Lock |
| `n0-computer/iroh-ffi` | permissive | DEPEND/COPY candidate | Swift/Kotlin/mobile P2P bridge patterns | exact package/mobile artifacts provenance |
| `y-crdt/y-crdt` | MIT | DEPEND/COPY candidate later | collaborative editable-note CRDT | do not use CRDT for canonical append-only evidence unnecessarily |
| `extism/extism` | BSD-3-Clause | DEPEND/COPY candidate | Wasm plugin host | capability sandbox must be Himsat-owned policy |
| `bytecodealliance/wasmtime` | Apache-2.0 | DEPEND alternative | lower-level Wasm runtime | larger complexity surface |
| `tauri-apps/tauri` | permissive | DEPEND | desktop shell | Himsat core must remain UI-framework independent |
| `mozilla/uniffi-rs` | MPL-2.0 | DEPEND/manual review | Rust-to-Swift/Kotlin FFI generation | covered-file modification obligations |
| `typst/typst` | Apache-2.0 | DEPEND/COPY candidate | professional PDF/PDF-A/PDF-UA | fonts/templates/assets separately reviewed |
| `bokuweb/docx-rs` | MIT | DEPEND/COPY candidate | semantic DOCX | qualify OOXML coverage/RTL/templates |
| `tokimo-lab/tokimo-package-fileparser` | MIT/Apache candidate | EXPERIMENT/DEPEND/COPY | Rust PDF/Office parsing | hostile-input fuzzing/maturity required |
| `xberg-io/xberg` | current v1 MIT | EXPERIMENT/DEPEND/COPY | Rust-first broad document extraction | bundled/vendor/plugins/models/assets separately reviewed |
| `kreuzberg-dev/kreuzberg-lts` | release-era license varies | HISTORICAL/SPECIAL candidate | historical parser implementations | exact snapshot license/security horizon |
| `microsoft/markitdown` | MIT | COPY selective / REFERENCE | Markdown normalization/conversion UX | Python not preferred for mandatory core |
| `docling-project/docling` / `DS4SD/docling-core` | MIT code | COPY selective / optional worker | document IR/layout/table/formula quality | Python-heavy; models separate |
| `opendatalab/MinerU` | custom public terms | COPY selective under special permission where grant covers source | document parsing/benchmark ideas | models/assets/third-party code independent |
| Marker-family document parsers discussed | public/copyleft/model terms vary | COPY selective under special permission where exact grant/path exists | document parsing/OCR/layout | identify exact repository before adoption; model rights separate |
| current Screenpipe source | current public license restricts competing reuse | COPY selective under special permission where grant covers exact project-owned source | screen/context capture and indexing patterns | exact current path ownership; third-party/media/assets separate; security/privacy review |
| historical permissive Screenpipe snapshot | MIT-era source | COPY selective | historical screen/context implementation | security fixes after snapshot require independent evaluation |
| RustCrypto projects (`argon2`, `hkdf`, `chacha20poly1305`, `zeroize`, etc.) | permissive | DEPEND | reviewed crypto primitives | exact versions/security review; never copy crypto to customize primitive behavior |

## Behavioral references not converted into code donors

The founder's source-code permission statement does not by itself turn products with no supplied/discussed donor source into source-code donors. Keep these as behavioral references unless exact source and rights are separately established:

- Circleback;
- Otter.ai;
- Granola;
- Fireflies;
- Fathom;
- Krisp;
- Plaud;
- Limitless-style products;
- proprietary OS/app behavior.

## Adoption priority by subsystem

### Capture

Arbitrate OpenWhispr vs Meetily vs Anarlog vs native implementation. Do not copy all three full stacks.

### Desktop runtime/dictation

Use OpenWhispr and VoiceStudio as the main code/behavior sources, but rebuild behind Himsat Rust/native contracts.

### STT/model lifecycle

Use whisper.cpp + sherpa-onnx as engine candidates; use Vibe/Sona/VoiceStudio/OpenWhispr/Meetily as orchestration and failure-knowledge donors.

### Documents

Arbitrate Xberg/tokimo/Docling/MarkItDown/MinerU/Marker/native parsers by hostile-input safety, fidelity, deterministic IR, resource use, and platform packaging.

### Memory/search

Prefer one canonical SQLite/SQLCipher-based truth, with FTS5/sqlite-vec and optional Tantivy only when measured scale justifies it.

### P2P/plugins/publishing

Iroh, Extism/Wasmtime, Typst, and docx-rs remain focused dependency candidates rather than wholesale donor apps.

## Machine-gate note

Current `governance/provenance/registry.json` does not yet model special-permission adoption for publicly restricted licenses. Issue #14 tracks the required extension. Until that machine gate is implemented and an active SpecGrain unit authorizes a path, this matrix is planning evidence only.
