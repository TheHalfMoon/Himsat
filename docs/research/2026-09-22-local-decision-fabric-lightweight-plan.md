# Himsat Lightweight Local Decision Fabric Plan
**Status:** planning amendment; no implementation authority  
**Date:** 2026-09-22  
**Founder direction:** ship a Jev-like typed-decision capability locally with Himsat, with no cloud inference and no hidden fallback, while keeping the application light.  
**Current canonical implementation frontier:** Specification 008 remains open on real-Windows Gate E evidence. This document does not advance that frontier.
## 1. Purpose
Himsat should gain a small, fast, local typed-decision subsystem for bounded classification, choice, ranking, scoring, abstention, and routing. The subsystem should provide the useful engineering properties associated with Jev-style System One decision interfaces without creating a production dependency on hosted Jev, a cloud model API, or one donor model.
The permanent product boundary is:
> **Himsat owns the decision contract, policy, calibration requirements, authority boundary, provenance, and product behavior. Models are replaceable local providers.**
The first implementation should be optimized for small install size, low memory, CPU-first execution, fast startup, predictable latency, and zero unexpected network egress.
## 2. Non-goals
This plan does not authorize:
- changing the active Specification 008 implementation frontier;
- claiming Gate E Windows hardware qualification;
- importing model weights into Git;
- adding a production model dependency before the model-registry/adoption authority exists;
- replacing deterministic policy with probabilistic decisions;
- granting a decision model permission, security, privacy, review, or verification authority;
- bundling a Python development environment as a normal product dependency;
- silently using cloud inference when a local model is unavailable;
- selecting Laya or any other candidate before Himsat-owned qualification;
- turning every classification into an AI call when deterministic code is sufficient.
## 3. Founder GitHub source-use expansion
The founder has stated that Himsat may copy/adapt source code available through the founder's GitHub access. This expands the source pool for engineering evaluation, but does not make every source an automatic dependency.
The selection rule remains:

> **Use the smallest technically superior reusable part behind Himsat-owned contracts.**

Exact source path, immutable revision, embedded third-party material, notices, model/data/asset rights, transitive dependencies, Himsat tests, and security qualification remain mandatory before adoption.

## 4. GitHub-wide source reconnaissance

The connected GitHub inventory contained 36 founder-accessible repositories at this planning pass.

### 4.1 Directly relevant sources

| Repository | Relevance to Local Decision Fabric | Reuse posture |
| --- | --- | --- |
| TheHalfMoon/kernux | Mature provider-neutral Decision Fabric plan: typed requests/results, calibration, abstention, deterministic eligibility, disagreement/escalation, local-only provider requirements | ADAPT architecture/contracts selectively |
| TheHalfMoon/Morize | Typed decision envelope, reason codes, abstention, confidence != authority, model-identity/change rules | ADAPT semantics selectively |
| TheHalfMoon/Inercative | Separation of deterministic/decision/generative intelligence; provider-neutral router; decision-adapter qualification | ADAPT routing/evaluation concepts |
| TheHalfMoon/MedScale | Real local offline ONNX execution path using tract-onnx, bounded artifact loading, signed pack/provenance, tokenizer with no HTTP acquisition | strongest implementation donor for a light local runtime path |
| TheHalfMoon/Golam | ExecutionProfile identity, quantization/device/runtime metadata, warm residency, local-only fallback policy | ADAPT runtime-profile semantics |
| TheHalfMoon/commandMed | Ultra-compact model tournament research, exact artifact provenance, size-first-after-quality methodology; Granite 350M Q4 research around 237 MB | REFERENCE / benchmark methodology and compact challengers |
| TheHalfMoon/Wispral | Explicit COMPACT/BALANCED resource tiers and preregistered model/runtime qualification methodology | ADAPT resource-tier and benchmark discipline |
| TheHalfMoon/MESC | Reproducibility, validator-grounded outputs, runtime/model provenance and controlled provider qualification | ADAPT evidence methodology |
| TheHalfMoon/SpecGrain | Bounded planning authority | methodology only |
| TheHalfMoon/Diffcipline | Risk/scope/proof finish-line discipline | methodology only |
| TheHalfMoon/Ascout | Verification receipts and source-inventory discipline | review/evidence reference |
| TheHalfMoon/Sentrdel | Local-first fail-closed security/evidence patterns | security reference |
| TheHalfMoon/Delethos | Independent delegation/review/isolation patterns | review isolation reference |
| TheHalfMoon/HarnessMind | Local-first/no-cloud/no-telemetry and observable-facts discipline | observability reference |
| TheHalfMoon/Paina | Evidence-backed policy-learning research direction | future research reference |

