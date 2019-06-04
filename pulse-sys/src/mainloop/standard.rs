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

//! Standard/minimal main loop implementation based on poll().

use std::os::raw::{c_ulong, c_void};
use libc::pollfd;

/// An opaque main loop object
#[repr(C)] pub struct pa_mainloop { _private: [u8; 0] }

pub type pa_poll_func = Option<extern "C" fn(ufds: *mut pollfd, nfds: c_ulong, timeout: i32, userdata: *mut c_void) -> i32>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_mainloop_new() -> *mut pa_mainloop;
    pub fn pa_mainloop_free(m: *mut pa_mainloop);
    pub fn pa_mainloop_prepare(m: *mut pa_mainloop, timeout: i32) -> i32;
    pub fn pa_mainloop_poll(m: *mut pa_mainloop) -> i32;
    pub fn pa_mainloop_dispatch(m: *mut pa_mainloop) -> i32;
    pub fn pa_mainloop_get_retval(m: *const pa_mainloop) -> i32;
    pub fn pa_mainloop_iterate(m: *mut pa_mainloop, block: i32, retval: *mut i32) -> i32;
    pub fn pa_mainloop_run(m: *mut pa_mainloop, retval: *mut i32) -> i32;
    pub fn pa_mainloop_get_api(m: *const pa_mainloop) -> *const super::api::pa_mainloop_api;
    pub fn pa_mainloop_quit(m: *mut pa_mainloop, retval: i32);
    pub fn pa_mainloop_wakeup(m: *mut pa_mainloop);

    pub fn pa_mainloop_set_poll_func(m: *mut pa_mainloop, poll_func: pa_poll_func, userdata: *mut c_void);
}
