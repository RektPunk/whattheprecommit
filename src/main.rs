use rand::prelude::IndexedRandom;
use serde::Deserialize;
use std::time::Duration;
use std::{env, fs, process};
use ureq::Agent;

const DEFAULT_JOKES: &str = include_str!("jokes.txt");

#[derive(Deserialize)]
struct CommitResponse {
    #[serde(rename = "commit_message")]
    message: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let commit_msg_path = match args.get(1) {
        Some(path) => path,
        None => process::exit(0),
    };

    let final_msg = if rand::random::<f32>() < 0.5 {
        fetch_api_msg().ok()
    } else {
        None
    }
    .or_else(fetch_local_backup)
    .unwrap_or_else(|| "I have no idea what I'm doing.".to_string());
    if let Err(_) = fs::write(commit_msg_path, final_msg) {
        process::exit(1);
    }
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
