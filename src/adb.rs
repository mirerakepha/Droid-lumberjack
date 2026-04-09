/*
 * adb.rs
 * This is for piping & spawning adb logcart content to lumberjack
 */
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn get_foreground_pid() -> Option<String> {
    // Dump the activity stack and pull the foreground package name
    let output = Command::new("adb")
        .args(["shell", "dumpsys", "activity", "recents"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("Recent #0") {
            // line looks like: * Recent #0: Task{... #42 A=com.your.app ...}
            if let Some(start) = line.find("A=") {
                let rest = &line[start + 2..];
                let pkg = rest.split_whitespace().next()?
                    .trim_matches('}')
                    .to_string();
                // now resolve that package name to a PID
                let pid_out = Command::new("adb")
                    .args(["shell", "pidof", "-s", &pkg])
                    .output()
                    .ok()?;
                let pid = String::from_utf8_lossy(&pid_out.stdout)
                    .trim()
                    .to_string();
                if !pid.is_empty() {
                    return Some(pid);
                }
            }
        }
    }
    None
}

pub fn spawn_logcat() -> Box<dyn Iterator<Item = String>> {
    Command::new("adb").args(["logcat", "-c"]).output().ok();

    let mut args = vec!["logcat", "-v", "brief"];

    // Try to auto-detect the foreground app's PID
    let pid = get_foreground_pid();
    let pid_flag; // keep alive for the args slice
    if let Some(ref p) = pid {
        pid_flag = format!("--pid={}", p);
        args.push(&pid_flag);
    } else {
        // Fallback: all E/W/I logs
        args.extend_from_slice(&["*:E", "*:W", "*:I"]);
    }

    let mut child = Command::new("adb")
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start adb logcat");

    let stdout = child.stdout.take().expect("No stdout");
    // leak child so it isn't dropped (process keeps running)
    std::mem::forget(child);

    Box::new(BufReader::new(stdout).lines().filter_map(Result::ok))
}
