/*
 * This is for piping & spawning adb logcart content to lumberjack
 */
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};


pub fn spawn_logcat() -> impl Iterator<Item = String> {
    let mut child = Command::new("adb")
        .args(["logcat", "-T", "1"]) // command that gets logcart content
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start adb");

    let stdout = child.stdout.take().expect("No Stdout");

    BufReader::new(stdout)
        .lines()
        .filter_map(Result::ok)

}
