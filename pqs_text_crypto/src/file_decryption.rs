use std::fs;
use std::io::{self, Write};

use oqs::*;


use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};

use crate::error_handling::CryptoError;


pub fn message_decryption(private_key: &[u8], encrypted_symmetric_key: &[u8], nonce: &[u8], encrypted_text: &[u8]) -> std::result::Result<(), CryptoError> {
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    let private_key_corrected = kem.secret_key_from_bytes(&private_key)
        .ok_or_else(|| CryptoError::LiboqsErrorWMessage("Private key is malformed or wrong length".into()))?;

    let encrypted_symmetric_key_corrected = kem.ciphertext_from_bytes(&encrypted_symmetric_key)
        .ok_or_else(|| CryptoError::LiboqsErrorWMessage("Secret symmetric key is malformed or wrong length".into()))?;

    let secret_aes_key = kem.decapsulate(&private_key_corrected, &encrypted_symmetric_key_corrected)?;

    let secret_aes_key_in_bytes: &[u8] = secret_aes_key.as_ref();

    let aes_cipher = Aes256Gcm::new_from_slice(&secret_aes_key_in_bytes)?;

    let nonce_in_bytes: &[u8] = nonce.as_ref();
    let nonce = aes_gcm::Nonce::from_slice(&nonce_in_bytes); // &Nonce

    let decrypted_bytes: Vec<u8> = aes_cipher.decrypt(nonce, encrypted_text.as_ref())
        .map_err(|_| CryptoError::LiboqsErrorWMessage("encrypted file is malformed".into()))?;

    fs::write("../decrypted_text.txt", &decrypted_bytes)
        .map_err(|_| CryptoError::FileWriteError("Failed to write decrypted file".into()))?;

    Ok(())
}


pub fn file_decryption_prompt() -> std::result::Result<(), CryptoError>{
        // Prompt user
        print!("Paste the path to the encrypted message: ");
        io::stdout().flush()?;

        // Read message path
        let mut message_path = String::new();
        io::stdin().read_line(&mut message_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read path to file".into()))?;
        let message_path = message_path.trim();

        // Read files
        let encrypted_message = fs::read(message_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read message to file".into()))?;

    
        // Split the nonce and ciphertext
        let (nonce, encrypted_text) = encrypted_message.split_at(12);

        // Prompt user
        print!("Paste the path to your private key");
        io::stdout().flush()?;

        // Read key path
        let mut key_path = String::new();
        io::stdin().read_line(&mut key_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read path to private key".into()))?;
        let key_path = key_path.trim();

        //read key
        let private_key = fs::read(key_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read key".into()))?;

        // Prompt user
        print!("Paste the path to your encrypted symmetric key: ");
        io::stdout().flush()?;


        // Read the secret key path
        let mut encrypted_symmetric_key_path = String::new();
        io::stdin().read_line(&mut encrypted_symmetric_key_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read path to symmetric key".into()))?;
        let encrypted_symmetric_key_path = encrypted_symmetric_key_path.trim();   

        // read in encrypted symmetric key  
        let encrypted_symmetric_key = fs::read(encrypted_symmetric_key_path)
            .map_err(|_| CryptoError::FileReadError("Failed to read symmetric key".into()))?;

        //decrypt the message
        message_decryption(&private_key, &encrypted_symmetric_key, &nonce, &encrypted_text)?;
        Ok(())
}