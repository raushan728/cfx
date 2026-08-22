use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::error::{CfxError, Result};
use crate::format::CfxHeader;

pub fn execute(
    input: Option<PathBuf>,
    scan: Option<PathBuf>,
    force: bool,
    _no_progress: bool,
) -> Result<()> {
    if let Some(target) = input {
        recover_exact_file(&target, force)
    } else if let Some(dir) = scan {
        recover_directory(&dir, force)
    } else {
        unreachable!("Clap should enforce either input or scan is provided");
    }
}

fn recover_exact_file(path: &Path, force: bool) -> Result<()> {
    let metadata = fs::metadata(path).map_err(CfxError::Io)?;
    if !metadata.is_file() {
        return Err(CfxError::NotARegularFile(path.to_path_buf()));
    }

    if !is_cfx_temporary_file(path) {
        return Err(CfxError::InvalidFormat(
            "The file could not be verified as a CFX temporary file.".into(),
        ));
    }

    let size = metadata.len();
    println!(
        "Verified CFX temporary file: {} ({} bytes)",
        path.display(),
        size
    );

    if confirm_deletion(force) {
        fs::remove_file(path).map_err(CfxError::Io)?;
        println!("Cleanup successful.");
    } else {
        println!("Skipped deletion.");
    }

    Ok(())
}

fn recover_directory(dir: &Path, force: bool) -> Result<()> {
    let metadata = fs::metadata(dir).map_err(CfxError::Io)?;
    if !metadata.is_dir() {
        return Err(CfxError::NotARegularFile(dir.to_path_buf()));
    }

    println!("Scanning directory: {}", dir.display());

    let mut candidates = Vec::new();
    let entries = fs::read_dir(dir).map_err(CfxError::Io)?;

    for entry in entries.filter_map(std::result::Result::ok) {
        let path = entry.path();
        if path.is_file() && is_cfx_temporary_file(&path) {
            candidates.push(path);
        }
    }

    if candidates.is_empty() {
        println!("No CFX temporary files found in the directory.");
        return Ok(());
    }

    println!(
        "\nFound {} verified CFX temporary file(s):",
        candidates.len()
    );
    for path in &candidates {
        let size = fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        println!("  - {} ({} bytes)", path.display(), size);
    }
    println!();

    if confirm_deletion(force) {
        for path in candidates {
            if let Err(e) = fs::remove_file(&path) {
                eprintln!("Failed to delete {}: {}", path.display(), e);
            } else {
                println!("Deleted {}", path.display());
            }
        }
        println!("Cleanup successful.");
    } else {
        println!("Skipped deletion.");
    }

    Ok(())
}

fn is_cfx_temporary_file(path: &Path) -> bool {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    match CfxHeader::read_any(&mut file) {
        Ok(header) => header.is_tmp,
        Err(_) => false,
    }
}

fn confirm_deletion(force: bool) -> bool {
    if force {
        return true;
    }

    print!("Are you sure you want to delete these file(s)? [y/N]: ");
    let _ = io::stdout().flush();

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let response = input.trim().to_lowercase();
        response == "y" || response == "yes"
    } else {
        false
    }
}
