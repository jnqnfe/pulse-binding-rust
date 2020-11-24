libpulse_mainloop_glib_sys
==========================

A Rust language linking library for the PulseAudio libpulse-mainloop-glib library. See also the
higher-level `libpulse_glib_binding` crate.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse-sys = { version = "1.0", features = "" }
libpulse-mainloop-glib-sys = { version = "1.0", features = "" }
```

and this to your crate root:

```rust
extern crate libpulse_sys as pulse;
extern crate libpulse_mainloop_glib_sys as pulse_glib;
```

Finally, fill in the `features` attribute of the dependency added above with the right compatibility
flag (listed within this crates’ toml file) to disable functionality for versions of the PulseAudio
client library that are too new for you. (This crate targets the entire PA client C API, and you
will naturally encounter problems where mismatching version support with the version of PA on your
systems). See the overall project `COMPATIBILITY.md` file for further details.
