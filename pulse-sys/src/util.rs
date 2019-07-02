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

//! Assorted utility functions.

use std::os::raw::{c_char, c_ulong};

#[link(name="pulse")]
extern "C" {
    pub fn pa_get_user_name(s: *mut c_char, l: usize) -> *mut c_char;
    pub fn pa_get_host_name(s: *mut c_char, l: usize) -> *mut c_char;
    pub fn pa_get_fqdn(s: *mut c_char, l: usize) -> *mut c_char;
    pub fn pa_get_home_dir(s: *mut c_char, l: usize) -> *mut c_char;
    pub fn pa_get_binary_name(s: *mut c_char, l: usize) -> *mut c_char;
    pub fn pa_path_get_filename(p: *const c_char) -> *mut c_char;
    pub fn pa_msleep(t: c_ulong) -> i32;
    #[cfg(any(feature = "pa_v13", feature = "dox"))]
    pub fn pa_thread_make_realtime(rtprio: i32) -> i32;
}
