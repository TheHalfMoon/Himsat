# Voice Source Plan Hardening — 2026-09-08

Status: roadmap research and successor-shaping input only  
Canonical active authority remains `specs/CURRENT.md`.  
This document does not authorize voice implementation while Specification 004 is active.

## Objective

Harden Himsat's voice, capture, speech-processing, transcription, diarization, dictation, and model-routing roadmap using the strongest currently identified sources while preserving Himsat's local-first, Rust-first, evidence-linked, privacy-respecting architecture.

The goal is not to maximize donor count. The goal is to reduce product risk by assigning each external source a narrow role, comparing competing implementations under one Himsat-owned contract, and adopting only the smallest source/dependency surface that wins on measured Himsat requirements.

## Authority and provenance

The founder/user has recorded permission to use/copy the source code from the sources discussed for Himsat. Newly explicit voice-source permission is recorded in:

```text
governance/provenance/source-use-authorization-2026-09-08-voice-extension.md
```

Permission makes `REFERENCE`, `COPY`, `ADAPT`, `DEPEND`, and `VENDOR` eligible engineering choices. It does not waive exact path/revision provenance, public-license/NOTICE obligations, embedded third-party review, model/data/asset rights, security review, benchmark evidence, active-spec authority, or expected-head merge discipline.

## Executive findings

The current Himsat master plan is structurally sound, but the voice roadmap has nine material gaps or ambiguities:

1. **Universal-engine bias risk.** A single ASR engine cannot be assumed best for live dictation, mobile, Arabic/code-switching, medical vocabulary, long-form multi-speaker audio, and offline batch quality.
2. **VAD ownership ambiguity.** Capture Health mentions VAD-like signal health while Unit 010 owns audio processing, but the roadmap does not explicitly assign speech segmentation/VAD selection and qualification.
3. **Noise-conditioning candidate gap.** Sonora/WebRTC AudioProcessing are recorded, but DeepFilterNet and RNNoise are not yet part of the explicit arbitration set.
4. **Multilingual/low-resource gap.** Broad multilingual coverage is a product goal and Arabic is first-class, but Omnilingual ASR is not represented as a quality/benchmark challenger.
5. **Medical-specialization gap.** Medical ASR sources exist, but the plan lacks an explicit specialized model lane and medical-number/drug/term error evaluation.
6. **Joint long-form ASR+diarization gap.** MOSS-like joint long-form transcription/diarization can behave differently from a separate ASR + diarizer pipeline; both architectures need comparison.
7. **Runtime diagnostics gap.** VoiceStudio demonstrates useful engine preflight, diagnostics, model-store, error-journal, and no-silent-fallback patterns that should become Himsat contract requirements rather than donor-specific UX.
8. **Remote-code/runtime isolation gap.** Some quality-pass sources require Python/Transformers or remote/custom model code. The roadmap needs an explicit isolated-worker boundary and immutable code/model pinning rule.
9. **Wake-word sequencing gap.** Wake-word support is useful but creates an always-listening/privacy/battery surface. Push-to-talk/global shortcut should remain the earlier default; wake word should be a later opt-in leaf with independent qualification.

## Revised voice architecture

The roadmap should converge on the following Himsat-owned architecture rather than donor-owned application stacks:

```text
OS-native capture
  -> Capture Health + durable source lineage
  -> audio conditioning
       AEC / NS / AGC / limiting / resampling
  -> VAD + segmentation
  -> Voice Runtime Router
       capability + locality + device + resource + policy selection
  -> transcription lane
       live-general
       mobile-low-resource
       multilingual/code-switch
       medical-specialized
       long-form multi-speaker
       offline-quality/import
  -> diarization/alignment
       joint when selected model supports it
       separate when a measured diarizer wins
  -> non-destructive transcript revision ledger
  -> terminology/context hints with provenance
  -> EvidenceRef/source-time mapping
  -> downstream memory/intelligence/publishing
```

No stage may silently replace source evidence. A quality pass produces a new revision linked to the captured source and prior transcript revision.

## Voice Runtime Router

Unit 011 should own a Himsat-defined `Voice Runtime Router`, not merely an engine interface.

A route decision must be explainable from explicit metadata:

```text
requested_task
language_or_language_set
streaming_required
speaker_diarization_required
timestamp_granularity
medical_or_domain_profile
device_class
available_ram
accelerator_kind
thermal_or_battery_policy
offline_required
network_lock_state
model_digest
engine_digest
model_license_or_permission_state
runtime_health
```

