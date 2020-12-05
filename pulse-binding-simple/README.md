libpulse-simple-binding
=======================

[<img alt="travis.com" src="https://img.shields.io/travis/com/jnqnfe/pulse-binding-rust?style=for-the-badge" height="24">](https://travis-ci.com/jnqnfe/pulse-binding-rust)
[<img alt="crates.io" src="https://img.shields.io/crates/v/libpulse-simple-binding?style=for-the-badge" height="24">](https://crates.io/crates/libpulse-simple-binding)
[<img alt="docs.rs" src="https://img.shields.io/crates/v/libpulse-simple-binding?color=5479ab&label=docs.rs&style=for-the-badge" height="24">](https://docs.rs/libpulse-simple-binding)
[<img alt="min-rust-version" src="https://img.shields.io/static/v1?label=RUST&message=1.41%2B&color=informational&style=for-the-badge" height="24">](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)

A Rust language binding for the PulseAudio libpulse-simple library.

## Usage

Add the following two crates to the dependencies specified in your `Cargo.toml` (you will likely
need to use components from the main binding crate in addition to this crate itself):

```toml
[dependencies]
libpulse-binding = "2.0"
libpulse-simple-binding = "2.0"
```

Though you may wish to rename the crates to shorter names (for example `pulse` and `psimple`) for
cleaner references within your code:

```toml
[dependencies]
pulse = { version = "2.0", package = "libpulse-binding" }
psimple = { version = "2.0", package = "libpulse-simple-binding" }
```

An alternative to that which some may prefer is:

```toml
[dependencies.pulse]
version = "2.0"
package = "libpulse-binding"

[dependencies.psimple]
version = "2.0"
package = "libpulse-simple-binding"
```

### PulseAudio version compatibility

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
