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

//! Main loop abstraction layer API.

use std::os::raw::c_void;
use libc::timeval;

pub type pa_io_event_flags_t = u32;

pub use self::io_event_flags::*;

pub mod io_event_flags {
    use super::pa_io_event_flags_t;

    pub const PA_IO_EVENT_NULL: pa_io_event_flags_t = 0;
    pub const PA_IO_EVENT_INPUT: pa_io_event_flags_t = 1;
    pub const PA_IO_EVENT_OUTPUT: pa_io_event_flags_t = 2;
    pub const PA_IO_EVENT_HANGUP: pa_io_event_flags_t = 4;
    pub const PA_IO_EVENT_ERROR: pa_io_event_flags_t = 8;
}

/// An opaque IO event source object
pub enum pa_io_event {}
pub type pa_io_event_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_io_event, fd: i32, events: pa_io_event_flags_t, userdata: *mut c_void)>;
pub type pa_io_event_destroy_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_io_event, userdata: *mut c_void)>;

/// An opaque timer event source object
pub enum pa_time_event {}
pub type pa_time_event_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_time_event, tv: *const timeval, userdata: *mut c_void)>;
pub type pa_time_event_destroy_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_time_event, userdata: *mut c_void)>;

/// An opaque deferred event source object.
/// Events of this type are triggered once in every main loop iteration
pub enum pa_defer_event {}
pub type pa_defer_event_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void)>;
pub type pa_defer_event_destroy_cb_t = Option<extern "C" fn(a: *const pa_mainloop_api, e: *mut pa_defer_event, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_mainloop_api {
    pub userdata: *mut c_void,

    pub io_new: Option<extern "C" fn(a: *const pa_mainloop_api, fd: i32, events: pa_io_event_flags_t, cb: pa_io_event_cb_t, userdata: *mut c_void) -> *mut pa_io_event>,
    pub io_enable: Option<extern "C" fn(e: *mut pa_io_event, events: pa_io_event_flags_t)>,
    pub io_free: Option<extern "C" fn(e: *mut pa_io_event)>,
    pub io_set_destroy: Option<extern "C" fn(e: *mut pa_io_event, cb: pa_io_event_destroy_cb_t)>,

    pub time_new: Option<extern "C" fn(a: *const pa_mainloop_api, tv: *const timeval, cb: pa_time_event_cb_t, userdata: *mut c_void) -> *mut pa_time_event>,
    pub time_restart: Option<extern "C" fn(e: *mut pa_time_event, tv: *const timeval)>,
    pub time_free: Option<extern "C" fn(e: *mut pa_time_event)>,
    pub time_set_destroy: Option<extern "C" fn(e: *mut pa_time_event, cb: pa_time_event_destroy_cb_t)>,

    pub defer_new: Option<extern "C" fn(a: *const pa_mainloop_api, cb: pa_defer_event_cb_t, userdata: *mut c_void) -> *mut pa_defer_event>,
    pub defer_enable: Option<extern "C" fn(e: *mut pa_defer_event, b: i32)>,
    pub defer_free: Option<extern "C" fn(e: *mut pa_defer_event)>,
    pub defer_set_destroy: Option<extern "C" fn(e: *mut pa_defer_event, cb: pa_defer_event_destroy_cb_t)>,

    pub quit: Option<extern "C" fn(a: *const pa_mainloop_api, retval: i32)>,
}

pub type pa_mainloop_api_once_cb = Option<extern "C" fn(m: *const pa_mainloop_api, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_mainloop_api_once(m: *const pa_mainloop_api, callback: pa_mainloop_api_once_cb, userdata: *mut c_void);
}