### 4.2 Adjacent or lower-direct-relevance repositories

The following repositories were included in the GitHub-wide inventory and were checked for reusable direction, but no stronger Local Decision Fabric implementation donor was identified than the sources above during this pass:

- TheHalfMoon/Fanatir
- TheHalfMoon/Kodac
- TheHalfMoon/Hikma
- TheHalfMoon/Pluma
- TheHalfMoon/Winds
- TheHalfMoon/Coddev
- TheHalfMoon/commandF
- TheHalfMoon/wepld
- TheHalfMoon/ProtocolWISE
- TheHalfMoon/Trcel (empty repository at inspection time)
- TheHalfMoon/kodac-phase-b-gate
- TheHalfMoon/MSTR
- TheHalfMoon/Golam-research
- TheHalfMoon/Ecra
- TheHalfMoon/Tarif
- TheHalfMoon/Signthos
- TheHalfMoon/Qdrat
- TheHalfMoon/Zyara
- TheHalfMoon/Balott
- TheHalfMoon/acarat

This classification is not a permanent rejection. If a future bounded unit identifies a better reusable component in any founder repository, it should be compared at that time.

### 4.3 Important source distinctions

Do not conflate:

- aayushch/laya: local-first event/action application architecture;
- convaiinnovations/laya: Jev-like typed-decision model family.

The second is the relevant model candidate for this plan.

## 5. Candidate model/provider pool

### 5.1 Primary specialized candidate — Laya

convaiinnovations/laya is the first specialized candidate because current upstream metadata describes a compact text-classification / typed-decision model with calibrated-decision/routing/scoring intent.

Observed planning facts must be reverified at adoption time:

- approximately 421M parameters;
- Apache-2.0 model repository metadata at the observed revision;
- classification-oriented rather than free-form text generation;
- intended bounded decision/scoring use.

Himsat must independently prove its real artifact size, runtime compatibility, Arabic behavior, calibration, abstention, latency, memory, and offline packaging.

### 5.2 SemIf-style direct scoring

TheoLeeCJ/SemIf is a strong algorithm/reference candidate for direct option-logit scoring from ordinary local models.

Useful ideas:

- avoid generating prose that is immediately reparsed;
- score known options directly;
- expose normalized probability/score semantics;
- preserve a small deterministic interface;
- support reusable prefixes/state;
- keep calibration measurable.

If SemIf code is adopted, use the smallest source subset required behind Himsat contracts.

### 5.3 Decider-style typed wire surface

Mapika/decider is a useful candidate/reference for typed boolean/choice/score decision surfaces, option-cardinality handling, schema caching, and TypeSafe-compatible concepts.

It is not architecture authority.

### 5.4 Compact general-model challengers

The founder's commandMed research already identified low-resource candidates and artifact evidence useful for a Himsat tournament. Examples include:

- IBM Granite 4.0 350M base with an observed official Q4_K_M artifact around 237 MB;
- Qwen 0.6B/0.8B class candidates;
- smaller controls such as SmolLM2 360M and other ultra-compact families.

These are challengers, not default winners. A specialized 421M classifier may outperform a similarly sized generative model for bounded decisions and may have lower runtime overhead.

### 5.5 Deterministic provider

A deterministic provider is always available for decisions reducible to exact rules.

Examples:

- schema validity;
- exact duplicate identity;
- hard privacy classification;
- source permission state;
- hard capability constraints;
- explicit user choices;
- resource bounds;
- platform support truth.

This provider is part of the product even when no model pack is installed.

## 6. Himsat-owned contract

Future implementation should create a provider-neutral contract conceptually equivalent to:

    DecisionRequest
      -> deterministic eligibility/policy filter
      -> LocalDecisionProvider
      -> DecisionResult
      -> deterministic downstream policy

### 6.1 Request fields

A request should carry:

- request ID;
- workload ID;
- bounded state/context;
- question;
- answer type;
- options/rubric;
- input provenance/evidence refs where available;
- language;
- maximum context;
- latency budget;
- memory/resource budget;
- required calibration profile;
- privacy mode;
- provider allow/deny set;
- cancellation/deadline;
- consequence class.

Supported answer families should include:

