use std::fs;
use std::io::{self, Write};

use oqs::*;

use crate::error_handling::CryptoError;

pub fn generate_kyber_keys() -> std::result::Result<(), CryptoError>{
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    // Generate key pair
    let (public_key, secret_key) = kem.keypair()?;

    // Write keys to files one folder above project (panic if fails)
    fs::write("../public_key.bin", &public_key)?;
    fs::write("../private_key.bin", &secret_key)?;

    Ok(())
}


pub fn kyber_keys_prompt() -> std::result::Result<(), CryptoError>{
    //Prompt user
    let mut input = String::new();
    println!("Would you like to generate a public key and private key pair? (y/n):");
    io::stdout().flush()?; // make sure prompt appears
    io::stdin().read_line(&mut input)?;
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            println!("Your keys will be generated and written as files in the directory that contains the parent directory of this project.");
            generate_kyber_keys()?;
            Ok(())
        }
        "n" | "no" => {
            // do nothing, just continue
            Ok(())
        }
        _ => {
            println!("Invalid input, please restart the program and enter y/n");
            Ok(())
        }
    }
}
