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

//! PulseAudio Rust language linking library for the ‘GLIB mainloop’ component.

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

#![allow(non_camel_case_types, non_snake_case)]

extern crate libpulse_sys as pulse;

//TODO: link this to a Glib crate object
/// According to Glib headers, this is an opaque type!
pub enum GMainContext {}

/// An opaque GLIB main loop object.
#[repr(C)] pub struct pa_glib_mainloop { _private: [u8; 0] }

#[link(name="pulse-mainloop-glib")]
extern "C" {
    pub fn pa_glib_mainloop_new(c: *mut GMainContext) -> *mut pa_glib_mainloop;
    pub fn pa_glib_mainloop_free(g: *mut pa_glib_mainloop);
    pub fn pa_glib_mainloop_get_api(g: *const pa_glib_mainloop) -> *const ::pulse::mainloop::api::pa_mainloop_api;
}
