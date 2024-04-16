use ed25519_dalek::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rand::rngs::OsRng;
use crate::{error::Error, Result};
use rand::thread_rng;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};

#[derive(Debug)]
pub struct DkimKeyPair {
    pub private_key_x509: String,
    pub public_key_x509: String,
}

impl DkimKeyPair {
    pub fn generate_rsa(bits: usize) -> Result<Self> {
        let priv_key = RsaPrivateKey::new(&mut thread_rng(), bits).map_err(Error::Rsa)?;
        let pub_key = RsaPublicKey::from(&priv_key);
        let private_key_x509 = priv_key
            .to_pkcs1_pem(Default::default())
            .map_err(Error::RsaPkcs1Der)?
            .as_bytes()
            .to_owned();
        let public_key_x509 = pub_key
            .to_pkcs1_pem(Default::default())
            .map_err(Error::RsaPkcs1Der)?
            .as_bytes()
            .to_owned();

        Ok(DkimKeyPair {
            private_key_x509: String::from_utf8_lossy(&private_key_x509).to_string(),
            public_key_x509: String::from_utf8_lossy(&public_key_x509).to_string(),
        })
    }

    pub fn generate_ed25519() -> Result<Self> {
        let mut csprng = OsRng;
        // Generate the ED25519 key pair
        let signing_key = ed25519_dalek::SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        // Encode the keys in X509 format
        let priv_key_x509 = signing_key
            .to_pkcs8_pem(Default::default())
            .map_err(Error::Ed25519)?
            .as_bytes()
            .to_owned();
        let pub_key_x509 = verifying_key
            .to_public_key_pem(Default::default())
            .map_err(|e| Error::Generic(e.to_string()))?
            .as_bytes()
            .to_owned();
        
        Ok(DkimKeyPair {
            private_key_x509: String::from_utf8_lossy(&priv_key_x509).to_string(),
            public_key_x509: String::from_utf8_lossy(&pub_key_x509).to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_rsa() {
        let key_pair = DkimKeyPair::generate_rsa(2048).unwrap();
        assert!(key_pair.private_key_x509.starts_with("-----BEGIN RSA PRIVATE KEY-----"));
        assert!(key_pair.public_key_x509.starts_with("-----BEGIN RSA PUBLIC KEY-----"));
        println!("{}", key_pair.private_key_x509);
        println!("{}", key_pair.public_key_x509);
    }

    #[test]
    fn test_generate_ed25519() {
        let key_pair = DkimKeyPair::generate_ed25519().unwrap();
        assert!(key_pair.private_key_x509.starts_with("-----BEGIN PRIVATE KEY-----"));
        assert!(key_pair.public_key_x509.starts_with("-----BEGIN PUBLIC KEY-----"));
        println!("{}", key_pair.private_key_x509);
        println!("{}", key_pair.public_key_x509);
    }
}
