libpulse_glib_binding
=====================

A Rust language binding for the PulseAudio libpulse-mainloop-glib library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse_binding = "2.0"
libpulse_glib_binding = "2.0"
```

and this to your crate root:

```rust
extern crate libpulse_binding as pulse;
extern crate libpulse_glib_binding as pulse_glib;
```
