//! A variation of the standard main loop implementation, using a background thread.

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

use std::os::raw::c_char;

/// An opaque threaded main loop object
pub enum pa_threaded_mainloop {}

#[link(name="pulse")]
extern "C" {
    pub fn pa_threaded_mainloop_new() -> *mut pa_threaded_mainloop;
    pub fn pa_threaded_mainloop_free(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_start(m: *mut pa_threaded_mainloop) -> i32;
    pub fn pa_threaded_mainloop_stop(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_lock(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_unlock(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_wait(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_signal(m: *mut pa_threaded_mainloop, wait_for_accept: i32);
    pub fn pa_threaded_mainloop_accept(m: *mut pa_threaded_mainloop);
    pub fn pa_threaded_mainloop_get_retval(m: *mut pa_threaded_mainloop) -> i32;
    pub fn pa_threaded_mainloop_get_api(m: *mut pa_threaded_mainloop) -> *mut ::mainloop::api::pa_mainloop_api;
    pub fn pa_threaded_mainloop_in_thread(m: *mut pa_threaded_mainloop) -> i32;
    pub fn pa_threaded_mainloop_set_name(m: *mut pa_threaded_mainloop, name: *const c_char);
}
