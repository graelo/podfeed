# `podfeed`

[![crate](https://img.shields.io/crates/v/podfeed.svg)](https://crates.io/crates/podfeed)
[![documentation](https://docs.rs/podfeed/badge.svg)](https://docs.rs/podfeed)
[![minimum rustc 1.95](https://img.shields.io/badge/rustc-1.95-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![rust 2024 edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![build status](https://github.com/graelo/podfeed/actions/workflows/ci-essentials.yml/badge.svg)](https://github.com/graelo/podfeed/actions)

## Name

**podfeed** — generate podcast RSS feeds from yt-dlp metadata

## Synopsis

```sh
podfeed generate --data-dir PATH --base-url URL
podfeed generate-completion SHELL
```

## Description

`podfeed` generates podcast RSS feeds from media files and `.info.json` files
created by [yt-dlp]. It finds each channel directory below `--data-dir`, writes
an XML feed adjacent to that directory, and creates square 1400×1400 artwork
for channels and episodes when needed.

`--base-url` is the public URL corresponding to `--data-dir`; it is used for
media and artwork URLs in the generated feeds.

## Getting Started

Install the latest release with Cargo:

```sh
cargo install podfeed
```

Generate feeds for a yt-dlp data directory:

```sh
podfeed generate \
  --data-dir ./data \
  --base-url https://podcasts.example.com
```

Set `DATADIR` and `BASEURL` instead of passing the corresponding options:

```sh
DATADIR=./data BASEURL=https://podcasts.example.com podfeed generate
```

## Shell Completions

Generate a completion script for Bash, Elvish, Fish, PowerShell, or Zsh. For
example, install Bash completions with:

```sh
podfeed generate-completion bash >"$(brew --prefix)/etc/bash_completion.d/podfeed"
```

## Development

The [`Makefile`](Makefile) is the canonical definition of local verification
tasks. Run `make help` to list them, `make check` before pushing, and
`make check-all` before opening a pull request. `make man` validates
[`man/podfeed.1`](man/podfeed.1), which documents the command-line interface.

## Caveats

- This is an alpha version.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[yt-dlp]: https://github.com/yt-dlp/yt-dlp
