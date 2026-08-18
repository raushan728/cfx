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

    match &cli.command {
        Commands::Encrypt { .. } => {
            eprintln!("Error: The 'encrypt' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Decrypt { .. } => {
            eprintln!("Error: The 'decrypt' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Info { .. } => {
            eprintln!("Error: The 'info' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Verify { .. } => {
            eprintln!("Error: The 'verify' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Recover { .. } => {
            eprintln!("Error: The 'recover' feature is not implemented yet.");
            std::process::exit(1);
        }
    }
}
