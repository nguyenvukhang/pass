pub mod clip {
    use std::process::Command;
    use std::process::Output;
    use std::process::Stdio;

    pub const RESTORE_DELAY: usize = 45;

    // run a shell command synchronously and get the output
    fn run_shell(command: &str) -> Output {
        Command::new("sh")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .args(["-c", command])
            .output()
            .unwrap()
    }

    const PID: &str = "password store sleep";
    pub fn read() -> String {
        run_shell(&format!("pkill -f \"^{PID}\" && sleep 0.5"));
        String::from_utf8_lossy(&run_shell("pbpaste").stdout).to_string()
    }

    pub fn write(contents: &str) {
        run_shell(&format!("printf \"{contents}\" | pbcopy"));
    }

    pub fn temp_write(contents: &str) {
        restore(&read());
        write(contents);
    }

    pub fn restore(restore: &str) {
        run_shell(&format!(
            "((exec -a \"{PID}\" sleep {RESTORE_DELAY}); printf \"{restore}\" | pbcopy) >/dev/null 2>&1 &"
        ));
    }
}
//
// fn main() {
//     clip::write("before entering");
//
//     let before = clip::read();
//
//     println!("[previous] ({before})");
//
//     clip::write("artificial write");
//
//     clip::clear(&before, 3);
// }
