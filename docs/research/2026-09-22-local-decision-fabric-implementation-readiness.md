# Himsat Local Decision Fabric — Implementation Readiness Plan
**Status:** implementation-ready planning package; no current implementation authority  
**Date:** 2026-09-22  
**Depends on:** the canonical Lightweight Local Decision Fabric plan and future activation of its dependency frontier  
**Current project constraint:** Specification 008 remains active until real-Windows Gate E is closed
## 1. Objective
This document turns the Local Decision Fabric direction into an execution package that can be implemented without repeating broad architecture research once the dependency frontier authorizes it.
The target is a small, fully local, Jev-like typed-decision subsystem that:
- runs on-device after installation;; requires no cloud inference or API key;; has no telemetry dependency or silent remote fallback;; keeps deterministic policy authoritative;; exposes calibrated typed decisions instead of free-form prose;; is provider/runtime replaceable;; preserves Himsat provenance, privacy, security, offline, evidence, and recovery rules;; remains light enough that recording and transcription reliability always outrank decision inference.
## 2. Frozen implementation direction
Unless later evidence disproves them, future implementation should preserve these decisions:
1. Himsat owns DecisionRequest, DecisionResult, DecisionProvider, routing, policy, calibration identity, and authority semantics.
2. Decision outputs are observations, never authority.
3. Deterministic rules run before model routing.
4. The default decision path has no cloud branch.
5. Model weights are not linked into the Himsat core executable.
6. The normal product ships a separately versioned local Decision Pack.
7. CPU-only correctness is mandatory; accelerators are optional.
8. Python is not required in the normal product.
9. Only one default decision model is resident at a time.
10. Model loading is lazy and evictable.
11. Capture/transcription resource pressure wins over decision inference.
12. Arabic, English, and mixed Arabic/English are first-class qualification axes.
13. Laya is the first specialized candidate, not a preselected winner.
14. A pure-Rust ONNX path adapted from MedScale is the first lightweight runtime experiment.
15. SemIf-style direct scoring and compact general-model challengers remain valid alternatives.
16. Large Decider-family checkpoints are benchmark/reference controls unless a smaller artifact fits the Himsat resource envelope.
17. Model/runtime/tokenizer/quantization/scoring/calibration changes create explicit engine identity changes.
18. CONFIDENCE != AUTHORITY.
19. MODEL_OUTPUT != PERMISSION.
20. MODEL_OUTPUT != VERIFICATION.
## 3. Dependency placement
This plan does not advance the current Specification 008 frontier.
Preferred future dependency placement:
    011 Model registry + model runtime foundation
      -> Local Decision Fabric foundation
          -> typed contracts
          -> Decision Pack identity
          -> local runtime
          -> provider qualification
    025 Evidence engine
      + Local Decision Fabric
      -> evidence-sensitive typed decisions
    027 Intelligence + Himsat Brief
      + qualified Local Decision Fabric
      -> production intelligence routing/classification
    031/033 and later action surfaces
      -> optional decision-assisted proposals/routing
      -> deterministic capability and approval authority remains controlling
