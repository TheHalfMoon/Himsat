# Himsat Agent Instructions

## Language

Repository-facing technical content, code, comments, plans, specifications, tasks, reports, evidence, commit messages, PR bodies, reviewer responses, and governance artifacts must be written in English.

## Canonical authority

Read in this order before changing the repository:

1. `specs/CURRENT.md`
2. `.specify/memory/constitution.md`
3. `docs/execution-master-plan.md`
4. the active specification's `spec.md`, `plan.md`, and `tasks.md`
5. referenced ADR/research/security/donor/qualification documents
6. live GitHub state, exact branch/head, diff, checks, reviews, and mergeability

Live repository and GitHub truth override stale plans, chat summaries, cached CI, stale hashes, stale donor assumptions, and remembered authority.

## SpecGrain discipline

Use SpecGrain's model as the planning and delivery control discipline:

- refine large objectives recursively;
- keep one independently understandable outcome per leaf;
- make `scope_in` and `scope_out` explicit;
- declare dependencies instead of relying on narrative order;
- define acceptance and evidence before implementation;
- include risk and recovery plans;
- bound context and change surface;
- do not call a candidate a Grain merely because it is small;
- do not claim native SpecGrain lifecycle state unless the actual SpecGrain tooling/state proves it.

The relevant upstream methodology repository is `TheHalfMoon/SpecGrain`. Himsat planning may adopt its concepts without copying its product code into Himsat.

## Diffcipline discipline

Use Diffcipline's behavioral contract at the finish line:

**Think → Challenge → Minimize → Change → Prove**

Rules:

- `NOT RUN` is never `PASS`.
- Agent self-report is not verification evidence.
- Exact diff and exact revision matter.
- Minimality is subordinate to correctness, security, accessibility, data integrity, and compatibility.
- Risk determines rigor.
- Public performance/quality/superiority claims require reproducible evidence.
- Do not claim complete, verified, merged, qualified, secure, private, offline, or benchmark-superior without evidence that supports that exact claim.

The relevant upstream methodology repository is `TheHalfMoon/Diffcipline`.

## Product invariants

Do not weaken these without an explicit constitutional/governance change:

- no mandatory Himsat-hosted cloud for core product behavior;
- no mandatory account for local core behavior;
- no hidden fallback from local AI to remote AI;
- no telemetry SDK in default core binaries unless a future specification explicitly changes the constitution;
- user-visible, permission-respecting recording only;
- no attempts to bypass OS microphone/screen indicators or protected-content capture policies;
- evidence-linked factual intelligence;
- local encryption and user-controlled keys;
- exportable user-owned data;
- platform capability detection instead of unsupported promises;
- donor/model/dataset provenance before adoption.

## Donor code

Never copy or closely adapt a donor merely because its repository is public.

Before non-trivial adoption:

1. identify repository and immutable revision/tag;
2. verify the exact path's controlling license;
3. inspect model/weight/data licenses separately from code licenses;
4. preserve required notices;
5. create/update a provenance record;
6. explain why adoption is better than a smaller native implementation or dependency;
7. write Himsat-owned behavior tests;
8. keep incompatible/mixed-license paths out of the Himsat dependency graph.

Current source-available, non-commercial, GPL/AGPL/SSPL, unknown-license, or custom-restriction material is `REFERENCE_ONLY` by default until an explicit legal/governance decision says otherwise.

Do not copy current Screenpipe, current Kreuzberg/Xberg ELv2 code, MinerU custom-license code, Marker restricted code/models, or Anarlog `enterprise/**` into Himsat under the permissive-core plan.

## Security rules

Treat captured audio, transcripts, speaker embeddings, screenshots, documents, memory graph data, and connector credentials as sensitive.

- never log plaintext secrets, credentials, vault keys, raw speaker embeddings, or full private transcripts by default;
- use OS secure storage for key material where available;
- plugins and agents receive explicit capabilities, not ambient authority;
- external writes require explicit policy and normally user approval;
- connector egress is content-minimized and auditable;
- model and plugin packages require hashes/signatures/manifests before execution;
- archive/document importers are untrusted-input parsers and require traversal/bomb/resource-limit defenses;
- media decoders and parsers require fuzz/adversarial testing proportional to risk.

## Platform truth

Desktop, iOS/iPadOS/watchOS, and Android differ materially. Do not generalize one platform's audio behavior to another.

Examples of known constraints at planning time:

- iOS background microphone recording requires the correct audio session/background configuration and remains subject to interruptions and OS policy;
- Apple recording intents require visible system recording state/Live Activity behavior where specified by the OS;
- Android microphone background capture requires a foreground service and modern Android restricts starting microphone foreground services from the background;
- Android third-party apps generally cannot capture cellular call uplink/downlink without privileged permissions;
- Android playback capture depends on MediaProjection permission and the source app's capture policy;
- newer ScreenCaptureKit capabilities must be version-gated; beta/future APIs are not baseline requirements.

## Branching and merge discipline

- no force-push or shared-history rewriting unless repository governance is explicitly changed;
- prefer bounded feature/planning branches;
- re-read canonical `main` after merges;
- verify exact head, scope, CI/checks, reviews/threads, and mergeability immediately before merge;
- use expected-head protection where supported;
- unavailable, skipped, neutral, billing-blocked, or absent review systems are not PASS;
- preserve residual risks and blockers in closeout evidence.

## Claims

“Himsat is the best” is an ambition, not a repository fact.

Benchmark claims must name:

- dataset/corpus and license;
- device/hardware/software versions;
- model and exact hash/version;
- protocol and metric;
- baseline products/versions where comparison is permitted;
- run artifacts and failure cases;
- date and reproducibility instructions.

Negative results remain evidence and must not be hidden.
