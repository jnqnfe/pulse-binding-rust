//! UNIX signal support for main loops.

// This file is part of the PulseAudio Rust language linking library.
//
// Copyright (c) 2017 Lyndon Brown
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

use std::os::raw::c_void;

/// An opaque UNIX signal event source object
pub enum pa_signal_event {}

pub type pa_signal_cb_t = Option<extern "C" fn(api: *const ::mainloop::api::pa_mainloop_api, e: *mut pa_signal_event, sig: i32, userdata: *mut c_void)>;

pub type pa_signal_destroy_cb_t = Option<extern "C" fn(api: *const ::mainloop::api::pa_mainloop_api, e: *mut pa_signal_event, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_signal_init(api: *const ::mainloop::api::pa_mainloop_api) -> i32;
    pub fn pa_signal_done();
    pub fn pa_signal_new(sig: i32, callback: pa_signal_cb_t, userdata: *mut c_void) -> *mut pa_signal_event;
    pub fn pa_signal_free(e: *mut pa_signal_event);
    pub fn pa_signal_set_destroy(e: *mut pa_signal_event, callback: pa_signal_destroy_cb_t);
}
