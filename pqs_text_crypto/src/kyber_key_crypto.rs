use oqs::*;
use std::fs;

pub fn generate_kyber_keys() -> Result<()>{
    let kem = kem::Kem::new(kem::Algorithm::Kyber512)?;

    // Generate key pair
    let (public_key, secret_key) = kem.keypair()?;

    // Write keys to files one folder above project (panic if fails)
    fs::write("../public_key.bin", &public_key).unwrap();
    fs::write("../secret_key.bin", &secret_key).unwrap();

    //println!("Keys written successfully!");
    

    Ok(())
}
