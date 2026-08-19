mod crypto;
mod error;
mod format;
mod ops;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cfx")]
#[command(version, about = "Secure file encryption and decryption utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file
    Encrypt {
        /// The file to encrypt
        input: PathBuf,

        /// The output file path
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        /// Overwrite the output file if it exists
        #[arg(short, long)]
        force: bool,

        /// Retain the original input file after encryption
        #[arg(long)]
        keep: bool,

        /// Disable progress reporting
        #[arg(long)]
        no_progress: bool,
    },
    /// Decrypt a file
    Decrypt {
        /// The file to decrypt
        input: PathBuf,

        /// The output file path
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        /// Overwrite the output file if it exists
        #[arg(short, long)]
        force: bool,

        /// Retain the original input file after decryption
        #[arg(long)]
        keep: bool,

        /// Disable progress reporting
        #[arg(long)]
        no_progress: bool,
    },
    /// Display information about an encrypted file
    Info {
        /// The .cfx file
        input: PathBuf,
    },
    /// Verify the integrity of an encrypted file
    Verify {
        /// The .cfx file
        input: PathBuf,

        /// Disable progress reporting
        #[arg(long)]
        no_progress: bool,
    },
    /// Recover a corrupted encrypted file
    Recover {
        /// The file to recover
        input: PathBuf,

        /// The output file path
        #[arg(short, long, value_name = "PATH")]
        output: Option<PathBuf>,

        /// Overwrite the output file if it exists
        #[arg(short, long)]
        force: bool,

        /// Retain the original input file after recovery
        #[arg(long)]
        keep: bool,

        /// Disable progress reporting
        #[arg(long)]
        no_progress: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Encrypt {
            input,
            output,
            force,
            keep,
            no_progress,
        } => ops::encrypt::execute(&input, output, force, keep, no_progress),
        Commands::Decrypt {
            input,
            output,
            force,
            keep,
            no_progress,
        } => ops::decrypt::execute(&input, output, force, keep, no_progress),
        Commands::Info { input } => ops::info::execute(&input),
        Commands::Verify { input, no_progress } => ops::verify::execute(&input, no_progress),
        Commands::Recover {
            input,
            output,
            force,
            keep,
            no_progress,
        } => ops::recover::execute(&input, output, force, keep, no_progress),
    };

    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
