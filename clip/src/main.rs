use std::process::Command;
use std::process::Output;

const SLEEP_ARGV0: &str = "password store sleep";

fn run_shell(command: &str) -> Output {
    Command::new("sh").args(["-c", command]).output().unwrap()
}

fn clip_read() -> String {
    String::from_utf8_lossy(&run_shell("pbpaste").stdout).to_string()
}

fn clip_write(contents: &str) {
    run_shell(&format!("printf \"{contents}\" | pbcopy"));
}

fn delayed_clear(before: &str) {
    run_shell(&format!(
        "\
(
  (exec -a \"{SLEEP_ARGV0}\" sleep 3)
  printf \"{before}\" | pbcopy
) >/dev/null 2>&1 &"
    ));
}

// clip() {
//   local SLEEP_ARGV0="password store sleep for user $(id -u)"
//   pkill -f "^$SLEEP_ARGV0" 2>/dev/null && sleep 0.5
//   local before="$(pbpaste | $BASE64)"
//   printf "$1" | pbcopy
//   (
//     (exec -a "$SLEEP_ARGV0" sleep "$CLIP_TIME")
//     local now="$(pbpaste | $BASE64)"
//     [[ $now != $(echo -n "$1" | $BASE64) ]] && before="$now"
//     printf "$before" | $BASE64 -d | pbcopy
//   ) >/dev/null 2>&1 &
//   disown
//   echo "Copied $2 to clipboard. Will clear in $CLIP_TIME seconds."
// }

fn main() {
    let x = "hello there";
    println!("{:?}", x.split_once('\n'));
    // clip_write("before entering");
    //
    // run_shell(&format!("pkill -f \"^{SLEEP_ARGV0}\" 2>/dev/null && sleep 0.5"));
    // let before = clip_read();
    //
    // println!("[previous] ({before})");
    //
    // clip_write("artificial write");
    //
    // delayed_clear(&before);
}
