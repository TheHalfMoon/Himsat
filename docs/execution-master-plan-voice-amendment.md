# Himsat Execution Master Plan — Voice Hardening Amendment

Date: 2026-09-08  
Status: master-plan supplement after canonical merge; `specs/CURRENT.md` remains the active-authority source of truth

## Purpose

This amendment hardens the voice-related candidate units in `docs/execution-master-plan.md` using the source and gap analysis in:

```text
docs/research/2026-09-08-voice-source-plan-hardening.md
governance/provenance/source-use-authorization-2026-09-08-voice-extension.md
```

It does not renumber the existing program, activate a blocked successor, change Specification 004, add product dependencies, or adopt donor/model bytes.

## Voice dependency refinement

The existing program dependency graph is refined conceptually as follows:

```text
005 crash-safe media journal
  -> 006 capture abstraction + Capture Health
       -> 007/008/009 platform capture
       -> 010 speech conditioning + VAD + segmentation
            -> 011 model registry + Voice Runtime Router
                 -> 012 live transcription
                 -> 013 quality-pass transcription/alignment
                 -> 014 diarization + optional speaker vault

007/008/009 + 010 + 011 + 012/014
  -> 015 desktop runtime + dictation/input controls + meeting detection

005 + 010 + 011
  -> 019 media import + isolated quality workers
```

Unit 006 owns source health and observability. Unit 010 owns processing/VAD/segmentation behavior. Unit 011 owns executable engine/model identity and routing. This ownership boundary is required to prevent duplicated incompatible speech state machines.

## 006 — Capture abstraction and Capture Health

Keep the existing outcome and add these shaping requirements:

- report source attachment, route, silence, clipping, drop/backpressure, self-capture risk, and durable-checkpoint health without retaining or logging private content;
- expose stable health signals that Unit 010 can consume without embedding a specific VAD/denoiser into the capture contract;
- distinguish `no signal`, `silence`, `capture broken`, `source disappeared`, and `processing backpressure` when platform evidence permits;
- preserve raw/source lineage so later conditioning or ASR improvements never erase original evidence.

## 007/008/009 — Platform capture

At each platform-shaping frontier, explicitly compare the smallest relevant implementation paths from:

- OpenWhispr;
- Meetily;
- Anarlog;
- OpenSuperWhisper where platform-relevant;
- direct native APIs.

Select per platform rather than assuming one donor architecture is portable everywhere. Himsat contracts and lifecycle tests remain authoritative.

## 010 — Speech conditioning, VAD, and segmentation

Expand the current audio-processing outcome to include a swappable conditioning and speech-segmentation contract.

### Conditioning challengers

- Sonora;
- WebRTC AudioProcessing;
- DeepFilterNet;
- RNNoise;
- native platform facilities when justified.

### VAD challengers

- Silero VAD;
- TEN VAD;
- sherpa-onnx-hosted variants;
- a simple/non-neural baseline where useful for benchmark sanity.

### Required evidence

- AEC/NS/AGC behavior where applicable;
- VAD false positive/negative and boundary error;
- ASR WER before/after conditioning;
- quiet-speech damage;
- Arabic and Arabic-English code switching;
- far-field/Bluetooth/noise/music cases;
- CPU/RAM/latency/warm-up;
- battery/thermal on mobile;
- six-hour stability;
- exact native/model/package provenance.

No source wins because its own benchmark claims it is best.

## 011 — Model registry and Voice Runtime Router

The router must select from explicit Himsat route lanes rather than one universal engine:

```text
LIVE_GENERAL
MOBILE_LOW_RESOURCE
MULTILINGUAL_CODE_SWITCH
MEDICAL_SPECIALIZED
LONG_FORM_MULTI_SPEAKER
OFFLINE_QUALITY_IMPORT
```

Every route decision is constrained by language, task, device, resource budget, accelerator, locality/network policy, exact engine/model digests, model rights, runtime health, and requested capabilities.

### First-class native runtime challengers

- sherpa-onnx;
- whisper.cpp;
- NeMo-Speech.cpp;
- qualified platform-native acceleration where appropriate.

### Optional/heavier worker challengers

- full NeMo Speech/model tooling;
- Omnilingual ASR;
- MOSS Transcribe-Diarize;
- pyannote pipelines;
- MedASR;
- other selected specialized models.

A Python/Transformers/custom-code model is not automatically a core dependency. When selected, run it through an immutable, resource-bounded, capability-restricted isolated worker.