The router must never perform a hidden local-to-cloud fallback. If no approved local route satisfies policy, it returns an explicit unavailable/unsupported result unless the user has separately enabled a remote route.

### Required route lanes

#### Lane A — live general dictation and meetings

Optimize first-token latency, incremental stability, timestamp usefulness, low CPU/GPU disruption, cancellation/backpressure, and six-hour survival.

Primary challengers at shaping time:

- `k2-fsa/sherpa-onnx`;
- `ggml-org/whisper.cpp`;
- `NVIDIA/NeMo-Speech.cpp` where supported models/device targets fit;
- Argmax/OpenSuperWhisper/OpenWhispr runtime patterns where they improve platform integration.

#### Lane B — mobile / low-resource

Optimize memory, startup time, battery, thermal behavior, offline packaging, and predictable fallback within approved local models.

Primary challengers include sherpa-onnx, quantized whisper.cpp-compatible models, Moonshine-class models, and platform-native acceleration when proven.

#### Lane C — multilingual / Arabic / code-switching

Arabic and Arabic-English code switching are first-class benchmark dimensions, not a generic multilingual checkbox.

Primary challengers include the best qualified general engines plus `facebookresearch/omnilingual-asr` as a broad multilingual quality/coverage challenger. Large models may remain quality-pass workers rather than live defaults if device evidence does not justify them.

#### Lane D — medical-specialized transcription

Medical speech is an optional specialized route, not evidence that Himsat provides medical advice.

Primary research challengers:

- `Omi-Health/omi-med-stt-runtime`;
- `Google-Health/medasr`;
- suitable Parakeet/NeMo-family models and other qualified medical ASR sources.

Medical evaluation must separately score ordinary WER and safety-sensitive token classes such as medication names, doses, units, numbers, negation, anatomy, and procedure terms. Transcript review/correction remains user-visible; no ASR benchmark permits a clinical correctness claim.

#### Lane E — long-form multi-speaker quality

Compare two architectures:

1. ASR plus a separate diarizer/alignment stage;
2. joint long-form transcription+diarization models such as `OpenMOSS/MOSS-Transcribe-Diarize`.

A joint model may win on speaker consistency but still lose on runtime isolation, language quality, timestamps, memory, or model governance. The plan must measure rather than assume.

#### Lane F — offline quality / imported media

Optimize final accuracy, alignment, speaker consistency, punctuation/format fidelity, resumability, and source mapping. Heavy isolated workers are acceptable when they remain optional, local, bounded, and crash-isolated.

## Source role arbitration

