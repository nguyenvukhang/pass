use std::io::Write;
use std::process::{Command, Stdio};
use std::{env, fs, io};

pub struct Gpg {
    id: String,
}

impl Gpg {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }

    fn cmd(&self) -> Command {
        let mut cmd = Command::new("gpg");
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        cmd.args(["--compress-algo=none"]);
        cmd
    }

    pub fn encrypt<B: AsRef<[u8]>>(
        &self,
        plaintext: B,
    ) -> Result<Vec<u8>, io::Error> {
        let mut cmd = self.cmd();
        cmd.stdin(Stdio::piped());
        cmd.args(["--recipient", &self.id, "--encrypt"]);

        let mut child = cmd.spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(plaintext.as_ref())?;
        drop(stdin);

        let output = child.wait_with_output()?;
        Ok(output.stdout)
    }

    pub fn decrypt<B: AsRef<[u8]>>(
        &self,
        bytes: B,
    ) -> Result<Vec<u8>, io::Error> {
        let tmp_path = env::temp_dir().join("pass.tmp");
        fs::write(&tmp_path, bytes)?;

        let mut cmd = self.cmd();
        cmd.args(["--quiet", "--decrypt"]);
        cmd.arg(&tmp_path);

        let child = match cmd.spawn() {
            Ok(v) => v,
            Err(e) => {
                let _ = fs::remove_file(&tmp_path);
                return Err(e);
            }
        };
        let output = child.wait_with_output()?;
        let _ = fs::remove_file(&tmp_path);
        Ok(output.stdout)
    }
}
