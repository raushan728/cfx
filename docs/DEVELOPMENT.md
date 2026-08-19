# CFX Development Guide

## Project Structure

```
cfx/
├── Cargo.toml            # Project dependencies and configurations
├── README.md             # Project overview
├── src/
│   ├── main.rs           # CLI entry point (clap router)
│   ├── crypto.rs         # Cryptographic primitives and KDF boundaries
│   ├── error.rs          # Centralized error domain
│   ├── format.rs         # CFX file format binary specification
│   └── ops/              # Core command operations
│       ├── encrypt.rs
│       ├── decrypt.rs
│       ├── info.rs
│       ├── verify.rs
│       └── recover.rs
└── tests/
    └── cli.rs            # End-to-end integration tests
```

## Local Development Workflow

CFX uses a standard Rust toolchain. To contribute to the project, you must have `cargo` installed.

### 1. Code Formatting
```bash
cargo fmt --all
```
Uses `rustfmt` to ensure the source code matches standard Rust community formatting rules. This must be run before any PR.

### 2. Static Analysis & Compilation Check
```bash
cargo check --all-targets
```
Rapidly compiles the codebase without producing binaries to verify that everything is syntactically valid and compiles cleanly.

### 3. Linting
```bash
cargo clippy --all-targets --all-features -- -D warnings
```
Runs Rust's official linter to catch unidiomatic code, performance issues, or logical bugs. The `-D warnings` flag treats all warnings as hard errors.

### 4. Testing
```bash
cargo test --all-targets
```
Executes the integration suite against your local changes.

### 5. Building the Binary
```bash
cargo build --release
```
Compiles a highly optimized, production-ready standalone executable. The output binary is located at `target/release/cfx`.