| Source | Best candidate role in Himsat | Preferred reuse posture | Why not wholesale adoption |
| --- | --- | --- | --- |
| `OpenWhispr/openwhispr` | cross-platform capture, meeting detection, self-audio exclusion, dictation/runtime failure knowledge | COPY/ADAPT/REFERENCE selective | Electron/app architecture should not displace Himsat's Rust/native core |
| `Zackriya-Solutions/meetily` | recording lifecycle, local meeting capture, STT integration | COPY/ADAPT selective | avoid importing unrelated application/backend coupling |
| `debpalash/VoiceStudio` | engine registry, GPU/runtime preflight, diagnostics/error journal, model orchestration, dictation output safety | ADAPT/COPY selective after machine permission gate | Python service/app stack is broader than Himsat needs; public AGPL truth remains recorded |
| `Starmel/OpenSuperWhisper` | macOS recorder/control/input patterns, model UX, file queue | COPY/ADAPT/REFERENCE selective | Himsat requires broader platform architecture |
| `ggml-org/whisper.cpp` | portable Whisper baseline and compatibility engine | DEPEND/VENDOR/COPY candidate | Whisper is one model family, not the complete routing policy |
| `k2-fsa/sherpa-onnx` | broad cross-platform speech substrate: ASR/VAD/diarization/speaker/KWS/enhancement candidates | DEPEND/COPY candidate | keep each capability behind Himsat contracts; model rights/package closure stay separate |
| `NVIDIA/NeMo-Speech.cpp` | lightweight native Parakeet/Nemotron/Sortformer-class challenger | DEPEND/COPY candidate | third-party notices, model conversion, model terms, and platform support need exact qualification |
| `NVIDIA-NeMo/Speech` | research/model source, conversion/reference, heavyweight worker candidate | REFERENCE/isolated worker/selected COPY | full Python/CUDA framework is not a default portable core runtime |
| `snakers4/silero-vad` | mature VAD baseline | DEPEND/VENDOR/model candidate | may be redundant when qualified through sherpa-onnx; compare packaging/latency first |
| `TEN-framework/ten-vad` | low-latency VAD challenger | DEPEND/COPY candidate | upstream performance claims require Himsat reproduction; exact notices/additional conditions matter |
| `dignifiedquire/sonora` | Rust-oriented AEC/NS/AGC challenger | DEPEND/COPY selective | maturity and speech-damage performance require benchmarking |
| `tonarino/webrtc-audio-processing` | established AEC/NS/AGC baseline | DEPEND/COPY selective | native/C++ build and binary surface cost |
| `Rikorose/DeepFilterNet` | full-band neural speech-enhancement challenger | DEPEND/COPY/REFERENCE | maintenance cadence, model/runtime footprint, and real-time cost must be justified |
| `xiph/rnnoise` | lightweight neural noise-suppression baseline | DEPEND/COPY candidate | build-downloaded model artifact needs independent integrity/provenance |
| `pyannote/pyannote-audio` | quality-pass diarization benchmark/optional isolated worker | REFERENCE/worker/COPY selective | Python/model runtime is heavier; model artifacts separately governed |
| `OpenMOSS/MOSS-Transcribe-Diarize` | joint long-form transcription+diarization benchmark/optional worker | REFERENCE/isolated worker/COPY selective | Python/Transformers/custom-code execution surface must be pinned and isolated |
| `facebookresearch/omnilingual-asr` | multilingual/low-resource quality challenger | EXPERIMENT/worker/selected dependency | resource demands may preclude live/default use |
| `Omi-Health/omi-med-stt-runtime` | local medical ASR specialized challenger and benchmark methodology | EXPERIMENT/DEPEND/COPY selective | specialized English medical performance does not establish general-language superiority |
| `Google-Health/medasr` | medical ASR benchmark/optional specialized worker | REFERENCE/worker/COPY code selective | code and model terms are independent; model distribution/use must close separately |
| `dscripka/openWakeWord` | later opt-in wake-word challenger | REFERENCE/DEPEND/COPY selective | always-listening/privacy/battery surface must not become a default prerequisite |

## VAD and segmentation hardening

### Ownership

- Unit 006 `Capture Health` observes capture-source health and reports signal/route/backpressure conditions.
- Unit 010 owns speech conditioning plus VAD/segmentation algorithm selection and processing behavior.
- Unit 011 owns model/runtime package identity when VAD is model-backed.

This prevents three subsystems from each creating incompatible speech/silence state machines.

### VAD qualification set

At minimum, shape an exact benchmark among:

- Silero VAD;
- TEN VAD;
- the selected sherpa-onnx-hosted variants when they differ operationally;
- a non-neural/simple baseline when useful for detecting benchmark regressions.

Required evidence:

- speech miss rate;
- false activation rate;
- start/end boundary error;
- overlap/noisy-room behavior;
- Arabic and Arabic-English code switching;
- far-field and Bluetooth audio;
- latency;
- warm-up time;
- CPU/RAM;
- battery/thermal impact on mobile;
- six-hour state stability;
- packaging/native dependency footprint.

No upstream benchmark table is accepted as Himsat PASS evidence without reproduction on Himsat target hardware/corpora.

## Audio conditioning hardening

Unit 010 should explicitly arbitrate:

- Sonora;
- WebRTC AudioProcessing;
- DeepFilterNet;
- RNNoise;
- native platform facilities where they are materially better or required.

A pipeline may combine capabilities, but every stage must justify itself. Do not stack multiple denoisers merely because they are available.

Required evidence should include:

- clipping/level stability;
- echo suppression when applicable;
- noise reduction;
- speech intelligibility;
- ASR WER before/after processing;
- deletion/substitution damage to quiet speech;
- music/non-speech false processing;
- CPU/RAM/latency;
- long-session drift/leak behavior.

Improved signal metrics with worse ASR or intelligibility is a failure, not a win.

## Model registry hardening

Unit 011 should require one immutable manifest per executable model/runtime combination rather than treating an engine name as sufficient identity.

Minimum fields:

