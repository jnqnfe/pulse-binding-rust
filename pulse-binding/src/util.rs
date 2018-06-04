//! Assorted utility functions.

// This file is part of the PulseAudio Rust language binding.
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

use capi;
use std::ffi::CStr;

macro_rules! fn_string_with_buffer {
    ( $fn_name:ident, $fn_call:ident ) => {
        pub fn $fn_name(l: usize) -> Option<String> {
            let mut tmp = Vec::with_capacity(l);
            unsafe {
                // Need to check NULL return here because `get_binary_name` function is not
                // supported on all architectures and so may return NULL, and might as well check
                // NULL return for other uses anyway.
                let ptr = capi::$fn_call(tmp.as_mut_ptr(), l);
                match ptr.is_null() {
                    true => None,
                    false => Some(CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()),
                }
            }
        }
    };
}

/// Return the current username. Returns `None` on failure.
fn_string_with_buffer!(get_user_name, pa_get_user_name);

/// Return the current hostname. Returns `None` on failure.
fn_string_with_buffer!(get_host_name, pa_get_host_name);

/// Return the fully qualified domain name. Returns `None` on failure.
fn_string_with_buffer!(get_fqdn, pa_get_fqdn);

/// Return the home directory of the current user. Returns `None` on failure.
fn_string_with_buffer!(get_home_dir, pa_get_home_dir);

/// Return the binary file name of the current process. Returns `None` on failure. This is not
/// supported on all architectures (in which case `NULL` is returned).
fn_string_with_buffer!(get_binary_name, pa_get_binary_name);

