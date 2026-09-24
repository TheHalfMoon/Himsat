# Specification 008 Gate E — hardware execution runbook

Status: execution support only. This runbook does not close O008 and does not create authority for Specification 009.

## Purpose

Gate E needs real Windows runtime evidence. The prior founder machine could not compile because the MSVC compiler/linker was incomplete. The repository therefore supports a second execution mode:

1. GitHub-hosted Windows CI compiles the exact Himsat test binary for the candidate SHA.
2. CI publishes a content-addressed Gate E bundle.
3. A real Windows hardware machine downloads/copies that bundle.
4. The hardware machine runs the already-compiled binary and records runtime evidence.

This removes a local C/C++ compiler/linker as a prerequisite for **executing** Gate E. It does not remove the real hardware requirement, the consent requirement for live audio, or any evidence row.

## Bundle contents

The CI bundle contains:

- `himsat-core-tests.exe`
- `windows_gate_e.ps1`
- `manifest.json`

`manifest.json` binds the bundle to the exact source SHA and the SHA-256 of the test binary.

Do not run a bundle whose manifest SHA does not match the candidate being qualified.

## Preflight without live capture

No live audio is captured by this command:

```powershell
Set-ExecutionPolicy -Scope Process Bypass
.\windows_gate_e.ps1 -EvidenceDir .\gate-e-evidence
```

The preflight:

- refuses a non-empty evidence directory so an earlier run cannot be overwritten;
- verifies the binary SHA-256 against the manifest;
- verifies the binary contains both env-gated live tests;
- records Windows/PowerShell identity;
- records Windows Audio service status;
- attempts present `AudioEndpoint` enumeration;
- writes sanitized endpoint names/status/classes only;
- does not run microphone or loopback capture.

A zero endpoint count is not a Gate E pass. It means the current session cannot prove usable hardware.

## Live smoke

Live capture requires explicit user-visible consent for that session.

Only after consent exists:

```powershell
.\windows_gate_e.ps1 `
  -EvidenceDir .\gate-e-evidence `
  -RunLiveSmoke `
  -ConsentLiveCapture
```

This runs exactly:

- `capture_windows::windows_tests::live_stream_open_reports_frames_or_classified_fault`
  with `HIMSAT_LIVE_MIC_TEST=1`;
- `capture_windows_system_audio::windows_tests::live_loopback_open_reports_frames_or_classified_fault`
  with `HIMSAT_LIVE_LOOPBACK_TEST=1`.

The runner writes the raw logs and a machine-readable result record.

A failed live test is evidence and must be preserved. Do not repeatedly rerun and discard red evidence.

## Gate E matrix still required

The prebuilt bundle only removes the local compilation blocker and automates the first live smoke probes. O008 still requires the full canonical hardware matrix:

1. live microphone endpoint connect/disconnect;
2. USB/Bluetooth endpoint change;
3. real Windows suspend/resume;
4. multi-hour capture;
5. microphone privacy toggle;
6. exclusive/shared-mode contention;
7. real storage-exhaustion refusal;
8. loopback first-audio.

Use the current Specification 008 spec/plan/tasks and evidence records as authority for each row.

## Evidence identity

For every hardware run record:

- exact source SHA from `manifest.json`;
- test binary SHA-256;
- Windows version;
- device class/role and relevant driver/software version;
- exact command and environment;
- start/end timestamps and duration;
- PASS/FAIL;
- raw logs/artifacts;
- frame/loss accounting where applicable.

Do not record unnecessary serial numbers or machine-unique identifiers.

## Hardware requirements

The real machine must provide the physical capabilities needed by the matrix. A GitHub-hosted VM cannot substitute for:

- real endpoint transitions;
- USB/Bluetooth attach/detach;
- physical/OS suspend-resume;
- microphone privacy changes;
- a second holder for exclusive-mode contention;
- real volume pressure/exhaustion;
- real first-audio timing.

## Toolchain rule

A local MSVC compiler/linker is optional when the exact prebuilt Gate E bundle is used.

If the operator chooses to compile from source instead, the then-current qualified Windows toolchain must be proven usable. Directory existence alone is not proof; actual compilation/linking must succeed.

## Consent rule

The presence of a Gate E bundle is not consent to record audio.

The `-ConsentLiveCapture` switch is an operator assertion that explicit user-visible consent exists for that session. The script refuses live smoke without it.

## Closeout

Successful smoke tests are not enough to close Specification 008.

After the complete matrix is satisfied:

evidence record -> hashes -> Alibaba Open Code Review accounting -> independent review -> focused/full tests -> exact-head CI/R3 -> expected-head merge -> merge verification -> post-merge CI/R3 -> reconciliation -> Specification 008 closeout.

Specification 009 and LDF implementation remain unauthorized until their canonical dependency frontiers arrive.
