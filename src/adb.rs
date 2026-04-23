use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::Sender;

pub fn spawn_logcat() -> Box<dyn Iterator<Item = String> + Send> {
    Command::new("adb").args(["logcat", "-c"]).output().ok();

    let (tx, rx) = std::sync::mpsc::channel::<String>();

    std::thread::spawn(move || {
        run_logcat_stream(tx);
    });

    Box::new(rx.into_iter())
}

fn run_logcat_stream(tx: Sender<String>) {
    let mut child = Command::new("adb")
        .args([
            "logcat", 
            "-v", "brief",
            "*:E", "*:W", "*:I"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start adb logcat");

    let stdout = child.stdout.take().expect("No stdout");
    // let mut latched_pid: Option<String> = None;

    for line in BufReader::new(stdout).lines().filter_map(Result::ok) {
        
        if tx.send(line).is_err() {
            break;
        }

        /*
        // Try to latch a PID if we haven't yet
        if latched_pid.is_none() {
            if let Some(pid) = extract_pid(&line) {
                if let Some(pkg) = package_for_pid(&pid) {
                    eprintln!("[lumberjack] watching: {} (pid {})", pkg, pid);
                    latched_pid = Some(pid);
                }
            }
        }

        // If we have a latched PID, only forward lines from that process
        // If not latched yet, forward everything so the raw log panel isn't empty
        let should_forward = match &latched_pid {
            Some(pid) => line_matches_pid(&line, pid),
            None => true,
        };

        if should_forward {
            if tx.send(line).is_err() {
                break;
            }
        }
        */
    }
    // make sure child is cleaned up
    let _ = child.wait();
}


// Check if a logcat line belongs to a specific pid
fn line_matches_pid(line: &str, pid: &str) -> bool {
    if let Some(p) = extract_pid(line) {
        p == pid
    } else {
        false
    }
}

fn extract_pid(line: &str) -> Option<String> {
    let open  = line.find('(')?;
    let close = line[open..].find(')')? + open;
    let pid = line[open + 1..close].trim().to_string();
    if !pid.is_empty() && pid.chars().all(|c| c.is_ascii_digit()) {
        Some(pid)
    } else {
        None
    }
}

fn package_for_pid(pid: &str) -> Option<String> {
    let out = Command::new("adb")
        .args(["shell", "cat", &format!("/proc/{}/cmdline", pid)])
        .output()
        .ok()?;

    let raw = String::from_utf8_lossy(&out.stdout);
    let pkg = raw.split('\0').next()?.trim().to_string();

    // Must look like a real app package:
    // - has at least 2 dots (com.example.app, not com.launcher)
    // - doesn't start with known system prefixes
    let dot_count = pkg.chars().filter(|&c| c == '.').count();
    let is_system = pkg.starts_with("com.android.")
        || pkg.starts_with("com.google.android.")
        || pkg.starts_with("android.")
        || pkg.starts_with("com.sec.")      // Samsung system
        || pkg.starts_with("com.samsung.")
        || pkg.starts_with("com.miui.")     // Xiaomi system
        || pkg.starts_with("com.real.")     // launcher
        || pkg.starts_with("launcher")
        || pkg.contains("launcher")
        || pkg.contains("systemui")
        || pkg.contains("inputmethod");

    if dot_count >= 2 && !is_system && !pkg.is_empty() {
        Some(pkg)
    } else {
        None
    }
}
