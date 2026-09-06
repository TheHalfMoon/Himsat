# Contributing to Himsat

Himsat is evidence-first and local-first. Contributions must preserve repository governance before optimizing for feature velocity.

## Before changing code

Read, in order:

1. `AGENTS.md`
2. `specs/CURRENT.md`
3. `.specify/memory/constitution.md`
4. `docs/execution-master-plan.md`
5. the active specification

Live repository and GitHub state override stale summaries.

## Development baseline

The repository pins its Rust toolchain in `rust-toolchain.toml`. Run:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --all-targets --locked
```

A configured check that was not executed is not a pass.

## Scope discipline

- Do not introduce donor code without the exact provenance/license authority required by the active specification.
- Do not add dependencies merely for convenience.
- Do not add network, telemetry, model, capture, or product-domain behavior unless the active specification authorizes it.
- Keep changes bounded to the declared change surface.
- Preserve failed or unavailable verification honestly.

## Licensing

Himsat-owned source is licensed under Apache-2.0 unless a repository artifact states otherwise. Donor code, models, datasets, fonts, media, and other third-party artifacts keep their controlling licenses and notice obligations; Apache-2.0 does not overwrite them.
