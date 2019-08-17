libpulse_glib_binding
=====================

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