- BOOLEAN;
- CHOICE;
- ORDINAL_SCORE;
- RANK;
- ABSTAIN / NO_FIT.

No provider is required to support every family.

### 6.2 Result fields

A result should record:

- provider identity;
- model identity;
- immutable model revision;
- artifact digest;
- tokenizer/config digest;
- runtime identity/revision;
- quantization/precision identity;
- request/schema digest;
- selected value;
- probability distribution or exact documented score semantics;
- calibration profile and revision;
- abstention/OOD signal where available;
- latency;
- device/backend;
- peak or sampled resource metadata where measured;
- warnings for truncation/degraded mode;
- local data-boundary assertion backed by runtime evidence.

## 7. Authority boundary

A DecisionResult is advisory evidence.

It cannot:

- grant permission;
- lower consequence;
- approve its own side effect;
- lower a deterministic privacy/security class;
- bypass user approval;
- mark evidence verified;
- mark tests or CI passed;
- promote memory trust;
- declare a source reliable;
- declare a release ready;
- override SpecGrain/Diffcipline/governance.

If probabilistic output conflicts with hard deterministic policy, hard policy wins.

    CONFIDENCE != AUTHORITY
    MODEL_OUTPUT != PERMISSION
    MODEL_OUTPUT != VERIFICATION

## 8. Lightweight product architecture

The app must remain light even when a local decision capability is included.

### 8.1 Core binary vs decision pack

Do not bake model weights into the Rust executable.

Ship two separable artifacts:

1. **Himsat core application**
   - no model weights;
   - no Python runtime;
   - no hidden model downloader;
   - deterministic decision provider always available.

2. **Himsat Lite Decision Pack**
   - installed locally with the normal offline-capable distribution;
   - signed/content-addressed;
   - model/tokenizer/calibration/runtime metadata;
   - independently removable/reinstallable;
   - versioned separately from the app.

The standard user experience can still come with the local model by packaging the Decision Pack in the installer/offline bundle while preserving a small independently updatable core.

### 8.2 Provisional resource budgets

These are planning targets, not evidence claims:

    DEFAULT_DECISION_PACK_TARGET      <= 256 MiB
    DEFAULT_DECISION_PACK_HARD_CAP    <= 384 MiB
    EXCEPTIONAL_QUALITY_CAP           <= 512 MiB only with measured justification

    PEAK_INCREMENTAL_RAM_TARGET       <= 700 MiB
    PEAK_INCREMENTAL_RAM_HARD_CAP     <= 1 GiB on baseline desktop qualification

    GPU_REQUIRED                      NO
    NETWORK_REQUIRED_AFTER_INSTALL    NO
    PYTHON_REQUIRED_IN_PRODUCT        NO
    MULTIPLE_MODELS_RESIDENT_DEFAULT  NO

A candidate exceeding the default hard cap must prove a material product-quality gain and still cannot become the default if it breaks baseline hardware support.

### 8.3 Residency policy

Use lazy load and explicit eviction:

- do not load the model at application startup unless required;
- keep at most one default decision model resident;
- keep-warm TTL is bounded and configurable;
- release model memory under OS memory pressure;
- background tasks do not pin the model indefinitely;
- preload is optional only after startup/resource measurements justify it.

### 8.4 CPU-first

The baseline local path must work on CPU.

Accelerators may improve performance but may not be required for correctness.

Platform acceleration should be optional:

- Metal/CoreML/MLX-like path only when separately qualified;
- CUDA/DirectML/etc. only as optional acceleration;
- CPU remains the portable fallback inside LOCAL_ONLY policy.

## 9. Preferred runtime strategy

### 9.1 First runtime candidate — pure Rust ONNX path

The strongest existing founder implementation donor for a light runtime is the MedScale tract-onnx path.

Useful proven design patterns include:

- pure local execution;
- no Hugging Face HTTP runtime dependency;
- bounded artifact reads;
- digest verification before execution;
- verified bytes passed directly to runtime to avoid post-verification swap/growth;
- prepared reusable sessions;
- fail-closed input/output shape limits;
- model-source provenance bound to runtime output.

Himsat should adapt these patterns, not copy the MedScale clinical/domain authority model.

### 9.2 Laya-to-ONNX qualification

Do not assume Laya can be converted to ONNX correctly.

Before selecting this path, prove:

