use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce
};

pub fn aes_encrypt_file(text: &[u8]) -> (Vec<u8>, Nonce<Aes256Gcm>, Key<Aes256Gcm>) {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let ciphertext = cipher.encrypt(&nonce, text).unwrap();

    (ciphertext, nonce, key)
}
//this file is totally busted