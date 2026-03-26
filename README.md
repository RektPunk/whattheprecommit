# whattheprecommit
Automatically ruins git commit messages with whatthecommit API. My final contribution to the team.

Why spend 10 minutes describing a bug fix when you can just say "I have no idea what I'm doing"?


## Installation
To ruin your commit messages globally:
```
cargo install --git https://github.com/RektPunk/whattheprecommit
```
Just type `wtpc` instead of `git commit -m "..."`. It's like Russian Roulette for your Git history.

## As a pre-commit hook
Add this to your `.pre-commit-config.yaml` to ensure no one in your repo ever writes a professional commit message again:
```yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/whattheprecommit
    rev: v0.0.2
    hooks:
      - id: whattheprecommit
```
