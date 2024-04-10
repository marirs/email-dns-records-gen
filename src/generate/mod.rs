use crate::{error::Error, Result};
use rand::thread_rng;
use rsa::{
    pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey},
    RsaPrivateKey, RsaPublicKey,
};

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
}
