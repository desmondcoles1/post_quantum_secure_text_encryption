use std::fs;
use std::io::{self, Write};

//mod aes_encryption; // declares the module

//use aes_encryption::aes_encrypt_file; // import function
//use aes_gcm::{Aes256Gcm, Key, Nonce};

fn main() {
    // Prompt user
    print!("Paste the file path: ");
    io::stdout().flush().unwrap();

    // Read input path
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path).unwrap();
    let input_path = input_path.trim();

    // Read file
    let text = fs::read(input_path).unwrap();

    // Print first line
    println!("{}", String::from_utf8_lossy(&text).lines().next().unwrap());


    // AES encrypt the file
    //let (ciphertext, nonce, key) = aes_encrypt_file(&text);

    //println!("Ciphertext: {:?}", ciphertext);
    //println!("Nonce: {:?}", nonce);
    //println!("Key: {:?}", key);
}
