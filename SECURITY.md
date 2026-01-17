# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0.x   | :white_check_mark: |
| < 1.0   | :x:                |

## Security Audit

OP-VM has been professionally audited by **[Verichains](https://verichains.io/)**.

Audit reports and findings are available in the [AUDIT](./AUDIT) directory.

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue, please report it responsibly.

### How to Report

1. **DO NOT** create a public GitHub issue for security vulnerabilities
2. Email security concerns to the maintainers via the [OP_NET](https://opnet.org/) contact channels
3. Include as much detail as possible:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
- **Assessment**: We will assess the vulnerability and determine its severity
- **Updates**: We will keep you informed of our progress
- **Resolution**: We aim to resolve critical vulnerabilities as quickly as possible
- **Credit**: With your permission, we will credit you in the security advisory

### Scope

The following are in scope for security reports:

- OP-VM core runtime vulnerabilities
- Memory safety issues
- Gas metering bypasses
- Sandbox escapes
- Cryptographic weaknesses

### Out of Scope

- Denial of service attacks that require significant resources
- Issues in dependencies (please report these to the respective projects)
- Issues that require physical access to the machine

## Security Best Practices

When using OP-VM in production:

1. Always use the latest stable release
2. Keep dependencies up to date
3. Run the VM in a sandboxed environment
4. Monitor for unusual resource consumption
5. Follow the principle of least privilege

## Security Updates

Security updates will be released as patch versions (e.g., 1.0.1, 1.0.2) and announced through:

- GitHub Releases
- GitHub Security Advisories
- The [OP_NET](https://opnet.org/) website
