use std::fs;

use std::path::Path;

use std::io::{self, Write};

use oqs::*;

use crate::error_handling::CryptoError;

pub fn generate_kyber_keys() -> std::result::Result<(), CryptoError>{
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    // Generate key pair
    let (public_key, secret_key) = kem.keypair()?;

    //paths
    let public_output_path = "../public_key.bin";
    let public_file_path = Path::new(public_output_path);
    let private_output_path = "../private_key.bin";
    let private_file_path = Path::new(private_output_path);

    // Check if file exists
    if public_file_path.exists() {
        print!("Warning: '{}' already exists. Overwrite? (y/N): ", public_output_path);
        io::stdout().flush().map_err(|_| CryptoError::FileWriteError("Failed to flush stdout".into()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| CryptoError::FileReadError("Failed to read input".into()))?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Aborting write to '{}'", public_output_path);
            return Ok(()); // Do not overwrite
        }
    } 

    if private_file_path.exists() {
        print!("Warning: '{}' already exists. Overwrite? (y/N): ", private_output_path);
        io::stdout().flush().map_err(|_| CryptoError::FileWriteError("Failed to flush stdout".into()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| CryptoError::FileReadError("Failed to read input".into()))?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Aborting write to '{}'", private_output_path);
            return Ok(()); // Do not overwrite
        }
    }     

    // Write keys to files one folder above project (panic if fails)
    fs::write("../public_key.bin", &public_key)
        .map_err(|_| CryptoError::FileWriteError("Failed to write public key".into()))?;
    fs::write("../private_key.bin", &secret_key)
        .map_err(|_| CryptoError::FileWriteError("Failed to write private key".into()))?;

    Ok(())
}


pub fn kyber_keys_prompt() -> std::result::Result<(), CryptoError>{
        println!("Your keys will be generated and written as files in the directory that contains the parent directory of this project.");
        generate_kyber_keys()?;
        Ok(())
}
