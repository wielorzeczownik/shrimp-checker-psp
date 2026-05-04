# Contributing to shrimp-checker-psp

Thank you for considering a contribution. This document covers everything you need to get started.

## Overview

shrimp-checker-psp is a homebrew application for the PlayStation Portable that monitors shrimp tank parameters. It targets the `mipsel-sony-psp` platform and is built with Rust nightly using `cargo-psp`.

## Project structure

```text
.
├── src/
│   ├── main.rs            entry point
│   ├── app.rs             main application loop
│   ├── constants.rs       thresholds and configuration constants
│   └── i18n/              localisation (en, pl)
├── assets/                embedded assets (sound, graphics)
├── build.rs               PSP build metadata
├── Cargo.toml
└── cliff.toml             changelog configuration
```

## Running checks locally

### With tools installed locally

```bash
# Rust
cargo fmt --check
cargo clippy --all-targets -Zbuild-std -- -D warnings
cargo audit
cargo psp --release

# Shell
shfmt --diff scripts/

# Markdown
markdownlint-cli2 "**/*.md"
```

### With Docker (no local installs required)

```bash
docker run --rm -v "$(pwd):/src" -w /src mvdan/shfmt --diff scripts/

docker run --rm -v "$(pwd):/workdir" davidanson/markdownlint-cli2 "**/*.md"

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

| Prefix      | When to use                         |
| ----------- | ----------------------------------- |
| `feat:`     | New feature or behavior             |
| `fix:`      | Bug fix                             |
| `test:`     | Adding or updating tests            |
| `chore:`    | Maintenance, dependency updates     |
| `refactor:` | Code change without behavior change |
| `docs:`     | Documentation only                  |
| `ci:`       | CI/CD changes                       |
| `build:`    | Build system or build-dependency changes |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

Keep commits focused on a single concern. If a change touches both logic and tests, a single commit is fine – if it touches unrelated areas, split it.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- All CI checks must pass before merging.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/shrimp-checker-psp/issues) and include:

- What you did
- What you expected
- What actually happened
- Your environment (OS, emulator or real hardware, and which one)

> For security issues, read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [WTFPL](LICENSE).
