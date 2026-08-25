use std::process::{Command, exit};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const JOKES: &str = include_str!("jokes.txt");
static SEED_COUNTER: AtomicU64 = AtomicU64::new(0);

fn main() {
    let commit_msg = get_random_message();
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
            eprintln!("Failed to execute 'git': {e}");
            exit(1);
        }
    }
}

fn get_random_message() -> String {
    let mut chosen = None;
    let mut count = 0;

    for line in JOKES.lines().filter_map(|line| {
        let line = line.trim();
        (!line.is_empty()).then_some(line)
    }) {
        count += 1;
        if fast_random(count) == 0 {
            chosen = Some(line);
        }
    }

    chosen
        .unwrap_or("I have no idea what I'm doing.")
        .to_owned()
}

fn fast_random(max: usize) -> usize {
    if max <= 1 {
        return 0;
    }

    let time_ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let counter = SEED_COUNTER.fetch_add(1, Ordering::Relaxed);
    let mut seed = time_ns ^ counter;

    seed = seed
        .wrapping_mul(6364136223846793005)
        .wrapping_add(1442695040888963407);

    (seed as usize) % max
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_value_is_zero_for_max_zero_or_one() {
        assert_eq!(fast_random(0), 0);
        assert_eq!(fast_random(1), 0);
    }

    #[test]
    fn random_value_is_within_range() {
        for max in 2..100 {
            let value = fast_random(max);
            assert!(value < max);
        }
    }

    #[test]
    fn random_message_is_not_empty() {
        let message = get_random_message();
        assert!(!message.is_empty());
    }

    #[test]
    fn random_message_is_one_of_the_jokes() {
        let message = get_random_message();
        assert!(
            JOKES
                .lines()
                .map(str::trim)
                .any(|line| !line.is_empty() && line == message)
        );
    }

    #[test]
    fn random_message_is_trimmed() {
        let message = get_random_message();
        assert_eq!(message, message.trim());
    }

    #[test]
    fn jokes_contains_non_empty_messages() {
        assert!(
            JOKES.lines().any(|line| !line.trim().is_empty()),
            "jokes.txt must contain at least one non-empty message"
        );
    }
}
