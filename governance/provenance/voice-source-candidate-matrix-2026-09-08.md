# Himsat Voice Source Candidate Matrix — 2026-09-08

Status: planning/research matrix only; no dependency or source adoption authority  
Active authority remains `specs/CURRENT.md`.

## Purpose

Record the current voice-source challenger set, research-time upstream identities, intended Himsat roles, and the exact gaps each future specification must close before selection.

Research-time revisions are not adoption pins. Reverify upstream truth at the active implementation frontier.

## Candidate matrix

| Source | Research revision observed | Candidate role | Main Himsat value | Mandatory qualification before adoption |
| --- | --- | --- | --- | --- |
| `OpenWhispr/openwhispr` | reverify at shaping | capture/runtime/dictation donor | cross-platform capture, meeting detection, self-audio exclusion, global dictation/runtime knowledge | exact paths, Electron/native boundary extraction, platform lifecycle tests, model/binary separation |
| `Zackriya-Solutions/meetily` | reverify at shaping | capture/meeting donor | local recording lifecycle and STT integration | exact path rights, current architecture delta, long-session/platform tests |
| `debpalash/VoiceStudio` | reverify at shaping | orchestration/diagnostics donor | engine registry, runtime preflight, model lifecycle, diagnostics, dictation output safety | separate-permission machine support, exact AGPL/public terms, selected path scope, external engine/model separation |
| `Starmel/OpenSuperWhisper` | reverify at shaping | macOS controls/recorder donor | global controls, recorder state, engine abstraction, model/file queue UX | submodule/package/model closure and Apple target qualification |
| `ggml-org/whisper.cpp` | reverify at shaping | native ASR baseline | mature portable Whisper runtime | exact version/build features, native dependency closure, model terms, target benchmark |
| `k2-fsa/sherpa-onnx` | reverify at shaping | broad speech substrate | cross-platform ASR/VAD/diarization/speaker/KWS/enhancement candidates | exact selected capabilities/models only, native/package closure, model rights, Himsat behavior tests |
| `NVIDIA/NeMo-Speech.cpp` | `ffa38cb2408f1e832a36d46fef5e3e1e80d07e6c` | native ASR/diarization challenger | lightweight local Parakeet/Nemotron/Sortformer-class runtime path | exact license/NOTICE closure, third-party code, model conversion, model terms, platform/device benchmark |
| `NVIDIA-NeMo/Speech` | `de26b36962fe69751c8a4286e9c1969fa9b5e2db` | research/model tooling or isolated worker | broad current speech/model ecosystem | Python/CUDA/runtime footprint, model-specific rights, worker isolation, no mandatory core dependency without proof |
| `snakers4/silero-vad` | `867c2aa692646a1f1de3e94a15c9dd9f614c0acb` | VAD baseline | mature ONNX/Torch VAD candidate | model artifact digest/rights, direct-vs-sherpa packaging, target latency/accuracy/battery evidence |
| `TEN-framework/ten-vad` | `22a3bcd4509d0faaa8eef4881e8af5f39c178950` | VAD challenger | low-latency streaming candidate and sherpa integration option | exact license/NOTICE/additional-condition review, model files, reproduced benchmark, target packaging |
| `dignifiedquire/sonora` | reverify at shaping | audio-conditioning challenger | Rust-oriented AEC/NS/AGC | maturity, objective/perceptual quality, ASR damage, CPU/latency, long-session evidence |
| `tonarino/webrtc-audio-processing` | reverify at shaping | audio-conditioning baseline | established WebRTC APM behavior | native/C++ build closure, target settings, speech-damage and resource evidence |
| `Rikorose/DeepFilterNet` | `d375b2d8309e0935d165700c91da9de862a99c31` | neural enhancement challenger | full-band speech enhancement | model/runtime provenance, maintenance/security horizon, real-time resource cost, ASR/intelligibility delta |
| `xiph/rnnoise` | `70f1d256acd4b34a572f999a05c87bf00b67730d` | lightweight NS baseline | compact recurrent noise suppression | downloaded model artifact provenance/integrity, build reproducibility, quality/resource benchmark |
| `pyannote/pyannote-audio` | reverify at shaping | diarization benchmark/worker | strong separate diarization reference and worker path | Python/model terms, worker isolation, model digest, DER/JER/overlap/device evidence |
| `OpenMOSS/MOSS-Transcribe-Diarize` | `61bc29cd4120be7b5d3b761b64cd5dff57263642` | joint long-form ASR+diarization worker/challenger | single-model long-form speaker/time transcription architecture | exact model/runtime pins, custom/remote code policy, worker isolation, languages/resources, long-form benchmark |
| `facebookresearch/omnilingual-asr` | `81f51e224ce9e74b02cc2a3eaf21b2d91d743455` | multilingual quality challenger | broad language/low-resource research path | model-specific terms/digests, Arabic/code-switch benchmark, resource/device cost, isolated worker if heavy |
| `Omi-Health/omi-med-stt-runtime` | `41689b213622b0bf87cdcd75047b272facec9393` | medical ASR challenger | local medical transcription runtime/evaluation reference | runtime/model rights separated, exact weights, medical corpus rights, medical-term/number error benchmark, no clinical claims |
| `Google-Health/medasr` | `ad843cb81b3e610e1868ed38f7230a70b66ed7e8` | medical ASR benchmark/worker | independent medical ASR challenger | code/model terms separated, exact model use/redistribution rights, model digest, medical benchmark and worker/runtime closure |
| `dscripka/openWakeWord` | `368c03716d1e92591906a84949bc477f3a834455` | later opt-in wake-word challenger | local wake-word detection/training ecosystem | model/phrase rights, privacy ring-buffer policy, false activation, battery/thermal, accent/language, opt-in/disable proof |

