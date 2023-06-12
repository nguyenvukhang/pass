use crate::database::Database;
use crate::skim::skim;

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
    Insert { name: String },

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
        skim(db);
        return;
    }

    match args.command.unwrap() {
        Commands::Insert { name } => insert_password(db, name),
        Commands::Edit { name } => edit_password(db, name),
        Commands::Remove { name } => remove_password(db, name),
    }
}

/// Prompt the user twice for a password to insert
fn insert_password(mut db: Database, name: String) {
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
    db.write().unwrap();
}

/// Use skim to select a context to edit,
/// then open current password in a temporary $EDITOR buffer
/// save the entire buffer as the password
fn edit_password(db: Database, name: Option<String>) {}

/// Use skim to select a context to remove.
fn remove_password(db: Database, name: Option<String>) {}