```text
engine_id
engine_version_or_revision
engine_binary_digest
model_id
model_revision
model_weight_digest
model_license_or_permission_basis
model_task
languages
streaming_capability
diarization_capability
timestamp_capability
context_or_hotword_capability
required_runtime_features
minimum_ram
preferred_accelerator
network_requirement
remote_code_requirement
third_party_notice_closure
benchmark_artifact_set
```

### Remote/custom code rule

A model that requires `trust_remote_code`, custom Python modules, dynamic package installation, or equivalent executable model code must not execute inside the mandatory Himsat core process.

If selected, it must use an isolated worker with:

- immutable source revision;
- immutable model revision/digest;
- no floating remote import;
- explicit filesystem/network capability policy;
- bounded CPU/RAM/time;
- crash containment;
- scrubbed logs;
- protocol/version handshake;
- output treated as untrusted until parsed/validated.

## Runtime diagnostics and no-silent-fallback rule

Adapt the strongest orchestration lessons from VoiceStudio and OpenWhispr into Himsat-owned behavior:

- runtime self-check/doctor command;
- accelerator detection before model execution;
- model integrity verification before load;
- explicit reason when GPU/accelerator is unavailable;
- no silent GPU-to-CPU or local-to-cloud fallback when it violates selected policy;
- bounded error journal without private transcript/audio leakage;
- crash-loop detection and quarantine for repeatedly failing model/runtime combinations;
- model download/side-load progress, resumability, digest verification, and rollback;
- deterministic route explanation for debugging.

A user may explicitly choose a slower CPU fallback if policy permits it; the system must not hide that decision.

## Diarization and speaker privacy hardening

Unit 014 should compare:

- sherpa-onnx diarization/speaker capabilities;
- pyannote quality-pass pipelines;
- NeMo-Speech.cpp/Sortformer-class native paths;
- MOSS joint transcription+diarization where supported.

Required evidence expands beyond DER/JER to include:

- timestamp drift over long recordings;
- overlap handling;
- speaker-count error;
- speaker-label stability across chunks;
- ASR/diarization interaction under interruptions;
- Arabic/code-switch sessions;
- correction/relabel propagation;
- reset/deletion proof;
- no persistent identity created unless the user explicitly enables speaker identity.

Speaker segmentation labels such as `SPEAKER_01` are not the same thing as biometric speaker identity. Persistent speaker embeddings remain encrypted, opt-in, deletable sensitive data.

## Terminology and contextual biasing

Terminology support should be routed as bounded model hints or deterministic post-recognition replacement rules with auditability. It must not become an opaque LLM rewrite that silently changes factual transcript content.

For specialized medical terminology, preserve both raw ASR output and any corrected/quality-pass revision when corrections could materially alter numbers, medications, or clinical terms.

## Wake-word sequencing

Push-to-talk/global shortcut remains the earlier default interaction because it has lower privacy, battery, and false-activation risk.

A wake-word leaf may be shaped later only after the Input Control Layer exists. `openWakeWord` and any platform-native alternative can be compared then.

Wake-word acceptance must include:

- explicit opt-in and visible enabled state;
- local processing by default;
- no durable raw-audio retention from the detector ring buffer unless separately authorized;
- false accepts/rejects across accents/noise/devices;
- Arabic/English invocation strategy if selected;
- CPU/battery/thermal cost;
- suspend/resume/interruption behavior;
- model integrity/provenance;
- clear disable/reset behavior.

## Master-plan unit changes recommended

This research does not renumber the master plan. It hardens the shaping requirements of existing units.

### 006 — Capture abstraction and Capture Health

Add explicit ownership boundary: signal/source health only; processing algorithms and VAD selection live in Unit 010. Capture Health must expose enough observability to diagnose silence, clipping, route loss, source disappearance, backpressure, and self-capture without logging private content.

### 007/008/009 — desktop capture

At shaping time, compare OpenWhispr, Meetily, Anarlog, OpenSuperWhisper, and native implementation for exact platform paths. OpenWhispr becomes an explicit Windows/macOS/Linux capture and meeting-detection challenger rather than a generic reference.

### 010 — audio conditioning, VAD, and segmentation

Expand from `AEC/NS/AGC/limiting/resampling` to a swappable speech-conditioning + segmentation contract. Require Sonora/WebRTC/DeepFilterNet/RNNoise arbitration and Silero/TEN/sherpa-hosted VAD arbitration.

### 011 — model registry and Voice Runtime Router

