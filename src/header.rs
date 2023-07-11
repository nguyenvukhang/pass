use crate::{Error, Result};

use chacha20::cipher::KeyIvInit;
use chacha20::ChaCha20;
use rand::Rng;

use std::ops::Range;

pub const HEADER_BYTE_LEN: usize = 32 + 12;
const CHACHA_KEY_LEN: usize = 32;
const CHACHA_NONCE_LEN: usize = 12;
const CHACHA_KEY_RANGE: Range<usize> = 0..32;
const CHACHA_NONCE_RANGE: Range<usize> = 32..44;

#[derive(Debug)]
pub struct Header {
    data: [u8; HEADER_BYTE_LEN],
}

impl Header {
    pub fn new(
        chacha_key: &[u8; CHACHA_KEY_LEN],
        chacha_nonce: &[u8; CHACHA_NONCE_LEN],
    ) -> Self {
        let mut data = [0u8; HEADER_BYTE_LEN];
        data[CHACHA_KEY_RANGE].copy_from_slice(chacha_key);
        data[CHACHA_NONCE_RANGE].copy_from_slice(chacha_nonce);
        Self { data }
    }

    /// Generate a brand new Header
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let chacha_key = rng.gen::<[u8; CHACHA_KEY_LEN]>();
        let chacha_nonce = rng.gen::<[u8; CHACHA_NONCE_LEN]>();
        Self::new(&chacha_key, &chacha_nonce)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn chacha_key(&self) -> [u8; CHACHA_KEY_LEN] {
        self.data[CHACHA_KEY_RANGE].try_into().unwrap()
    }

    pub fn chacha_nonce(&self) -> [u8; CHACHA_NONCE_LEN] {
        self.data[CHACHA_NONCE_RANGE].try_into().unwrap()
    }

    pub fn cipher(&self) -> ChaCha20 {
        ChaCha20::new(&self.chacha_key().into(), &self.chacha_nonce().into())
    }
}

impl From<[u8; HEADER_BYTE_LEN]> for Header {
    fn from(data: [u8; HEADER_BYTE_LEN]) -> Self {
        Header { data }
    }
}

impl TryFrom<&Vec<u8>> for Header {
    type Error = Error;
    fn try_from(data: &Vec<u8>) -> Result<Self> {
        let mut fixed = [0u8; HEADER_BYTE_LEN];
        if data.len() != HEADER_BYTE_LEN {
            // TODO: fix. This curernt shows when the gpg key is not set yet
            return Err(Error::RawBytesReadError);
        }
        fixed.copy_from_slice(&data);
        Ok(Header { data: fixed })
    }
}
