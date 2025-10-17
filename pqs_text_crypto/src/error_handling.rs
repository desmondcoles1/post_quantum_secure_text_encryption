use thiserror::Error;
use aes_gcm::Error as AesError;
use aes_gcm::aes::cipher::InvalidLength;
use std::io;


#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Error writing file(s): {0}")]
    FileWriteError(String),

     #[error("Error reading file(s): {0}")]
    FileReadError(String),
    
    #[error("Invalid Kyber public key: {0}")]
    InvalidPublicKey(String),

    #[error("liboqs error")]
    LiboqsError(#[from] oqs::Error),

    #[error("liboqs error: {0}")]
    LiboqsErrorWMessage(String),

    #[error("AES invalid key length")]
    InvalidAesKeyLength(String),

    #[error("AES encryption/decryption failed")]
    AesError(String),

    #[error("Error reading or writing a file")]
    IoError(#[from] io::Error),
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
