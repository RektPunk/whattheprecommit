<div style="text-align: center;">
  <img src="https://capsule-render.vercel.app/api?type=transparent&height=300&color=gradient&text=whattheprecommit&section=subheader&reversal=false&height=120&fontSize=60&fontColor=ff5500">
</div>
<p align="center">
  <a href="https://github.com/RektPunk/whattheprecommit/releases/latest">
      <img alt="release" src="https://img.shields.io/github/v/release/RektPunk/whattheprecommit.svg">
  </a>
  <a href="https://github.com/RektPunk/whattheprecommit/blob/main/LICENSE">
    <img alt="License" src="https://img.shields.io/github/license/RektPunk/whattheprecommit.svg">
  </a>
</p>

Automatically ruins git commit messages with whatthecommit API. My final contribution to the team.

## Installation
### As a pre-commit hook
Add this to your `.pre-commit-config.yaml` to kill professional commit messages forever:
```yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/whattheprecommit
    rev: v0.0.4
    hooks:
      - id: whattheprecommit
```
### Globally as a CLI
To ruin your commit messages globally:
```bash
cargo install --git https://github.com/RektPunk/whattheprecommit
```
Just type `wtc` instead of `git commit -m "..."`. It's like Russian Roulette for your Git history.
