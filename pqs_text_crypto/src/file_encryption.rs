use std::fs;
use std::io::{self, Write};

use std::path::Path;

use oqs::*;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm
};

use crate::error_handling::CryptoError;


pub fn message_encryption(recipient_public_key: &[u8], text: &[u8]) -> std::result::Result<(), CryptoError> {
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    let recipient_public_key_corrected 
        = kem.public_key_from_bytes(&recipient_public_key)
        .ok_or_else(|| CryptoError::InvalidPublicKey("Malformed or wrong-length public key".into()))?;


    let (encrypted_secret_symmetric_key, secret_symmetric_key) 
        = kem.encapsulate(&recipient_public_key_corrected)
            .map_err(|_| CryptoError::LiboqsErrorWMessage("Failed to encapsulate keys".into()))?;

    let secret_symmetric_key_in_bytes: &[u8] = secret_symmetric_key.as_ref();
    let cipher = Aes256Gcm::new_from_slice(&secret_symmetric_key_in_bytes)?;


    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted_text_raw = cipher.encrypt(&nonce, &*text)?;  


    let mut encrypted_message = Vec::with_capacity(nonce.len() + encrypted_text_raw.len());
    encrypted_message.extend_from_slice(&nonce);          
    encrypted_message.extend_from_slice(&encrypted_text_raw);

    let output_path = "../enecrypted_message.bin";
    let file_path = Path::new(output_path);

    // Check if file exists
    if file_path.exists() {
        print!("Warning: '{}' already exists. Overwrite? (y/N): ", output_path);
        io::stdout().flush().map_err(|_| CryptoError::FileWriteError("Failed to flush stdout".into()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| CryptoError::FileReadError("Failed to read input".into()))?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Aborting write to '{}'", output_path);
            return Ok(()); // Do not overwrite
        }
    }
  
    fs::write("../enecrypted_message.bin", &encrypted_message)
        .map_err(|_| CryptoError::FileWriteError("Failed to encrypted message".into()))?;

    let output_path = "../encrypted_secret_symmetric_key.bin";
    let file_path = Path::new(output_path);

    // Check if file exists
    if file_path.exists() {
        print!("Warning: '{}' already exists. Overwrite? (y/N): ", output_path);
        io::stdout().flush().map_err(|_| CryptoError::FileWriteError("Failed to flush stdout".into()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| CryptoError::FileReadError("Failed to read input".into()))?;
        let input = input.trim().to_lowercase();

        if input != "y" && input != "yes" {
            println!("Aborting write to '{}'", output_path);
            return Ok(()); // Do not overwrite
        }
    }

    fs::write("../encrypted_secret_symmetric_key.bin", &encrypted_secret_symmetric_key)
        .map_err(|_| CryptoError::FileWriteError("Failed to write encrypted secret symmetric key".into()))?;

    Ok(())
}




pub fn file_encryption_prompt() -> std::result::Result<(), CryptoError>{
        // Prompt user
        print!("Paste the file path: ");
        io::stdout().flush()?;

        // Read message path
        let mut message_path = String::new();
        io::stdin().read_line(&mut message_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read path to file".into()))?;
        let message_path = message_path.trim();

        // Read files
        let text = fs::read(message_path)
            .map_err(|_| CryptoError::FileReadError("Could not read message file as text".into()))?;

        // Prompt user
        print!("Paste the path to the public key of your recipient: ");
        io::stdout().flush()?;

        // Read key path
        let mut key_path = String::new();
        io::stdin().read_line(&mut key_path)
            .map_err(|_| CryptoError::FileWriteError("Failed to read public key path".into()))?;
        let key_path = key_path.trim();

        //read key
        let recipient_public_key = fs::read(key_path)
            .map_err(|_| CryptoError::FileWriteError("Failed to read public key".into()))?;

        //encrypt the message
        message_encryption(&recipient_public_key, &text)?;
        Ok(())
}