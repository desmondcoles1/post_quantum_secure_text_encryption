mod kyber_keypair_generation; //keypair generation module
mod file_encryption; //file encryption module
mod file_decryption; //file decryption module

mod error_handling;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    KeyGeneration,
    Encrypt,
    Decrypt,
}

fn run_app() ->std::result::Result<(), error_handling::CryptoError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::KeyGeneration => {
            kyber_keypair_generation::kyber_keys_prompt()?;
        }
        Commands::Encrypt => {
            file_encryption::file_encryption_prompt()?;
        }
        Commands::Decrypt => {
            file_decryption::file_decryption_prompt()?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Operation failed: {}", e); // just prints the error type/message to the user
    }
}