The future active SpecGrain unit decides exact numbering when dependencies permit.
## 4. Reuse map
### MedScale
Primary implementation donor for a lightweight local runtime.
Strong reusable surfaces include:
- crates/medscale-pack/src/onnx_runtime.rs; Pack manifest/provenance patterns; bounded artifact reads; verified-bytes execution; fail-closed shape checks; no-HTTP tokenizer/runtime configuration; prepared reusable local sessions; runtime/model provenance
Do not import MedScale clinical authority semantics.
### Kernux
Reuse architecture from:
- docs/canonical/platform-fabrics/ARCHITECTURE.md; docs/canonical/platform-fabrics/DELIVERY.md; docs/canonical/local-privacy/FOUNDATIONS.md; docs/canonical/local-privacy/QUALIFICATION.md
Reuse:
- provider-neutral request/result contracts; deterministic provider eligibility; calibration; abstention/OOD; disagreement/escalation; locality/privacy rules
### Morize
Reuse from docs/TYPED_DECISION_MODEL.md:
- confidence is evidence, not authority; abstention; reason codes; engine identity; model-change invalidation; audit/replay envelope
### Inercative
Reuse from docs/canonical/HARNESS_AND_ROUTING.md:
- deterministic vs decision vs generative intelligence; provider-neutral routing; explicit fallback lineage; decision-adapter qualification
### Golam
Reuse from its ExecutionProfile contract:
- model/runtime/tokenizer identity; quantization; device mapping; warm residency; resource budgets; privacy/locality class; explicit fallback policy
### commandMed and Wispral
Reuse:
- candidate preregistration; exact artifact identity; low-resource tournament methodology; quality-floor-before-size selection; exact package byte accounting; negative-evidence preservation
### SemIf and Decider
Evaluate selective reuse for:
- direct option-logit scoring; answer-slot logits; typed choice/score/boolean surfaces; schema caching; calibration experiments; avoiding prose generation/reparse
Himsat owns all permanent product interfaces.
## 5. Proposed future code layout
Final paths may change during activated shaping, but responsibilities should remain separated:
    crates/himsat-core/src/decision/
      mod.rs
      contracts.rs
      policy.rs
      errors.rs
    crates/himsat-decision/src/
      lib.rs
      router.rs
      registry.rs
      calibration.rs
      resource_policy.rs
      diagnostics.rs
      fake_provider.rs
    crates/himsat-decision-local/src/
      lib.rs
      pack.rs
      pack_store.rs
      runtime.rs
      onnx.rs
      provider.rs
      worker_protocol.rs
    crates/himsat-decision-worker/src/main.rs
      only if worker isolation wins the runtime-placement gate
    governance/provenance/
      decision-provider-*.json
      decision-runtime-*.json
      decision-model-*.json
    tests/decision/
      fixtures/
      calibration/
      multilingual/
      adversarial/
      network/
      resources/
Model weights remain outside Git source history.
## 6. Contract shape
Future contracts should be equivalent in meaning to:
    DecisionAnswerType:
      Boolean
      Choice
      OrdinalScore
      Rank
    DecisionRequest:
      request_id
      workload_id
      question
      bounded_state
      answer_type
      options_or_rubric
      language
      evidence_refs
      calibration_profile
      max_input_bytes
      max_options
      max_latency_ms
      max_memory_bytes
      privacy_mode = LocalOnly
      consequence_class
      deadline
    DecisionResult:
      request_id
      provider_identity
      engine_identity
      selected_value
      scores
      abstained
      ood
      calibration_identity
      latency_ms
      warnings
    DecisionProvider:
      capabilities()
      decide(request)
Callers must not depend on a provider-specific SDK.
## 7. Request validation
Fail before inference when:
- request ID or workload ID is invalid;; state exceeds byte/token bounds;; option count exceeds provider capability;; option IDs are duplicated;; rank cardinality is invalid;; score rubric is malformed;; language is unsupported;; required calibration profile does not match engine identity;; privacy mode is not LocalOnly;; latency/memory budget cannot be satisfied;; consequence policy forbids model use;; the model/runtime pack is not admitted.
No provider may reinterpret an invalid request.
## 8. Result validation
Reject output when:
- engine identity differs from the admitted pack;; request/schema digest mismatches;; a selected option is not in the request;; required scores are absent;; scores contain NaN/Inf;; normalization is invalid where normalized probabilities are claimed;; calibration identity is stale/incompatible;; a remote/external data boundary is reported;; schema limits are exceeded;; provider returns an unsupported answer type.
Invalid output becomes a typed provider failure, never a best-effort guess.
## 9. Stable error taxonomy
Use stable codes:
    INVALID_REQUEST
    UNSUPPORTED_ANSWER_TYPE
    UNSUPPORTED_LANGUAGE
    TOO_MANY_OPTIONS
    INPUT_TOO_LARGE
    PROVIDER_NOT_ELIGIBLE
    MODEL_PACK_MISSING
    MODEL_PACK_INVALID
    MODEL_PACK_UNTRUSTED
    RUNTIME_UNAVAILABLE
    RUNTIME_INCOMPATIBLE
    MODEL_LOAD_FAILED
    OUT_OF_MEMORY
    RESOURCE_PRESSURE
    DEADLINE_EXCEEDED
    CANCELLED
    INFERENCE_FAILED
    INVALID_PROVIDER_OUTPUT
    CALIBRATION_MISMATCH
    NETWORK_POLICY_VIOLATION
    ABSTAINED
    OUT_OF_DOMAIN
