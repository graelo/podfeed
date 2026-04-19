# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.3] - 2026-04-19

### Security

- Harden GitHub Actions workflows: pin third-party actions to commit SHAs,
  scope per-job permissions with least privilege, move secrets from action
  inputs to `env` blocks, and scope release/renovate secrets to dedicated
  GitHub Environments
- Replace long-lived PATs with short-lived GitHub App tokens for release
  automation (Homebrew tap bump, Renovate)
- Add SLSA build provenance attestation to release artifacts
- Skip the cargo cache on pushes to `main` to prevent cache poisoning
- Add zizmor and poutine for workflow and CI/CD supply-chain static analysis,
  extracted into reusable workflows
- Remove cache from release workflow to prevent cache poisoning

### Added

- Unit tests for JSON parsing, date conversion, path rewriting, image
  processing, episode-to-RSS conversion, and XML serialization

### Changed

- Bump MSRV to 1.95
- Linux release binaries are now statically linked against musl
  (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`)
- Switch test runner from `cargo test` to `cargo nextest`
- Switch dependency updates from Dependabot to Renovate (runs daily)
- Replace `ncipollo/release-action` with the pre-installed `gh` CLI
- Release artifacts retention shortened to 1 day
- Drop `cargo-outdated` and `cargo-deny` from the essentials workflow
  (cargo-deny moved to a reusable supply-chain audit workflow)
- Drop `cargo-audit` from CI (covered by `cargo-pants`)
- Remove Windows build target from CI and release workflows
- Bump dependencies

### Fixed

- Image resize existence check now uses `std::fs::exists()` which properly
  propagates I/O errors instead of silently returning `false`

## [0.3.2] - 2025-11-23

### Changed

- Bump MSRV to 1.88
- Switch from `async-std` to `smol`
- Improve `essentials` and `large-scope` CI workflows
- Add Linux aarch64 support to CI
- Bump dependencies

### Fixed

- Remove unused avif support
- Clippy lints

## [0.3.1] - 2025-06-07

### Fixed

- JPEG export: JPEG does not support Rgba, convert before saving

## [0.3.0] - 2025-06-06

### Changed

- Drop `photon-rs`, adopt `image` crate
- Bump to Rust edition 2024
- Bump dependencies

## [0.2.0] - 2025-02-23

### Changed

- Rename project from `podsync` to `podfeed`

## [0.1.3] - 2024-08-16

### Changed

- Bump CI actions versions
- Bump dependencies

## [0.1.2] - 2024-08-14

### Added

- macOS CI builds

### Fixed

- Correct project description in docs

## [0.1.1] - 2024-08-13

### Changed

- Bump MSRV to 1.74
- Bump dependencies

### Fixed

- Clippy lints
- Deactivate convco temporarily

## [0.1.0] - 2023-05-29

### Added

- Save resized images separately

### Changed

- Clean up constants
- Bump dependencies

## [0.0.9] - 2023-05-20

### Fixed

- Add `xmlns:content` namespace to RSS feed

### Changed

- Bump dependencies

## [0.0.8] - 2023-05-19

### Added

- Sort episodes by playlist index

### Changed

- Bump dependencies

## [0.0.7] - 2023-04-02

### Fixed

- Release workflow fix; re-release of 0.0.6 with no functional changes

## [0.0.6] - 2023-04-01

### Changed

- Bump minimum Rust version to 1.64
- Bump dependencies

### Fixed

- Allow old YouTube playlist IDs

## [0.0.5] - 2022-12-21

### Added

- Add padding to artwork images (1400x1400)
- Display number of episodes per channel

### Changed

- Bump dependencies

## [0.0.4] - 2022-12-20

### Fixed

- Properly handle image paths containing dots

## [0.0.3] - 2022-12-19

### Added

- Resize images if needed

### Fixed

- Add timezone to dates in RSS feed

### Changed

- Bump dependencies

## [0.0.2] - 2022-12-18

### Fixed

- Better handling of directories with dots

### Changed

- Bump dependencies

## [0.0.1] - 2022-12-10

Initial public release.

### Added

- Generate RSS feeds from `info.json` files

[Unreleased]: https://github.com/graelo/podfeed/compare/v0.3.3...HEAD
[0.3.3]: https://github.com/graelo/podfeed/compare/v0.3.2...v0.3.3
[0.3.2]: https://github.com/graelo/podfeed/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/graelo/podfeed/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/graelo/podfeed/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/graelo/podfeed/compare/v0.1.3...v0.2.0
[0.1.3]: https://github.com/graelo/podfeed/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/graelo/podfeed/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/graelo/podfeed/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/graelo/podfeed/compare/v0.0.9...v0.1.0
[0.0.9]: https://github.com/graelo/podfeed/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/graelo/podfeed/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/graelo/podfeed/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/graelo/podfeed/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/graelo/podfeed/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/graelo/podfeed/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/graelo/podfeed/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/graelo/podfeed/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/graelo/podfeed/releases/tag/v0.0.1
