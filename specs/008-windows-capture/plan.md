# Specification 008 Plan — Windows Capture

## Prospective implementation leaves

The exact split is re-bounded only after shaping qualifies and the
first leaf reconciles against then-live canonical truth. Current
candidates (not Grains until SpecGrain readiness establishes them):

```text
008A Windows microphone capture pathway (WASAPI endpoint
  enumeration, selection, stable identity, fault classification,
  format negotiation, first-audio evidence)
008B authorized Windows system-audio pathway (OS-sanctioned WASAPI
  loopback only, endpoint/default resolution, no private-API or
  entitlement-escape capture)
008C Windows lifecycle and evidence matrix (microphone privacy
  granted/denied/revoked, shared/exclusive-mode conflict, endpoint
  and Bluetooth/USB device change, interruption, long-session
  checkpoint/loss accounting, storage-pressure refusal)
```

Each leaf must declare outcome, `scope_in`/`scope_out`, dependencies,
acceptance, risk, recovery path, context budget, change surface,
evidence requirements, minimality rationale, and safety/security
implications before implementation begins.

## R3 verification strategy

- pinned provider/toolchains; fmt/lint/build/tests on claimed targets
  including `windows-latest`, plus `ubuntu-latest`/`macos-latest` for
  the portable contract (Gate E: the platform claim is proven on the
  platform, never by redefinition);
- exact dependency/license/SBOM/provenance closure: any new external
  dependency enters only through a 004P-style adoption gate with
  exact revision/path/permission/provenance evidence; cpal's
  `wasapi` backend path must be separately evidenced before it is
  trusted for the loopback claim;
- positive: microphone start/stop round-trip into the 005 journal,
  system-audio loopback first audio, endpoint-change survival,
  privacy-grant flow, long session with checkpoint cadence and loss
  account reconciling to zero unexplained;
- adversarial: endpoint removed mid-session, microphone privacy
  revoked mid-session, exclusive-mode device lock, Bluetooth/USB
  disconnect/reconnect, format change mid-stream, clock drift and
  discontinuity injection, torn journal beneath a live session,
  storage-pressure refusal, transplanted source descriptors;
- platform: Windows adapter behavior evidenced on Windows only (XI);
  the portable 006 contract never promises what Windows cannot
  prove, and 008 never narrows the portable contract to fit one
  platform;
- deletion: capture artifacts and hooks removed through the B506
  boundary with reread verification.

## Diffcipline scope control

Shaping is docs/governance only: no product code, Cargo
manifest/lockfile, provenance-adoption, SBOM, workflow, donor, model,
dataset, or asset change. The shaping packet admits
`specs/008-windows-capture/**` to expected_files (forward-only scope
authorization mirroring the 005/006/007 precedent) plus the shaping
ledger line. If shaping exceeds diff bounds, split it rather than
weakening bounds.

## Recovery plan

- shaping review finds a design flaw: revise forward and re-qualify
  the exact new head;
- implementation failure later: preserve failed evidence and repair
  forward;
- platform truth contradicts the plan (WASAPI loopback unavailable
  for a target endpoint class, privacy model change, exclusive-mode
  behavior change): refresh pins at implementation time and narrow
  claims to what the current OS proves, never widen the promise by
  redefinition.

## Closeout rule

Specification 008 becomes `CLOSED_CANONICAL` only after reviewed
shaping, bounded implementation leaves, R3 adversarial/platform
evidence, reconciliation, expected-head merges, post-merge
verification, and durable closeout evidence all close. Only then may
Specification 009 (Linux capture) shaping begin.
