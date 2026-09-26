# 008F — Gate E live evidence on real Windows hardware

Status: live-execution record. This file proves what ran on real hardware
against exact canonical SHAs; it does not close `O008` and does not create
authority for Specification 009. Rows that could not run are recorded with
their exact blocker, never as PASS.

## Binding and provenance

```text
HARDWARE_HOST_OS      = Microsoft Windows NT 10.0.26200.0 (Windows 10 Home)
HARDWARE_SHELL        = PowerShell 7.6.6
AUDIO_SERVICE         = Audiosrv Running
ENDPOINTS             = Onboard microphone (Intel Smart Sound, OK) + Speakers (OK)
BUNDLE_C238CDC_SHA256 = 14c5f7e8ac5c5b4ec40107e2e17599d7e2c3ff7f5f6053dfbab81fa420f93a22
BUNDLE_C238CDC_CI     = 36027842174 SUCCESS (push, exact merge c238cdc)
BUNDLE_C238CDC_R3     = 36027842465 SUCCESS (push, exact merge c238cdc)
BUNDLE_009F74A_SHA256 = 007f338a761bb5927b5e96547296d81271c41664ad1b3f24a891e6f3f4bc9838
BUNDLE_009F74A_CI     = 36202725793 SUCCESS (push, exact merge 009f74a)
BUNDLE_009F74A_R3     = 36202725792 SUCCESS (push, exact merge 009f74a)
TOOLCHAIN             = rustc 1.98.1 / cargo 1.98.1 (pinned, per both manifests)
CONSENT               = explicit founder consent for Gate E microphone plus
                        system-audio capture on this machine, this session only
```

Device friendly names are paraphrased here; the OS-provided localized
microphone name is preserved verbatim only inside the off-repo evidence
artifacts, never as repository content.

## Canonical smoke passes (exact c238cdc bundle, exit 0, LIVE_SMOKE_COMPLETE)

- Preflight: `AUDIO_ENDPOINT_COUNT=2`, both live tests present in the binary,
  manifest SHA and binary digest verified before every run.
- Smoke run 1 (ambient room signal): microphone `frames/1920`,
  loopback `frames/1920`.
- Smoke run 2 (programmatic tone emission active): microphone `frames/960`,
  loopback `frames/1920`.
- Smoke run 3 (programmatic 440 Hz emission active, fresh evidence dir):
  microphone `frames/960`, loopback `frames/1920`, machine-readable
  `live-smoke.json` bound to `source_sha = c238cdc` and the binary digest.
- Post-resume smoke after a real 8-second Modern Standby cycle
  (kernel log enter 21:56:13, exit 21:56:21): microphone `frames/1920`,
  loopback `frames/1920`, audio stack healthy after wake.

## Targeted probes (exact c238cdc binary unless noted)

- Microphone stress: 8/8 strict passes, `outcome=frames` every iteration.
- Loopback negative control: 8 consecutive full-10-second runs reporting
  `outcome=silence, non_silent_frames=0`. The steady-state render mix on this
  host is digital silence, and the probe reports it honestly instead of
  hallucinating signal.
- Loopback deliberate-signal differential: immediate `frames/1920` in 0.36 s
  during active 440 Hz emission, against the 8 full-timeout silences without
  emission. The earlier transient loopback passes are therefore explained as
  real render-mix transients, and the emission-window pass is the deliberate
  first-audio proof.
- Independent corroboration with ffmpeg 8.1 (second implementation, not
  Himsat code): 5 s DirectShow microphone capture at mean -36.7 dBFS,
  max -16.2 dBFS, proving live acoustic signal on the same endpoint.
- Privacy revoke (founder toggled OS microphone access off): microphone open
  refused as `classified_fault/stream-build-fault`, clean test pass, no crash,
  no silent stop. Raw log preserved.
- Privacy restore (toggled back on): `frames/2880`, strict PASS. The
  off-then-on pair locks the attribution to the OS control.
