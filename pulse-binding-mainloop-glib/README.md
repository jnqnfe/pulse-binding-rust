libpulse-glib-binding
=====================

[<img alt="travis.com" src="https://img.shields.io/travis/com/jnqnfe/pulse-binding-rust?style=for-the-badge" height="24">](https://travis-ci.com/jnqnfe/pulse-binding-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/libpulse-glib-binding?style=for-the-badge" height="24">](https://crates.io/crates/libpulse-glib-binding)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/libpulse-glib-binding?color=5479ab&label=docs.rs&style=for-the-badge" height="24">](https://docs.rs/libpulse-glib-binding)
[<img alt="min-rust-version" src="https://img.shields.io/static/v1?label=RUST&message=1.40%2B&color=informational&style=for-the-badge" height="24">](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

A Rust language binding for the PulseAudio libpulse-mainloop-glib library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse-binding = { version = "2.0", features = "" }
libpulse-glib-binding = { version = "2.0", features = "" }
```

and this to your crate root:

```rust
extern crate libpulse_binding as pulse;
extern crate libpulse_glib_binding as pulse_glib;
```

Finally, fill in the `features` attribute of the dependencies added above with the right
compatibility flags (listed within the respective crates’ toml files) to disable functionality for
versions of the PulseAudio client library that are too new for you. (This compatibility layer
targets the entire PA client C API, and you will naturally encounter problems where mismatching
version support with the version of PA on your systems). See the overall project `COMPATIBILITY.md`
file for further details.

## License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
