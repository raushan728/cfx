# Recovery

CFX implements a specific `recover` command to handle disk cleanup for interrupted operations.

## How It Works

During normal `encrypt` operations, CFX writes output to a temporary `.tmp` file (e.g., `secret.cfx.tmp`). To protect against accidental deletions, CFX writes a special cryptographic marker (`CFX~`) into the binary header of this temporary file. 

When the encryption successfully completes, CFX overwrites this marker with the final `CFX1` magic bytes and renames it. If the process is interrupted via `SIGINT` (Ctrl+C), a sudden power loss, or a system crash, this `.tmp` file is orphaned on the disk containing the `CFX~` marker.

The `cfx recover` command exists exclusively to safely clean up these orphaned files.

## Safe Deletion Guarantees

Unlike naive cleanup scripts, `cfx recover`:
1. **Never trusts the filename**: It will never delete an arbitrary `.tmp` file just because of its extension.
2. **Cryptographic Verification**: It strictly reads the first 4 bytes of the file. If they are not exactly `CFX~`, it will explicitly refuse to delete it, protecting your user data and completely valid `.cfx` files.
3. **Interactive Confirmation**: It will ask for your explicit `[y/N]` confirmation before deleting anything (unless you pass the `--force` flag).

## Usage Modes

### 1. Exact File Recovery
Target a specific file for cleanup:
```bash
cfx recover incomplete_file.cfx.tmp
```

### 2. Directory Scanning
Search an entire directory for orphaned CFX temporary files. It will inspect the binary contents of the files and gather a list of verified candidates.
```bash
cfx recover --scan /path/to/directory
```

You can optionally append `-f` or `--force` to skip the confirmation prompt.

## What `cfx recover` Does NOT Do

CFX is a secure cryptographic tool. Therefore:
- `cfx recover` **cannot** magically decrypt corrupted payloads.
- `cfx recover` **cannot** fix files missing their final STREAM blocks (truncation).
- `cfx recover` **will refuse** to target standard `.cfx` files. It only works on `.tmp` files.

If a fully generated `.cfx` file is somehow corrupted on disk by bit-rot or tampering, it is mathematically unrecoverable. 
