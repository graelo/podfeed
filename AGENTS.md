# AGENTS.md

This file contains instructions for coding agents working in this repository.

- Repository: <https://github.com/graelo/podfeed>
- Prefer `gh` for GitHub operations.
- Do not mention an agent or assistant in issues, pull requests, comments, or
  commit messages.
- Do not expose private local information, including machine-specific paths.

## Project

`podfeed` generates podcast RSS feeds from media and `.info.json` files
created by yt-dlp. Its `podfeed` binary provides two subcommands:

- `generate` finds channel directories below a data directory, generates an
  adjacent XML feed for each, and creates 1400×1400 channel and episode
  artwork when necessary.
- `generate-completion` writes a shell completion script to stdout.

Rust 1.95 or later is required. The crate uses edition 2024.

## Architecture

1. `src/bin/main.rs` parses the CLI and invokes feed generation.
2. `src/convert.rs` discovers channel directories, parses their metadata, and
   builds RSS channels and episodes. It also resizes artwork.
3. `src/info/` parses yt-dlp channel and episode `.info.json` files.
4. `src/rss/` defines and serializes the RSS XML model.
5. `src/config.rs` defines the Clap command-line interface and environment
   variables.
6. `src/error.rs` defines the crate error type.

## Verification

The `Makefile` is the canonical definition of local verification tasks. **Read
it before choosing or running verification commands**; do not duplicate its
command implementations here. `make help` lists every target.

The primary targets are:

- `make check`: pre-push gate (formatting, linting, and tests).
- `make check-all`: pre-PR gate (adds dependency, commit-message, Markdown,
  manpage, and GitHub Actions security checks).
- `make fix`: formats code and applies Clippy fixes.
- `make md`: lints Markdown against `rumdl.toml`.
- `make man`: lints `man/podfeed.1`.
- `make ci-security`: runs the Poutine and Zizmor GitHub Actions scans.

The check targets mirror the GitHub workflows and use locked dependency
resolution where applicable. They assume their external tools (for example
`cargo-nextest`, `cargo-deny`, `cargo-pants`, `convco`, `poutine`, `zizmor`,
`rumdl`, `mandoc`, and `cargo-llvm-cov`) are already installed locally.

For focused Rust tests, use `cargo nextest run <test_name>` or
`cargo nextest run <module::tests::name>`. The complete CI test sequence is
implemented in `ci/test_full.sh`; its Nextest CI profile is configured in
`.config/nextest.toml`.

## Documentation and releases

Keep user-facing documentation in sync with behavior:

- Update `README.md` and `man/podfeed.1` when changing the CLI, environment
  variables, generated files, or feed behavior. `README.md` is included as the
  crate documentation, so it is also published on docs.rs.
- Lint Markdown with `make md` and the manpage with `make man`. Update the
  manpage `.TH` version and date for releases.
- For a release version bump, update `Cargo.toml`, `Cargo.lock`, the versioned
  section and comparison links in `CHANGELOG.md`, and the manpage `.TH`
  header. Create a `vX.Y.Z` tag; the release workflow derives artifact and
  GitHub Release versions from it.
- Commit messages must follow `.convco` Conventional Commit rules. Use
  `make commits` to check them.

`Cargo.toml`, `Cargo.lock`, `deny.toml`, and the GitHub workflows define the
release and supply-chain constraints. Preserve `--locked` behavior in Cargo
commands that resolve dependencies.
