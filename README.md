<div style="text-align: center;">
  <img src="https://capsule-render.vercel.app/api?type=transparent&height=300&color=gradient&text=whattheprecommit&section=subheader&reversal=false&height=120&fontSize=60&fontColor=ff5500">
</div>

Automatically makes your git history worse. My final contribution to the team.

## Installation

### As a pre-commit hook

Add this to your `.pre-commit-config.yaml` to kill professional commit messages forever:

```yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/whattheprecommit
    rev: v0.0.10
    hooks:
      - id: whattheprecommit
```

### Globally as a CLI

To ruin your commit messages globally:

```bash
cargo install --git https://github.com/RektPunk/whattheprecommit
```

Just type `wtc` instead of `git commit -m "..."`. It's like Russian Roulette for your Git history.

### Sources

Some of the commit messages were borrowed from [ngerakines/commitment](https://github.com/ngerakines/commitment).
