// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
//
// This library is free software; you can redistribute it and/or modify it under the terms of the
// GNU Lesser General Public License as published by the Free Software Foundation; either version
// 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with this library;
// if not, see <http://www.gnu.org/licenses/>.

//! PulseAudio Rust language linking library.
//!
//! This crate is a *sys* type crate targetting the PulseAudio C API. As a *sys* type crate it does
//! nothing more than simply describe the C API in Rust form. Please be aware that there is a
//! “higher level” *binding* crate available (`libpulse-binding`) built on top of this, which you
//! will most likely prefer to use instead.
//!
//! Virtually no documentation is provided here, since it is pointless to duplicate it here from the
//! C header files, considering that most users will be using the binding crate (which is heavily
//! documented).

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

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
