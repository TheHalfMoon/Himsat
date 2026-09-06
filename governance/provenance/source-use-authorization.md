# Himsat Source-Use Authorization Record

## Status

```text
AUTHORIZATION_RECORDED_DATE = 2026-09-07
AUTHORIZATION_BASIS = FOUNDER_USER_ATTESTATION
COVERED_REPOSITORY_SNAPSHOT = 97535af240aabf130153fce7718f9ca2eb3f85bf
COVERAGE = ALL_EXTERNAL_SOURCES_REFERENCED_ANYWHERE_IN_THE_REPOSITORY_AT_OR_BEFORE_THE_COVERED_SNAPSHOT
PERMITTED_REUSE_MODES = REFERENCE | COPY | ADAPT | DEPEND | VENDOR
AUTOMATIC_ADOPTION = NO
ACTIVE_SPEC_AUTHORITY_CHANGED = NO
PROVENANCE_GATES_WAIVED = NO
LICENSE_NOTICE_GATES_WAIVED = NO
MODEL_DATA_ASSET_GATES_WAIVED = NO
```

## Founder attestation

The project founder/user has explicitly stated that Himsat has permission to use the code/material from every source already recorded in the repository, including the sources most recently added through the Superwhisper/OpenSuperWhisper research.

Himsat records that statement as project-level source-use authorization. Agents should no longer treat a source as unusable merely because an earlier Himsat planning document assumed that no additional permission existed.

This authorization is an input to adoption decisions, not evidence that every byte in every upstream repository is owned by the same rightsholder or governed by the same terms.

## What this authorization enables

For a covered source, a future authorized implementation leaf may choose the technically best reuse mode:

- `REFERENCE` — study behavior/architecture and implement independently;
- `COPY` — copy selected source paths;
- `ADAPT` — closely adapt selected source paths into Himsat-owned contracts;
- `DEPEND` — consume an exact upstream package/library;
- `VENDOR` — preserve an immutable source snapshot where deterministic/offline distribution justifies it.

The existence of founder permission means Himsat should compare reuse against native reimplementation on engineering merit rather than defaulting to reimplementation solely because permission was previously uncertain.

## What this authorization does not prove

Before actual adoption, Himsat still must establish the exact facts needed for a clean, reproducible, redistributable codebase:

1. exact upstream repository, immutable revision/tag, and exact source path(s);
2. which copyright/license/permission terms control those exact paths;
3. whether the asserted permission covers modification, redistribution, sublicensing, and commercial distribution as required by the intended Himsat use;
4. required attribution, copyright, NOTICE, trademark, patent, or other obligations;
5. separately governed embedded/vendor/generated code;
6. submodules and transitive dependencies;
7. model weights, datasets, fonts, icons, screenshots, templates, sample media, firmware, binaries, and other non-code assets;
8. security/privacy/platform implications of the candidate implementation;
9. exact Himsat destination paths and adaptation description;
10. Himsat-owned behavior tests and applicable regression/security/platform evidence.

If an upstream project contains material from a rightsholder outside the asserted permission scope, that material is not automatically covered by this record. Himsat must isolate, replace, separately qualify, or avoid that material.

## Interaction with license classification

Earlier Himsat documents sometimes used `REFERENCE_ONLY` as a conservative default because no additional permission was assumed. For covered sources, that reason alone is superseded by this authorization record.

A restrictive, source-available, copyleft, custom, unknown, or historically changed public license therefore does not automatically prevent evaluation of a covered source when Himsat has a separate permission grant. However, the exact permission scope must be recorded in the adoption provenance entry before restricted-license code is copied or redistributed.

Where the source's ordinary public license is already compatible, Himsat should continue to rely on and preserve the exact public license/notice evidence because it is durable, auditable, and usually simpler than relying on a separate permission assertion.

## Source availability rule

Permission does not create source bytes that Himsat cannot access. Proprietary products with no available source remain behavioral/product references until the relevant source material is actually available to the project and can be pinned to an immutable identity.

## Authority boundary

This record does not pull implementation authority forward.

The active specification, `specs/CURRENT.md`, SpecGrain/Diffcipline gates, security review requirements, provenance registry, dependency/native closure rules, and exact-head merge qualification remain controlling.

In particular:

```text
FOUNDER_PERMISSION != ACTIVE_GRAIN_AUTHORITY
FOUNDER_PERMISSION != LICENSE_PROVENANCE_CLOSURE
FOUNDER_PERMISSION != DEPENDENCY_APPROVAL
FOUNDER_PERMISSION != MODEL_OR_ASSET_APPROVAL
FOUNDER_PERMISSION != SECURITY_QUALIFICATION
FOUNDER_PERMISSION != MERGE_APPROVAL
```

## Future source additions

This record covers sources referenced in the repository at or before the exact covered snapshot above. A later source should carry its own explicit permission/license evidence unless a newer authorization record expands this scope.

## Required agent behavior

When a covered source becomes relevant:

1. read this authorization record;
2. reverify live upstream truth;
3. prefer the smallest technically superior reuse strategy;
4. do not reject reuse solely because an earlier Himsat document assumed permission was absent;
5. do not bulk-copy an upstream repository merely because reuse is permitted;
6. register exact adopted material through Himsat provenance machinery before canonical adoption;
7. preserve the boundary between source permission and third-party/model/data/asset provenance;
8. keep Himsat architecture and tests authoritative after adoption.
