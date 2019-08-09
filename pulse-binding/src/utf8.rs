// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
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

//! UTF-8 validation functions.
//!
//! Bindings are not provided here for most of the PulseAudio UTF-8 functions since Rust has built
//! in UTF-8 handling and thus they should be entirely unnecessary.

use capi;
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};

/// Converts a UTF-8 string to the current locale.
pub fn utf8_to_locale(s: &str) -> Option<String> {
    // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
    // as_ptr() giving dangling pointers!
    let c_str = CString::new(s.clone()).unwrap();
    let tmp_ptr: *const c_char = unsafe { capi::pa_utf8_to_locale(c_str.as_ptr()) };
    if tmp_ptr.is_null() {
        return None;
    }
    unsafe {
        let ret = Some(CStr::from_ptr(tmp_ptr).to_string_lossy().into_owned());
        capi::pa_xfree(tmp_ptr as *mut c_void);
        ret
    }
}
