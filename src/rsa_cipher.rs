use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::Result;

use std::fs;
use std::path::Path;

pub struct RsaCipher {
    private_key: RsaPrivateKey,
}

const PRIVATE_KEY_FILE_PATH: &str = "private_key.pem";

impl RsaCipher {
    pub fn new() -> Result<Self> {
        Self::from_file(PRIVATE_KEY_FILE_PATH)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let key = fs::read_to_string(path)?;
        Ok(Self { private_key: RsaPrivateKey::from_pkcs1_pem(&key)? })
    }

    fn pub_key(&self) -> rsa::RsaPublicKey {
        self.private_key.to_public_key()
    }

    pub fn encrypt<D: AsRef<[u8]>>(&self, data: D) -> Result<Vec<u8>> {
        Ok(self.pub_key().encrypt(
            &mut rand::thread_rng(),
            Pkcs1v15Encrypt,
            data.as_ref(),
        )?)
    }

    pub fn decrypt<D: AsRef<[u8]>>(&self, data: D) -> Result<Vec<u8>> {
        Ok(self.private_key.decrypt(Pkcs1v15Encrypt, data.as_ref())?)
    }
}
