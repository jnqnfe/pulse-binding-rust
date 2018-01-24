//! UTF-8 validation functions.

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

#[link(name="pulse")]
extern "C" {
    pub fn pa_utf8_valid(s: *const c_char) -> *mut c_char;
    pub fn pa_ascii_valid(s: *const c_char) -> *mut c_char;
    pub fn pa_utf8_filter(s: *const c_char) -> *mut c_char;
    pub fn pa_ascii_filter(s: *const c_char) -> *mut c_char;
    pub fn pa_utf8_to_locale(s: *const c_char) -> *mut c_char;
    pub fn pa_locale_to_utf8(s: *const c_char) -> *mut c_char;
}
