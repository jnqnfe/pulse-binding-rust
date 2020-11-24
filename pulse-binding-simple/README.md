libpulse_simple_binding
=======================

[<img alt="travis.com" src="https://img.shields.io/travis/com/jnqnfe/pulse-binding-rust?style=for-the-badge" height="24">](https://travis-ci.com/jnqnfe/pulse-binding-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/libpulse-simple-binding?style=for-the-badge" height="24">](https://crates.io/crates/libpulse-simple-binding)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/libpulse-simple-binding?color=5479ab&label=docs.rs&style=for-the-badge" height="24">](https://docs.rs/libpulse-simple-binding)
<img alt="min-rust-version" src="https://img.shields.io/static/v1?label=RUST&message=1.40%2B&color=informational&style=for-the-badge" height="24">
<img alt="license" src="https://img.shields.io/crates/l/libpulse-simple-binding.svg?style=for-the-badge" height="24">

A Rust language binding for the PulseAudio libpulse-simple library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse-binding = { version = "2.0", features = "" }
libpulse-simple-binding = { version = "2.0", features = "" }
```

and this to your crate root:

```rust
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
```

Finally, fill in the `features` attribute of the dependencies added above with the right
compatibility flags (listed within the respective crates’ toml files) to disable functionality for
versions of the PulseAudio client library that are too new for you. (This compatibility layer
targets the entire PA client C API, and you will naturally encounter problems where mismatching
version support with the version of PA on your systems). See the overall project `COMPATIBILITY.md`
file for further details.
