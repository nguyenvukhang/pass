use crate::chacha_io::{ChaReader, ChaWriter};
use crate::database::Database;
use crate::gpg::Gpg;
use crate::header::HEADER_ENCODED_BYTE_LEN;
use crate::read_ext::{SizedRead, SizedWrite};
use crate::{Header, Result, RsaCipher, DATA_FILE, GPG_ID};

use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Read, Write};

pub struct App {
    cipher: RsaCipher,
}

const GPG_ID_BYTE_LENGTH: usize = 64;

impl App {
    pub fn new() -> Result<Self> {
        let cipher = RsaCipher::new()?;
        Ok(Self { cipher })
    }

    pub fn read(&self) -> Result<Database> {
        let mut file = match File::open(DATA_FILE) {
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Ok(Database::new()),
                _ => return Err(e)?,
            },
            Ok(v) => v,
        };
        let gpg_id = file.sized_read()?;
        let gpg_id = String::from_utf8_lossy(&gpg_id);
        let gpg_id = gpg_id.trim_matches(char::from(0));
        println!("using gpg id: {gpg_id}");

        let gpg = Gpg::new(gpg_id);

        let enc_header_data = file.sized_read()?;
        let header_data = gpg.decrypt(&enc_header_data)?;
        println!("buffer -> {:?}", header_data);

        let header = Header::try_from(&header_data)?;

        let cipher = header.cipher();
        let reader = ChaReader::new(file, cipher);

        let db = serde_json::from_reader::<_, Database>(reader)?;
        Ok(db)
    }

    pub fn write(&self, database: &Database) -> Result<()> {
        let mut writer = File::create(DATA_FILE)?;

        writer.sized_write(&GPG_ID.as_bytes())?;

        let gpg = Gpg::new(GPG_ID);

        let header = Header::generate();
        println!("PRECODED -> {:?}", header.as_bytes());

        let enc_header_data = gpg.encrypt(header.as_bytes())?;

        writer.sized_write(&enc_header_data)?;

        // final write
        let writer = ChaWriter::new(writer, header.cipher());
        serde_json::to_writer::<_, Database>(writer, &database).unwrap();

        Ok(())
    }
}
