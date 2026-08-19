use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use indicatif::{ProgressBar, ProgressStyle};

use crate::crypto::{create_encryptor, derive_key, generate_nonce, generate_salt, get_password};
use crate::error::{CfxError, Result};
use crate::format::CfxHeader;
use crate::ops::CHUNK_SIZE;

const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10 GiB

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
    let file_size = metadata.len();
    if file_size > MAX_FILE_SIZE {
        return Err(CfxError::FileTooLarge);
    }

    let default_output = PathBuf::from(format!("{}.cfx", input.to_string_lossy()));
    let out_path = output.unwrap_or(default_output);
    if out_path.exists() && !force {
        return Err(CfxError::AlreadyExists(out_path));
    }

    let tmp_out_path = PathBuf::from(format!("{}.tmp", out_path.to_string_lossy()));

    // 2. Prompt Password
    let password = get_password("Enter encryption password: ")?;
    let confirm = get_password("Confirm password: ")?;
    if password != confirm {
        return Err(CfxError::PasswordPrompt("Passwords do not match".into()));
    }

    // 3. Cryptography setup
    let salt = generate_salt();
    let nonce = generate_nonce();
    let key = derive_key(&password, &salt)?;
    let mut encryptor = create_encryptor(&key, &nonce);

    let header = CfxHeader::new(salt, nonce);

    // 4. Create files
    let mut f_in = File::open(input).map_err(CfxError::Io)?;
    let mut f_out = File::create(&tmp_out_path).map_err(CfxError::Io)?;

    // 5. Write header
    header.write_to(&mut f_out)?;

    // 6. Encrypt metadata chunk (filename length, filename, file_size)
    let filename = input.file_name().unwrap_or_default().to_string_lossy();
    let filename_bytes = filename.as_bytes();
    let mut meta_chunk = Vec::new();
    meta_chunk.extend_from_slice(&(filename_bytes.len() as u32).to_le_bytes());
    meta_chunk.extend_from_slice(filename_bytes);
    meta_chunk.extend_from_slice(&file_size.to_le_bytes());

    let ciphertext = encryptor
        .encrypt_next(meta_chunk.as_slice())
        .map_err(|_| CfxError::Crypto("Failed to encrypt metadata".into()))?;
    f_out.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
    f_out.write_all(&ciphertext)?;

    // 7. Encrypt file data
    let pb = if !no_progress {
        let pb = ProgressBar::new(file_size);
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

    let mut buffer = vec![0u8; CHUNK_SIZE];
    let mut n = f_in.read(&mut buffer)?;

    // Special case for completely empty file
    if n == 0 {
        if let Some(ref pb) = pb {
            pb.finish_with_message("Done");
        }
        let ciphertext = encryptor
            .encrypt_last(&[] as &[u8])
            .map_err(|_| CfxError::Crypto("Encryption failed".into()))?;
        f_out.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
        f_out.write_all(&ciphertext)?;
    } else {
        loop {
            if let Some(ref pb) = pb {
                pb.inc(n as u64);
            }

            let mut next_buffer = vec![0u8; CHUNK_SIZE];
            let next_n = f_in.read(&mut next_buffer)?;

            if next_n == 0 {
                // current buffer is the last block
                let ciphertext = encryptor
                    .encrypt_last(&buffer[..n])
                    .map_err(|_| CfxError::Crypto("Encryption failed".into()))?;
                f_out.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
                f_out.write_all(&ciphertext)?;
                break;
            } else {
                // not the last block
                let ciphertext = encryptor
                    .encrypt_next(&buffer[..n])
                    .map_err(|_| CfxError::Crypto("Encryption failed".into()))?;
                f_out.write_all(&(ciphertext.len() as u32).to_le_bytes())?;
                f_out.write_all(&ciphertext)?;
                buffer = next_buffer;
                n = next_n;
            }
        }
        if let Some(ref pb) = pb {
            pb.finish_with_message("Done");
        }
    }

    // 8. Finalize (rename tmp to target and optionally remove input)
    f_out.sync_all()?;
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

    println!("Encryption successful: {}", out_path.display());
    Ok(())
}
