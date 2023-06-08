use crate::chacha_io::{ChaReader, ChaWriter};
use crate::database::Database;
use crate::header::HEADER_ENCODED_BYTE_LEN;
use crate::{Header, Result, RsaCipher, DATA_FILE};

use std::fs::File;
use std::io::{BufReader, ErrorKind, Read, Write};

pub struct App {
    cipher: RsaCipher,
}

impl App {
    pub fn new() -> Result<Self> {
        let cipher = RsaCipher::new()?;
        Ok(Self { cipher })
    }

    pub fn read(&self) -> Result<Database> {
        let file = match File::open(DATA_FILE) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Ok(Database::new()),
                _ => return Err(e)?,
            },
            Ok(v) => v,
        };
        let mut reader = BufReader::new(file);

        let mut enc_header_data = [0u8; HEADER_ENCODED_BYTE_LEN];
        reader.read_exact(&mut enc_header_data)?;
        let header = Header::decrypt(&self.cipher, &enc_header_data)?;

        let cipher = header.cipher();
        let reader = ChaReader::new(reader, cipher);

        let db = serde_json::from_reader::<_, Database>(reader)?;
        Ok(db)
    }

    pub fn write(&self, database: &Database) -> Result<()> {
        let mut writer = File::create(DATA_FILE)?;

        let header = Header::generate();
        let header_bytes = header.encrypt(&self.cipher)?;
        assert_eq!(header_bytes.len(), HEADER_ENCODED_BYTE_LEN);
        writer.write_all(&header_bytes)?;

        // final write
        let writer = ChaWriter::new(writer, header.cipher());
        serde_json::to_writer::<_, Database>(writer, &database).unwrap();

        Ok(())
    }
}