1. exact upstream architecture is supported;
2. export is deterministic enough to provenance-pin;
3. tokenizer/template behavior is preserved;
4. output choice/score semantics match reference inference;
5. calibration remains valid or is refitted;
6. no unsupported custom ops are required;
7. quantization does not invalidate output quality;
8. all required languages remain qualified.

A converted model is a new execution identity.

### 9.3 Secondary runtime candidates

If ONNX cannot satisfy the contract:

- mistral.rs is the preferred Rust-native general-model candidate to evaluate;
- llama.cpp is a compatibility candidate, preferably isolated out-of-process if C/C++ FFI materially increases crash/security risk;
- Candle/Burn/ORT remain benchmark candidates where they materially improve size, portability, or accelerator support.

No runtime is architecture authority.

## 10. Provider process isolation

Prefer an isolated local worker when it materially reduces crash/resource risk.

The worker must have:

- narrow typed IPC;
- no ambient vault access;
- no arbitrary filesystem access;
- no credentials;
- no outbound network in LOCAL_ONLY mode;
- bounded memory/CPU/time;
- explicit model-pack directory read scope;
- crash supervision;
- restart backoff;
- no authority state.

A worker crash must not corrupt the vault, media journal, transcript lineage, or durable evidence.

In-process inference may be allowed only if the selected runtime proves a meaningfully smaller and safer deployment boundary.

## 11. No-silent-cloud invariant

Allowed failure responses for a local decision provider:

1. retry locally under a bounded retry policy;
2. choose another already-qualified local provider;
3. abstain;
4. degrade to deterministic-only mode;
5. surface a local capability limitation.

Not allowed:

- calling a hosted model;
- sending content to a cloud API;
- activating a remote fallback because latency/quality is poor;
- downloading a model during inference without explicit update/acquisition authority.

## 12. Packaging and update model

### 12.1 Decision Pack contents

A future pack should contain or reference:

- manifest;
- model artifact;
- tokenizer/config;
- label/answer metadata if applicable;
- calibration profile;
- runtime compatibility metadata;
- immutable upstream source identity;
- conversion/quantization lineage;
- SHA-256 digests;
- license/permission/notice metadata;
- Himsat qualification record ID.

### 12.2 Install modes

Support:

- standard offline installer with Lite Decision Pack included;
- core-only install for constrained environments;
- offline side-load of a verified pack;
- explicit local update/replacement.

Core-only mode must remain functional with deterministic decisions and explicit feature degradation.

### 12.3 Model cache lifecycle

Define:

- location;
- quota;
- atomic install;
- digest verification;
- rollback to prior qualified pack;
- deletion;
- cleanup after failed install;
- no deleted pack left in shadow indexes;
- user-visible installed size.

## 13. Qualification tournament

Selection must be based on Himsat-owned evidence, not model-card claims.

### 13.1 Candidate lanes

At minimum:

- Laya exact upstream/reference inference;
- Laya converted/quantized local candidate if technically valid;
- SemIf-style direct scoring on the smallest qualified practical model;
- at least one compact general-model challenger from the founder's existing low-resource research;
- deterministic baseline.

### 13.2 Workload fixtures

Include:

- English;
- Arabic;
- Arabic dialectal text where realistic;
- mixed Arabic/English;
- RTL/non-Latin;
- transcript punctuation errors;
- ASR substitutions/deletions;
- speaker-label mistakes;
- short state;
- long state;
- many options;
- near ties;
- ambiguous inputs;
- out-of-domain inputs;
- adversarial/prompt-injected text;
- contradictory evidence;
- intentionally insufficient evidence.

### 13.3 Himsat-specific tasks

Candidate evaluation should include later product tasks such as:

- decision / commitment / question / risk classification;
- relevance classification;
- notification/brief routing;
- source/evidence candidate ranking;
- contradiction candidate detection;
- transcript segment routing;
- insufficient-evidence abstention;
- meeting-context category selection.

Evidence-sensitive uses remain blocked until the relevant evidence-engine dependency exists.

### 13.4 Metrics

Measure independently:

- task accuracy or workload utility;
- macro F1 where appropriate;
- Brier score;
- ECE or another explicit calibration metric;
- selective accuracy;
- abstention precision/recall;
- OOD false-confidence rate;
- false-automatic-progression rate;
- Arabic/English gap;
- code-switch gap;
- repeatability;
- cold start;
- warm p50/p95 latency;
- throughput;
- peak RAM;
- package bytes;
- CPU utilization;
- accelerator-specific results where applicable;
- idle residency;
- failure/recovery behavior.

