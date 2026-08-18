use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cfx")]
#[command(version, about = "Secure file encryption and decryption utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt a file (Not yet implemented)
    Encrypt,
    /// Decrypt a file (Not yet implemented)
    Decrypt,
    /// Display information about an encrypted file (Not yet implemented)
    Info,
    /// Verify the integrity of an encrypted file (Not yet implemented)
    Verify,
    /// Recover a corrupted encrypted file (Not yet implemented)
    Recover,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt => {
            eprintln!("Error: The 'encrypt' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Decrypt => {
            eprintln!("Error: The 'decrypt' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Info => {
            eprintln!("Error: The 'info' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Verify => {
            eprintln!("Error: The 'verify' feature is not implemented yet.");
            std::process::exit(1);
        }
        Commands::Recover => {
            eprintln!("Error: The 'recover' feature is not implemented yet.");
            std::process::exit(1);
        }
    }
}
