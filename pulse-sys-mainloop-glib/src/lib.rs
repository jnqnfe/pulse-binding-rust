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

//! PulseAudio Rust language linking library for the ‘GLIB mainloop’ component.
//!
//! This crate is a *sys* type crate targetting the PulseAudio (GLIB mainloop extension) C API. As a
//! *sys* type crate it does nothing more than simply describe the C API in Rust form. Please be
//! aware that there is a “higher level” *binding* crate available (`libpulse-glib-binding`) built
//! on top of this, which you will most likely prefer to use instead.
//!
//! Virtually no documentation is provided here, since it is pointless to duplicate it here from the
//! C header files, considering that most users will be using the binding crate (which is heavily
//! documented).

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

extern crate libpulse_sys as pulse;
extern crate glib_sys as glib;

use glib::GMainContext;
use pulse::mainloop::api::pa_mainloop_api;

/// An opaque GLIB main loop object.
#[repr(C)] pub struct pa_glib_mainloop { _private: [u8; 0] }

#[link(name="pulse-mainloop-glib")]
extern "C" {
    pub fn pa_glib_mainloop_new(c: *mut GMainContext) -> *mut pa_glib_mainloop;
    pub fn pa_glib_mainloop_free(g: *mut pa_glib_mainloop);
    pub fn pa_glib_mainloop_get_api(g: *const pa_glib_mainloop) -> *const pa_mainloop_api;
}