No failure may silently widen to cloud inference.
## 10. Deterministic eligibility filter
Before provider ranking, ordinary code filters by:
1. privacy/locality;
2. answer-type support;
3. language;
4. input/context limit;
5. option count;
6. installed/admitted pack;
7. runtime compatibility;
8. baseline hardware support;
9. memory/resource budget;
10. deadline;
11. calibration availability;
12. workload qualification;
13. consequence policy.
Only already-eligible local providers enter ranking.
A model cannot make an ineligible provider eligible.
## 11. Router behavior
Initial routing policy should be simple:
1. deterministic provider if an exact rule applies;
2. explicitly pinned local provider if eligible;
3. default qualified local provider;
4. optional second qualified local provider only if installed and policy permits;
5. abstain/degrade.
There is no remote fallback branch.
Record:
- eligible providers;; exclusions and reason codes;; selected provider;; fallback lineage if any;; final status.
## 12. Decision Engine identity
Material identity includes:
- provider adapter ID/version;; upstream model repository;; immutable source revision;; model artifact digest;; tokenizer/config digest;; runtime family/version;; conversion revision;; quantization;; question/prompt template revision;; option encoding;; scoring algorithm;; calibration profile/revision.
Any material change creates a new engine identity.
Historical decisions preserve their original engine identity.
## 13. Decision Pack manifest
Future pack manifest fields:
    schema_version
    pack_id
    provider_id
    engine_id
    source_repository
    source_revision
    source_license
    permission_basis
    runtime_family
    runtime_version_range
    model_format
    precision_or_quantization
    tokenizer_identity
    supported_answer_types
    supported_languages
    supported_workloads
    max_input_bytes
    max_context_tokens
    max_options
    baseline_cpu_arches
    minimum_ram_bytes
    artifact_files[]
    calibration_profiles[]
    qualification_record
    min_himsat_version
    created_at
Every artifact entry includes:
    path
    kind
    byte_size
    sha256
Admission fails closed on unsafe paths, unknown schema, byte overrun, digest mismatch, unsupported runtime, missing provenance, or missing required calibration.
## 14. Packaging architecture
The core application contains:
- no model weights;; no Python runtime;; no API key setup;; no remote decision SDK dependency;; deterministic provider support.
The Lite Decision Pack contains:
- one selected default local model;; tokenizer/config;; calibration profile;; pack manifest;; license/notice metadata;; runtime compatibility metadata.
The normal installer may include the pack so local intelligence comes with the app.
The pack remains separately versioned, removable, reinstallable, rollback-capable, and offline side-loadable.
## 15. Lightweight hard gates
Default desktop planning gates:
    MODEL_PACK_TARGET <= 256 MiB
    MODEL_PACK_HARD_CAP <= 384 MiB
    DECISION_SUBSYSTEM_INSTALL_TARGET <= 320 MiB
    DECISION_SUBSYSTEM_HARD_CAP <= 450 MiB
    INCREMENTAL_PEAK_RAM_TARGET <= 700 MiB
    INCREMENTAL_PEAK_RAM_HARD_CAP <= 1 GiB
    CPU_BASELINE_REQUIRED = YES
    GPU_REQUIRED = NO
    PYTHON_RUNTIME_REQUIRED = NO
    INTERNET_AFTER_INSTALL_REQUIRED = NO
    MULTIPLE_RESIDENT_MODELS_DEFAULT = NO
