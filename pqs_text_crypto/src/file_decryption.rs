use std::fs;
use std::io::{self, Write};

use oqs::*;


use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};



pub fn message_decryption(private_key: &[u8], encrypted_symmetric_key: &[u8], nonce: &[u8], encrypted_text: &[u8]) -> Result<()> {
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    let private_key_corrected = kem.secret_key_from_bytes(&private_key).expect("Failed to create private key from bytes");

    let encrypted_symmetric_key_corrected = kem.ciphertext_from_bytes(&encrypted_symmetric_key).expect("Failed to create e symmetric key from bytes");

    let secret_aes_key = kem.decapsulate(&private_key_corrected, &encrypted_symmetric_key_corrected)?;

    let secret_aes_key_in_bytes: &[u8] = secret_aes_key.as_ref();

    let aes_cipher = Aes256Gcm::new_from_slice(&secret_aes_key_in_bytes).expect("Failed to create cipher from key");

    let nonce_in_bytes: &[u8] = nonce.as_ref();
    let nonce = aes_gcm::Nonce::from_slice(&nonce_in_bytes); // &Nonce

    let decrypted_bytes: Vec<u8> = aes_cipher.decrypt(nonce, encrypted_text.as_ref()).unwrap();

    println!("Decrypted bytes: {:?}", decrypted_bytes);

    //let text_decrypted_once = String::from_utf8(decrypted_bytes?);
    //let text_decrypted: Vec<u8> = text_decrypted_once.unwrap().into();
    //println!("Decrypted text: {}", text_decrypted);

    fs::write("../decrypted_text.txt", &decrypted_bytes).expect("Failed to write decrypted text to file");

    Ok(())


}


pub fn file_decryption_prompt() -> Result<()>{
    let mut input = String::new(); 
    println!("Would you like to decrypt a file you recieved? (y/n):");
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
            print!("Paste the path to your private key");
            io::stdout().flush().unwrap();

            // Read key path
            let mut key_path = String::new();
            io::stdin().read_line(&mut key_path).unwrap();
            let key_path = key_path.trim();

            //read key
            let private_key = fs::read(key_path).unwrap();

            // Prompt user
            print!("Paste the path to your encrypted symmetric key: ");
            io::stdout().flush().unwrap();


            // Read the secret key path
            let mut encrypted_symmetric_key_path = String::new();
            io::stdin().read_line(&mut encrypted_symmetric_key_path).unwrap();
            let encrypted_symmetric_key_path = encrypted_symmetric_key_path.trim();   

            // read in encrypted symmetric key  
            let encrypted_symmetric_key = fs::read(encrypted_symmetric_key_path).unwrap();

            // Prompt user
            print!("Paste the path to the nonce: ");
            io::stdout().flush().unwrap();

            // read nonce path
            let mut nonce_path = String::new();
            io::stdin().read_line(&mut nonce_path).unwrap();
            let nonce_path = nonce_path.trim();      

            // read in nonce    
            let nonce = fs::read(nonce_path).unwrap();  

            //decrypt the message
            message_decryption(&private_key, &encrypted_symmetric_key, &nonce, &text).expect("message encryption failed :(((");
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