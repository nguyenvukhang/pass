use crate::{clipboard::clip, database::Database, error::Error, gpg::Gpg};

use clap::{Parser, Subcommand};
use rand::{distributions::Alphanumeric, Rng};
use rpassword::read_password;

const LINE: &str = "──────────────────────────────";

use std::{fs, io::Write, path::PathBuf, process::Command};

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
    /// Create a new password store
    Init { gpg_id: String },

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
        Err(Error::DataFileNotFound) => {
            return eprintln!(
                "Database not found. Run `pass init <gpg-id>` first."
            );
        }
        Err(e) => {
            return eprintln!("Failed to read pass.store.\nError: {e:?}");
        }
    };

    if let Some(name) = args.name {
        return get_password(db, &name);
    }

    if let None = args.command {
        return search_password(db);
    }

    match args.command.unwrap() {
        Commands::Init { gpg_id } => initialize_db(db, gpg_id),
        Commands::Insert { name, password } => {
            insert_password(db, name, password)
        }
        Commands::Edit { name } => edit_password(db, name),
        Commands::Remove { name } => remove_password(db, name),
    }
}

fn initialize_db(db: Database, gpg_id: String) {
    if let Some(_) = db.gpg_id() {
        return println!("Current database already has an owner id.");
    }
    println!("Creating new database using {gpg_id}");
    let test = Command::new("gpg").args(["-K", &gpg_id]).output().unwrap();
    let ok = String::from_utf8_lossy(&test.stdout).contains(&gpg_id);

    if ok {
        let _ = db.write();
    } else {
        println!("Invalid key id given. Try using `gpg -K` to show the available keys");
    }
}

fn search_password(db: Database) {
    let selection = match db.select_one() {
        None => return println!("Nothing selected"),
        Some(v) => v,
    };

    println!("[{selection}]");
    get_password(db, &selection)
}

fn get_password(db: Database, name: &str) {
    let data = match db.get(name) {
        None => return println!("No password found for [{name}]"),
        Some(v) => v,
    };

    if let Some((password, metadata)) = data.split_once('\n') {
        println!("{metadata}");
        clip::temp_write(password);
    } else {
        clip::temp_write(data);
    }
    println!(
        "{LINE}\nCopied password to clipboard. Will reset after {} seconds.",
        clip::RESTORE_DELAY
    )
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
fn edit_password(mut db: Database, name: Option<String>) {
    let name = match name.or_else(|| db.select_one()) {
        None => return println!("No name selected to edit"),
        Some(v) => v,
    };

    let old_value = match db.get(&name) {
        None => return println!("No value found for [{name}]"),
        Some(v) => v,
    };

    let editor = match get_editor() {
        None => return println!("No editor found."),
        Some(v) => v,
    };

    println!("using editor: {editor:?}");

    let tmp_file = get_temp_file();
    println!("using tmp file: {tmp_file:?}");

    fs::write(&tmp_file, old_value.as_bytes()).unwrap();

    edit_file(&editor, &tmp_file);
    // TODO: shred this file or encrypt it, because this seems to be a
    // weak point

    let new_value = fs::read_to_string(&tmp_file).unwrap();

    let _ = fs::remove_file(&tmp_file);

    if old_value == &new_value {
        return println!("No change required.");
    } else {
        println!("Update from:\n{old_value}\nTo:\n{new_value}");
    }

    db.update(&name, new_value.trim());
    db.write().unwrap();
}

/// Use skim to select a context to remove.
fn remove_password(mut db: Database, name: Option<String>) {
    if let Some(name) = name {
        db.remove(&name);
        db.write().unwrap()
    }
}

/// Get an installed editor
fn get_editor() -> Option<PathBuf> {
    use which::which;
    if let Ok(v) = std::env::var("EDITOR") {
        if let Ok(v) = which(v) {
            return Some(PathBuf::from(v));
        }
    }
    if let Ok(v) = which("nvim") {
        return Some(v);
    }
    if let Ok(v) = which("vim") {
        return Some(v);
    }
    if let Ok(v) = which("nano") {
        return Some(v);
    }
    None
}

fn edit_file(editor: &PathBuf, filepath: &PathBuf) {
    let mut cmd = Command::new(&editor);
    cmd.arg(&filepath);
    let child = cmd.spawn().unwrap();
    let _ = child.wait_with_output();
}

/// Get a random filename for temporary buffer
fn get_temp_file() -> PathBuf {
    let random: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    std::env::temp_dir().join(&format!("pass.{random}"))
}
