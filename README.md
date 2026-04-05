<h1 align="center">shrimp-checker-psp</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&labelColor=2d333b&color=3fb950" alt="Latest Release"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-WTFPL-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-WTFPL-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-WTFPL-3fb950?style=flat-square&labelColor=2d333b" alt="License: WTFPL"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/PSP-003087?style=flat-square&logo=playstation&logoColor=white" alt="PSP"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/shrimp-checker-psp/main/assets/logo.png" alt="shrimp-checker-psp PSP homebrew logo" width="200" />
</p>

<p align="center">🇬🇧 English | 🇵🇱 <a href="README.pl.md">Polski</a></p>

The world's most advanced shrimp detection software — now on Sony PSP.

[![shrimp-checker-psp trailer](https://img.youtube.com/vi/wL6nVgfnFyc/maxresdefault.jpg)](https://youtu.be/wL6nVgfnFyc?si=YG_1AoTWyzysv6mC)

## What is this?

**shrimp-checker-psp** is a PSP homebrew application that answers the one question humanity has always needed answered: _are you a shrimp?_

A port of [shrimp-checker](https://github.com/wielorzeczownik/shrimp-checker) to the Sony PlayStation Portable. Written in Rust using [cargo-psp](https://github.com/overdrivenpotato/rust-psp), it runs on real PSP hardware and in the [PPSSPP](https://www.ppsspp.org/) emulator.

## Running on PSP

Each release includes a prebuilt `EBOOT.PBP` inside a zip archive.  
Latest release: [GitHub Releases](https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest)

1. Download [shrimp-checker-psp.zip](https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest/download/shrimp-checker-psp.zip).
2. Extract it — you'll get `EBOOT.PBP`.
3. Copy it to your PSP's memory stick under `PSP/GAME/shrimp-checker-psp/EBOOT.PBP`.
4. Launch from the PSP's Game menu.

Works on real hardware and [PPSSPP](https://www.ppsspp.org/).

## Building from source

### With Docker (recommended)

```bash
make docker-build
```

The output `EBOOT.PBP` ends up in `target/mipsel-sony-psp/release/`.

### Without Docker

Requires Rust nightly and `cargo-psp`:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install cargo-psp
cargo psp --release
```
