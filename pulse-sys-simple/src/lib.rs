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

//! PulseAudio Rust language linking library for the ‘simple’ component

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

extern crate libpulse_sys as pulse;

use std::os::raw::{c_char, c_void};

/// An opaque simple connection object
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
