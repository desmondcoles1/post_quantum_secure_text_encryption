use std::fs;

use oqs::*;

use crate::error_handling::CryptoError;

pub fn generate_kyber_keys() -> std::result::Result<(), CryptoError>{
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    // Generate key pair
    let (public_key, secret_key) = kem.keypair()?;

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
