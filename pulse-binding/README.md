libpulse_binding
================

A Rust language binding for the PulseAudio libpulse library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse-binding = { version = "2.0", features = "" }
```

and this to your crate root:

```rust
extern crate libpulse_binding as pulse;
```

Finally, fill in the `features` attribute of the dependency added above with the right compatibility
flag (listed within this crates’ toml file) to disable functionality for versions of the PulseAudio
client library that are too new for you. (This compatibility layer targets the entire PA client C
API, and you will naturally encounter problems where mismatching version support with the version
of PA on your systems). See the overall project `COMPATIBILITY.md` file for further details.

## Licensing

This crate is primarily licensed under LGPL 2.1+ (per PulseAudio itself). Alternate MIT/Apache-2.0
licensing is available under certain circumstances. See the main project `README.md` file for
further details.
