# CFX

CFX is a cross-platform CLI utility for securely encrypting and decrypting individual files.

## Features
- **Secure by Default:** Uses ChaCha20Poly1305 with streaming authenticated encryption.
- **Password-Based:** Derives robust cryptographic keys using Argon2id.
- **Platform Agnostic:** Works natively on Windows, Linux, and macOS.
- **Universal Support:** Processes any file type (PDF, Images, Source Code, Binaries, etc.).
- **Streaming I/O:** Can encrypt/decrypt files up to 10 GiB using minimal constant memory.
- **Failure Safe:** Preserves original files until successful operation completion and gracefully handles interrupted processes.

## Installation
*(Coming soon: Pre-compiled binaries and platform-specific installers)*

For now, you can build it from source:
```bash
cargo build --release
```

## Usage

### Encrypt a File
Encrypt `secret.pdf` into `secret.cfx`:
```bash
cfx encrypt secret.pdf
```
By default, the original file is removed. Use `--keep` to retain it, or `--output <PATH>` to specify a different output location.

### Decrypt a File
Decrypt `secret.cfx` back to its original form:
```bash
cfx decrypt secret.cfx
```
The original filename is preserved in the encrypted payload and automatically restored.

### Inspect an Encrypted File
View non-secret metadata (like CFX version and algorithm) without needing the password:
```bash
cfx info secret.cfx
```

### Verify File Integrity
Authenticate a file's integrity and verify the password without producing a decrypted output file:
```bash
cfx verify secret.cfx
```

### Recovery
Clean up temporary files (`.tmp`) left behind if an operation was interrupted:
```bash
cfx recover incomplete_file.cfx.tmp
```

## Security Overview
CFX utilizes a custom binary format (`.cfx`) to securely wrap your files. It employs `argon2` for password-based key derivation alongside a randomly generated 32-byte salt. The file contents are chunked and encrypted sequentially using the STREAM construction of `chacha20poly1305`, guaranteeing both data confidentiality and full file integrity. Each chunk is independently authenticated, preventing truncation or tampering attacks.

**Warning:** There are no backdoors. If you lose your password, the encrypted data is permanently inaccessible.

## Development
To format, lint, and test the project:
```bash
cargo fmt --all
cargo check --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```
