use std::fs::File;
use std::path::Path;

use crate::error::{CfxError, Result};
use crate::format::CfxHeader;

pub fn execute(input: &Path) -> Result<()> {
    let mut f_in = File::open(input).map_err(CfxError::Io)?;
    let header = CfxHeader::read_from(&mut f_in)?;

    println!("CFX File Information for: {}", input.display());
    println!("--------------------------------------------------");
    println!("Format Version      : {}", header.version);

    let kdf = if header.kdf_id == 1 {
        "Argon2id"
    } else {
        "Unknown"
    };
    println!("Key Derivation      : {}", kdf);

    let cipher = if header.cipher_id == 1 {
        "ChaCha20Poly1305 STREAM"
    } else {
        "Unknown"
    };
    println!("Encryption Algorithm: {}", cipher);
    println!("--------------------------------------------------");
    println!(
        "Note: Original filename and size are encrypted in the metadata chunk for privacy and cannot be viewed without the password. Use `cfx verify` to authenticate and verify the file."
    );

    Ok(())
}
