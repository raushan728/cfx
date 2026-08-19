# CLI Reference

CFX is driven via the command line interface. The interface relies on a subcommand structure managed by `clap`.

## Global Flags

- `--help`: Print short usage information for a command or CFX generally.
- `--version`: Print the current CFX version.

---

## `cfx encrypt`

**Purpose**: Encrypt a target file into a `.cfx` archive.

**Syntax**:
```bash
cfx encrypt <INPUT> [OPTIONS]
```

**Arguments**:
- `<INPUT>`: The required file path to encrypt.

**Options**:
- `-o, --output <PATH>`: The explicit path to write the encrypted file to. If omitted, CFX automatically appends `.cfx` to the input filename.
- `-f, --force`: Overwrite the destination output file if it already exists.
- `--keep`: Do not delete the original `<INPUT>` file after successful encryption.
- `--no-progress`: Disable the interactive progress bar.

**Examples**:
```bash
# Basic usage (deletes standard.pdf, creates standard.pdf.cfx)
cfx encrypt standard.pdf

# Keep original file and save output elsewhere
cfx encrypt standard.pdf --keep -o /backups/safe.cfx
```

---

## `cfx decrypt`

**Purpose**: Decrypt a `.cfx` file back into its original form.

**Syntax**:
```bash
cfx decrypt <INPUT> [OPTIONS]
```

**Arguments**:
- `<INPUT>`: The required `.cfx` file to decrypt.

**Options**:
- `-o, --output <PATH>`: The explicit path to write the decrypted file. If omitted, CFX reads the original filename from the encrypted metadata and uses it automatically.
- `-f, --force`: Overwrite the destination output file if it already exists.
- `--keep`: Do not delete the encrypted `<INPUT>` file after successful decryption.
- `--no-progress`: Disable the interactive progress bar.

**Examples**:
```bash
# Decrypt preserving original filename, and delete secret.cfx
cfx decrypt secret.cfx
```

---

## `cfx info`

**Purpose**: Display non-secret header metadata of a `.cfx` file without requiring a password.

**Syntax**:
```bash
cfx info <INPUT>
```

**Arguments**:
- `<INPUT>`: The `.cfx` file to inspect.

**Expected Behavior**:
Reads the plaintext binary header and prints the format version, key derivation algorithm, and encryption algorithm used. It purposefully does not display the original filename or size, as those are encrypted for privacy.

---

## `cfx verify`

**Purpose**: Cryptographically authenticate an encrypted `.cfx` file and verify the password without extracting any data to disk.

**Syntax**:
```bash
cfx verify <INPUT> [--no-progress]
```

**Arguments**:
- `<INPUT>`: The `.cfx` file to verify.

**Options**:
- `--no-progress`: Disable the interactive progress bar.

**Expected Behavior**:
Prompts for a password, derives the key, and iterates through every cryptographic chunk in the file to validate the Poly1305 MACs. Prints a success message if the file is pristine. Throws a cryptographic error on tampering or incorrect passwords.

---

## `cfx recover`

**Purpose**: Detect and safely clean up temporary files (`.tmp`) left behind by interrupted or failed operations.

**Syntax**:
```bash
cfx recover <INPUT> [OPTIONS]
```

**Arguments**:
- `<INPUT>`: The target `.tmp` file.

**Expected Behavior**:
Validates that the file is an orphaned `.tmp` payload. If valid, deletes the corrupted file to clean up disk space. It will explicitly refuse to process standard `.cfx` files and does not attempt magical data recovery on corrupted cryptography.
