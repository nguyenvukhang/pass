use crate::chacha_io::{ChaReader, ChaWriter};
use crate::error::Error;
use crate::gpg::Gpg;
use crate::sized_io::{SizedRead, SizedWrite};
use crate::skim;
use crate::{Header, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;
use std::{env, io};

#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    #[serde(skip)]
    gpg_id: Option<String>,

    pairs: HashMap<String, String>,
}

#[allow(unused)]
impl Database {
    pub fn new(gpg_id: Option<String>) -> Self {
        Self { gpg_id, pairs: HashMap::new() }
    }

    pub fn gpg_id(&self) -> Option<&String> {
        self.gpg_id.as_ref()
    }

    pub fn set_gpg_id(&mut self, gpg_id: &str) {
        self.gpg_id = Some(gpg_id.to_string())
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

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.pairs.remove(key)
    }
}

/// Read/write operations
impl Database {
    fn default_dir() -> PathBuf {
        dirs::config_dir().unwrap().join("pass")
    }

    pub fn path() -> PathBuf {
        const FILENAME: &str = "pass.store";
        if let Ok(pass_dir) = env::var("PASSWORD_STORE_DIR") {
            PathBuf::from(pass_dir).join(FILENAME)
        } else {
            Database::default_dir().join(FILENAME)
        }
    }

    pub fn read() -> Result<Self> {
        let db_path = Database::path();
        eprintln!("--> {db_path:?}");
        Self::read_from_file(&db_path)
    }

    fn read_gpg_id<R: Read>(reader: &mut R) -> Result<String> {
        let bytes = reader.sized_read()?;
        let gpg_id = String::from_utf8_lossy(&bytes);
        Ok(gpg_id.to_string())
    }

    fn read_header<R: Read>(reader: &mut R, gpg: &Gpg) -> Result<Header> {
        eprintln!("Get encoded data...");
        let enc_header_data = reader.sized_read()?;
        eprintln!("Decode header data...");
        let header_data = gpg.decrypt(&enc_header_data)?;
        eprintln!("Build header...");
        Header::try_from(&header_data)
    }

    pub fn read_from_file(data_file: &PathBuf) -> Result<Self> {
        let mut reader = File::open(data_file).map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Error::DataFileNotFound,
            _ => Error::IoError(e),
        })?;

        eprintln!("Reading GPG ID...");
        let gpg_id = Self::read_gpg_id(&mut reader)?;

        eprintln!("Creating GPG...");
        let gpg = Gpg::new(&gpg_id);

        eprintln!("using GPG ID: [{gpg_id}]");

        eprintln!("Reading header...");
        let header = Self::read_header(&mut reader, &gpg)?;

        eprintln!("getting reader...");
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
        let dir = Database::default_dir();
        if !dir.is_dir() {
            fs::create_dir_all(dir)?;
        }
        let mut writer = File::create(Database::path())?;

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
