# 008A dependency adoption — Windows-only `cpal =0.18.2` target edge (candidate)

## Grain declaration

- Outcome: the already-registered `cpal 0.18.2` dependency is admitted
  to the `cfg(target_os = "windows")` target of `himsat-core` under the
  004P-style adoption gate, with exact closure, provenance, and
  WASAPI-backend evidence and with no other platform edge touched.
- `scope_in`: one `Cargo.toml` target-dependency line
  (`cpal = { version = "=0.18.2", default-features = false }` under
  `[target.'cfg(target_os = "windows")'.dependencies]`), the matching
  `EXPECTED_WINDOWS_DIRECT` expectation in
  `tools/004p_dependency_closure.py`, and this evidence/ledger record.
- `scope_out`: any Windows adapter code (`capture_windows.rs`),
  system-audio loopback work, feature addition, workflow change, lock
  change, registry entry change, donor material, or claim of Windows
  capture support. No `.cargo/config`, feature flag, or `[features]`
  table is introduced.
- Dependencies: `cpal 0.18.2` itself is not new to the repository — it
  was adopted for macOS in B007A (PR #185/#189 lineage) and is already
  locked and registered. This unit adds only the Windows target edge.
- Acceptance: exact direct-manifest agreement with the closure tool,
  unchanged registry count and unchanged `Cargo.lock`, WASAPI backend
  subset evidenced from immutable index metadata, and exact-head
  CI + R3 SUCCESS with the `windows-latest` Rust job green.
- Risk: R3 (platform capture). The adopted edge exposes the WASAPI
  backend surface; it grants no capability by itself and no capture
  code consumes it in this unit.
- Recovery: repair forward; the edge is one line and can be removed or
  re-pinned by a new commit without touching any other target.
- Minimality: the smallest possible representation of "Windows may use
  the closed cpal binding" — no global dependency, no version bump, no
  feature widening, no lock churn.
- Safety/security: no donor code copied; no new crate bytes enter the
  lock or registry; `unsafe_code = "forbid"` remains the workspace
  lint, so the adapter grains must consume cpal through safe APIs.

## Adopted dependency identity (exact, unchanged from the macOS adoption)

```text
PACKAGE            = cpal
VERSION            = 0.18.2 (exact pin "=0.18.2")
SOURCE             = registry+https://github.com/rust-lang/crates.io-index
CHECKSUM           = 6f02e8d0327b42d3e2e4ab2119af397344eb9fc54a34bf0ddeaa1277af8681f1
SOURCE_REPOSITORY  = https://github.com/RustAudio/cpal
SOURCE_REVISION    = e1612d5d98152f8dc2a62e1b51ef7cbf4f7f26b7
SOURCE_LICENSE     = Apache-2.0 (policy disposition: allow)
REGISTRY_ENTRY     = cargo-cpal-0.18.2 (adoption_mode depend, adopted true)
LICENSE_EVIDENCE   = crate LICENSE at immutable VCS revision (registry closure table)
```

The registry entry, checksum, repository, revision, and selected
license are re-verified mechanically by
`tools/004p_dependency_closure.py`, which fails closed on any drift
between `Cargo.lock`, the registry, and the pinned expectations.

## Windows target edge (the only product-byte change)

`crates/himsat-core/Cargo.toml`:

```toml
[target.'cfg(target_os = "windows")'.dependencies]
cpal = { version = "=0.18.2", default-features = false }
```

`tools/004p_dependency_closure.py`:

```python
EXPECTED_WINDOWS_DIRECT: dict[str, dict[str, Any]] = {
    "cpal": {"version": "=0.18.2", "default-features": False},
    ...
}
```

`default-features = false` is a semantic no-op for this crate and is
kept only for symmetry with the macOS edge; the evidence below shows
that cpal 0.18.2 declares `default = []`, so no feature is being
withheld from the Windows backend.

## WASAPI backend evidence (why this edge is sufficient)

`cargo tree --locked --target x86_64-pc-windows-msvc -p cpal -e normal`
resolves the exact subtree below on this workstation (metadata only; no
Windows build is claimed here):

```text
cpal v0.18.2
├── dasp_sample v0.11.0
├── windows v0.62.2
│   ├── windows-collections v0.3.2 -> windows-core v0.62.2
│   ├── windows-core v0.62.2 (proc-macros: windows-implement 0.60.2, windows-interface 0.59.3)
│   ├── windows-future v0.3.2
│   ├── windows-numerics v0.3.1
│   ├── windows-result v0.4.1 -> windows-link v0.2.1
│   ├── windows-strings v0.5.1 -> windows-link v0.2.1
│   └── windows-threading v0.2.1
└── windows-core v0.62.2
```

Every crate in that subtree was already present in `Cargo.lock` and in
the provenance registry before this unit, because the lock records the
union of all platform dependencies of `cpal` (and because the Windows
protector closure already carried `windows-*` crates). The adoption
therefore introduces zero new bytes and zero new license obligations.

Immutable crates.io index metadata for `cpal 0.18.2` establishes the
backend facts that matter for the later 008 claims:

```text
features.default        = []            (no withheld default feature)
windows non-optional    = windows ^0.62 with Win32_Media,
                          Win32_Media_Audio, Win32_Media_KernelStreaming,
                          Win32_Media_Multimedia, Win32_Devices_Properties,
                          Win32_System_Com_StructuredStorage,
                          Win32_System_Threading, Win32_System_Performance,
                          Win32_Security, Win32_System_SystemServices,
                          Win32_System_Variant, Win32_UI_Shell_PropertiesSystem,
                          Win32_Foundation
windows non-optional    = windows-core ^0.62
windows optional        = asio-sys, audio_thread_priority, jack, num-traits (NOT enabled)
```

Consequences recorded exactly, without over-claiming:

- the WASAPI backend is compiled in on Windows without enabling any
  feature, so no `[features]` entry or workflow change is required;
- no ASIO SDK is fetched or linked (`asio-sys` stays disabled), so the
  bound edge cannot pull a non-free SDK or a driver-kernel component;
- no realtime thread-priority helper (`audio_thread_priority`) is
  pulled in by this edge;
- whether cpal's WASAPI backend can actually open a microphone or a
  loopback endpoint is **not** proven here. That remains a Gate E
  platform claim for the adapter grains on `windows-latest`; this
  adoption only makes the binding addressable.

## Closure and lock invariants

```text
EXPECTED_TOTAL_EXTERNAL_COUNT = 211 (unchanged)
EXPECTED_PROVIDER_EXTERNAL_COUNT = 41 (unchanged)
Cargo.lock delta              = none (single-file diff: Cargo.toml + closure tool)
registry delta                = none (cpal 0.18.2 already registered)
notices/SBOM delta            = none (no distribution-component change)
other target edges            = untouched (macos, linux expectations re-pinned unchanged)
```

`cargo metadata --locked` succeeds against the edited manifest with the
existing lockfile, which is the mechanical proof that no lock
resolution change was needed.

## Verification

Local (this workstation, rustc/cargo 1.98.1, Python 3.12 gate runtime):

```text
python tools/004p_dependency_closure.py              -> 004P P005/P006 CLOSURE PASS
python tools/provenance_gate.py validate             -> PROVENANCE V2 PASS
python tools/provenance_gate.py check-generated      -> GENERATED OUTPUTS PASS
cargo metadata --locked --format-version 1 --no-deps -> exit 0
cargo tree --locked --target x86_64-pc-windows-msvc -i cpal -> cpal -> himsat-core
cargo fmt --all -- --check                           -> clean
```

Not claimed locally: Windows compilation, `clippy`, and tests. This
workstation has no MSVC linker, so those are evidenced only by the
`windows-latest` CI job on the exact head, and by the post-merge run on
the canonical merge SHA. Local `cargo tree` is dependency-resolution
evidence, not build evidence.

## Pre-existing host-specific self-test condition (preserved negative evidence)

Running `python tools/provenance_gate.py self-test` on this Windows
workstation fails with:

```text
PROVENANCE V2 FAIL: legacy self-test failed: fixture allowed-permissive-copy
unexpectedly failed: entry demo: digest mismatch for third_party/demo.txt
```

Root cause, proven rather than assumed: the self-test fixture writes
`third_party/demo.txt` through `pathlib.Path.write_text(..., encoding="utf-8")`
(`tools/provenance.py`), which performs newline translation to CRLF on a
Windows host. The fixture's expected digest
`eb9c26baee47f19e4993a77bca936d0ff09e355a82d3db79bf154ebff1a80604` is the
digest of `b"demo\n"`; the CRLF byte sequence hashes to
`a33f1eaa6dde74881fb719324e3255265f37fdfe27291b3253b86099fec6aca0`, so the
positive fixture is reported as a digest mismatch.

Classification: pre-existing and host-specific, **not** introduced by
this unit. Proof: the identical failure reproduces on a pristine
detached worktree of canonical `main` at
`57efb2e3c5ef34d8205babae68121c13c609519c` with no candidate changes
applied, while `validate` and `check-generated` pass both there and
here. CI runs the adversarial self-test job on `ubuntu-latest` (POSIX
newline semantics), which is why `main` is green.

Disposition: this is outside the grain's `scope_in`, so the defect is
not silently repaired here, and the affected local command is not used
as acceptance evidence. It is carried forward as a recorded follow-up
item; no claim is made that the self-test passes on a Windows host.

Exact-head CI/R3 and the canonical merge/parents/tree/post-merge
qualification for this unit are recorded in the adopting PR and in the
immediately following reconciliation, which is also the unit that
promotes the Windows microphone implementation grain. Nothing in this
document is a Windows capture support claim; `SPEC_008_PLATFORM_SCOPE`
remains `WINDOWS_ONLY_PENDING_GATE_E_EVIDENCE`.
