# `podfeed`

[![crate](https://img.shields.io/crates/v/podfeed.svg)](https://crates.io/crates/podfeed)
[![documentation](https://docs.rs/podfeed/badge.svg)](https://docs.rs/podfeed)
[![minimum rustc 1.95](https://img.shields.io/badge/rustc-1.95-red.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![rust 2024 edition](https://img.shields.io/badge/edition-2024-blue.svg)](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
[![build status](https://github.com/graelo/podfeed/actions/workflows/ci-essentials.yml/badge.svg)](https://github.com/graelo/podfeed/actions)

<!-- cargo-sync-readme start -->

Generates podcast RSS feeds from media and `.info.json` files created by
yt-dlp.

Version requirement: _rustc 1.95+_

## Features

- Generate RSS feeds for podcast players
- Resize channel and episode artwork
- Generate shell completion scripts

## Getting started

Generate feeds for a yt-dlp data directory:

```sh
podfeed generate \
  --data-dir ./data \
  --base-url https://podcasts.example.com
```

Generate shell completions with `podfeed generate-completion <shell>`.

## Caveats

- This is an alpha version

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

<!-- cargo-sync-readme end -->