Do not collapse these into one score that can hide a hard failure.

### 13.5 Selection order

Use hard gates before optimization:

1. provenance and redistribution;
2. no-cloud/offline proof;
3. schema correctness;
4. authority-boundary safety;
5. minimum task quality;
6. Arabic/code-switch minimum floor;
7. calibration/abstention floor;
8. baseline hardware/resource ceiling;
9. then minimize package size;
10. then minimize latency/resource use.

The smallest model wins only among candidates that pass all hard quality/safety gates.

## 14. Calibration and abstention

Calibration is workload-specific.

Requirements:

- no global confidence threshold;
- calibration profile binds model + runtime + quantization + dataset/version;
- changing quantization may require recalibration;
- changing tokenizer/template may require a new provider identity;
- low-margin/near-tie requests should abstain or escalate locally;
- high-confidence wrong-answer rate is measured explicitly;
- confidence may route review but cannot grant authority.

## 15. Arabic and multilingual requirements

Arabic is a first-class acceptance axis, not a later localization patch.

Qualification must include:

- Modern Standard Arabic;
- Saudi/Gulf conversational forms where redistributable fixtures exist;
- Arabic names/organizations/projects;
- mixed Arabic-English technical conversation;
- Arabic numerals/dates;
- punctuation/diacritics variation;
- RTL rendering of decision explanations where surfaced.

A candidate with materially weak Arabic behavior cannot be the universal default merely because its English benchmark is strong.

## 16. Privacy and threat model

Threats include:

- model package tampering;
- malicious tokenizer/config;
- model/runtime parser bugs;
- prompt-injected transcript/doc text;
- data exfiltration through SDK telemetry;
- hidden HTTP dependency;
- oversized/malformed model artifacts;
- resource exhaustion;
- malicious labels/schema;
- unsafe native runtime behavior;
- cross-vault data leakage;
- stale calibration reused after model change.

Mitigations:

- signed/content-addressed packs;
- size/resource ceilings before parse/load;
- no executable model code/custom ops by default;
- isolated worker where appropriate;
- network-deny qualification;
- exact provider identity;
- bounded request context;
- deterministic authority layer outside the model;
- crash/restart containment;
- fuzz/adversarial pack parsing.

## 17. Network privacy proof

Qualification must prove the local decision path under:

- no Internet;
- DNS blocked;
- external HTTP blocked;
- no API keys;
- telemetry disabled;
- update checks disabled;
- empty cloud-provider configuration.

Evidence should record process/component, attempted destinations, bytes, and expected loopback IPC.

The target claim is:

> **Zero unexpected outbound connections from the Himsat decision path.**

## 18. Upgrade and replacement semantics

A material model change creates a new decision-engine identity.

Material changes include:

- model revision;
- tokenizer;
- prompt/template;
- runtime;
- quantization;
- calibration mapping;
- label/option encoding;
- scoring algorithm.

Cached decisions are not automatically valid under a new identity.

Historical decisions retain original engine identity.

## 19. Product integration boundaries

### 19.1 Early allowed uses after future authority

The first production uses should be bounded and reversible, such as:

- routing;
- tagging;
- prioritization;
- candidate ranking;
- review recommendation;
- abstention.

### 19.2 Later evidence-sensitive uses

After the Evidence Engine and memory dependencies exist:

- decision/commitment extraction candidates;
- contradiction candidates;
- brief relevance;
- temporal-memory update candidates;
- evidence-support scoring.

The model still produces candidates/observations, not canonical facts.

### 19.3 External actions

No local decision model may directly execute connector/tool actions.

Himsat Flows/action proposals remain governed by deterministic capability/approval policy.

## 20. Roadmap placement

Do not renumber the existing 000–040 roadmap from this planning amendment.

Preferred dependency placement:

    011 Model registry + speech-engine contract
      -> Local Decision Fabric foundation candidate

    025 Evidence engine
      + Local Decision Fabric
      -> evidence-sensitive typed decisions

    027 Intelligence + Himsat Brief
      -> deep Local Decision Fabric use

    031/033
      -> optional decision-assisted routing/proposals
         without authority transfer

The Local Decision Fabric may ultimately become:

- a bounded child of 011;
- a separately numbered successor selected later;
- or a cross-cutting provider layer introduced when its first real consumer is shaped.