- Endpoint disable (Device Manager disable, PnP node in Error state): the OS
  did NOT tear down the audio-engine capture endpoint — WASAPI and DirectShow
  capture kept working. Recorded as an OS-behavior finding, not an adapter
  defect and not a removal proof. True removal still needs physical detach.
- Microphone re-enable: PnP OK on both endpoints plus strict `frames/960`.

## 008F harness grains (new env-gated tests, NOT RUN in CI)

PR #226 (harness, merged as `1b0d8ff`): `live_sustained_hold_reports_signal_without_interruption`
(`HIMSAT_LIVE_SUSTAINED_TEST=1`, hold via `HIMSAT_LIVE_SUSTAINED_SECS`,
default 60 clamped 5..600, mid-hold pause/resume) and
`live_mic_frames_round_trip_into_the_journal` (`HIMSAT_LIVE_ROUNDTRIP_TEST=1`:
bounded live capture through the real 008E accumulator and 008D commit path
under a test-only VRK, journal close, decrypt of the exact captured bytes).
Delegate adversarial review of the first head found 3 blocking and 8
non-blocking findings; all were repaired forward and the superseded heads
stay visible (`b50d694` superseded pre-verdict; `101df3d` failed exact-head CI
with a Windows-only E0599 test defect; `3612db5` failed exact-head CI with a
Windows-only clippy `assertions_on_constants` finding). Accepted head
`43a1c7c` passed pull-request CI `36197802832` and R3 `36197802628`; canonical
merge `1b0d8ff` (parents `c238cdc` + `43a1c7c`, tree-equal, verified) passed
push CI `36199462901` and R3 `36199462890`.

PR #227 (round-trip window sizing, merged as `009f74a`): the first live
round-trip run failed honestly and deterministically (239872 dropped bytes)
because the host microphone default is stereo F32 at 48 kHz, so two seconds
deliver about 768 KiB against the 512 KiB cap; the drop counter behaved
exactly as designed. The window is now one second (about 384 KiB on this
host). Accepted head `7e9e59e` passed CI `36201222161` and R3 `36201222179`;
canonical merge `009f74a` (parents `1b0d8ff` + `7e9e59e`, tree-equal) passed
push CI `36202725793` and R3 `36202725792`. During this unit one repair commit
was briefly created on local `main`; it was never pushed and `main` was
restored to `origin/main` while the fix continued on the feature branch via
cherry-pick, so no shared history was rewritten.

Live runs against the exact `009f74a` bundle:

- Sustained 60 s hold: `callbacks=5946, frames=5704320,
  non_silent_frames=4381056`, mid-hold pause/resume clean, strict PASS.
  The sustained run also measured the true device format (stereo 48 kHz).
- Live round-trip: `captured_bytes=380160` pushed, sealed, committed as chunk
  0, journal closed with `commits=1`, decrypted byte-exact, accumulator
  closed cleanly, strict PASS.

## Residuals (honest non-claims)

```text
USB_BT_ATTACH_DETACH      = UNAVAILABLE (no USB/Bluetooth audio hardware on
                            this host, founder-confirmed)
TRUE_PHYSICAL_REMOVAL     = NOT_RUN (onboard endpoint only; OS software
                            disable does not remove the engine endpoint)
EXCLUSIVE_MODE_LIVE       = NOT_RUN (no exclusive-mode holder available
                            in-session: no MSVC/dotnet toolchain and custom
                            COM QI blocked; mapping unit-proven in CI)
STORAGE_EXHAUSTION_LIVE   = NOT_RUN (no full-volume harness; pure admission
                            policy unit-proven)
MULTI_HOUR_CAPTURE        = NOT_RUN (time; 60 s sustained hold plus
                            unit-proven loss accounting)
```

`O008` stays open on exactly these residuals. Specification 008 stays open
until its closeout rule is genuinely satisfied; no row above is converted
into PASS.