These are qualification gates, not claims that Laya already passes them.
## 16. Laya feasibility gate
Laya is approximately 421M parameters in currently observed upstream metadata.
Do not assume it fits the default pack.
The first model-intake experiment must measure exact:
- artifact bytes;; tokenizer/config bytes;; runtime binary delta;; calibration bytes;; installed bytes;; peak RAM.
Risk to test explicitly:
- FP32 is outside the intended lightweight envelope;; FP16/BF16 is outside the intended lightweight envelope;; INT8 may exceed the default hard cap after total overhead;; lower-bit quantization may fit but can require a different runtime and can damage calibration/Arabic quality.
Rules:
- if Laya passes quality and hard resource gates, it remains default-eligible;; if it passes quality but violates default resource gates, it may remain optional but not default;; if quantization fits but fails calibration/language floors, reject that quantization;; never inflate resource caps merely to preserve Laya.
## 17. Runtime candidate ladder
### Lane A — tract-onnx
First experiment.
Must prove:
- exact candidate export;; supported operators;; bounded graph/input/output behavior;; local tokenizer;; reference-score equivalence within preregistered tolerance;; no network acquisition during inference;; resource gates.
### Lane B — Rust-native runtime
Evaluate mistral.rs, Candle, Burn, or another qualified Rust-native runtime if Lane A fails compatibility or quantization requirements.
### Lane C — llama.cpp
Evaluate when GGUF/low-bit compatibility materially improves lightweight deployment.
Prefer worker isolation if native FFI materially increases crash/security risk.
### Lane D — ORT
Evaluate only if compatibility/performance gains justify runtime binary/update footprint.
Ship one default runtime family, not every benchmark runtime.
## 18. Runtime placement gate
In-process execution is eligible only if:
- crash risk is acceptable;; malformed-pack tests do not expose fatal-process behavior;; cancellation is reliable;; OOM behavior is contained;; binary-size delta passes;; runtime global state cannot threaten capture.
Worker mode is preferred when:
- unsafe/native runtime surface is material;; model load can crash;; OOM containment matters;; cancellation requires process termination;; runtime dependencies are large.
Worker invariants:
- local typed IPC only;; no network;; no credentials;; no vault access;; model-pack read scope only;; bounded CPU/RAM/time;; parent supervision;; explicit protocol version;; bounded restart/backoff.
## 19. Resource arbitration
Decision inference is lower priority than capture and durable recording.
Inputs:
- recording active;; transcription active;; available RAM;; memory pressure;; CPU load;; battery/power mode;; thermal state where observable;; model residency;; request consequence/latency.
Deterministic outcomes:
    RUN_NOW
    RUN_LOWER_PRIORITY
    DEFER
    UNLOAD_AND_DEFER
    ABSTAIN_RESOURCE_PRESSURE
The model cannot override this policy.
No decision request is allowed to damage recording reliability.
## 20. Residency policy
Default behavior:
- lazy model load;; one resident default model;; bounded warm TTL;; unload under memory pressure;; optional idle unload;; background tasks cannot pin indefinitely;; reload failures degrade explicitly.
Measure:
- model load time;; unload time;; resident bytes;; peak bytes;; cold latency;; warm latency.
## 21. Calibration
Calibration is workload-specific.
No global confidence threshold.
A calibration profile binds:
- engine ID;; workload ID;; language/language group;; fixture revision;; calibration method;; parameters;; metrics;; acceptance thresholds.
Model, tokenizer, runtime, quantization, scoring, or answer-encoding changes require revalidation and may require recalibration.
## 22. Abstention
Abstention is a valid successful outcome.
Abstain on:
- near tie;; OOD;; unsupported language/domain;; harmful truncation;; missing calibration;; insufficient selective-risk confidence;; resource refusal;; insufficient evidence.
Downstream code never converts abstention into a guessed answer.
## 23. Language qualification
Universal-default qualification requires:
- English;; Modern Standard Arabic;; Saudi/Gulf conversational Arabic where valid fixtures exist;; mixed Arabic/English;; Arabic names/entities;; technical English embedded in Arabic;; punctuation/direction variants;; optional diacritics/no-diacritics variants.
If a provider fails Arabic but passes English, it may qualify only for an English-scoped profile.
## 24. Transcript-noise qualification
Include:
- missing/wrong punctuation;; substitutions;; deletions;; repeated fragments;; speaker swaps;; overlap artifacts;; filler words;; code-switch ASR errors;; truncated context.
Measure degradation relative to clean text.
## 25. Adversarial fixtures
Include:
- prompt-like instructions inside transcript/document text;; malicious option names;; attempts to redefine answer schema;; contradictory evidence;; near-duplicate options;; catch-all/other-option traps;; long irrelevant context;; Unicode confusables;; malformed JSON-like state;; OOD inputs.
The decision model is never an instruction authority.
## 26. Candidate tournament
Freeze candidates before primary results.
Minimum lanes:
1. deterministic baseline;
2. Laya reference inference;
3. Laya converted/quantized candidate if valid;
4. SemIf-style scoring on the smallest qualified practical model;
5. at least one compact general-model challenger;
6. optional Decider-family control for benchmarking.
Freeze:
- model/source revision;; artifacts;; runtime;; precision;; option/question encoding;; fixture split;; metrics;; hard gates.
Do not change candidates because early results are inconvenient.
## 27. Hard-gate ordering
A candidate must pass:
1. rights/provenance;
2. offline proof;
3. zero unexpected egress;
4. request/result schema;
5. CPU baseline;
6. minimum task utility;
7. Arabic floor for universal default;
8. English floor;
9. calibration/selective-risk floor;
10. OOD/abstention floor;
11. crash/recovery;
12. package/RAM hard caps.
Only then compare size and latency.
## 28. Metrics
Keep separate:
- accuracy/task utility;; macro F1;; Brier score;; ECE;; NLL where meaningful;; selective accuracy;; selective coverage;; abstention precision/recall;; OOD false-confidence rate;; high-confidence wrong-answer rate;; Arabic/English gap;; code-switch gap;; transcript-noise gap;; repeatability;; cold start;; warm p50/p95;; throughput;; model bytes;; runtime bytes;; total installed bytes;; peak/resident RAM;; CPU;; battery/thermal where applicable;; crash/recovery.
No single aggregate score may hide a hard-gate failure.
## 29. Network privacy proof
Run qualification with:
- Internet disconnected;; DNS unavailable;; outbound HTTP blocked;; no API keys;; telemetry disabled;; update checks disabled;; cloud configuration absent.
Observe process-level egress.
Pass condition:
    ZERO_UNEXPECTED_OUTBOUND_CONNECTIONS_FROM_DECISION_PATH
