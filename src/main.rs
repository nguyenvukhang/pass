mod chacha_io;
mod cli;
mod clipboard;
mod database;
mod error;
mod gpg;
mod header;
mod sized_io;
mod skim;

use error::{Error, Result};
use header::Header;

// Data file structure
// ───────────────────────────────────────────────────────────────────
// <GNUPG private key id to use>
// <One-time generated ChaCha20 key><ChaCha20 Nonce>
// <key>:<value>
// <key>:<value>
// ...
// ───────────────────────────────────────────────────────────────────
// Everything below is encrypted with the last key above it.
//  * GNUPG private key id is unencrypted
//  * ChaCha20 keys are encrypted with the choice of GNUPG's key
//  * <key>:<value> pairs are encrypted with ChaCha20

fn main() {
    cli::run();
}
