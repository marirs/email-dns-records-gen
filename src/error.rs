use ed25519_dalek;
use rsa;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("RSA crypto error: {0}")]
    Rsa(#[from] rsa::errors::Error),
    #[error("RSA PKCS1 DER error: {0}")]
    RsaPkcs1Der(#[from] rsa::pkcs1::Error),
    #[error("SPKI error: {0}")]
    Spki(#[from] rsa::pkcs8::spki::Error),
    #[error("Ed25519 crypto error: {0}")]
    Ed25519(#[from] ed25519_dalek::pkcs8::Error),
    #[error("Error: {0}")]
    Generic(String),
}
