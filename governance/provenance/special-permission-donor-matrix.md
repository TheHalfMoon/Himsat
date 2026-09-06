# Special-Permission Donor Matrix

Date: 2026-09-07
Status: planning matrix only; machine `registry.json` remains the byte-adoption authority

The founder/user has attested to source-use permission for every external source referenced anywhere in Himsat at or before the exact snapshot recorded in `source-use-authorization.md`.

This table summarizes implementation-relevant donors. It does not limit the global authorization coverage and it does not make an unlisted covered source ineligible. `PUBLIC` records public-license truth where known; `AUTHORIZED` records planning eligibility under the founder attestation. Exact revision/path/rightsholder/foreign-content review remains mandatory at adoption time.

| Source | PUBLIC / known posture | AUTHORIZED planning posture | Best Himsat use | Independent exclusions / cautions |
| --- | --- | --- | --- | --- |
| `Zackriya-Solutions/meetily` | community MIT | COPY/ADAPT selective | desktop capture, recording lifecycle, STT integration, Tauri patterns | separately governed/private/vendor/model/assets still need exact evidence |
| `fastrepl/anarlog` | mixed community/commercial paths | COPY/ADAPT selective for exact covered project-owned paths | Rust modular architecture, mobile/watch, sessions, MCP/CLI | contributor/vendor/generated/model/asset ownership remains separate |
| `OpenWhispr/openwhispr` | MIT | COPY/ADAPT/DEPEND selective | macOS/Windows/Linux capture, meeting detection, self-audio exclusion, background/runtime regressions | downloaded models/binaries independent |
| `debpalash/VoiceStudio` | public AGPL-3.0 application | COPY/ADAPT selective under separate permission after machine gate | dictation/output safety, speech control/data plane, engine registry, diagnostics/model orchestration | public AGPL truth preserved; external engines/weights/assets separate |
| `Starmel/OpenSuperWhisper` | reviewed top-level MIT source | COPY/ADAPT/REFERENCE selective | macOS dictation, global controls, recorder state, engine abstraction, file queue/model UX | submodules/packages/models independently closed |
| Superwhisper product/source | proprietary product behavior; source availability not established in repo | PRODUCT_REFERENCE now; COPY/ADAPT only when actual covered source bytes are available and pinnable | system-wide dictation UX, modes, context, vocabulary/replacements, routing | permission does not create unavailable source bytes |
| `ggml-org/whisper.cpp` | permissive | DEPEND/VENDOR/COPY candidate | portable Whisper backend | model weights separate |
| `k2-fsa/sherpa-onnx` | permissive code | DEPEND/COPY candidate | streaming ASR, VAD, diarization/speaker/mobile runtime | model licenses and native packaging separate |
| `argmaxinc/argmax-oss-swift` | MIT code | DEPEND/COPY/ADAPT selective | Apple-native speech acceleration | third-party notices/weights separate |
| `moonshine-ai/moonshine` | MIT code with model-specific history | EXPERIMENT/DEPEND/COPY | low-latency edge speech | exact model-weight terms separate |
| `dignifiedquire/sonora` | BSD-3-Clause | DEPEND/COPY selective | Rust AEC/NS/AGC | benchmark maturity/quality |
| `tonarino/webrtc-audio-processing` | permissive wrapper | DEPEND/COPY selective | WebRTC audio-processing alternative | C++/system build/native closure |
| Vibe/Sona/local STT sources recorded in repo | exact path/repo posture must be reverified | COPY/ADAPT/DEPEND selective | media import, isolated STT/model lifecycle | do not import server architecture when a smaller Himsat contract wins |
| `pyannote/pyannote-audio` | MIT code | REFERENCE/worker/COPY selective | diarization benchmark/pipelines | Python/runtime/model weights separate |
| `EricLBuehler/mistral.rs` | MIT | DEPEND/COPY candidate | Rust-native local LLM runtime | model weights/provider support separate |
| `ggml-org/llama.cpp` | MIT | DEPEND/VENDOR/COPY candidate | GGUF compatibility fallback | model weights separate |
| `huggingface/candle` | permissive | DEPEND/COPY candidate | focused Himsat-owned ML components | avoid duplicate inference stacks without measured need |
| `Anush008/fastembed-rs` | Apache-2.0 | DEPEND/COPY candidate | local embeddings/reranking | model terms/cache integrity separate |
| `sqlcipher/sqlcipher` | BSD-style community source | DEPEND/VENDOR/COPY candidate | encrypted SQLite | exact native/provider identity, crypto review, notices |
| `asg017/sqlite-vec` | MIT/Apache-2.0 | DEPEND/COPY candidate | embedded semantic search | pre-v1 API/migration risk |
| `quickwit-oss/tantivy` | MIT | DEPEND/COPY candidate later | large-corpus text search | use only when benchmarks prove SQLite FTS insufficient |
| `n0-computer/iroh` / `iroh-ffi` | permissive | DEPEND/COPY candidate | authenticated P2P/QUIC and mobile bridge | relay/network behavior must obey Network Lock |
| `y-crdt/y-crdt` | MIT | DEPEND/COPY candidate later | collaborative editable-note CRDT | do not use CRDT for canonical append-only evidence without need |
| `extism/extism` | BSD-3-Clause | DEPEND/COPY candidate | Wasm plugin host | capability sandbox remains Himsat policy |
| `bytecodealliance/wasmtime` | Apache-2.0 | DEPEND alternative | lower-level Wasm runtime | larger security/update surface |
| `tauri-apps/tauri` | permissive | DEPEND | desktop shell | shared core remains UI-framework independent |
| `mozilla/uniffi-rs` | MPL-2.0 | DEPEND/manual review | Rust-to-Swift/Kotlin FFI generation | covered-file obligations |
| `typst/typst` | Apache-2.0 | DEPEND/COPY candidate | professional PDF/PDF-A/PDF-UA | fonts/templates/assets separate |
| `bokuweb/docx-rs` | MIT | DEPEND/COPY candidate | semantic DOCX | OOXML/RTL/template qualification |
| `tokimo-lab/tokimo-package-fileparser` | permissive candidate | EXPERIMENT/DEPEND/COPY | Rust PDF/Office parsing | hostile-input fuzzing/maturity |
| `xberg-io/xberg` | current v1 MIT | EXPERIMENT/DEPEND/COPY/ADAPT | broad Rust-first document extraction | vendor/plugins/models/assets separate |
| historical Kreuzberg/Kreuzberg-LTS sources | release-era terms vary | HISTORICAL/COPY candidate under exact rights | historical parser implementations | exact snapshot, security horizon, no era mixing |
| `microsoft/markitdown` | MIT | COPY/ADAPT selective / REFERENCE | normalization/conversion UX | Python not preferred for mandatory core |
| `docling-project/docling` / `DS4SD/docling-core` | MIT code | COPY selective / isolated worker | document IR/layout/table/formula quality | Python-heavy; models separate |
| `opendatalab/MinerU` | custom public terms | COPY/ADAPT selective under exact separate permission | document parsing/benchmark implementation | models/assets/third-party code independent |
| Marker-family sources recorded in repo | public/copyleft/model terms vary | COPY/ADAPT selective under exact permission | document parsing/OCR/layout | exact repository/path and model rights separate |
| current Screenpipe source | public restrictions recorded | COPY/ADAPT selective under exact separate permission | screen/context capture/indexing patterns | exact path ownership; third-party/media/assets/security/privacy separate |
| historical permissive Screenpipe source | historical permissive paths where proven | COPY selective | historical screen/context implementation | later security fixes require independent evaluation |
| RustCrypto crates selected by Spec004 | permissive | DEPEND | reviewed cryptographic primitives | exact versions/security review; no custom crypto primitive forks |

