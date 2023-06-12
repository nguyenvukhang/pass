use crate::chacha_io::{ChaReader, ChaWriter};
use crate::error::Error;
use crate::gpg::Gpg;
use crate::sized_io::{SizedRead, SizedWrite};
use crate::skim;
use crate::{Header, Result, DATA_FILE};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    #[serde(skip)]
    gpg_id: Option<String>,

    pairs: HashMap<String, String>,
}

#[allow(unused)]
impl Database {
    pub fn new() -> Self {
        Self { gpg_id: None, pairs: HashMap::new() }
    }

    pub fn gpg_id(&self) -> Option<&String> {
        self.gpg_id.as_ref()
    }

    pub fn has_name(&self, key: &str) -> bool {
        self.pairs.contains_key(key)
    }

    pub fn count(&self) -> usize {
        self.pairs.len()
    }

    pub fn list_all(&self) -> Vec<String> {
        self.pairs.iter().map(|v| v.0.to_string()).collect()
    }

    pub fn select_one(&self) -> Option<String> {
        skim::select_one(self.pairs.iter().map(|v| v.0.to_string()).collect())
    }

    pub fn insert(&mut self, key: &str, value: &str) {
        self.pairs.insert(key.to_string(), value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.pairs.get(key)
    }

    pub fn get_unchecked(&self, key: &str) -> &String {
        self.get(key).unwrap()
    }

    pub fn update(&mut self, key: &str, value: &str) {
        self.pairs.insert(key.to_string(), value.to_string());
    }

    pub fn remove(&mut self, key: &str) {
        self.pairs.remove(key);
    }
}

/// Read/write operations
impl Database {
    pub fn read() -> Result<Self> {
        Self::read_from_file(DATA_FILE)
    }

    fn read_gpg_id<R: Read>(reader: &mut R) -> Result<String> {
        let bytes = reader.sized_read()?;
        let gpg_id = String::from_utf8_lossy(&bytes);
        Ok(gpg_id.to_string())
    }

    fn read_header<R: Read>(reader: &mut R, gpg: &Gpg) -> Result<Header> {
        let enc_header_data = reader.sized_read()?;
        let header_data = gpg.decrypt(&enc_header_data)?;
        Header::try_from(&header_data)
    }

    pub fn read_from_file(data_file: &str) -> Result<Self> {
        let mut reader = File::open(data_file).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Error::DataFileNotFound,
            _ => Error::IoError(e),
        })?;

        let gpg_id = Self::read_gpg_id(&mut reader)?;

        let gpg = Gpg::new(&gpg_id);

        // println!("using gpg id: [{gpg_id}]");

        let header = Self::read_header(&mut reader, &gpg)?;

        // wrap the reader in the header's decryptor
        let reader = ChaReader::new(reader, header.cipher());

        let mut db = serde_json::from_reader::<_, Database>(reader)?;
        db.gpg_id = Some(gpg_id);
        Ok(db)
    }

    pub fn write(&self) -> Result<()> {
        let gpg_id = match self.gpg_id() {
            None => return Err(Error::GpgIdNotFound),
            Some(v) => v,
        };
        let mut writer = File::create(DATA_FILE)?;

        writer.sized_write(gpg_id.as_bytes())?;

        let gpg = Gpg::new(gpg_id);

        let header = Header::generate();

        let enc_header_data = gpg.encrypt(header.as_bytes())?;

        writer.sized_write(&enc_header_data)?;

        // final write
        let writer = ChaWriter::new(writer, header.cipher());
        serde_json::to_writer::<_, Database>(writer, &self).unwrap();

        Ok(())
    }
}
