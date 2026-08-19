# Error Handling

CFX uses a centralized error domain called `CfxError` (powered by the `thiserror` crate). 

Errors are mapped explicitly to user-facing console output to provide actionable feedback.

## `CfxError` Variants & Behavior

### `Io(std::io::Error)`
- **Trigger**: System-level I/O failures (e.g., missing files, permission denied, disk full).
- **Behavior**: Halts execution. If it occurs during encryption or decryption midway, the temporary `.tmp` file is immediately deleted to avoid leaving corrupted artifacts.

### `Crypto(String)`
- **Trigger**: MAC mismatches, tampered data, truncation, or key derivation failures.
- **Behavior**: Fails immediately. Often indicates the file has been maliciously modified or the `STREAM` chunks are corrupted.

### `IncorrectPassword`
- **Trigger**: The derived key fails to authenticate the first metadata chunk of the `.cfx` payload.
- **Behavior**: Throws exactly `"Invalid password or corrupted CFX file"`. CFX cannot distinguish between a wrong password and a corrupted first chunk by design.

### `FileTooLarge`
- **Trigger**: CFX intercepts files larger than 10 GiB during the initial metadata validation phase.
- **Behavior**: The application halts cleanly before allocating large buffers or opening file streams.

### `AlreadyExists(PathBuf)`
- **Trigger**: A user attempts to output to a file path that already exists on disk without providing the `--force` flag.
- **Behavior**: Halts immediately. Protects users from accidentally overwriting important files.

### `InvalidFormat(String)`
- **Trigger**: The magic bytes don't match `b"CFX1"`, the version is unsupported, algorithms don't match expected values, or chunk sizes violate boundaries.
- **Behavior**: Fails the file inspection before any password prompting or cryptographic operations occur. 

### `NotARegularFile(PathBuf)`
- **Trigger**: The user provides a directory or symlink pointing to an invalid type.
- **Behavior**: Returns immediately. CFX explicitly does not support directory encryption.

### `PasswordPrompt(String)`
- **Trigger**: Failed to securely open `/dev/tty` or read `stdin` while acquiring the password.
- **Behavior**: Exits with an I/O context error.

## Exit Codes
CFX relies on standard Rust application boundaries. 
- Success yields an exit code of `0`.
- Any propagated `CfxError` will output to `stderr` and yield a non-zero exit code (typically `1`).
