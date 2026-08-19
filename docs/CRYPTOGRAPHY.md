# Cryptography in CFX

CFX does not implement custom cryptographic primitives. It relies entirely on well-tested, vetted, and widely trusted third-party Rust crates (such as `argon2` and `chacha20poly1305`) to provide robust confidentiality and authenticity.

## Key Derivation (Argon2id)

Passwords provided by the user are never used directly as encryption keys. Instead, they are passed through Argon2id, the current industry-standard Key Derivation Function (KDF) recommended by OWASP and the Password Hashing Competition.

**Parameters Used**:
- CFX generates a cryptographically secure **32-byte Salt** using the operating system's CSPRNG (`rand::rng().fill_bytes()`).
- CFX leverages the `Argon2id` default parameters initialized by the `argon2` crate:
  - Memory cost: Defaults to `19456` (19 MiB).
  - Iteration (Time) cost: Defaults to `2`.
  - Parallelism: Defaults to `1`.
- This process deterministically yields a strong **32-byte cryptographic key**.

## Authenticated Encryption (ChaCha20-Poly1305)

CFX utilizes ChaCha20-Poly1305, an extremely fast and secure Authenticated Encryption with Associated Data (AEAD) cipher.

### The STREAM Construction
Because files can be very large (up to 10 GiB), encrypting the entire file at once in memory is impossible. CFX uses the `STREAM` construction of ChaCha20-Poly1305. 

- **Nonce Handling**: An 8-byte secure base nonce is generated via CSPRNG and stored in the plaintext header. The `STREAM` construction internally derives unique nonces for each individual chunk by appending an incrementing 32-bit counter and a final-block flag.
- **Chunk Authentication**: The file is segmented into 64 KiB chunks. Every single chunk is encrypted and uniquely authenticated with its own Poly1305 MAC. 
- **Tamper Detection**: If any byte in any chunk is modified on disk, the Poly1305 MAC will fail to authenticate during decryption, immediately halting the process and throwing a `Crypto` error.
- **Truncation Protection**: The `STREAM` protocol mathematically distinctifies the very last chunk. If an attacker truncates the end of the file, the decryptor will reach EOF before validating a chunk flagged as "final", causing an immediate failure.
- **Wrong Password Behavior**: If an incorrect password is provided, Argon2id derives a wrong 32-byte key. The decryption of the very first chunk (the metadata block) will instantly fail its MAC check, causing CFX to securely abort.
