# Testing CFX

CFX utilizes an integration testing suite to validate end-to-end correctness. Unit tests are currently deferred in favor of full operational testing.

## Integration Tests (`tests/cli.rs`)

The test suite leverages the `assert_cmd` and `predicates` crates to test the CFX binary identically to how a user would interact with it via their shell.

### Temporary File Isolation
Each test isolates its file operations inside unique, ephemeral directories created by the `tempfile` crate. This guarantees that parallel tests do not cause race conditions on disk and that test pollution is eliminated.

### Stdin Password Testing
Because CFX strictly avoids environment variable passwords for security, integration tests interact with the `crypto` password prompt natively by piping strings via `stdin` (`.write_stdin("password\n")`). This rigorously exercises the production fallback logic used when CLI environments pipe inputs directly.

## Covered Categories
The integration suite successfully validates:
- **Roundtrip Integrity**: Encrypting and decrypting a file identically outputs the original plaintext bytes.
- **Wrong Password Rejection**: Attempting decryption or verification with incorrect passwords immediately yields a failure.
- **Verification Integrity**: Files with correct passwords pass `.cfx` cryptographic verification.
- **`--keep` Flags**: Asserts that original files are not deleted when strictly asked.
- **File Overwrites**: Asserts that `CfxError::AlreadyExists` triggers correctly unless `--force` is used.

*Note: Comprehensive manual validation on edge-cases (like 10 GiB stress tests and terminal disruption tests) is handled during manual validation phases prior to major releases.*
