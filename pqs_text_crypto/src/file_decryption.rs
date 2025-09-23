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
    println!("Would you like to deencrypt a file that you recieved? (y/n):");
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
    print!("Paste the encrypted file path: ");
    io::stdout().flush().unwrap();

    // Read encrypted path
    let mut encrypted_message_path = String::new();
    io::stdin().read_line(&mut encrypted_message_path).unwrap();
    let encrypted_message_path = encrypted_message_path.trim();

    //read in encrypted text
    let encrypted_text = fs::read(encrypted_message_path).unwrap();

    /////////////////////////////////

    // Prompt user
    print!("Paste your private key path: ");
    io::stdout().flush().unwrap();

    // read private key path
    let mut private_key_path = String::new();
    io::stdin().read_line(&mut private_key_path).unwrap();
    let private_key_path = private_key_path.trim();

    // read in private key
    let private_key = fs::read(private_key_path).unwrap();

    //////////////////////////////////
    
    // Prompt user
    print!("Paste the path to your encrypted symmetric key: ");
    io::stdout().flush().unwrap();


    // Read the secret key path
    let mut encrypted_symmetric_key_path = String::new();
    io::stdin().read_line(&mut encrypted_symmetric_key_path).unwrap();
    let private_key_path = private_key_path.trim();   

    // read in encrypted symmetric key  
    let encrypted_symmetric_key = fs::read(encrypted_symmetric_key_path).unwrap();

    /////////////////////////////////  

    // Prompt user
    print!("Paste the path to the nonce: ");
    io::stdout().flush().unwrap();

    // read nonce path
    let mut nonce_path = String::new();
    io::stdin().read_line(&mut nonce_path).unwrap();
    let nonce_path = nonce_path.trim();      

    // read in nonce    
    let nonce = fs::read(nonce_path).unwrap();  

    //////////// decrypt symmetric key
     
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    let secret_aes_key = kemalg.decapsulate(&private_key, &encrypted_symmetric_key)?;

    let aes_cipher = Aes256Gcm::new(secret_aes_key);
    let plaintext = cipher.decrypt(nonce, encrypted_text)?;


    let text_decrypted = String::from_utf8(decrypted_bytes)?;
    println!("Decrypted text: {}", plaintext);

    Ok(())

}