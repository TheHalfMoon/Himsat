# 008G — Gate E residual harness qualification evidence

Status: `CANONICAL_QUALIFIED_TOOLING`; this record qualifies the residual Gate E harnesses merged by PR #229. It does **not** convert any unexecuted live/hardware row into PASS, does **not** close `O008`, and does **not** authorize Specification 009.

## Exact identity

```text
BASE_SHA                    = 091491bf647db72e9b7e688a11e346638d985c52
ACCEPTED_HEAD               = a88ac74bf14602500dffdca9656f2ce60d7cd9a3
CANONICAL_MERGE             = 84dd679a1acae351896b5d6c5a378ef881572931
CANONICAL_TREE              = 2bca466bbe07aaa5efe15226096f40e2b64e862b
MERGE_PARENT_1              = 091491bf647db72e9b7e688a11e346638d985c52
MERGE_PARENT_2              = a88ac74bf14602500dffdca9656f2ce60d7cd9a3
PR                           = 229
```

The merge used a normal merge commit with expected-head protection. No force push, rebase, branch-protection bypass, or gate weakening was used.

## Qualified change surface

PR #229 changed only:

- `crates/himsat-core/src/capture_windows_gate_e_residuals.rs`;
- `crates/himsat-core/src/lib.rs`;
- `tools/windows_gate_e.ps1`.

The accepted tooling adds explicit, env/consent/prerequisite-gated probes for:

- measured host storage-threshold refusal through the real 008C admission policy;
- exact typed `exclusive-mode-conflict` expectation when an independently established exclusive holder exists;
- 2–4 hour continuous live microphone flow with callback-progress and runtime-error checks;
- physical detachable-endpoint removal requiring exact `endpoint-unavailable` after pre-removal flow is first proven.

The PowerShell runner preserves machine-readable evidence and fails closed when required consent, environment flags, hardware, an exclusive holder, or other prerequisites are absent.

## Exact-head qualification

```text
PR_CI_RUN                   = 36234360248
PR_CI_ATTEMPT_1             = FAILURE
PR_CI_ATTEMPT_1_FAILED_JOB  = Diffcipline / R2 exact diff
PR_CI_ATTEMPT_1_FAILURE     = pre-existing vault_backup_sqlcipher staged-byte-identity test returned Ok where FileIdentityMismatch was expected
PR_CI_ATTEMPT_1_CAUSALITY   = NOT_ATTRIBUTABLE_TO_008G_RANGE
PR_CI_ATTEMPT_2             = SUCCESS
PR_R3_RUN                   = 36234360252
PR_R3                       = SUCCESS
```

The first CI attempt failed only in the Diffcipline workspace-test invocation on the existing SQLCipher backup-staging test. On the same exact head, the ordinary Ubuntu, macOS, and Windows Rust jobs passed, including the Windows build and Gate E bundle path. After root-cause/scope reconciliation, only the failed Diffcipline job was rerun once; attempt 2 passed without changing the head or weakening any test or gate.

An exact-head informational range audit was submitted as GitHub review `5325622710`. It found no blocking issue in the 008G range and explicitly did not represent itself as Alibaba Open Code Review, Jev, an independent human review, or any stronger canonical security gate. No Alibaba OCR or Jev execution is claimed by this evidence record.

## Post-merge qualification

```text
POST_MERGE_CI_RUN            = 36236248468
POST_MERGE_CI                = SUCCESS
POST_MERGE_R3_RUN            = 36236248540
POST_MERGE_R3                = SUCCESS
POST_MERGE_HEAD              = 84dd679a1acae351896b5d6c5a378ef881572931
```

The post-merge Windows job passed formatting, lint, full workspace tests, Gate E bundle build/upload, and the registered dependency-closure invariant on the exact canonical merge. Ubuntu and the remaining CI matrix also completed successfully. R3 completed successfully on the same merge SHA.

## Honest non-claims and remaining O008 frontier

The harnesses remove missing-tooling limitations; they do not themselves prove the external condition they are designed to observe.

```text
USB_BT_ATTACH_DETACH      = UNAVAILABLE on the previously qualified founder host; no detachable USB/Bluetooth audio hardware was available
TRUE_PHYSICAL_REMOVAL     = NOT_RUN; requires a physically detachable endpoint and an actual detach during the probe
EXCLUSIVE_MODE_LIVE       = NOT_RUN; requires an independently established exclusive-mode holder before the Himsat probe runs
STORAGE_EXHAUSTION_LIVE   = NOT_RUN; the new safe storage probe proves measured threshold/refusal policy, not literal host-volume exhaustion
MULTI_HOUR_CAPTURE        = NOT_RUN; requires an actual 7200..14400 second live run on a Windows host with capture consent
```

`DEVICE_MANAGER_DISABLE != PHYSICAL_ENDPOINT_REMOVAL`: the earlier onboard-device software disable left the audio engine endpoint usable and therefore remains degraded-PnP robustness evidence only.

`HARNESS_CANONICAL != LIVE_PLATFORM_PASS`: no residual row above is promoted to PASS by PR #229 or by this reconciliation record.

## Frontier consequence

`O008` remains open until the Specification 008 Gate E closeout rule is genuinely satisfied or canonical governance explicitly dispositions the remaining platform residuals. Specification 009 shaping remains unauthorized while Specification 008 is open.
