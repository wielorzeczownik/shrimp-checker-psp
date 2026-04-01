# Contributing to shrimp-checker-psp

Thank you for considering a contribution. This document describes how to get started.

## Prerequisites

- [Rust](https://rustup.rs/) nightly toolchain with `rust-src` component
- [`cargo-psp`](https://github.com/overdrivenpotato/rust-psp)
- Docker (optional, for the containerized build)

## Development setup

```bash
git clone https://github.com/wielorzeczownik/shrimp-checker-psp.git
cd shrimp-checker-psp
cargo psp --release
```

Or with Docker:

```bash
make docker-build
```

## Before submitting a PR

Make sure these pass locally:

```bash
cargo fmt --check
cargo psp --release
```

## Commit style

This project uses [Conventional Commits](https://www.conventionalcommits.org/). Commit messages drive automatic changelog generation and version bumping.

Common prefixes:

| Prefix | When to use |
|--------|-------------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `chore:` | Maintenance, dependency updates |
| `refactor:` | Code change without behavior change |
| `docs:` | Documentation only |
| `style:` | Formatting, no logic change |
| `ci:` | CI/CD changes |
| `build:` | Build system or build-dependency changes |

Breaking changes must include `BREAKING CHANGE:` in the commit footer.

## Pull requests

- Keep PRs focused on a single concern.
- Reference any related issue in the PR description.
- The Validate workflow must pass.

## Reporting bugs

Open an [issue](https://github.com/wielorzeczownik/shrimp-checker-psp/issues) and include:
- What you did
- What you expected
- What actually happened
- Whether you're running on real hardware or an emulator (and which one)

> For security issues, please read [SECURITY.md](SECURITY.md) before opening a public issue.

## License

By contributing you agree that your changes will be licensed under the [WTFPL](LICENSE).
