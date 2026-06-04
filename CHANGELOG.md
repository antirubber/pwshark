# Changelog

All notable changes to pwshark are documented here.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-06-04

### Added
- Prebuilt static binaries (`amd64`, `arm64`) published to GitHub Releases on
  every `v*` tag, so installing no longer requires a Rust toolchain.
- `pwshark update` is now version-aware: re-running the installer is a fast no-op
  when you already have the latest release.

### Changed
- `install.sh` downloads the prebuilt binary and verifies its SHA256, falling
  back to a source build only on an unsupported arch or a failed download.

### Fixed
- Installer no longer re-runs rustup on every update. It now sources
  `~/.cargo/env` before probing for `cargo`, so an existing Rust install is
  detected even when the user's shell (fish/zsh) doesn't put `~/.cargo/bin` on
  PATH.

## [0.1.0] - 2026-06-04

### Added
- Initial release: NIST-compliant random and memorable password generator with a
  ratatui TUI, `--stdout` mode, clipboard copy, entropy meter, and embedded
  wordlist.

[0.1.1]: https://github.com/antirubber/pwshark/releases/tag/v0.1.1
[0.1.0]: https://github.com/antirubber/pwshark/releases/tag/v0.1.0
