use std::fs;
use std::io::{self, Write};

use oqs::*;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm
};




pub fn message_encryption(recipient_public_key: &[u8], text: &[u8]) -> Result<()> {
    let kem = kem::Kem::new(kem::Algorithm::Kyber512).expect("Failed to create KEM instance");

    let recipient_public_key_corrected = kem.public_key_from_bytes(&recipient_public_key).expect("Failed to create public key from bytes");


    let (encrypted_secret_symmetric_key, secret_symmetric_key) = kem.encapsulate(&recipient_public_key_corrected)?;

    let secret_symmetric_key_in_bytes: &[u8] = secret_symmetric_key.as_ref();
    let cipher = Aes256Gcm::new_from_slice(&secret_symmetric_key_in_bytes).expect("Failed to create cipher from key");


    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let encrypted_text = cipher.encrypt(&nonce, &*text).expect("Encryption failed");  
  
    fs::write("../enecrypted_text.bin", &encrypted_text).expect("Failed to write encrypted text to file oh nooOOOoOOoo :(((999((" );
    fs::write("../nonce.bin", &nonce).expect("Failed to write nonce to file");
    fs::write("../encrypted_secret_symmetric_key.bin", &encrypted_secret_symmetric_key).expect("Failed to encrypted symmetric key text to file");

    Ok(())

}




pub fn file_encryption_prompt() -> Result<()>{
    let mut input = String::new(); 
    println!("Would you like to encrypt a file to be sent? (y/n):");
    io::stdout().flush().unwrap(); // make sure prompt appears
    io::stdin().read_line(&mut input).expect("Error, cannot read input");
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            // Prompt user
            print!("Paste the file path: ");
            io::stdout().flush().unwrap();

            // Read message path
            let mut message_path = String::new();
            io::stdin().read_line(&mut message_path).unwrap();
            let message_path = message_path.trim();

            // Read files
            let text = fs::read(message_path).unwrap();

            // Prompt user
            print!("Paste the path to the public key of your recipient: ");
            io::stdout().flush().unwrap();

            // Read key path
            let mut key_path = String::new();
            io::stdin().read_line(&mut key_path).unwrap();
            let key_path = key_path.trim();

            //read key
            let recipient_public_key = fs::read(key_path).unwrap();

            //encrypt the message
            message_encryption(&recipient_public_key, &text).expect("message encryption failed :(((");
            Ok(())
        }
        "n" | "no" => {
            // do nothing, just continue
            Ok(())
        }
        _ => {
            println!("Invalid input, please enter y/n");
            Ok(())
        }
    }
}