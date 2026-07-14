use std::error::Error;
use std::io::{Error as IoError, Result as IoResult};
use std::process::{Command, Output, exit};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const DEFAULT_JOKES: &str = include_str!("jokes.txt");
static SEED_COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    let commit_msg = generate_message();
    match Command::new("git")
        .args(["commit", "-m", &commit_msg])
        .output()
    {
        Ok(output) if output.status.success() => (),
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stdout.is_empty() {
                eprintln!("{stdout}");
            }
            if !stderr.is_empty() {
                eprintln!("{stderr}");
            }
            exit(1);
        }
        Err(e) => {
            eprintln!("Failed to execute 'git': {}", e);
            exit(1);
        }
    }
}

fn generate_message() -> String {
    match fast_random(2) {
        0 => fetch_api_msg().unwrap_or_else(|_| fetch_local_backup()),
        _ => fetch_local_backup(),
    }
}

fn fetch_api_output() -> IoResult<Output> {
    let mut cmd = Command::new("curl");
    cmd.args([
        "-s",
        "--max-time",
        "2",
        "https://whatthecommit.com/index.txt",
    ]);
    let output = cmd.output()?;
    if output.status.success() {
        Ok(output)
    } else {
        Err(IoError::other("Command returned non-zero status"))
    }
}

fn fetch_api_msg() -> Result<String, Box<dyn Error>> {
    let output = fetch_api_output()?;
    let msg = String::from_utf8_lossy(&output.stdout);
    let trimmed = msg.trim();
    if !trimmed.is_empty() {
        return Ok(trimmed.to_string());
    }

    Err("API fetch failed".into())
}

fn fetch_local_backup() -> String {
    let mut chosen = None;
    let mut count = 0;
    for line in DEFAULT_JOKES.lines().filter_map(|l| {
        let l = l.trim();
        (!l.is_empty()).then_some(l)
    }) {
        count += 1;
        if fast_random(count) == 0 {
            chosen = Some(line);
        }
    }

    chosen
        .unwrap_or("I have no idea what I'm doing.")
        .to_string()
}

fn fast_random(max: usize) -> usize {
    if max <= 1 {
        return 0;
    }
    let time_sn = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut seed = time_sn ^ counter;
    seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);

    (seed as usize) % max
}
