//! Define version.

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

/// The version of the PulseAudio headers this linking library is designed to link to. (It may also
/// incidentally be backwards compatible with older versions).
pub const LINK_TARGET_VERSION: &str = "12.0.0";

#[inline(always)]
pub fn pa_get_headers_version() -> &'static str {
    LINK_TARGET_VERSION
}

pub const PA_API_VERSION: u8 = 12;
pub const PA_PROTOCOL_VERSION: u16 = 32;
pub const PA_MAJOR: u8 = 12;
pub const PA_MINOR: u8 = 0;
pub const PA_MICRO: u8 = 0;

#[inline(always)]
pub fn pa_check_version(major: u8, minor: u8, micro: u8) -> bool {
    ((PA_MAJOR  > major) ||
     (PA_MAJOR == major && PA_MINOR  > minor) ||
     (PA_MAJOR == major && PA_MINOR == minor && PA_MICRO >= micro))
}

#[link(name="pulse")]
extern "C" {
    pub fn pa_get_library_version() -> *const c_char;
}
