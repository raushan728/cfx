use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{CfxError, Result};

pub fn execute(
    input: &Path,
    _output: Option<PathBuf>,
    _force: bool,
    _keep: bool,
    _no_progress: bool,
) -> Result<()> {
    let metadata = fs::metadata(input).map_err(CfxError::Io)?;
    if !metadata.is_file() {
        return Err(CfxError::NotARegularFile(input.to_path_buf()));
    }

    let ext = input.extension().unwrap_or_default().to_string_lossy();
    if ext == "tmp" {
        println!("Detected a temporary file (*.tmp): {}", input.display());
        println!("This file was likely left behind by an interrupted or failed CFX operation.");
        println!("Cleaning up the interrupted file...");
        fs::remove_file(input).map_err(CfxError::Io)?;
        println!("Cleanup successful.");
        return Ok(());
    }

    println!("Recovery tool: Checking file {}", input.display());
    println!("CFX cannot magically recover corrupted cryptographic data.");
    println!("If your file is corrupted, you must restore it from a backup.");
    println!(
        "Temporary files from interrupted operations (ending in .tmp) can be passed here to be safely cleaned up."
    );

    Ok(())
}
