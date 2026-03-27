# whattheprecommit
Automatically ruins git commit messages with whatthecommit API. My final contribution to the team.

## Installation
### As a pre-commit hook
Add this to your `.pre-commit-config.yaml` to kill professional commit messages forever:
```yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/whattheprecommit
    rev: v0.0.3
    hooks:
      - id: whattheprecommit
```
### Globally as a CLI
To ruin your commit messages globally:
```bash
cargo install --git https://github.com/RektPunk/whattheprecommit
```
Just type `wtc` instead of `git commit -m "..."`. It's like Russian Roulette for your Git history.
