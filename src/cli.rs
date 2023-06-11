use crate::app::App;
use crate::skim::skim;

use clap::{Parser, Subcommand};
use rpassword::read_password;

use std::io::Write;

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

pub fn run_app() {
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
