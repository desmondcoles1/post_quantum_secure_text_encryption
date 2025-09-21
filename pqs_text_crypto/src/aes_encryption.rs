use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};

use aes_gcm::aead::generic_array::typenum::U12;

pub fn aes_encrypt_file(text: &[u8]) -> Result<(Vec<u8>, Nonce<U12>, Key<Aes256Gcm>),aes_gcm::Error>  {
    let key = Aes256Gcm::generate_key(OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);

    let ciphertext = cipher.encrypt(&nonce, &*text)?;

    return Ok((ciphertext, nonce, key));
}

use std::fs;


pub fn  aes_decrypt_file(ciphertext: &[u8], nonce: &Nonce<U12>, key: &Key<Aes256Gcm>) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    return Ok(plaintext);
}

fn files_are_equal(path1: &str, path2: &str) -> std::io::Result<bool> {
    let file1 = fs::read(path1)?;
    let file2 = fs::read(path2)?;
    Ok(file1 == file2)
}
//this file is totally busted