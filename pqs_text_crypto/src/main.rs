mod kyber_keypair_generation; //keypair generation module
mod file_encryption; //file encryption module
mod file_decryption; //file decryption module

mod error_handling;

use oqs::*;


fn main() -> Result<()>{
    
    kyber_keypair_generation::kyber_keys_prompt().expect("prompting failed :(((");

    file_encryption::file_encryption_prompt().expect("prompting failed :(((");

    file_decryption::file_decryption_prompt().expect("prompting failed :(((");
    Ok(())
}
