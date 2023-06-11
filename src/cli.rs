use crate::app::App;

use clap::{Parser, Subcommand};
use rpassword::read_password;
use skim::prelude::*;

use std::io::Write;

pub const DATA_FILE: &str = "pass.store";

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

fn run_app() {
    let args = Args::parse();

    let app = match App::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to create a pass.store.\nError: {e:?}");
            return;
        }
    };

    if let None = args.command {
        skim(app);
        return;
    }

    match args.command.unwrap() {
        Commands::Insert { name } => {
            let mut db = app.read().unwrap();
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
            app.write(&db).unwrap();
        }
    }
}

fn main() {}

struct Name(String);

impl SkimItem for Name {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.0)
    }

    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::Text(self.0.to_string())
    }
}

fn skim(app: App) {
    let options = SkimOptionsBuilder::default()
        .height(Some("10"))
        .multi(false)
        .no_clear(false)
        .build()
        .unwrap();

    let db = app.read().unwrap();
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) =
        skim::prelude::bounded(db.count());
    for name in db.list_all() {
        let _ = tx_item.send(Arc::new(Name(name)));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    // `run_with` would read and show items from the stream
    let selected = Skim::run_with(&options, Some(rx_item))
        .and_then(|mut out| out.selected_items.pop());

    let selected = match selected {
        Some(v) => v,
        None => return,
    };

    let name = selected.output();
    let password = db.get_unchecked(&name);

    print!("{} -> {}\n", name, password);
}

// openssl genrsa -out private_key.pem 4096
// openssl rsa -in private_key.pem -out public_key.pem -pubout -outform PEM
// openssl rsa -in private_key.pem -text -noout | less
// ssh-keygen -y -f private_key.pem > id_rsa.pub