Expand the router to the six explicit route lanes in this document. Add NeMo-Speech.cpp as a first-class native challenger and classify heavy Python/custom-code engines as isolated workers rather than default core dependencies.

### 012 — live transcription

Require route stability, partial-revision semantics, first-token/finalization latency, backpressure, device/resource evidence, target-insertion safety for dictation, and no hidden route fallback.

### 013 — quality-pass transcription/alignment

Add multilingual/Arabic, domain-specialized, and long-form challengers. Require non-destructive revision lineage and exact source-time mapping.

### 014 — diarization and speaker vault

Require separate and joint diarization architecture comparison. Keep speaker identity independent and opt-in.

### 015 — desktop runtime, controls, meeting detection

Add OpenWhispr and VoiceStudio explicitly to donor arbitration for meeting detection, runtime diagnostics, output insertion, hotkey/input behavior, and crash recovery. Push-to-talk/global shortcuts precede wake word.

### 019 — audio/video import

Permit heavier quality-pass engines and joint transcription/diarization workers because import is asynchronous, but require resumability, resource budgets, hostile-media isolation, and exact source mapping.

### 031 — capability broker

Future context-aware dictation may receive terminology/project/context grants only through explicit capability scopes. Reading app context must not imply access to raw audio, all documents, or speaker biometrics.

## Benchmark framework required before source selection

Source selection must use shared Himsat corpora and protocols so each donor is measured under identical conditions.

### Accuracy

- WER and CER;
- normalized and verbatim scoring where appropriate;
- Arabic;
- English;
- Arabic-English code switching;
- accents/noisy/far-field/Bluetooth;
- long-form meetings;
- timestamp alignment error;
- punctuation/number preservation when measured separately.

### Medical specialization

When a medical route is evaluated:

- general WER;
- medical-term WER;
- medication-name errors;
- dose/unit/number errors;
- negation errors;
- named anatomy/procedure term errors;
- confidence/uncertainty behavior where the model exposes it;
- correction/review workflow.

Medical benchmark success is transcription evidence only and does not establish diagnostic or clinical safety.

### Diarization

- DER;
- JER;
- speaker-count error;
- overlap performance;
- label stability;
- long-session drift;
- timestamp alignment.

### VAD

- false positive/negative rate;
- speech-boundary error;
- missed quiet speech;
- noise/music behavior;
- latency and state stability.

### Runtime

- cold start;
- warm start;
- real-time factor;
- first-token latency;
- finalization latency;
- CPU/GPU/NPU usage;
- peak and steady RAM;
- package/model size;
- battery/thermal behavior;
- six-hour stability;
- cancellation and crash recovery;
- behavior when the expected accelerator disappears or fails.

### Supply chain and privacy

- exact engine/model revision and digests;
- offline startup after side-load;
- no unapproved network request;
- model/runtime tamper rejection;
- dependency/native closure;
- NOTICE generation;
- log scrubbing;
- isolated-worker capability denial tests.

## Selection rule

For every voice subsystem, use this decision order:

1. define Himsat behavior and evidence contract;
2. identify the smallest serious donor/native candidates;
3. pin exact candidate source/model identities;
4. close code/model/data/asset provenance separately;
5. benchmark on representative Himsat targets/corpora;
6. reject candidates that violate local-first/privacy/platform invariants regardless of raw quality;
7. choose one default per route/device class only when evidence supports it;
8. keep replaceability behind Himsat contracts;
9. write Himsat-owned positive, negative, failure, and long-session tests;
10. preserve losing/negative benchmark results as evidence.

## Deliberate non-decisions

This hardening does not select a universal default engine, a universal VAD, a universal denoiser, a medical model, a diarizer, or a wake word.

Those selections would be premature before the relevant specification becomes active and before Himsat executes comparable benchmarks.

It also does not authorize:

- bulk donor copies;
- mandatory Python in the core product;
- hidden cloud fallback;
- covert/indicator-bypassing recording;
- always-on wake word by default;
- persistent speaker biometrics by default;
- medical/clinical decision claims;
- unpinned remote/custom model code;
- model/data/asset reuse solely because source-code permission exists.

## Success criterion for future shaping

When Units 006-015 become active, an agent should be able to read this document and produce bounded SpecGrain leaves without rediscovering the architecture-level decisions above. Each selected leaf must still reverify upstream truth because research-time revisions and product capabilities can change.
