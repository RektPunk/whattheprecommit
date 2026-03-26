# whattheprecommit
Automatically ruins git commit messages with whatthecommit API. My final contribution to the team.

```yaml
# .pre-commit-config.yaml
default_install_hook_types: [pre-commit, prepare-commit-msg]

repos:
  - repo: https://github.com/RektPunk/whattheprecommit
    rev: v0.0.1
    hooks:
      - id: whattheprecommit
```
