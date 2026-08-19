# Recovery

CFX implements a specific `recover` command to handle disk cleanup for interrupted operations.

## What `cfx recover` Does

During normal `encrypt` or `decrypt` operations, CFX writes output to a temporary `.tmp` file (e.g., `secret.cfx.tmp`). This file is only renamed to the final requested output file if the entire cryptographic operation succeeds from start to finish. 

If the process is interrupted via `SIGINT` (Ctrl+C), a sudden power loss, or a system crash, this `.tmp` file is orphaned on the disk.

The `cfx recover` command exists exclusively to target these `.tmp` files and safely delete them to reclaim storage space.

## What `cfx recover` Does NOT Do

CFX is a secure cryptographic tool. Therefore:
- `cfx recover` **cannot** magically decrypt corrupted payloads.
- `cfx recover` **cannot** fix files missing their final STREAM blocks (truncation).
- `cfx recover` **will refuse** to target standard `.cfx` files. It only works on `.tmp` files.

If a fully generated `.cfx` file is somehow corrupted on disk by bit-rot or tampering, it is mathematically unrecoverable. 
