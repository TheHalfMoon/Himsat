# Founder Source-Code Reuse Authority

Date recorded: 2026-09-06
Status: project-governance authority; not a substitute for path-level provenance

## Founder attestation

The Himsat founder has stated that they have permission from the owners of **all source-code projects discussed for Himsat up to and including 2026-09-06** to use, copy, modify, adapt, and incorporate their project-owned source code for Himsat as needed.

For Himsat governance, this statement is accepted as founder-granted reuse authority for source code that is actually owned or relicensable by the granting project/rightsholder.

This authority is intentionally broader than the repositories' public licenses where the founder's special permission is valid. It allows Himsat planning to consider source code that would otherwise be `REFERENCE_ONLY` under the default public-license posture, subject to the controls below.

## What this authority covers

When the granting party has the necessary rights, the founder attestation may support selective reuse of:

- application/library source code;
- tests and test helpers;
- build scripts and source-level platform helpers;
- source-level examples needed to implement or verify the adopted behavior;
- source comments/documentation embedded in copied code where required to understand or preserve the implementation.

Himsat still prefers the smallest useful source surface rather than copying whole repositories.

## What this authority does not automatically cover

The permission statement is about source code. It must **not** be silently stretched to cover material whose rights may belong to someone else, including:

- third-party or vendored source code not owned/relicensable by the granting project;
- Git submodules or downloaded dependencies;
- model weights, adapters, tokenizers, datasets, benchmarks, training corpora, or generated model artifacts;
- fonts, icons, logos, trademarks, screenshots, sample media, music, voices, templates, or other creative assets;
- prebuilt binaries whose corresponding source/provenance is different from the project-owned source;
- operating-system SDKs/runtime components;
- patents or trademark rights unless separately granted;
- proprietary products that were discussed only as behavioral references and whose source code was never supplied as a donor source.

If a copied path contains mixed copyright ownership or imported/generated material, Himsat must separate the project-owned portion from foreign material before adoption.

## Public license and special permission are separate facts

Every adopted source path records two independent dimensions:

```text
public_license
permission_basis
```

Allowed `permission_basis` values for future machine governance should include:

```text
public_license
founder_attested_special_permission
public_license_and_special_permission
separate_written_permission
```

The public license remains valuable provenance even when special permission exists. Himsat must not rewrite history by pretending an AGPL/custom/commercial source was publicly permissive merely because the founder has separate permission.

## Required adoption record

Before any donor source bytes enter canonical Himsat, the adoption record must identify at minimum:

```text
donor_name
source_repository
source_revision
source_path(s)
public_license
permission_basis
permission_scope
rights-holder/grantor identity when known
permission_evidence_reference
third_party_exclusions
copyright/notice handling
destination_path(s)
adoption_mode
adaptation_description
Himsat contract/behavior target
source tests or behavioral evidence reused
Himsat tests
security/platform qualification
upstream-tracking owner
```

`permission_evidence_reference` may initially reference this founder attestation for planning, but high-risk or publicly non-permissive source should retain any written owner permission when available so the repository can preserve durable evidence independent of a chat transcript.

## Special-permission rule for otherwise restricted code

A public AGPL/GPL/source-available/custom/commercial license no longer makes a donor automatically `REFERENCE_ONLY` **if** all of the following are true:

1. the founder attestation applies to that source project;
2. the exact copied path is project-owned or otherwise covered by the special permission;
3. Himsat records the public license truth rather than relabeling it;
4. foreign/vendored/generated material is excluded or separately authorized;
5. copyright/attribution obligations or requested notices are preserved unless the special permission explicitly says otherwise;
6. the path passes the normal provenance, security, architecture, and test gates;
7. the active SpecGrain unit explicitly authorizes the bounded adoption.

The special permission changes the **rights gate**. It does not waive engineering, security, quality, privacy, or provenance gates.

## Relicensing posture

The founder has described the permission as allowing use of the source code "as I like." Himsat therefore may plan for incorporation into the Himsat permissive codebase where the granting rightsholder has authority to grant those rights.

However, Himsat should preserve original copyright/provenance notices in `THIRD_PARTY_NOTICES` and source headers where practical. Do not remove attribution merely because broader permission may exist.

If a path has multiple independent copyright holders and there is no evidence that the grant covers all of them, do not assume the special permission can relicense that path; either rely on a compatible public license for those contributions, obtain separate permission, or exclude/reimplement the uncertain portion.

## No blanket repository copy

Founder permission is not an instruction to concatenate repositories.

The mandatory shape remains:

```text
source truth
  -> rights/provenance decomposition
  -> behavior contract
  -> donor arbitration
  -> smallest selected transplant/dependency
  -> Himsat adaptation
  -> focused + adversarial tests
  -> full regression/benchmark/security proof
  -> exact-head merge
```

This prevents legal permission from becoming architectural debt.

## Current authority boundary

This document changes donor-planning rights assumptions only. It does **not** override `specs/CURRENT.md`, activate blocked implementation units, authorize donor bytes during Specification 004, or make the provenance registry non-empty.

The machine adoption registry remains the final canonical adoption authority when the dependency-ordered frontier reaches each donor-consuming unit.
