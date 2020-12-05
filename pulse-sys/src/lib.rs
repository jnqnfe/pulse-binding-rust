// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.
//
// Portions of documentation are copied from the LGPL 2.1+ licensed PulseAudio C headers on a
// fair-use basis, as discussed in the overall project readme (available in the git repository).

//! PulseAudio Rust language linking library.
//!
//! This crate is a *sys* type crate targetting the PulseAudio C API. As a *sys* type crate it does
//! nothing more than simply describe the C API in Rust form. Please be aware that there is a
//! “higher level” *binding* crate available ([`libpulse-binding`]) built on top of this, which you
//! will most likely prefer to use instead.
//!
//! Virtually no documentation is provided here, since it is pointless to duplicate it here from the
//! C header files, considering that most users will be using the binding crate (which is heavily
//! documented).
//!
//! [`libpulse-binding`]: https://docs.rs/libpulse-binding

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

#[cfg_attr(docsrs, feature(doc_cfg))]

pub mod channelmap;
pub mod context;
pub mod def;
pub mod direction;
pub mod error;
pub mod format;
pub mod mainloop;
pub mod operation;
pub mod proplist;
pub mod rtclock;
pub mod sample;
pub mod stream;
pub mod timeval;
pub mod utf8;
pub mod util;
pub mod version;
pub mod volume;
pub mod xmalloc;

// Re-export
pub use self::channelmap::*;
pub use self::context::*;
pub use self::def::*;
pub use self::direction::*;
pub use self::error::*;
pub use self::format::*;
pub use self::mainloop::*;
pub use self::operation::*;
pub use self::proplist::*;
pub use self::rtclock::*;
pub use self::sample::*;
pub use self::stream::*;
pub use self::timeval::*;
pub use self::utf8::*;
pub use self::util::*;
pub use self::version::*;
pub use self::volume::*;
pub use self::xmalloc::*;
