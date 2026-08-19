# CFX File Format (v1)

CFX files use a strictly defined binary layout. A CFX file consists of a plaintext header, an encrypted metadata chunk, and multiple encrypted payload chunks.

## Binary Layout

### 1. Plaintext Header

| Field | Size | Type | Description |
|------|------|-------------|-------------|
| Magic Bytes | 4 bytes | `[u8; 4]` | Always exactly `b"CFX1"`. Identifies the file. |
| Version | 1 byte | `u8` | The format version. Currently exactly `1`. |
| KDF ID | 1 byte | `u8` | Key Derivation Algorithm. `1` = Argon2id. |
| Cipher ID | 1 byte | `u8` | Encryption Algorithm. `1` = ChaCha20Poly1305 STREAM. |
| Salt | 32 bytes | `[u8; 32]` | Random cryptographically secure salt generated per file. |
| Nonce | 8 bytes | `[u8; 8]` | Random cryptographically secure base nonce for STREAM. |

**Total Header Size**: 47 bytes.

### 2. Encrypted Metadata Chunk

Immediately following the header is the first chunk of the `STREAM` encryption. 
This chunk securely hides file context information to prevent metadata leaks (like exposing original filenames or file sizes without the password).

**Encrypted inside the chunk:**
| Field | Size | Type | Description |
|------|------|-------------|-------------|
| Filename Length | 4 bytes | `u32` (LE) | The byte length of the original filename string. |
| Filename | Variable | UTF-8 Bytes | The original string filename of the encrypted file. |
| Original Size | 8 bytes | `u64` (LE) | The original byte length of the plaintext file. |

*Note*: Because this is a `STREAM` chunk, it is padded with the ChaCha20Poly1305 MAC tag (16 bytes).

### 3. Payload Chunks

Following the metadata chunk, the remaining raw data of the file is broken down into exactly 64 KiB chunks (with the final chunk potentially being smaller). 

Each chunk on disk is formatted as:
| Field | Size | Type | Description |
|------|------|-------------|-------------|
| Chunk Length | 4 bytes | `u32` (LE) | The byte length of the upcoming ciphertext block. |
| Ciphertext Block | Variable | Bytes | The actual ciphertext, exactly ending with a 16-byte MAC tag. |

The file terminates securely. If the file ends prematurely, the required `decrypt_last` STREAM authentication step will forcefully fail, protecting against truncation.
