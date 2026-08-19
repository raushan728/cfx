# CFX

CFX is a cross-platform CLI utility for securely encrypting and decrypting individual files.


## Key Features
- **Secure by Default**: Employs ChaCha20Poly1305 with streaming authenticated encryption for robust confidentiality and integrity.
- **Password-Based**: Derives extremely strong cryptographic keys using the modern Argon2id key derivation function.
- **Universal Support**: Seamlessly processes any file type (PDFs, Images, Source Code, Raw Binaries, etc.) as raw byte streams.
- **Streaming I/O**: Capable of securely processing files using minimal constant memory, rather than loading entire files into RAM.
- **10 GiB File Limit**: Built-in guardrails restrict operations to files 10 GiB or smaller to maintain streaming stability.
- **Failure Safe**: Preserves original files until operations fully succeed and gracefully catches interruption to avoid data corruption.
- **Platform Agnostic**: Runs entirely natively on Windows, Linux, and macOS without dependencies.

## Usage

CFX provides simple, intuitive commands for securely managing your files:
- `cfx encrypt`: Securely encrypt a file.
- `cfx decrypt`: Decrypt a `.cfx` file back to its original state.
- `cfx info`: View non-secret metadata about an encrypted file.
- `cfx verify`: Cryptographically verify a file without decrypting it.
- `cfx recover`: Clean up temporary files from interrupted operations.

**For full beginner-friendly instructions and detailed examples of how to run these commands, please see the [Command Reference & Usage Guide](docs/CLI.md).**

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
- [Examples & Cookbook](docs/EXAMPLES.md)
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

## Reach Out
Have questions, suggestions, or just want to say hi? Feel free to connect:
- **Twitter / X**: [@Raushan_090](https://twitter.com/Raushan_090)
- **LinkedIn**: [Raushan Singh](https://www.linkedin.com/in/raushan-singh-807916390)
- **Email**: [raushansinghrajpoot687@gmail.com](mailto:raushansinghrajpoot687@gmail.com)

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
