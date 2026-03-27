use std::process::{Command, exit};
use std::{env, fs};

const DEFAULT_JOKES: &str = include_str!("jokes.txt");

fn main() {
    let commit_msg_path = env::args().nth(1);
    let commit_msg = generate_message();
    if let Some(path) = commit_msg_path {
        if let Err(e) = fs::write(path, &commit_msg) {
            eprintln!("Failed to write commit message: {}", e);
            exit(1);
        }
    } else {
        match Command::new("git")
            .args(["commit", "-m", &commit_msg])
            .status()
        {
            Ok(status) if status.success() => (),
            Ok(status) => {
                eprintln!(
                    "Git commit failed with exit code: {}",
                    status.code().unwrap_or(1)
                );
                eprintln!("Tip: Are there any staged changes? (git add .)");
                exit(1);
            }
            Err(e) => {
                eprintln!("Failed to execute 'git': {}", e);
                exit(1);
            }
        }
    }
}

fn generate_message() -> String {
    if fastrand::f32() < 0.5 {
        if let Ok(msg) = fetch_api_msg() {
            return msg;
        }
    }
    fetch_local_backup()
}

fn fetch_api_msg() -> Result<String, Box<dyn std::error::Error>> {
    let msg = minreq::get("https://whatthecommit.com/index.txt")
        .with_timeout(1)
        .send()?
        .as_str()?
        .trim()
        .to_string();
    Ok(msg)
}

fn fetch_local_backup() -> String {
    let mut chosen = None;
    let mut count = 0;

    for line in DEFAULT_JOKES.lines().filter(|l| !l.is_empty()) {
        count += 1;
        if fastrand::usize(..count) == 0 {
            chosen = Some(line);
        }
    }
    chosen
        .unwrap_or("I have no idea what I'm doing.")
        .to_string()
}