## Cross-candidate rules

### One engine does not win by feature count

A source that exposes ASR, VAD, diarization, speaker ID, KWS, and enhancement is not automatically preferred for all those functions. Consolidation reduces packaging cost but may reduce quality or increase blast radius. Himsat measures both capability quality and operational consolidation value.

### Route stability

The Voice Runtime Router must avoid model thrashing. A language/device signal change does not automatically replace the active engine mid-utterance or mid-segment.

Future shaping must define:

- session/segment route-lock granularity;
- explicit transition points;
- hysteresis for language/resource changes;
- cancellation/drain semantics;
- transcript revision boundary when a route changes;
- route-change evidence in diagnostics.

### Clock and source synchronization

Meetings may combine microphone and system audio with independent clocks, device changes, resampling, sleep/wake, and Bluetooth routes.

Before multi-source capture is qualified, Himsat must prove:

- monotonic source timestamps;
- drift measurement and bounded correction;
- resampling behavior;
- discontinuity markers;
- route-change realignment;
- no duplicate/self-captured participant audio after mixing/exclusion policy;
- transcript/source time mapping after correction.

A six-hour recording with gradually wrong timestamps is a capture failure even when no bytes are lost.

### Context/hotword bias safety

Vocabulary hints and medical/project terminology can improve recall but can also create false insertions.

Qualification must measure:

- target-term recall improvement;
- non-target false insertion rate;
- number/name corruption;
- behavior when a hint is wrong or stale;
- reversibility and auditability;
- preservation of raw ASR output when a deterministic replacement changes text.

No terminology feature may silently rewrite evidence without revision lineage.

### Confidence is not portable truth

Engine confidence/log-probability values are not assumed comparable across model families.

If Himsat exposes confidence:

- store the engine-native score with engine/model identity;
- calibrate only on declared evaluation data;
- never compare unrelated raw confidence scales as if they shared one probability meaning;
- prefer explicit uncertainty/unsupported behavior downstream over fabricated certainty.

### Native/model parser isolation

Model files and native runtimes are supply-chain and parser attack surfaces even when execution is local.

Future units must assess whether each engine can run in-process or should use a worker boundary based on:

- memory-safety profile;
- dynamic/custom code;
- native library complexity;
- model parser attack surface;
- crash frequency;
- GPU driver/runtime interaction;
- recoverability;
- performance cost of isolation.

Python/Transformers custom-code engines default to isolated-worker consideration. Native engines are not automatically trusted merely because they avoid Python.

### Model update and rollback

A model/runtime update requires:

- immutable identity/digests;
- migration/compatibility check;
- benchmark delta where the default route changes;
- rollback to the previously qualified version;
- no deletion of the last working local route until the replacement is proven usable;
- explicit storage cleanup after safe retirement.

### Benchmark corpus governance

Himsat benchmark corpora are supply-chain objects.

Record per corpus:

- source/license/permission;
- privacy status;
- redistribution status;
- language/accent/domain labels;
- reference transcript provenance;
- normalization/scoring rules;
- synthetic versus real origin;
- train/eval contamination risk when known.

Medical and biometric/speaker corpora require stricter privacy/rights handling than ordinary synthetic speech fixtures.

## Current non-adoption state

```text
SOURCE_CODE_ADOPTED_BY_THIS_MATRIX = NO
MODEL_BYTES_ADOPTED_BY_THIS_MATRIX = NO
DEPENDENCIES_CHANGED_BY_THIS_MATRIX = NO
ACTIVE_SPEC_CHANGED_BY_THIS_MATRIX = NO
VOICE_IMPLEMENTATION_AUTHORITY = BLOCKED_UNTIL_FUTURE_ACTIVE_SPEC
```

This matrix exists to make future selection faster and safer, not to pre-select winners.
