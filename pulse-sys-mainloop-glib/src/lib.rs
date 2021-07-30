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

//! PulseAudio FFI binding for the `libpulse-mainloop-glib` system library.
//!
//! This crate does nothing more than offer a simple FFI binding to the C API of the [PulseAudio]
//! client system library, specifically the GLIB mainloop component only. Please note that there is
//! a “higher-level” binding available (the [`libpulse-glib-binding`] crate), built on top of this,
//! which offers a more Rust-oriented interface.
//!
//! Unlike the “higher-level” binding just mentioned, virtually no documentation is provided here.
//! Things that *are* documented here are typically only those directly re-exported by the
//! “higher-level” binding. Please see either the equivalent documentation in that, or the
//! documentation of the actual PulseAudio C header files, if you need documentation.
//!
//! [`libpulse-glib-binding`]: https://docs.rs/libpulse-glib-binding
//! [PulseAudio]: https://en.wikipedia.org/wiki/PulseAudio

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.svg",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate libpulse_sys as pulse;
extern crate glib_sys as glib;

use glib::GMainContext;
use pulse::mainloop::api::pa_mainloop_api;

/// An opaque GLIB main loop object.
#[repr(C)] pub struct pa_glib_mainloop { _private: [u8; 0] }

#[rustfmt::skip]
#[link(name="pulse-mainloop-glib")]
extern "C" {
    pub fn pa_glib_mainloop_new(c: *mut GMainContext) -> *mut pa_glib_mainloop;
    pub fn pa_glib_mainloop_free(g: *mut pa_glib_mainloop);
    pub fn pa_glib_mainloop_get_api(g: *const pa_glib_mainloop) -> *const pa_mainloop_api;
}
