# Security Policy

Himsat is designed to handle sensitive local data, but the current repository foundation does not yet implement product storage, capture, networking, or cryptography.

## Reporting a vulnerability

Use GitHub's private security-advisory reporting surface for this repository when available. Do not publish exploit details or sensitive user data in a public issue.

Include the affected revision, reproduction conditions, impact, and the smallest safe proof needed to demonstrate the issue.

## Current security boundary

Specification 001 establishes repository and delivery controls only. It must not introduce secrets, telemetry, networked runtime behavior, donor code, models, or product-data processing.

Future capture, cryptography, parser, plugin, biometric, synchronization, and external-action work requires the stronger risk-specific evidence defined by repository governance.

## Supply-chain expectations

- Prefer immutable revisions for CI actions and external control tools.
- Keep workflow permissions least-privilege.
- Treat models, plugins, assets, datasets, and copied source as independently licensed supply-chain objects.
- Never treat public source availability as permission to copy.
