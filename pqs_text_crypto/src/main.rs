use std::fs;
use std::io::{self, Write};

mod aes_encryption; // declares the module

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
    // println!("{}", String::from_utf8_lossy(&text).lines().next().unwrap());


    // AES encrypt the file
    let (ciphertext, nonce, key) = aes_encryption::aes_encrypt_file(&text).expect("AESEncryption failled :(((");

    println!("Ciphertext: {:?}", ciphertext);
    println!("Nonce: {:?}", nonce);
    println!("Key: {:?}", key);
    // double check that the decrypting the encryption returns the original text

    //let decrypted_text = aes_encryption::aes_decrypt_file(&ciphertext, &nonce, &key).expect("AESDecryption failled :(((");

    //if text == decrypted_text {
    //    println!("decryption worked");
    //} else {
     //   println!("decryption didn't work :((");
    //}
}
