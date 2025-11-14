# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Which versions are eligible for receiving such patches depends on the CVSS v3.0 Rating:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:
- **Email**: security@example.com (replace with actual email)

You should receive a response within 48 hours. If for some reason you do not, please follow up via email to ensure we received your original message.

Please include the following information in your report:

- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit the issue

This information will help us triage your report more quickly.

## Preferred Languages

We prefer all communications to be in English or Japanese.

## Disclosure Policy

When we receive a security bug report, we will:

1. Confirm the problem and determine the affected versions
2. Audit code to find any potential similar problems
3. Prepare fixes for all supported versions
4. Release new security versions as soon as possible

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request or open an issue to discuss.

## Security Update Process

1. **Assessment**: We assess the vulnerability and its impact
2. **Fix Development**: We develop a fix in a private repository
3. **Testing**: We thoroughly test the fix
4. **Release**: We release a security patch
5. **Notification**: We notify users through:
   - GitHub Security Advisories
   - Release notes
   - Social media/announcements
   - Direct email (for critical vulnerabilities)

## Security Best Practices for Users

### For CLI Usage

1. **Verify Downloads**
   - Always download from official sources (GitHub Releases, crates.io)
   - Verify SHA256 checksums

2. **Keep Updated**
   - Regularly update to the latest version
   - Subscribe to security advisories

3. **Review Code**
   - When building from source, review the code
   - Check the commit signatures

### For Development

1. **Dependency Management**
   - We regularly update dependencies
   - We use `cargo audit` to check for vulnerabilities

2. **Code Review**
   - All PRs are reviewed by maintainers
   - Security-sensitive changes receive extra scrutiny

3. **Testing**
   - Comprehensive test suite
   - CI/CD security checks

## Known Security Considerations

### File System Access

Lala accesses the file system to:
- Read and write text files
- Scan directories for file tree display
- Respect `.gitignore` files

**Mitigations**:
- Does not follow symbolic links (prevents directory traversal)
- Respects file system permissions
- Displays access denied errors gracefully

### Terminal Output

Lala renders content to the terminal which could potentially include:
- ANSI escape codes
- Control characters

**Mitigations**:
- Sanitizes terminal output
- Uses safe terminal libraries
- Limits escape sequence usage

### External Tools

Lala may invoke external tools:
- ripgrep for search functionality

**Mitigations**:
- Uses well-maintained, security-audited tools
- Validates input before passing to external tools
- Runs with user permissions (no privilege escalation)

## Security Audit History

| Date | Type | Findings | Status |
|------|------|----------|--------|
| 2025-11-14 | Initial Review | None | Baseline |

## Acknowledgments

We would like to thank the following individuals for responsibly disclosing security issues:

- *None yet*

## Hall of Fame

Security researchers who have helped make Lala more secure:

- *Waiting for first contributor*

---

**Last Updated**: 2025-11-14