### Runtime requirements

- model/runtime integrity verification before load;
- accelerator preflight;
- explicit route explanation;
- no silent accelerator-to-CPU fallback when policy forbids it;
- no hidden local-to-cloud fallback;
- scrubbed diagnostics/error journal;
- crash-loop quarantine;
- resumable verified model side-load/download;
- explicit unsupported/unavailable results.

## 012 — Live transcription

Add acceptance dimensions for:

- first-token and finalization latency;
- partial revision stability;
- cancellation/backpressure;
- exact source-time mapping;
- device/resource class;
- Arabic/English/code-switch performance;
- long-session stability;
- deterministic dictation target insertion/fallback where shaped;
- route changes only through explicit policy, never hidden fallback.

## 013 — Quality-pass transcription and alignment

Require non-destructive quality revisions and compare route-specific challengers for:

- multilingual/Arabic quality;
- domain/medical specialization;
- long-form imported media;
- timestamp/alignment quality;
- resumability after worker failure.

The original live transcript remains historical evidence rather than being silently overwritten.

## 014 — Diarization and speaker vault

Compare both separate and joint architectures:

- sherpa-onnx diarization/speaker components;
- pyannote quality-pass workers;
- NeMo-Speech.cpp/Sortformer-class paths;
- MOSS joint transcription+diarization where qualified.

Measure DER/JER, overlap, speaker-count error, long-session label stability, timestamp drift, and ASR interaction.

Anonymous speaker segmentation is distinct from persistent biometric identity. Persistent speaker identity remains opt-in, encrypted, resettable, and deletable.

## 015 — Desktop runtime, dictation controls, and meeting detection

Explicitly arbitrate OpenWhispr, VoiceStudio, Meetily, Anarlog, OpenSuperWhisper, and native APIs for:

- meeting detection;
- capture/runtime survival;
- global shortcuts/push-to-talk;
- deterministic target insertion;
- model/runtime diagnostics;
- crash recovery;
- visible recording state.

Push-to-talk/global shortcut remains earlier than wake word.

## 019 — Media import

Allow heavier local quality workers because import is asynchronous, but require:

- bounded resources;
- process isolation where justified;
- resumability;
- hostile-media parsing boundaries;
- exact source mapping;
- immutable model/runtime identity;
- no unpinned dynamic remote code.

## Wake-word candidate sequencing

Do not create a mandatory always-listening dependency in Units 010-015.

A later wake-word leaf may compare openWakeWord and platform-native alternatives only after the Input Control Layer exists. It must be opt-in, local by default, visible, battery-qualified, false-activation-qualified, and use a non-durable or separately authorized detector ring buffer.

## Benchmark gates

Before choosing defaults, Himsat must execute comparable benchmarks across candidate engines on shared fixtures and target devices.

Minimum benchmark families:

```text
ASR: WER/CER, Arabic, English, code-switch, timestamp error
MEDICAL: medical-term WER, medication/dose/unit/number/negation errors
DIARIZATION: DER/JER, overlap, speaker count, long-session drift
VAD: false positive/negative, boundary error, quiet speech
CONDITIONING: intelligibility, ASR delta, speech damage
RUNTIME: cold/warm start, RTF, CPU/GPU/NPU, RAM, battery, thermal, six-hour stability
SUPPLY_CHAIN: hashes, offline startup, tamper rejection, notices, network denial
```

Public or donor-produced benchmark results are research inputs, not Himsat PASS evidence.

## Source-selection policy

For each active future leaf:

1. freeze the Himsat behavior contract first;
2. select only serious donor/native challengers;
3. pin exact source and model identities;
4. close code/model/data/asset provenance independently;
5. benchmark on Himsat fixtures/hardware;
6. reject any candidate that violates local-first/privacy/platform invariants;
7. adopt the smallest source/dependency surface that wins the required route;
8. keep replacement possible behind Himsat-owned interfaces;
9. add Himsat-owned positive/negative/failure/long-session tests;
10. preserve losing and negative evidence.

## Current authority remains unchanged

At the time this amendment was authored, Specification 004 remains the active specification and B104 is the next authorized implementation leaf under `specs/CURRENT.md` and live GitHub truth.

This amendment does not authorize Units 006-015, source-code adoption, model adoption, medical features, wake-word behavior, or any donor dependency before their own bounded future authority exists.
