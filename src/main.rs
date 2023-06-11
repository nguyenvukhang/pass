mod app;
mod chacha_io;
mod cli;
mod database;
mod error;
mod gpg;
mod header;
mod sized_io;
mod skim;

use app::App;
use database::Database;
use error::{Error, Result};
use header::Header;

pub const DATA_FILE: &str = "pass.store";
pub const GPG_ID: &str = "AEFA1C1E59E02600E64E7C1D4A9E6CC722E4AA25";

// Data file structure
// ───────────────────────────────────────────────────────────────────
// <GNUPG private key id to use>
// <One-time generated ChaCha20 key><ChaCha20 Nonce>
// <key>:<value>
// <key>:<value>
// ...
// ───────────────────────────────────────────────────────────────────
// Everything below is encrypted with the last key above it.
//  * ChaCha20 keys are encrypted with the choice of GNUPG's key
//  * <key>:<value> pairs are encrypted with ChaCha20

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
