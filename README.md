# CFX

CFX is a cross-platform CLI utility for securely encrypting and decrypting individual files.

**To start using CFX**, learn more in the [Command Reference & Usage Guide](docs/CLI.md).

**To explore real-world scenarios**, check out the [Examples & Cookbook](docs/EXAMPLES.md).

> CFX provides simple, intuitive commands for securely managing your files. 
> It utilizes a custom binary format (`.cfx`), modern key derivation (`argon2id`), and streaming authenticated encryption (`chacha20poly1305`) to guarantee absolute confidentiality and full file integrity.

## Code Status

[![CI](https://github.com/raushan728/cfx/actions/workflows/release.yml/badge.svg)](https://github.com/raushan728/cfx/actions/workflows/release.yml)

## Features

CFX is designed to be highly secure and reliable:

* **Secure by Default**: Employs ChaCha20Poly1305 with streaming authenticated encryption for robust confidentiality and integrity.
* **Password-Based**: Derives extremely strong cryptographic keys using the modern Argon2id key derivation function.
* **Universal Support**: Seamlessly processes any file type (PDFs, Images, Source Code, Raw Binaries, etc.) as raw byte streams.
* **Streaming I/O**: Capable of securely processing files up to 10 GiB using minimal constant memory, rather than loading entire files into RAM.
* **Failure Safe**: Preserves original files until operations fully succeed and gracefully catches interruption to avoid data corruption.
* **Platform Agnostic**: Runs entirely natively on Windows, Linux, and macOS without dependencies.

*Important Password Warning*: CFX does NOT implement backdoors, recovery phrases, or "forgot password" features. If you lose your password, your encrypted data is permanently inaccessible.

## Compiling from Source

### Requirements

CFX requires the following tools and packages to build:

* `cargo` and `rustc`
* A C compiler (for your platform)
* `git` (to clone this repository)

**Optional system libraries:**
CFX relies on robust Rust cryptography libraries. No external C dependencies like OpenSSL are required, making cross-platform compilation seamless.

### Compiling

First, you'll want to check out this repository:

```bash
git clone https://github.com/raushan728/cfx.git
cd cfx
```

With `cargo` already installed, you can simply run:

```bash
cargo build --release
```

## Documentation

Comprehensive documentation covering architecture, cryptography, the file format, and installation instructions is available in the [`docs/` directory](docs/).

## Releases

Automated releases for Windows, macOS, and Linux are available on the [GitHub Releases page](https://github.com/raushan728/cfx/releases). 
Detailed release notes are available in the [CHANGELOG](CHANGELOG.md).

## Reporting issues

Found a bug or have a feature request? We'd love to know about it!

Please report all issues on the GitHub [issue tracker](https://github.com/raushan728/cfx/issues).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for a complete introduction to contributing to CFX, and [SECURITY.md](SECURITY.md) for vulnerability reporting guidelines.

## Reach Out

Have questions, suggestions, or just want to say hi? Feel free to connect:
- **Twitter / X**: [@Raushan_090](https://twitter.com/Raushan_090)
- **LinkedIn**: [Raushan Singh](https://www.linkedin.com/in/raushan-singh-807916390)
- **Email**: [raushansinghrajpoot687@gmail.com](mailto:raushansinghrajpoot687@gmail.com)

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
