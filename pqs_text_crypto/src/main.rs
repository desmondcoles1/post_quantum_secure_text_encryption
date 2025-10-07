mod kyber_keypair_generation; //keypair generation module
mod file_encryption; //file encryption module
mod file_decryption; //file decryption module

mod error_handling;

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Operation failed: {}", e); // just prints the error type/message to the user
    }
}

fn run_app() -> std::result::Result<(), error_handling::CryptoError>{
    
    kyber_keypair_generation::kyber_keys_prompt()?;

    file_encryption::file_encryption_prompt()?;

    file_decryption::file_decryption_prompt()?;
    Ok(())
}
