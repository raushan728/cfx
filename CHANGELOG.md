# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0]

### Added
- Core `encrypt` and `decrypt` commands using ChaCha20Poly1305 STREAM authenticated encryption.
- Robust Argon2id key derivation using random 32-byte salts.
- `info` command to read `.cfx` headers and report metadata safely.
- `verify` command to cryptographically authenticate encrypted payloads without outputting plaintext.
- `recover` command to clean up interrupted operations (orphaned `.tmp` files).
- V1 binary `.cfx` format implementation with secure metadata chunks.
- Memory-efficient streaming chunk-based processing for files up to 10 GiB.
- Extensive CLI test suite testing all functional commands natively via piped `stdin`.
