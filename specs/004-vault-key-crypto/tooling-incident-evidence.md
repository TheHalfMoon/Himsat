# Specification 004A Tooling Incident Evidence

## Purpose

Preserve exact negative evidence for an accidental repository mutation during the Specification 004A review-only handoff sequence and document the forward-only recovery without weakening Diffcipline scope policy.

## Incident

After PR #21 merged, canonical Specification 004A design merge `cb8511c1b420c58f714768c3561e74f04f026b3a` passed post-merge qualification:

```text
CI = 34057861438 SUCCESS
R3_SECURITY_GATE = 34057861447 SUCCESS
```

While attempting to create a review-only branch, an incorrect GitHub tool action accidentally created root file `DO_NOT_CREATE` containing `noop` directly on `main`:

```text
ACCIDENTAL_ADD_COMMIT = 529b96411a5dbb40f6007b697097354cabf539b5
```

The error was disclosed immediately and repaired forward-only. No force-push, rebase, reset, or shared-history rewrite was used. The accidental file was deleted in:

```text
CLEANUP_COMMIT = 14a230d542c997d27fb029c9546814fcf16c5b7a
```

A GitHub comparison from `cb8511c1b420c58f714768c3561e74f04f026b3a` to `14a230d542c997d27fb029c9546814fcf16c5b7a` reported `files = []`, proving the cleanup tree returned to the exact pre-incident tree. The two accidental-history commits remain visible as required by forward-only history discipline.

## Negative qualification evidence

Exact cleanup SHA `14a230d542c997d27fb029c9546814fcf16c5b7a` triggered R3 Security Gate run `34057947555`, which correctly failed.

The failing Diffcipline proof compared the cleanup commit against its immediate parent `529b96411a5dbb40f6007b697097354cabf539b5` and reported:

```text
Verdict = FAIL
Risk = R3
Scope = FAIL — 1 violation
Changed = 1 file
Diff = +0 / -1
Reason = unexpected changed file: DO_NOT_CREATE
```

All verification commands inside that proof passed; the failure was specifically the out-of-scope path. This failure is valid governance evidence and MUST NOT be reclassified as PASS merely because the resulting tree matches an earlier qualified tree.

## Recovery rule

The recovery is a new bounded documentation/evidence leaf under the already allowed Specification 004 path. It does not alter cryptographic design semantics, dependencies, product source, provenance policy, Diffcipline configuration, workflow configuration, or adoption authority.

The next canonical revision may become review-eligible only after:

1. this recovery leaf passes exact-head CI and R3 without policy relaxation;
2. its merge uses expected-head protection after live reconciliation;
3. post-merge CI and R3 pass on the exact new canonical SHA;
4. the review-only head points directly to that exact canonical SHA with no review-content commit.

The failed cleanup run `34057947555` remains part of the durable evidence record.

## Authority

```text
SECURITY_DESIGN_CHANGE = NONE
DEPENDENCY_ADOPTION_AUTHORITY = BLOCKED
SPEC_004_IMPLEMENTATION_AUTHORITY = BLOCKED
REVIEW_ONLY_AUTHORITY = PENDING_EXACT_CANONICAL_REQUALIFICATION
```