If worker mode is used, declared loopback IPC is recorded separately.
## 30. Model-pack threat model
Treat all pack contents as untrusted input.
Threats:
- traversal/symlink escape;; oversized artifacts;; digest substitution;; parser bugs;; malformed ONNX/GGUF;; custom executable ops;; stale calibration;; incompatible runtime;; rollback attacks;; cache poisoning;; partial update;; post-verification artifact swap.
Mitigations:
- canonical safe paths;; size checks before full read;; digest verification before execution;; verified bytes used directly where practical;; no arbitrary custom ops by default;; content-addressed cache;; atomic install;; rollback metadata;; exact runtime compatibility;; fuzz/adversarial parsing.
## 31. Pack lifecycle
Required operations:
    inspect
    install-local
    verify
    activate
    deactivate
    rollback
    remove
    repair-cache
Install is atomic.
Failure leaves the prior active pack untouched.
Removal clears:
- model artifact;; tokenizer/config;; calibration;; active registry pointer;; cache index.
Installed size is user-visible.
## 32. Update semantics
Before activation:
- verify manifest;; verify digests;; verify provenance;; verify runtime compatibility;; verify qualification record;; verify calibration;; install side-by-side;; atomically switch active pointer.
Rollback restores the previous qualified pack.
Unverified packs are never executed.
## 33. Persistence boundary
Do not duplicate full private transcript state into generic decision logs.
Durable decision audit may contain:
- request ID;; workload ID;; evidence refs;; request/schema digest;; engine identity;; selected result;; scores only if policy requires;; abstention/OOD;; policy revision;; timestamp.
Sensitive source content remains in the canonical transcript/evidence store.
## 34. Local diagnostics
Expose locally:
- active engine;; pack version;; digest prefix;; runtime;; quantization;; languages;; supported decision types;; resident/unloaded state;; load failures;; resource-pressure deferrals;; local latency summaries.
No telemetry SDK is required.
## 35. Mobile profile
Do not force desktop pack size onto mobile.
Planning target:
    MOBILE_MODEL_TARGET <= 128 MiB
