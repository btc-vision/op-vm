# Security Audit

## Audited By

<a href="https://verichains.io/">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="./verichains-logo.svg">
    <source media="(prefers-color-scheme: light)" srcset="./verichains-logo-dark.svg">
    <img alt="Verichains" src="./verichains-logo.svg" height="48">
  </picture>
</a>

**[Verichains](https://verichains.io/)**

## Audit Status

| Component | Status | Auditor |
|-----------|--------|---------|
| OP-VM Core | Audited | Verichains |

OP-VM has been professionally audited and is considered **stable and secure** for mainnet deployment.

## Audit Scope

The security audit covered:

- Memory safety and bounds checking
- Gas metering accuracy and bypass prevention
- Sandbox isolation and escape prevention
- WASM execution environment security
- External function call handling
- State management and persistence
- Cryptographic operations

## Audit Reports

Audit reports and findings can be found in this directory.

## Reporting Security Issues

For reporting security vulnerabilities, please see our [Security Policy](../SECURITY.md).

**DO NOT** create public GitHub issues for security vulnerabilities. Use [GitHub Security Advisories](https://github.com/btc-vision/op-vm/security/advisories/new) instead.
