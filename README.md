# CFX

CFX is a cross-platform CLI utility for securely encrypting and decrypting individual files.

## Project Status
**Milestone 3 / v0.1.0**: Core functionality complete. All primary commands (`encrypt`, `decrypt`, `info`, `verify`, `recover`) are fully implemented and verified via comprehensive integration testing.

## Key Features
- **Secure by Default**: Employs ChaCha20Poly1305 with streaming authenticated encryption for robust confidentiality and integrity.
- **Password-Based**: Derives extremely strong cryptographic keys using the modern Argon2id key derivation function.
- **Universal Support**: Seamlessly processes any file type (PDFs, Images, Source Code, Raw Binaries, etc.) as raw byte streams.
- **Streaming I/O**: Capable of securely processing files using minimal constant memory, rather than loading entire files into RAM.
- **10 GiB File Limit**: Built-in guardrails restrict operations to files 10 GiB or smaller to maintain streaming stability.
- **Failure Safe**: Preserves original files until operations fully succeed and gracefully catches interruption to avoid data corruption.
- **Platform Agnostic**: Runs entirely natively on Windows, Linux, and macOS without dependencies.

## Usage

### Encrypt a File
Securely encrypt `secret.pdf` into `secret.cfx`:
```bash
cfx encrypt secret.pdf
```
By default, the original file is safely removed upon successful encryption. Use the `--keep` flag to retain the original file, or `-o / --output <PATH>` to dictate exactly where the encrypted file is written.

### Decrypt a File
Decrypt `secret.cfx` back to its original unencrypted form:
```bash
cfx decrypt secret.cfx
```
The original filename is securely stored inside the encrypted payload and will be automatically restored by default.

### Inspect an Encrypted File
View non-secret metadata (such as the CFX format version and encryption algorithms) without needing the password:
```bash
cfx info secret.cfx
```

### Verify File Integrity
Authenticate a file's integrity and verify your password without outputting a decrypted plaintext file to disk:
```bash
cfx verify secret.cfx
```

### Recovery
Safely clean up orphaned temporary files (`.tmp`) left behind if a CFX operation was abruptly interrupted:
```bash
cfx recover incomplete_file.cfx.tmp
```

## Security Overview
CFX utilizes a custom binary format (`.cfx`). It uses `argon2` for password-based key derivation alongside a 32-byte salt. The file contents are chunked and encrypted sequentially using the STREAM construction of `chacha20poly1305`. This guarantees data confidentiality and full file integrity—each chunk is independently authenticated, fully preventing truncation, reordering, or tampering attacks. 

**Important Password Warning**: CFX does NOT implement backdoors, recovery phrases, or "forgot password" features. If you lose your password, your encrypted data is permanently inaccessible. 

## Development
To format, lint, test, and build the project from source:
```bash
cargo fmt --all
cargo check --all-targets
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

## Documentation Index
Comprehensive documentation on the internal implementation of CFX is available in the `docs/` directory:
- [Architecture](docs/ARCHITECTURE.md)
- [CLI Reference](docs/CLI.md)
- [Cryptography](docs/CRYPTOGRAPHY.md)
- [File Format](docs/FILE_FORMAT.md)
- [Error Handling](docs/ERROR_HANDLING.md)
- [Recovery](docs/RECOVERY.md)
- [Testing](docs/TESTING.md)
- [Development](docs/DEVELOPMENT.md)
- [Installation](docs/INSTALLATION.md)

Also refer to:
- [Changelog](CHANGELOG.md)
- [Contributing](CONTRIBUTING.md)
- [Security](SECURITY.md)

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