Mobile default requires independent evidence for:
- memory;; startup;; battery;; thermal;; background behavior;; storage pressure.
Until then mobile may use deterministic-only decisions or a smaller separately qualified pack.
## 36. CI strategy
Fast CI:
- contracts;; fake provider;; router;; policy;; pack parser;; failure cases;; calibration identity;; no-cloud policy;; error taxonomy.
Model smoke CI:
- tiny redistributable fixture model through the real runtime path.
Scheduled qualification:
- exact candidate artifacts on controlled runners.
Hardware/locality lanes:
- CPU;; Windows;; macOS;; Linux;; optional accelerators.
Skipped/absent is never PASS.
## 37. Required test matrix
Cover:
- request validation;; result validation;; routing;; deterministic precedence;; provider failure;; abstention;; OOD;; calibration mismatch;; stale/corrupt/truncated pack;; traversal;; wrong digest;; unsupported runtime;; model-load/OOM;; cancellation;; deadline;; resource pressure;; worker crash;; restart/backoff;; no-cloud fallback;; egress denial;; Arabic;; English;; mixed Arabic/English;; transcript noise;; near ties;; many options;; adversarial text;; upgrade;; rollback;; remove;; core-only mode.
## 38. Performance protocol
For each exact engine/hardware class measure:
- process start;; model load;; first decision;; warm p50/p95;; throughput;; idle RAM;; resident RAM;; peak RAM;; unload;; reload.
Use fixed request sets and release builds.
Record compiler/runtime/build identities.
## 39. First product workloads
Start with low-consequence reversible uses:
1. transcript segment category;
2. brief/relevance routing;
3. prioritization;
4. candidate ranking;
5. review recommendation.
Do not start with:
- permissions;; deletion;; connector side effects;; security allow/deny;; permanent evidence truth;; destructive memory mutation.
## 40. Evidence-sensitive workloads
Only after Evidence Engine dependencies:
- decision candidate extraction;; commitment candidate extraction;; contradiction candidate detection;; evidence-support classification;; temporal update ranking.
The model proposes observations/candidates.
Evidence engine and deterministic policy establish canonical truth.
## 41. Action boundary
Future action flow:
    DecisionResult
      -> ActionProposal candidate
      -> deterministic capability/policy
      -> user/org approval where required
      -> execution
      -> reconciliation/evidence
The decision provider never directly performs consequential external effects.
## 41.1 Canonical request hashing and replay safety
Decision request identity must be derived from a canonical serialization, not ad-hoc JSON formatting.
Freeze:
- field ordering and normalization rules; Unicode normalization policy; language-tag normalization; option ordering semantics; maximum serialized bytes; schema-version inclusion; evidence-reference ordering; digest algorithm.
The request digest must exclude nondeterministic runtime fields such as wall-clock latency while including all fields that can change the semantic answer.
A cached result is reusable only when request digest, engine identity, calibration identity, workload policy revision, and consequence class all match.
No cache may cross vault/user/workspace boundaries unless the canonical scope identity is part of the cache key and policy explicitly permits it.
## 41.2 Context minimization and sensitive-data boundary
Decision providers receive the minimum bounded state required for the workload.
Do not send raw audio when transcript/evidence features are sufficient.
Do not copy full transcripts into generic model diagnostics.
Before inference, callers should construct a typed DecisionContext containing only authorized fields and evidence references.
Logs must redact user text by default. Error strings must not echo transcript/document content.
Fixtures containing private user material are forbidden from public benchmark artifacts.
## 41.3 Concurrency, backpressure, cancellation, and batching
The router/runtime must define concurrency explicitly:
- bounded request queue; bounded in-flight requests; cancellation propagation; deadline propagation; admission refusal under saturation; no unbounded retry; no starvation of recording/transcription work.
Micro-batching is optional and may be used only after correctness is preserved across independently scored questions and latency deadlines.
Batching must never cause one request's state/options to leak into another request.
Worker termination must resolve every in-flight request as a typed failure or cancellation; no request may hang indefinitely.
## 41.4 Cache and residency correctness
If prepared model sessions or prefix caches are used:
- cache identity includes exact engine/runtime/tokenizer/template identity; cache has a byte cap; cache is evictable; cache corruption fails closed; model update invalidates incompatible entries; logout/vault deletion clears scope-bound metadata.
Do not claim secure physical erasure of SSD/model-cache blocks that the OS/filesystem cannot guarantee; promise logical deletion and key-based protection only where evidence supports it.
## 41.5 Statistical qualification discipline
Before primary tournament execution, freeze:
- primary metrics; hard thresholds; sample-size rationale; confidence intervals or bootstrap method where applicable; tie handling; missing/error handling; language strata; noisy-transcript strata; OOD strata.
Preserve every candidate result, including losses.
Do not repeatedly tune on the held-out qualification set.
Calibration fitting data and final evaluation data must be separated.
If dataset size is too small for a strong superiority claim, qualify only the narrower supported claim.
## 41.6 Reproducible build and artifact production
Any model conversion/quantization pipeline must record:
- source model revision; converter repository/revision; converter command/config; toolchain versions; deterministic seeds where relevant; produced artifact digest; tokenizer/config digests; quantization parameters; host architecture where material.
If bit-for-bit reproducibility is not achievable, preserve the exact produced bytes and document the nondeterministic conversion boundary.
Production installers must not build/convert model weights on the user's machine by default.
## 41.7 Compatibility and schema migration
Version independently:
- DecisionRequest schema; DecisionResult schema; Decision Pack schema; worker IPC protocol; engine identity schema; calibration schema.
Readers must reject unknown incompatible major versions and handle known older versions only through explicit migrations.
A core app update must not silently activate a pack whose runtime contract is no longer qualified.
A pack update must declare minimum/maximum compatible Himsat/runtime versions.
## 41.8 User-facing failure semantics
Normal product UX must distinguish:
- model not installed; model loading; model unavailable; model corrupt; resource-pressure deferral; unsupported language/workload; abstention; local runtime failure.
Do not collapse these into generic AI failed messages when actionable distinction exists.
No failure state should imply that cloud fallback will occur.
## 41.9 Security review focus
R3 review for runtime/model adoption must explicitly inspect:
- parser attack surface; unsafe/native code; custom ops; dynamic library loading; environment-variable control; writable search paths; DLL/shared-library hijack risk; symlink/reparse-point behavior; archive traversal; temporary-file permissions; worker IPC authentication/scope; denial-of-service via huge option/state payloads.
A runtime that requires arbitrary remote code/custom model code is not default-eligible.
## 41.10 No-gap readiness rule
The plan is not considered implementation-ready merely because architecture prose exists.
It is ready only when contracts, limits, evidence methods, failure semantics, source identities, qualification thresholds, rollback behavior, and per-grain exit evidence are explicit enough that implementation can proceed without reopening foundational design.

