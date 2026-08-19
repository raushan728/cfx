# Installation

CFX is currently in its early development phases. 

**Official pre-compiled binaries, installation scripts, and uninstallation automation are currently unimplemented.**

## Building from Source

To use CFX today, you must compile it from source using Rust's package manager, `cargo`.

1. Ensure you have Rust installed via `rustup`.
2. Clone the repository and navigate into it:
   ```bash
   git clone https://github.com/raushan728/cfx.git
   cd cfx
   ```
3. Compile the release binary:
   ```bash
   cargo build --release
   ```

The resulting executable will be placed at:
```bash
target/release/cfx
```

You can optionally move this binary into your system's `PATH` (e.g., `/usr/local/bin` on Linux/macOS) to use the `cfx` command globally.

*(Future Scope: Cross-platform installation automation and GitHub Releases will be implemented in a later milestone.)*
