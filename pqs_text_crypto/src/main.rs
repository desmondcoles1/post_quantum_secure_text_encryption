use std::fs;
use std::io::{self, Write};

//kyber stuff

mod kyber_key_crypto; //kyber module

use oqs::*;

//for aes stuff

mod aes_encryption; //aes module

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};

use aes_gcm::aead::generic_array::typenum::U12;
use aes_gcm::aead::generic_array::typenum::U32;

use generic_array::GenericArray;



fn main() -> Result<()>{
    //Prompt user
    let mut input = String::new();
    println!("Would you like to generate a public key and private key pair? (y/n):");
    io::stdout().flush().unwrap(); // make sure prompt appears
    io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            println!("Your keys will be generated and written as files in the directory that contains the parent directory of this project.");
            kyber_key_crypto::generate_kyber_keys().expect("Key generation failed :(((");
        }
        "n" | "no" => {
            // do nothing, just continue
        }
        _ => {
            println!("Invalid input, please enter y/n");
        }
    }
   

    //Prompt user
    let mut input = String::new(); 
    println!("Would you like to encrypt a file to be sent? (y/n):");
    io::stdout().flush().unwrap(); // make sure prompt appears
    io::stdin().read_line(&mut input).expect("Error, cannot read input");
    let answer = input.trim().to_lowercase();
    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => {
            // Just continue
        }
        "n" | "no" => {
            std::process::exit(0); // stop the program
        }
        _ => {
            println!("Invalid input, please enter y/n");
        }
    }


    
    // Prompt user
    print!("Paste the file path: ");
    io::stdout().flush().unwrap();

    // Read message path
    let mut message_path = String::new();
    io::stdin().read_line(&mut message_path).unwrap();
    let message_path = message_path.trim();

     // Prompt user
    print!("Paste the path to the public key of your recipient: ");
    io::stdout().flush().unwrap();

    // Read key path
    let mut key_path = String::new();
    io::stdin().read_line(&mut key_path).unwrap();
    let key_path = key_path.trim();

    // Read files
    let text = fs::read(message_path).unwrap();
    
    let recipient_public_key = fs::read(key_path).unwrap(); //read bin file 
    //let recipient_public_key = kem::PublicKeyRef::from_slice(&recipient_public_key);

    let kem = kem::Kem::new(kem::Algorithm::Kyber512).expect("Failed to create KEM instance");

    let recipient_public_key_corrected = kem.public_key_from_bytes(&recipient_public_key).expect("Failed to create public key from bytes");


    let (encrypted_secret_symmetric_key, secret_symmetric_key) = kem.encapsulate(&recipient_public_key_corrected)?;

    let secret_symmetric_key_in_bytes: &[u8] = secret_symmetric_key.as_ref();
    let cipher = Aes256Gcm::new_from_slice(&secret_symmetric_key_in_bytes).expect("Failed to create cipher from key");


    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let encrypted_text = cipher.encrypt(&nonce, &*text).expect("Encryption failed");  

    fs::write("../enecrypted_text.bin", &encrypted_text).expect("Failed to write encrypted text to file oh nooOOOoOOoo :(((999((" );
    fs::write("../nonce.bin", &nonce).unwrap();
    fs::write("../encrypted_secret_symmetric_key.bin", &encrypted_secret_symmetric_key).unwrap();

    Ok(())

    // Print first line
    // println!("{}", String::from_utf8_lossy(&text).lines().next().unwrap());


    // NOT RIGHT///// AES encrypt the file
    //let (ciphertext, nonce, key) = aes_encryption::aes_encrypt_file(&text).expect("AESEncryption failled :(((");




    

    //println!("Ciphertext: {:?}", ciphertext);
    //println!("Nonce: {:?}", nonce);
    //println!("Key: {:?}", key);
    // double check that the decrypting the encryption returns the original text

    //let decrypted_text = aes_encryption::aes_decrypt_file(&ciphertext, &nonce, &key).expect("AESDecryption failled :(((");

    //if text == decrypted_text {
    //    println!("decryption worked");
    //} else {
     //   println!("decryption didn't work :((");
    //}
   

}