## 42. Future implementation grains
### LDF-01 — Contract foundation
Implement:
- request/result/provider contracts;; validation;; error taxonomy;; fake provider.
Exit evidence:
- exhaustive unit tests;; no model dependency;; deterministic serialization if persisted.
### LDF-02 — Router and authority boundary
Implement:
- deterministic eligibility;; routing;; no-cloud fallback invariant;; consequence classes;; reason codes.
Exit evidence:
- ineligible provider never selected;; remote fallback impossible;; deterministic precedence proven.
### LDF-03 — Decision Pack schema
Implement:
- manifest;; artifact records;; safe paths;; digest/bounds checks.
Exit evidence:
- corruption/traversal/bounds tests;; provenance mapping.
### LDF-04 — Pack store lifecycle
Implement:
- atomic install;; verify;; activate;; rollback;; remove.
Exit evidence:
- interruption/crash tests;; no half-active pack.
### LDF-05 — Runtime abstraction
Implement:
- runtime trait;; prepared-session lifecycle;; cancellation;; resource observations;; fake runtime.
Exit evidence:
- typed load/OOM/cancel failures.
### LDF-06 — tract-onnx runtime
Adapt MedScale's bounded local execution patterns.
Exit evidence:
- tiny fixture executes offline;; malformed model fails closed;; zero unexpected egress.
### LDF-07 — Laya reference qualification
Implement research-only intake:
- exact upstream identity;; model/tokenizer/license;; reference harness;; typed-answer mapping.
Exit evidence:
- immutable artifact manifest;; reference outputs;; no product adoption yet.
### LDF-08 — Laya conversion/quantization
Test:
- ONNX or alternate local representation;; equivalence;; size;; RAM;; latency;; calibration;; Arabic/English.
Reject if hard gates fail.
### LDF-09 — Alternative candidate lane
Evaluate:
- SemIf + compact model;; or a smaller provider.
Must use same contract, fixtures, metrics, and authority rules.
### LDF-10 — Qualification tournament
Run frozen candidates across:
- English;; Arabic;; code-switch;; noise;; OOD;; adversarial;; calibration;; resources;; network proof.
Select default only from hard-gate passers.
### LDF-11 — Resource governor/residency
Implement:
- lazy load;; warm TTL;; unload;; capture priority;; deferral/abstention.
Prove decision inference cannot damage recording reliability under test load.
### LDF-12 — Installer integration
Implement:
- offline bundle;; core-only install;; pack diagnostics;; rollback/delete.
Prove no Internet is required after installation.
### LDF-13 — First bounded product use
Integrate one low-consequence workload.
Prove:
- measurable utility;; abstention preserved;; deterministic fallback;; no authority transfer.
### LDF-14 — Closeout
Reconcile:
- tests;; qualification;; provenance;; notices/SBOM;; threat model;; docs;; residual risks;; successor authority.
## 43. Review protocol
Every future grain requires:
- SpecGrain;; Diffcipline;; focused tests;; broader risk-appropriate tests;; exact-head CI;; R3 where required;; Alibaba Open Code Review where supported;; independent review for excluded files;; expected-head merge protection;; post-merge verification.
For Alibaba OCR:
    0 reviewed files != PASS
