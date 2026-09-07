# Himsat Voice-Source Authorization Extension

Date: 2026-09-08
Status: founder/user attestation recorded for planning and future bounded adoption; not automatic adoption authority

## Purpose

This record extends the source-use authorization evidence for the voice, speech, diarization, audio-conditioning, and wake-word sources newly introduced or made implementation-relevant after the repository snapshot covered by `governance/provenance/source-use-authorization.md`.

The founder/user explicitly stated that Himsat has permission to use and copy the source code from the sources discussed for this project and the sources already recorded in the repository. Himsat records that statement as a project-level permission input for the sources listed below.

This file does not assert that every byte, model, dataset, binary, asset, generated file, submodule, vendor subtree, or transitive dependency in an upstream repository belongs to the same rightsholder or is controlled by the same permission. Exact adoption still requires path-level provenance.

## Authorization state

```text
AUTHORIZATION_RECORDED_DATE = 2026-09-08
AUTHORIZATION_BASIS = FOUNDER_USER_ATTESTATION
PERMITTED_REUSE_MODES = REFERENCE | COPY | ADAPT | DEPEND | VENDOR
AUTOMATIC_ADOPTION = NO
ACTIVE_SPEC_AUTHORITY_CHANGED = NO
PROVENANCE_GATES_WAIVED = NO
LICENSE_NOTICE_GATES_WAIVED = NO
MODEL_DATA_ASSET_GATES_WAIVED = NO
SECURITY_QUALIFICATION_WAIVED = NO
MERGE_QUALIFICATION_WAIVED = NO
```

## Newly covered voice-source set

The permission attestation recorded here covers Himsat evaluation and, when a future authorized implementation leaf selects it, bounded source-code reuse from these newly explicit source repositories:

- `NVIDIA/NeMo-Speech.cpp`;
- `NVIDIA-NeMo/Speech`;
- `snakers4/silero-vad`;
- `TEN-framework/ten-vad`;
- `Rikorose/DeepFilterNet`;
- `xiph/rnnoise`;
- `facebookresearch/omnilingual-asr`;
- `Google-Health/medasr`;
- `Omi-Health/omi-med-stt-runtime`;
- `OpenMOSS/MOSS-Transcribe-Diarize`;
- `dscripka/openWakeWord`.

Previously recorded covered sources such as `OpenWhispr/openwhispr`, `Zackriya-Solutions/meetily`, `debpalash/VoiceStudio`, `Starmel/OpenSuperWhisper`, `ggml-org/whisper.cpp`, `k2-fsa/sherpa-onnx`, `argmaxinc/argmax-oss-swift`, `moonshine-ai/moonshine`, `dignifiedquire/sonora`, `tonarino/webrtc-audio-processing`, and `pyannote/pyannote-audio` remain governed by the earlier authorization plus their exact public-license/provenance evidence.

## Observed research anchors

These are research-time upstream identities observed while hardening the plan. They are not immutable adoption pins. Any future adoption must reverify live upstream truth and record the exact selected revision/path again.

```text
NVIDIA/NeMo-Speech.cpp                 ffa38cb2408f1e832a36d46fef5e3e1e80d07e6c
NVIDIA-NeMo/Speech                     de26b36962fe69751c8a4286e9c1969fa9b5e2db
snakers4/silero-vad                    867c2aa692646a1f1de3e94a15c9dd9f614c0acb
TEN-framework/ten-vad                  22a3bcd4509d0faaa8eef4881e8af5f39c178950
Rikorose/DeepFilterNet                 d375b2d8309e0935d165700c91da9de862a99c31
xiph/rnnoise                           70f1d256acd4b34a572f999a05c87bf00b67730d
facebookresearch/omnilingual-asr       81f51e224ce9e74b02cc2a3eaf21b2d91d743455
Google-Health/medasr                   ad843cb81b3e610e1868ed38f7230a70b66ed7e8
Omi-Health/omi-med-stt-runtime         41689b213622b0bf87cdcd75047b272facec9393
OpenMOSS/MOSS-Transcribe-Diarize       61bc29cd4120be7b5d3b761b64cd5dff57263642
dscripka/openWakeWord                  368c03716d1e92591906a84949bc477f3a834455
```

## Required separation of rights and provenance

Founder permission makes source-code reuse eligible for engineering evaluation. It does not merge independent supply-chain objects into one permission bucket.

Before adoption, Himsat must separately close, as applicable:

1. exact source repository, revision/tag, and source paths;
2. public license/terms for those exact paths;
3. separate permission scope when public terms are insufficient for intended distribution/use;
4. copyright, NOTICE, attribution, trademark, and patent obligations;
5. generated/vendor/submodule/transitive code;
6. model weights and model cards/terms;
7. datasets, benchmark corpora, synthetic corpora, and evaluation rights;
8. downloadable runtime binaries and native libraries;
9. fonts, icons, sample media, templates, and other assets;
10. exact Himsat destination paths, adaptation rationale, and Himsat-owned behavior tests.

Examples of independent boundaries that must remain explicit include MedASR code versus its model terms, Omi runtime code versus model weights, RNNoise code versus downloaded model artifacts, and MOSS model/runtime code versus any remote-code execution path or external model dependency.

## Active-authority boundary

This extension is planning/provenance evidence only. It does not pull forward voice implementation while Specification 004 is active.

```text
FOUNDER_PERMISSION != ACTIVE_GRAIN_AUTHORITY
FOUNDER_PERMISSION != MACHINE_PROVENANCE_CLOSURE
FOUNDER_PERMISSION != MODEL_OR_DATASET_APPROVAL
FOUNDER_PERMISSION != SECURITY_QUALIFICATION
FOUNDER_PERMISSION != BENCHMARK_SUPERIORITY
FOUNDER_PERMISSION != MERGE_APPROVAL
```

No source named here may enter the canonical dependency/adoption graph merely because this file exists. The selected future specification must choose the smallest technically superior reuse mode, register exact adopted material through Himsat provenance machinery, and prove the resulting Himsat behavior.
