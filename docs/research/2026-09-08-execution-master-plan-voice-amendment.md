# Himsat Execution Master Plan — Voice Hardening Amendment

Date: 2026-09-08  
Status: roadmap supplement after canonical merge; `specs/CURRENT.md` remains controlling

## Purpose

Supplement the voice-related candidate units in `docs/execution-master-plan.md` with the decisions in `2026-09-08-voice-source-plan-hardening.md`. This file changes no active Specification 004 authority, dependency, model, donor byte, or runtime behavior.

## Refined dependency ownership

```text
005 durable media
 -> 006 capture abstraction + source health
    -> 007/008/009 platform capture
    -> 010 conditioning + VAD + segmentation
       -> 011 model registry + Voice Runtime Router
          -> 012 live transcription
          -> 013 quality-pass transcription/alignment
          -> 014 diarization + optional speaker vault

007/008/009 + 010 + 011 + 012/014 -> 015 desktop runtime/controls/meeting detection
005 + 010 + 011 -> 019 media import + isolated quality workers
```

- Unit 006 owns source/capture health and observability.
- Unit 010 owns conditioning, VAD, and segmentation behavior.
- Unit 011 owns executable engine/model identity and route selection.

## 006 — Capture abstraction and Capture Health

Add source attachment/route/silence/clipping/drop/backpressure/self-capture/checkpoint observability without content logging. Preserve source lineage and expose health to Unit 010 without embedding one VAD/denoiser in the capture contract.

## 007/008/009 — Platform capture

At shaping time compare the smallest relevant paths from OpenWhispr, Meetily, Anarlog, OpenSuperWhisper where relevant, and direct native APIs. Select per platform and prove permissions, route changes, interruptions, sleep/wake, device swaps, clock continuity, and long-session behavior.

## 010 — Conditioning, VAD, and segmentation

Conditioning challengers:

```text
Sonora
WebRTC AudioProcessing
DeepFilterNet
RNNoise
qualified native platform facilities
```

VAD challengers:

```text
Silero VAD
TEN VAD
sherpa-onnx-hosted variants
simple baseline where useful
```

Require VAD miss/false activation/boundary evidence, Arabic/code-switch/noise/Bluetooth cases, ASR WER before/after conditioning, quiet-speech damage, CPU/RAM/latency/warm-up, mobile battery/thermal, six-hour stability, and exact model/native provenance. Upstream self-benchmarks are not Himsat PASS evidence.

## 011 — Model registry and Voice Runtime Router

Use explicit route lanes rather than one universal engine:

```text
LIVE_GENERAL
MOBILE_LOW_RESOURCE
MULTILINGUAL_CODE_SWITCH
MEDICAL_SPECIALIZED
LONG_FORM_MULTI_SPEAKER
OFFLINE_QUALITY_IMPORT
```

First-class native challengers: sherpa-onnx, whisper.cpp, NeMo-Speech.cpp, and qualified platform-native acceleration. Heavy/custom-code challengers such as full NeMo tooling, Omnilingual ASR, MOSS, pyannote, or MedASR remain optional workers unless exact evidence justifies more.

The router must consider task, languages, device, RAM, accelerator, battery/thermal policy, locality/network lock, exact engine/model digests, model rights, runtime health, and requested capabilities. No hidden local-to-cloud or forbidden accelerator-to-CPU fallback.

Require integrity verification, accelerator preflight, route explanation, scrubbed diagnostics, crash-loop quarantine, verified resumable model acquisition/side-load, update rollback, and explicit unsupported/unavailable states.

Python/Transformers/custom/remote-code models default to immutable resource-bounded capability-restricted worker consideration. Native runtimes still require parser/native-library risk assessment.

## 012 — Live transcription

Require first-token/finalization latency, partial-revision stability, cancellation/backpressure, exact source-time mapping, Arabic/English/code-switch evidence, long-session stability, deterministic dictation insertion/fallback where shaped, and explicit route-transition boundaries.

## 013 — Quality-pass transcription and alignment

Keep quality revisions non-destructive. Compare multilingual, medical/domain, long-form, and alignment challengers; preserve source-time mapping and recovery after worker failure.

## 014 — Diarization and speaker vault

Compare separate and joint pipelines including sherpa-onnx, pyannote, NeMo-Speech.cpp/Sortformer-class paths, and MOSS where qualified. Measure DER/JER, overlap, speaker-count error, timestamp drift, long-session label stability, and ASR interaction.

Anonymous speaker segmentation is not persistent biometric identity. Persistent speaker identity remains opt-in, encrypted, resettable, and deletable.

## 015 — Desktop runtime and controls

Explicitly arbitrate OpenWhispr, VoiceStudio, Meetily, Anarlog, OpenSuperWhisper, and native APIs for meeting detection, runtime survival, global shortcuts/push-to-talk, deterministic target insertion, diagnostics, recovery, and visible recording state. Push-to-talk/global shortcut precedes wake word.

## 019 — Media import

Permit heavier local quality workers only with bounded resources, isolation when justified, resumability, hostile-media boundaries, exact source mapping, immutable runtime/model identity, and no unpinned dynamic remote code.

## Cross-cutting gates

Future voice shaping must additionally prove:

- mic/system-audio clock drift measurement and bounded correction;
- route hysteresis so language/device changes do not thrash models mid-utterance;
- terminology/hotword recall gains without unacceptable false insertion;
- engine confidence is stored/calibrated by engine identity rather than treated as a universal probability;
- model/native parser isolation decisions are evidence-based;
- model updates preserve rollback to the last qualified route;
- benchmark corpora have explicit license/permission, privacy, redistribution, reference-transcript, scoring, and contamination metadata.

## Benchmark minimums

```text
ASR: WER/CER, Arabic, English, code-switch, timestamp error
MEDICAL: term WER, medication/dose/unit/number/negation errors
DIARIZATION: DER/JER, overlap, speaker count, long-session drift
VAD: false positive/negative, boundary error, quiet speech
CONDITIONING: intelligibility, ASR delta, speech damage
RUNTIME: cold/warm start, RTF, CPU/GPU/NPU, RAM, battery, thermal, six-hour stability
SUPPLY_CHAIN: hashes, offline startup, tamper rejection, notices, network denial
```

## Selection rule

For every future active leaf: freeze Himsat behavior first; pin serious candidates; close code/model/data/asset provenance separately; benchmark on Himsat fixtures/hardware; reject invariant violations; adopt the smallest surface that wins its route; keep replacement behind Himsat contracts; add positive/negative/failure/long-session tests; preserve losing and negative evidence.

## Wake-word sequencing

Wake word is not a prerequisite for Units 010-015. A later opt-in leaf may compare openWakeWord/native alternatives only after the Input Control Layer exists and must prove local processing, visible enablement, bounded/non-durable detector buffering, false activation/rejection, language/accent behavior, battery/thermal cost, interruption recovery, model integrity, and disable/reset behavior.

## Authority boundary

At authoring time Specification 004 is active and B104 is the next authorized implementation leaf. This amendment does not authorize voice implementation, donor/model adoption, medical claims, wake-word behavior, or any blocked successor.
