<h1 align="center">shrimp-checker-psp</h1>

<p align="center">
  <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/actions/workflows/release.yml"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/actions/workflow/status/wielorzeczownik/shrimp-checker-psp/release.yml?branch=main&style=flat-square&labelColor=2d333b&color=3fb950" alt="release"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&labelColor=2d333b&color=3fb950"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&color=2ea043"/><img src="https://img.shields.io/github/v/release/wielorzeczownik/shrimp-checker-psp?style=flat-square&labelColor=2d333b&color=3fb950" alt="Najnowsze wydanie"/></picture></a> <a href="https://github.com/wielorzeczownik/shrimp-checker-psp/blob/main/LICENSE"><picture><source media="(prefers-color-scheme: dark)" srcset="https://img.shields.io/badge/License-WTFPL-3fb950?style=flat-square&labelColor=2d333b"/><source media="(prefers-color-scheme: light)" srcset="https://img.shields.io/badge/License-WTFPL-2ea043?style=flat-square"/><img src="https://img.shields.io/badge/License-WTFPL-3fb950?style=flat-square&labelColor=2d333b" alt="Licencja: WTFPL"/></picture></a>
  <br/>
  <img src="https://img.shields.io/badge/Rust-B7410E?style=flat-square&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/PSP-003087?style=flat-square&logo=playstation&logoColor=white" alt="PSP"/>
</p>

<p align="center">
  <img src="https://raw.githubusercontent.com/wielorzeczownik/shrimp-checker-psp/main/assets/logo.png" alt="shrimp-checker-psp PSP homebrew logo" width="200" />
</p>

<p align="center">🇬🇧 <a href="README.md">English</a> | 🇵🇱 Polski</p>

Najbardziej zaawansowane oprogramowanie do wykrywania krewetek na świecie — teraz na Sony PSP.

[![shrimp-checker-psp trailer](https://img.youtube.com/vi/wL6nVgfnFyc/maxresdefault.jpg)](https://youtu.be/wL6nVgfnFyc?si=YG_1AoTWyzysv6mC)

## Co to jest?

**shrimp-checker-psp** to aplikacja homebrew na PSP, która odpowiada na jedno pytanie, które ludzkość zawsze chciała zadać: _czy jesteś krewetką?_

Port [shrimp-checker](https://github.com/wielorzeczownik/shrimp-checker) na Sony PlayStation Portable. Napisany w Rust przy użyciu [cargo-psp](https://github.com/overdrivenpotato/rust-psp), działa na prawdziwym sprzęcie PSP oraz w emulatorze [PPSSPP](https://www.ppsspp.org/).

## Uruchomienie na PSP

Każde wydanie zawiera gotowy plik `EBOOT.PBP` w archiwum zip.  
Najnowsze wydanie: [GitHub Releases](https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest)

1. Pobierz [shrimp-checker-psp.zip](https://github.com/wielorzeczownik/shrimp-checker-psp/releases/latest/download/shrimp-checker-psp.zip).
2. Wypakuj archiwum — otrzymasz plik `EBOOT.PBP`.
3. Skopiuj go na kartę pamięci PSP do folderu `PSP/GAME/shrimp-checker-psp/EBOOT.PBP`.
4. Uruchom z menu Game na PSP.

Działa na prawdziwym sprzęcie oraz w emulatorze [PPSSPP](https://www.ppsspp.org/).

## Budowanie ze źródeł

### Z Dockerem (zalecane)

```bash
make docker-build
```

Wynikowy plik `EBOOT.PBP` znajdziesz w `target/mipsel-sony-psp/release/`.

### Bez Dockera

Wymagany Rust nightly oraz `cargo-psp`:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo install cargo-psp
cargo psp --release
```
