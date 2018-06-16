libpulse_simple_binding
=======================

A Rust language binding for the PulseAudio libpulse-simple library.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpulse_binding = "2.0"
libpulse_simple_binding = "2.0"
```

and this to your crate root:

```rust
extern crate libpulse_binding as pulse;
extern crate libpulse_simple_binding as psimple;
```