## Covered-source catchall

Any external source referenced elsewhere in the repository at or before the authorization snapshot is covered by the founder attestation even when not named in this table. Before implementation use, add or update an exact planning/machine record for that source and reverify live upstream truth.

## Behavioral/source-unavailable references

Products such as Circleback, Otter.ai, Granola, Fireflies, Fathom, Krisp, Plaud, Limitless-style products, Superwhisper, and OS first-party products remain behavioral references when no immutable source bytes are available to Himsat. If covered source bytes later become available, they can enter normal donor arbitration; do not fabricate code provenance from product observation.

## Adoption priority by subsystem

- **Capture:** arbitrate OpenWhispr vs Meetily vs Anarlog vs OpenSuperWhisper where applicable vs native implementation.
- **Desktop runtime/dictation:** OpenWhispr, VoiceStudio, OpenSuperWhisper, Superwhisper behavior, and native APIs are primary source inputs; Himsat owns the contract.
- **STT/model lifecycle:** whisper.cpp + sherpa-onnx as engine candidates; Vibe/Sona/VoiceStudio/OpenWhispr/OpenSuperWhisper/Meetily as orchestration and failure-knowledge inputs where appropriate.
- **Documents:** arbitrate Xberg/tokimo/Docling/MarkItDown/MinerU/Marker/native parsers by hostile-input safety, fidelity, deterministic IR, resources, and packaging.
- **Memory/search:** prefer one canonical SQLCipher-backed truth with FTS5/sqlite-vec; add Tantivy only when measured scale requires it.
- **P2P/plugins/publishing:** Iroh, Extism/Wasmtime, Typst, and docx-rs remain focused dependency candidates rather than wholesale app donors.

## Machine-gate note

Current `governance/provenance/registry.json` cannot yet represent separate permission as the non-reference rights basis for a publicly restricted license. Issue #14 tracks that extension. Until it exists and an active SpecGrain leaf authorizes an exact path, those sources are planning-eligible but machine-adoption blocked.
