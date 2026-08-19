use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};

use crate::crypto::{create_decryptor, derive_key, get_password};
use crate::error::{CfxError, Result};
use crate::format::CfxHeader;

pub fn execute(
    input: &Path,
    output: Option<PathBuf>,
    force: bool,
    keep: bool,
    no_progress: bool,
) -> Result<()> {
    // 1. Validation
    let metadata = fs::metadata(input).map_err(CfxError::Io)?;
    if !metadata.is_file() {
        return Err(CfxError::NotARegularFile(input.to_path_buf()));
    }
    if metadata.len() > 10 * 1024 * 1024 * 1024 {
        return Err(CfxError::FileTooLarge);
    }

    // 2. Open input and read header
    let mut f_in = File::open(input).map_err(CfxError::Io)?;
    let header = CfxHeader::read_from(&mut f_in)?;

    // 3. Prompt Password and derive key
    let password = get_password("Enter decryption password: ")?;
    let key = derive_key(&password, &header.salt)?;
    let mut decryptor = create_decryptor(&key, &header.nonce);

    // 4. Read and decrypt metadata chunk
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
        .map_err(|_| CfxError::IncorrectPassword)?; // The first authentication happens here

    // Parse metadata: filename_len(u32), filename, original_size(u64)
    if meta_plaintext.len() < 12 {
        return Err(CfxError::InvalidFormat("Corrupted metadata chunk".into()));
    }
    let filename_len = u32::from_le_bytes(meta_plaintext[0..4].try_into().unwrap()) as usize;
    if meta_plaintext.len() < 4 + filename_len + 8 {
        return Err(CfxError::InvalidFormat("Corrupted metadata chunk".into()));
    }
    let filename_bytes = &meta_plaintext[4..4 + filename_len];
    let original_filename = String::from_utf8_lossy(filename_bytes).into_owned();
    let original_size_bytes: [u8; 8] = meta_plaintext[4 + filename_len..4 + filename_len + 8]
        .try_into()
        .unwrap();
    let original_size = u64::from_le_bytes(original_size_bytes);

    let default_output = if original_filename.is_empty() {
        input.with_extension("") // strip .cfx
    } else {
        input.with_file_name(original_filename)
    };

    let out_path = output.unwrap_or(default_output);
    if out_path.exists() && !force {
        return Err(CfxError::AlreadyExists(out_path));
    }

    let tmp_out_path = PathBuf::from(format!("{}.tmp", out_path.to_string_lossy()));

    // Wrap the decryption loop to catch errors and cleanup tmp file
    let res = (|| -> Result<()> {
        let mut f_out = File::create(&tmp_out_path).map_err(CfxError::Io)?;

        // 5. Decrypt file data
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

            // Peek next chunk length
            let mut next_len_buf = [0u8; 4];
            has_chunk = match f_in.read_exact(&mut next_len_buf) {
                Ok(_) => true,
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => false,
                Err(e) => return Err(CfxError::Io(e)),
            };

            if has_chunk {
                // Not the last chunk
                let plaintext = decryptor.decrypt_next(ciphertext.as_slice()).map_err(|_| {
                    CfxError::Crypto("Decryption failed (corrupted data or wrong password)".into())
                })?;
                f_out.write_all(&plaintext).map_err(CfxError::Io)?;
                if let Some(ref pb) = pb {
                    pb.inc(plaintext.len() as u64);
                }
                current_len_buf = next_len_buf;
            } else {
                // The last chunk
                let plaintext = decryptor.decrypt_last(ciphertext.as_slice()).map_err(|_| {
                    CfxError::Crypto(
                        "Decryption failed at the final chunk (corrupted data or wrong password)"
                            .into(),
                    )
                })?;
                f_out.write_all(&plaintext).map_err(CfxError::Io)?;
                if let Some(ref pb) = pb {
                    pb.inc(plaintext.len() as u64);
                    pb.finish_with_message("Done");
                }
                break;
            }
        }
        f_out.sync_all().map_err(CfxError::Io)?;
        Ok(())
    })();

    if let Err(e) = res {
        let _ = fs::remove_file(&tmp_out_path);
        return Err(e);
    }

    // 8. Finalize (rename tmp to target and optionally remove input)
    fs::rename(&tmp_out_path, &out_path).map_err(|e| {
        let _ = fs::remove_file(&tmp_out_path);
        CfxError::Io(e)
    })?;

    if !keep {
        #[allow(clippy::collapsible_if)]
        if let Err(e) = fs::remove_file(input) {
            eprintln!(
                "Warning: Failed to remove original file '{}': {}",
                input.display(),
                e
            );
        }
    }

    println!("Decryption successful: {}", out_path.display());
    Ok(())
}
