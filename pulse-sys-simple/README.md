libpulse_simple_sys
===================

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
