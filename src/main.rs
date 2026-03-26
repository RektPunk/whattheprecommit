use rand::prelude::IndexedRandom;
use serde::Deserialize;
use std::process::{Command, exit};
use std::time::Duration;
use std::{env, fs};
use ureq::Agent;

const DEFAULT_JOKES: &str = include_str!("jokes.txt");

#[derive(Deserialize)]
struct CommitResponse {
    #[serde(rename = "commit_message")]
    message: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let commit_msg = generate_message();

    if let Some(commit_msg_path) = args.get(1) {
        if let Err(e) = fs::write(commit_msg_path, &commit_msg) {
            eprintln!("Failed to write commit message: {}", e);
            exit(1);
        }
    } else {
        let status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&commit_msg)
            .status();
        if let Ok(s) = status {
            if !s.success() {
                eprintln!("Git commit failed. Make sure you have staged changes.");
                exit(1);
            }
        } else {
            exit(1);
        }
    }
}

fn generate_message() -> String {
    if rand::random::<f32>() < 0.5 {
        fetch_api_msg().ok()
    } else {
        None
    }
    .or_else(fetch_local_backup)
    .unwrap_or_else(|| "I have no idea what I'm doing.".to_string())
}

fn fetch_api_msg() -> Result<String, Box<dyn std::error::Error>> {
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(1)))
        .build();
    let agent: Agent = config.into();
    let body = agent
        .get("https://whatthecommit.com/index.json")
        .call()?
        .body_mut()
        .read_json::<CommitResponse>()?;

    Ok(body.message)
}

fn fetch_local_backup() -> Option<String> {
    let lines: Vec<&str> = DEFAULT_JOKES.lines().filter(|l| !l.is_empty()).collect();
    lines.choose(&mut rand::rng()).map(|&s| s.to_string())
}
