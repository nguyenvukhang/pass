use crate::{database::Database, skim};

use clap::{Parser, Subcommand};
use rpassword::read_password;

use std::io::Write;

// The CLI app structure. The list of arguments available to the CLI user.
#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Name/context of the password stored
    name: Option<String>,
}

// The sub-commands available. These describe actions that the user can take
// to modify the database of passwords.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Insert a new password
    Insert {
        name: String,

        #[arg(short, long)]
        password: Option<String>,
    },

    /// Edit a password
    Edit { name: Option<String> },

    /// Remove a name-password pair
    Remove { name: Option<String> },
}

/// Main entrypoint for CLI to start running. Powered by clap-rs.
pub fn run() {
    let args = Args::parse();

    let db = match Database::read() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to read pass.store.\nError: {e:?}");
            return;
        }
    };

    if let None = args.command {
        return search_password(db);
    }

    match args.command.unwrap() {
        Commands::Insert { name, password } => {
            insert_password(db, name, password)
        }
        Commands::Edit { name } => edit_password(db, name),
        Commands::Remove { name } => remove_password(db, name),
    }
}

fn search_password(db: Database) {
    let selection = skim::select_one(db.list_all().into_iter());
    println!(
        "selected: {selection:?} -> {:?}",
        selection.as_ref().and_then(|v| db.get(&v))
    );
}

fn insert_password(mut db: Database, name: String, password: Option<String>) {
    if db.has_name(&name) {
        eprintln!("Database already has an entry for [{name}]");
        return;
    }

    let password = match password {
        Some(v) => v,
        None => match prompt_password_twice(&name) {
            Some(v) => v,
            None => {
                return println!("Passwords do not match.");
            }
        },
    };

    db.insert(&name, &password);
    db.write().unwrap();
}

/// Prompt the user twice for a password to insert
fn prompt_password_twice(name: &str) -> Option<String> {
    let mut stdout = std::io::stdout();

    print!("Enter password for [{name}] > ");
    stdout.flush().unwrap();
    let p1 = read_password().unwrap();

    print!("Retype password for [{name}] > ");
    stdout.flush().unwrap();
    let p2 = read_password().unwrap();

    (p1 == p2).then_some(p1)
}

/// Use skim to select a context to edit,
/// then open current password in a temporary $EDITOR buffer
/// save the entire buffer as the password
fn edit_password(db: Database, name: Option<String>) {}

/// Use skim to select a context to remove.
fn remove_password(db: Database, name: Option<String>) {}
