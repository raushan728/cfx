use std::fs::File;
use std::io::Read;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

use crate::crypto::{create_decryptor, derive_key, get_password};
use crate::error::{CfxError, Result};
use crate::format::CfxHeader;

pub fn execute(input: &Path, no_progress: bool) -> Result<()> {
    let mut f_in = File::open(input).map_err(CfxError::Io)?;
    let header = CfxHeader::read_from(&mut f_in)?;

    let password = get_password("Enter decryption password to verify: ")?;
    let key = derive_key(&password, &header.salt)?;
    let mut decryptor = create_decryptor(&key, &header.nonce);

    // Read and decrypt metadata chunk
    let mut len_buf = [0u8; 4];
    f_in.read_exact(&mut len_buf)
        .map_err(|_| CfxError::InvalidFormat("Missing metadata chunk".into()))?;
    let meta_len = u32::from_le_bytes(len_buf) as usize;
    if meta_len > 10 * 1024 * 1024 {
        return Err(CfxError::InvalidFormat("Metadata chunk too large".into()));
    }

    let mut meta_ciphertext = vec![0u8; meta_len];
    f_in.read_exact(&mut meta_ciphertext)
        .map_err(|_| CfxError::InvalidFormat("Incomplete metadata chunk".into()))?;

    let meta_plaintext = decryptor
        .decrypt_next(meta_ciphertext.as_slice())
        .map_err(|_| CfxError::IncorrectPassword)?;

    // Parse metadata
    if meta_plaintext.len() < 12 {
        return Err(CfxError::InvalidFormat("Corrupted metadata chunk".into()));
    }
    let filename_len = u32::from_le_bytes(meta_plaintext[0..4].try_into().unwrap()) as usize;
    if meta_plaintext.len() < 4 + filename_len + 8 {
        return Err(CfxError::InvalidFormat("Corrupted metadata chunk".into()));
    }
    let original_size_bytes: [u8; 8] = meta_plaintext[4 + filename_len..4 + filename_len + 8]
        .try_into()
        .unwrap();
    let original_size = u64::from_le_bytes(original_size_bytes);

    let pb = if !no_progress {
        let pb = ProgressBar::new(original_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        Some(pb)
    } else {
        None
    };

    let mut current_len_buf = [0u8; 4];
    let mut has_chunk = match f_in.read_exact(&mut current_len_buf) {
        Ok(_) => true,
        Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => false,
        Err(e) => return Err(CfxError::Io(e)),
    };

    while has_chunk {
        let len = u32::from_le_bytes(current_len_buf) as usize;
        let mut ciphertext = vec![0u8; len];
        f_in.read_exact(&mut ciphertext)
            .map_err(|_| CfxError::InvalidFormat("Incomplete ciphertext chunk".into()))?;

        let mut next_len_buf = [0u8; 4];
        has_chunk = match f_in.read_exact(&mut next_len_buf) {
            Ok(_) => true,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => false,
            Err(e) => return Err(CfxError::Io(e)),
        };

        if has_chunk {
            let plaintext = decryptor.decrypt_next(ciphertext.as_slice()).map_err(|_| {
                CfxError::Crypto("Verification failed (corrupted data or wrong password)".into())
            })?;
            if let Some(ref pb) = pb {
                pb.inc(plaintext.len() as u64);
            }
            current_len_buf = next_len_buf;
        } else {
            let plaintext = decryptor.decrypt_last(ciphertext.as_slice()).map_err(|_| {
                CfxError::Crypto(
                    "Verification failed at the final chunk (corrupted data or wrong password)"
                        .into(),
                )
            })?;
            if let Some(ref pb) = pb {
                pb.inc(plaintext.len() as u64);
                pb.finish_with_message("Done");
            }
            break;
        }
    }

    println!("Verification successful. The file is intact and authentic.");
    Ok(())
}
