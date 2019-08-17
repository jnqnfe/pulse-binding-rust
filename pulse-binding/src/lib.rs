// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! PulseAudio Rust language binding library.
//!
//! # About
//!
//! This library is a binding that allows Rust code to connect to the PulseAudio sound server via
//! PulseAudio’s existing C API. This binding provides a safe(r) Rust interface which might be
//! preferred over the raw C API provided by the underlying `sys` linking crate.
//!
//! The documentation below and throughout this crate have been largely lifted from the C API header
//! files.
//!
//! # Introduction
//!
//! The PulseAudio API comes in two flavours to accommodate different styles of applications and
//! different needs in complexity:
//!
//! * The complete but somewhat complicated to use asynchronous API.
//! * The simplified, easy to use, but limited synchronous API.
//!
//! All strings in PulseAudio are in the UTF-8 encoding, regardless of current locale. Some
//! functions will filter invalid sequences from the string, some will simply fail. To ensure
//! reliable behaviour, make sure everything you pass to the API is valid UTF-8.
//!
//! ## Simple API
//!
//! Use this if you develop your program in synchronous style and just need a way to play or record
//! data on the sound server. This functionality is kept in the separate `libpulse_simple_binding`
//! crate. See that for details.
//!
//! ## Asynchronous API
//!
//! Use this if you develop your programs in asynchronous, event loop based style or if you want to
//! use the advanced features of the PulseAudio API. A guide can be found in the [`mainloop`]
//! module.
//!
//! By using the built-in threaded main loop, it is possible to achieve a pseudo-synchronous API,
//! which can be useful in synchronous applications where the simple API is insufficient.
//!
//! ## Threads
//!
//! The PulseAudio client libraries are not designed to be directly thread-safe. They are however
//! designed to be re-entrant and thread-aware.
//!
//! To use the libraries in a threaded environment, you must assure that all objects are only used
//! in one thread at a time. Normally, this means that all objects belonging to a single context
//! must be accessed from the same thread.
//!
//! The included main loop implementation is also not thread safe. Take care to make sure event
//! objects are not manipulated when any other code is using the main loop.
//!
//! ## Logging
//!
//! You can configure different logging parameters for the PulseAudio client libraries. The
//! following environment variables are recognized:
//!
//! * `PULSE_LOG`: Maximum log level required. Bigger values result in a more verbose logging
//!   output. The following values are recognized:
//!   * `0`: Error messages
//!   * `1`: Warning messages
//!   * `2`: Notice messages
//!   * `3`: Info messages
//!   * `4`: Debug messages
//! * `PULSE_LOG_SYSLOG`: If defined, force all client libraries to log their output using the
//!   `syslog(3)` mechanism. Default behavior is to log all output to `stderr`.
//! * `PULSE_LOG_JOURNAL`: If defined, force all client libraries to log their output using the
//!   systemd journal. If both `PULSE_LOG_JOURNAL` and `PULSE_LOG_SYSLOG` are defined, logging to
//!   the systemd journal takes a higher precedence. Each message originating library file name and
//!   function are included by default through the journal fields `CODE_FILE`, `CODE_FUNC`, and
//!   `CODE_LINE`. Any backtrace attached to the logging message is sent through the
//!   PulseAudio-specific journal field `PULSE_BACKTRACE`. This environment variable has no effect
//!   if PulseAudio was compiled without systemd journal support.
//! * `PULSE_LOG_COLORS`: If defined, enables colored logging output.
//! * `PULSE_LOG_TIME`: If defined, include timestamps with each message.
//! * `PULSE_LOG_FILE`: If defined, include each message originating file name.
//! * `PULSE_LOG_META`: If defined, include each message originating file name and path relative to
//!   the PulseAudio source tree root.
//! * `PULSE_LOG_LEVEL`: If defined, include a log level prefix with each message. Respectively, the
//!   prefixes "E", "W", "N", "I", "D" stands for
//!   Error, Warning, Notice, Info, and Debugging.
//! * `PULSE_LOG_BACKTRACE`: Number of functions to display in the backtrace. If this variable is
//!   not defined, or has a value of zero, no backtrace is shown.
//! * `PULSE_LOG_BACKTRACE_SKIP`: Number of backtrace levels to skip, from the function printing the
//!   log message downwards.
//! * `PULSE_LOG_NO_RATE_LIMIT`: If defined, do not rate limit the logging output. Rate limiting
//!   skips certain log messages when their frequency is considered too high.
//!
//! # Usage
//!
//! Firstly, add a dependency on the crate in your program’s `Cargo.toml` file. Secondly, import the
//! crate to the root of your program:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//! ```
//!
//! See sub-modules for further information.
//!
//! [`mainloop`]: mainloop/index.html

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![deny(bare_trait_objects)]

extern crate libc;
extern crate libpulse_sys as capi;

pub mod callbacks;
pub mod channelmap;
pub mod context;
pub mod def;
pub mod direction;
pub mod error;
pub mod format;
pub mod mainloop;
pub mod operation;
pub mod proplist;
pub mod sample;
pub mod stream;
pub mod time;
pub mod utf8;
pub mod util;
pub mod version;
pub mod volume;
