mod app;
mod chacha_io;
mod database;
mod error;
mod header;
mod rsa_cipher;

use app::App;
use error::{Error, Result};
use header::Header;
use rpassword::read_password;
use rsa_cipher::RsaCipher;

use std::io::Write;

pub const DATA_FILE: &str = "pass.store";

use clap::{Parser, Subcommand};

// store file plan:
// ```
// <One-time generated AES key><Last-used AES Nonce>  ←  encrypted with RSA
// <key>:<value>
// <key>:<value>
// ...
// ```

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Name/context of the password stored
    name: Option<String>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Insert a new password
    Insert { name: String },
}

fn main() {
    let args = Args::parse();

    let app = match App::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create a pass.store.\nError: {e:?}");
            return;
        }
    };

    let mut db = app.read().unwrap();

    if let None = args.command {
        let names = db.list_all();
        for i in names {
            println!("-- {i}");
        }
        return;
    }

    match args.command.unwrap() {
        Commands::Insert { name } => {
            if db.has_name(&name) {
                eprintln!("Database already has an entry for [{name}]");
                return;
            }
            let mut stdout = std::io::stdout();

            print!("Enter password for [{name}] > ");
            stdout.flush().unwrap();
            let p1 = read_password().unwrap();

            print!("Retype password for [{name}] > ");
            stdout.flush().unwrap();
            let p2 = read_password().unwrap();

            if p1 != p2 {
                println!("Passwords dont match");
                return;
            }
            db.insert(&name, &p1);
            app.write(&db);
        }
    }
}

#[allow(unused)]
fn skim() {
    use skim::prelude::*;
    use std::io::Cursor;

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let input = "aaaaa\nbbbb\nccc".to_string();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    // `run_with` would read and show items from the stream
    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}

// openssl genrsa -out private_key.pem 4096
// openssl rsa -in private_key.pem -out public_key.pem -pubout -outform PEM
// openssl rsa -in private_key.pem -text -noout | less
// ssh-keygen -y -f private_key.pem > id_rsa.pub
