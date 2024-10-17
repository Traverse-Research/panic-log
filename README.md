# rust-template title

[![Actions Status](https://github.com/Traverse-Research/rust-template/actions/workflows/ci.yml/badge.svg)](https://github.com/Traverse-Research/rust-template/actions)
[![Latest version](https://img.shields.io/crates/v/rust-template.svg?logo=rust)](https://crates.io/crates/rust-template)
[![Documentation](https://docs.rs/rust-template/badge.svg)](https://docs.rs/rust-template)
[![MSRV](https://img.shields.io/badge/rustc-1.74.0+-ab6000.svg)](https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html)
[![Lines of code](https://tokei.rs/b1/github/Traverse-Research/rust-template)](https://github.com/Traverse-Research/rust-template)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4%20adopted-ff69b4.svg)](./CODE_OF_CONDUCT.md)

[![Banner](banner.png)](https://traverseresearch.nl)

A simple crate that allows you to write the panic message and backtrace to the output of the `log` macro as `error`, while providing
the possibility to keep the original panic hooks.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
panic-log = "0.1.0"
```

Call this somewhere at the start of your program (after initializing your logger):

```rust
    initialize_hook(Configuration::default());
```
