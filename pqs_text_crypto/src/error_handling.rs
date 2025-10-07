use thiserror::Error;
use aes_gcm::Error as AesError;
use aes_gcm::aes::cipher::InvalidLength;
use std::io;


#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("AES invalid key length")]
    InvalidAesKeyLength(String),

    #[error("AES encryption/decryption failed")]
    AesError(String),

    #[error("Error reading or writing a file")]
    IoError(#[from] io::Error),

    #[error("Invalid input provided")]
    InvalidInput,

    #[error("liboqs error")]
    LiboqsError(#[from] oqs::Error),
}


// Implement From manually for AES types
impl From<InvalidLength> for CryptoError {
    fn from(_: InvalidLength) -> Self {
        CryptoError::InvalidAesKeyLength("Invalid key length".to_string())
    }
}

impl From<AesError> for CryptoError {
    fn from(_: AesError) -> Self {
        CryptoError::AesError("Encryption/decryption failed".to_string())
    }
}
