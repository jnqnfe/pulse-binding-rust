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

//! PulseAudio Rust language linking library for the ‘simple’ component.
//!
//! This crate is a *sys* type crate targetting the PulseAudio (“simple” interface)  C API. As a
//! *sys* type crate it does nothing more than simply describe the C API in Rust form. Please be
//! aware that there is a “higher level” *binding* crate available ([`libpulse-simple-binding`])
//! built on top of this, which you will most likely prefer to use instead.
//!
//! Virtually no documentation is provided here, since it is pointless to duplicate it here from the
//! C header files, considering that most users will be using the binding crate (which is heavily
//! documented).
//!
//! [`libpulse-simple-binding`]: https://docs.rs/libpulse-simple-binding

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

#[cfg_attr(docsrs, feature(doc_cfg))]

extern crate libpulse_sys as pulse;

use std::os::raw::{c_char, c_void};

/// An opaque simple connection object.
#[repr(C)] pub struct pa_simple { _private: [u8; 0] }

#[link(name="pulse-simple")]
extern "C" {
    pub fn pa_simple_new(server: *const c_char, name: *const c_char, dir: pulse::stream::pa_stream_direction_t, dev: *const c_char, stream_name: *const c_char, ss: *const pulse::sample::pa_sample_spec, map: *const pulse::channelmap::pa_channel_map, attr: *const pulse::def::pa_buffer_attr, error: *mut i32) -> *mut pa_simple;
    pub fn pa_simple_free(s: *mut pa_simple);
    pub fn pa_simple_write(s: *mut pa_simple, data: *const c_void, bytes: usize, error: *mut i32) -> i32;
    pub fn pa_simple_drain(s: *mut pa_simple, error: *mut i32) -> i32;
    pub fn pa_simple_read(s: *mut pa_simple, data: *mut c_void, bytes: usize, error: *mut i32) -> i32;
    pub fn pa_simple_get_latency(s: *mut pa_simple, error: *mut i32) -> pulse::sample::pa_usec_t;
    pub fn pa_simple_flush(s: *mut pa_simple, error: *mut i32) -> i32;
}
