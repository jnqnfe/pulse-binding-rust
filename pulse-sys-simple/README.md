libpulse_simple_sys
===================

[<img alt="travis.com" src="https://img.shields.io/travis/com/jnqnfe/pulse-binding-rust?style=for-the-badge" height="24">](https://travis-ci.com/jnqnfe/pulse-binding-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/libpulse-simple-sys?style=for-the-badge" height="24">](https://crates.io/crates/libpulse-simple-sys)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/libpulse-simple-sys?color=5479ab&label=docs.rs&style=for-the-badge" height="24">](https://docs.rs/libpulse-simple-sys)
[<img alt="min-rust-version" src="https://img.shields.io/static/v1?label=RUST&message=1.40%2B&color=informational&style=for-the-badge" height="24">](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

A Rust language linking library for the PulseAudio libpulse-simple library. See also the
higher-level `libpulse_simple_binding` crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse-sys = { version = "1.0", features = "" }
libpulse-simple-sys = { version = "1.0", features = "" }
```

and this to your crate root:

```rust
extern crate libpulse_sys as pulse;
extern crate libpulse_simple_sys as psimple;
```

Finally, fill in the `features` attribute of the dependency added above with the right compatibility
flag (listed within this crates’ toml file) to disable functionality for versions of the PulseAudio
client library that are too new for you. (This crate targets the entire PA client C API, and you
will naturally encounter problems where mismatching version support with the version of PA on your
systems). See the overall project `COMPATIBILITY.md` file for further details.