SpecGrain decides the actual implementation unit when the dependency frontier permits it.

## 21. Lightweight acceptance targets to carry into shaping

Future shaping should begin with these provisional targets:

- default decision pack target <= 256 MiB;
- hard default cap <= 384 MiB;
- CPU-only support mandatory;
- no Python runtime in normal product;
- no cloud inference;
- no hidden model download during inference;
- one default model resident at most;
- lazy load;
- memory-pressure eviction;
- deterministic-only fallback;
- Arabic + English qualification mandatory;
- pack rollback/delete supported;
- exact model/runtime/calibration identity visible in diagnostics.

Any exception requires measured evidence and explicit planning amendment.

## 22. Reuse map

### From MedScale

Candidate reusable patterns/code:

- bounded local ONNX runtime;
- verified-bytes execution;
- signed/content-addressed pack concepts;
- runtime provenance;
- no-HTTP tokenizer/runtime configuration;
- artifact size and shape refusal.

Do not import clinical authority semantics.

### From Kernux

Candidate reusable concepts/types:

- DecisionRequest/DecisionResult shape;
- deterministic provider eligibility;
- disagreement/escalation;
- calibration metadata;
- local-provider privacy rules.

### From Morize

Candidate reusable concepts:

- confidence != authority;
- stable reason codes;
- abstention;
- material engine-change identity;
- replay/audit envelope.

### From Ineractive

Candidate reusable concepts:

- deterministic vs decision vs generative intelligence split;
- provider-neutral routing;
- exact fallback lineage;
- decision qualification metrics.

### From Golam

Candidate reusable concepts:

- execution profile identity;
- warm residency;
- device/runtime/quantization metadata;
- local-only fallback class.

### From commandMed/Wispral

Candidate reusable methodology:

- freeze candidate identity before results;
- quality floor before size ranking;
- explicit COMPACT/BALANCED resource tiers;
- exact model byte counts;
- no benchmark winner claim without execution evidence.

## 23. Implementation sequence once authorized

A future implementation should be split into bounded grains:

1. contracts only;
2. pack/provenance schema;
3. deterministic fake/local provider for tests;
4. isolated worker/runtime boundary;
5. first real runtime adapter;
6. exact Laya/reference candidate materialization;
7. conversion/quantization experiment if required;
8. calibration/abstention harness;
9. Arabic/English qualification corpus;
10. resource/network qualification;
11. default-provider selection;
12. installer/offline-pack integration;
13. first bounded product use;
14. reconciliation and long-run regression.

Do not combine model adoption, runtime, installer, calibration, and product usage into one giant grain.

## 24. Review requirements

Every implementation grain must preserve Himsat's existing CI/R3, SpecGrain, Diffcipline, provenance, and exact-head rules.

Use Alibaba Open Code Review where file types are supported.

Record:

- tool version;
- execution mode;
- reviewed files;
- excluded files;
- findings;
- dispositions.

0 reviewed files is not PASS.

Model/runtime/security-sensitive adoption is R3-class work unless the active specification proves a narrower risk.

## 25. Kill criteria

Do not force this subsystem into the product if qualification shows:

- default pack cannot remain within acceptable size/resource ceilings;
- Arabic quality is materially inadequate;
- calibration cannot be made reliable enough for intended bounded uses;
- local runtime is unstable or unsafe;
- hidden network/telemetry cannot be eliminated;
- deterministic rules solve the intended workload more reliably and cheaply;
- maintenance/security cost outweighs user value.

If Laya fails, the Local Decision Fabric survives and another provider may qualify.

## 26. Final planning position

The recommended architecture is:

    Himsat deterministic authority
            |
            v
    Himsat Local Decision Fabric
            |
            +-- DeterministicProvider
            |
            +-- LiteLocalModelProvider
                  |
                  +-- Laya candidate
                  +-- SemIf-style candidate
                  +-- compact general-model challenger
                  +-- future provider

The preferred first runtime experiment is a lightweight, offline, pure-Rust ONNX path adapted from the already-proven MedScale implementation patterns, with Laya as the first specialized candidate if exact export/runtime equivalence is demonstrated.

The default product should remain light by keeping the model in a separately versioned Lite Decision Pack included with the offline installer, lazily loaded, evictable, CPU-capable, and independently removable.

No cloud inference is part of the default architecture.

Implementation remains unauthorized until the canonical Himsat dependency frontier and SpecGrain state permit it.
