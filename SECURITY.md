# Security Policy

CFX is designed to provide secure, robust file encryption. 

## Security Model

- **Authenticated Encryption**: CFX uses the ChaCha20Poly1305 `STREAM` construction. It guarantees both confidentiality and full file integrity. Data truncation, reordering, and tampering are reliably detected.
- **Local Processing**: All cryptographic operations execute strictly on your local machine. No data or cryptographic material is ever transmitted over a network.
- **No Password Storage**: CFX never stores your password. Cryptographic keys are derived dynamically using Argon2id and discarded immediately after use.
- **Temporary-File Behavior**: Encrypted and decrypted payloads are generated as `.tmp` files. The original source file is preserved until the operation succeeds and authentication is confirmed, actively preventing data loss during interruption.

## Important Limitations & Warnings

- **Password Loss**: There are absolutely no backdoors, key escrows, or "forgot password" features. If you lose the encryption password, the data inside the `.cfx` file is permanently inaccessible.
- **Side Channels**: CFX is a CLI tool designed for normal consumer usage. It does not deploy advanced countermeasures against extreme threats like cold-boot attacks or hardware-level side-channel monitoring on physically compromised machines.

## Reporting a Vulnerability

If you discover a security vulnerability in CFX, please do not disclose it publicly. 

*CFX is currently in early development. Please report security issues directly to the repository maintainers via private channels or responsibly via GitHub's security advisory feature once established.*
