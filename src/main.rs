mod app;
mod chacha_io;
mod cli;
mod database;
mod error;
mod gpg;
mod header;
mod read_ext;
mod rsa_cipher;
mod skim;

use app::App;
use database::Database;
use error::{Error, Result};
use header::Header;

use rsa_cipher::RsaCipher;

pub const DATA_FILE: &str = "pass.store";
pub const GPG_ID: &str = "AEFA1C1E59E02600E64E7C1D4A9E6CC722E4AA25";

// store file plan:
// ```
// <gnupg private key id to use (has to be for RSA)>
// <One-time generated AES key><Last-used AES Nonce>  ←  encrypted with RSA
// <key>:<value>
// <key>:<value>
// ...
// ```

fn main() {
    let app = App::new().unwrap();
    let mut db = Database::new();
    db.insert("hello", "world");
    app.write(&db).unwrap();
    let db = app.read().unwrap();
    for i in db.list_all() {
        println!("{i} -> {:?}", db.get(&i));
    }
}

// openssl genrsa -out private_key.pem 4096
// openssl rsa -in private_key.pem -out public_key.pem -pubout -outform PEM
// openssl rsa -in private_key.pem -text -noout | less
// ssh-keygen -y -f private_key.pem > id_rsa.pub
