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

//! Version constants and functions
//!
//! The constants defined here follow those given in the C headers.
//!
//! - They are typically updated only following a new major release of PA.
//! - They are not typically updated following a new minor release of PA. i.e. we may declare
//!   version `12.0` here, but remain compatible with all `12.x`, so long as PA itself continues to
//!   adhere to semantic versioning (i.e. no breaking changes in minor releases).
//! - They are **not** the minimum compatible PA version. We have feature flags for providing
//!   backwards compatibility with a limited number of past major versions.
//!
//! Currently:
//!
//! - We primarily target PA version `12.x`
//! - We have backwards compatibility with PA version `11.x` (and it is believed `10.x`) if the
//!   `pa_encoding_from_string` feature flag is disabled.

use std::os::raw::c_char;

/// The version of the PulseAudio client library this linking library is targetted at.
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