## 44. Provenance before adoption
Record:
- repository;; immutable revision;; exact paths;; public terms/license;; founder permission basis where relied upon;; redistribution scope;; embedded third-party material;; model/weight license;; tokenizer/config license;; conversion tool/revision;; runtime license;; destination paths;; adaptation description;; notices;; Himsat tests;; reviewer confirmation.
Permission enables reuse but never waives provenance.
## 45. Release gates
The Local Decision Fabric may be called qualified only when:
- at least one default provider passes every hard gate;; deterministic-only core mode works;; network proof passes;; exact pack provenance is complete;; language claims match evidence;; calibration is versioned;; install/rollback/remove pass;; crash/recovery passes;; resource hard caps pass;; no unresolved high/critical security blocker remains;; installer behavior is qualified.
## 46. Kill and pivot criteria
Reject or pivot a candidate/runtime if:
- it cannot fit hard size/RAM caps;; Arabic quality misses the floor;; calibration remains unreliable;; high-confidence wrong answers remain unacceptable;; hidden network behavior cannot be removed;; isolation is unsafe;; crash/OOM threatens recording;; maintenance/security footprint is disproportionate;; deterministic rules are better for the workload.
If Laya fails, only the provider choice changes.
## 47. Pre-implementation checklist
Before LDF-01 starts, the activated future specification must answer YES:
- [ ] dependency frontier authorizes the work;
- [ ] active SpecGrain unit exists;
- [ ] Diffcipline paths include future files;
- [ ] source-use authorization is current;
- [ ] exact donor revisions are pinned;
- [ ] contract fields are frozen;
- [ ] authority boundary is frozen;
- [ ] error taxonomy is frozen;
- [ ] pack schema version is frozen;
- [ ] resource hard caps are frozen;
- [ ] baseline hardware classes are declared;
- [ ] tournament fixtures are licensed/provenanced;
- [ ] Arabic fixtures exist;
- [ ] no-cloud evidence method is defined;
- [ ] runtime candidate revisions are frozen;
- [ ] model candidate revisions are frozen;
- [ ] calibration method is preregistered;
- [ ] network evidence method is defined;
- [ ] rollback/delete semantics are defined;
- [ ] capture-vs-inference priority is defined;
- [ ] CI does not require huge model downloads on every run;
- [ ] Alibaba OCR accounting is defined;
- [ ] threat model is linked;
- [ ] every grain names its acceptance evidence before code.
Unknown items are shaped before production code.
## 48. Definition of implementation-ready
The planning package is implementation-ready when:
- PR #220 is canonically merged through its real gates;; this packet is canonically merged through its real gates;; no unresolved planning contradiction remains;; the dependency frontier reaches Local Decision Fabric;; SpecGrain activates the future implementation unit.
At that point implementation begins at LDF-01 rather than restarting broad architecture research.
Live upstream changes may still require bounded evidence-driven corrections.
## 49. Final architecture
    Himsat deterministic authority
             |
             v
    Eligibility + resource policy
             |
             v
    Local Decision Router
             |
             +--> DeterministicProvider
             |
             +--> Qualified Local DecisionProvider
                     |
                     +--> Decision Pack
                     |      model
                     |      tokenizer/config
                     |      calibration
                     |      provenance
                     |
                     +--> Local runtime
                            tract-onnx first experiment
                            alternate local runtime if required
    DecisionResult
             |
             v
    deterministic downstream policy
             |
             +--> advisory observation
             +--> review
             +--> abstain/defer
             +--> evidence candidate
             +--> separately authorized action proposal
There is no default cloud branch.
There is no model-owned authority branch.
There is no requirement that Laya win.
The implementation goal is a small, replaceable, local decision primitive that improves Himsat without making Himsat heavy.
