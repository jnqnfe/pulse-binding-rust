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

//! Version constants and functions
//!
//! The constants defined here follow those given in the `sys` crate and thus the C headers.
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

use capi;
use std::ffi::CStr;

/// The version of the PulseAudio client library this binding is targetted at.
pub const BINDING_TARGET_VERSION: &str = "12.0.0";

#[inline(always)]
pub fn get_headers_version() -> &'static str {
    BINDING_TARGET_VERSION
}

/// The current API version. Please note that this is only ever increased on incompatible API
/// changes!
pub const API_VERSION: u8 = 12;

/// The current protocol version.
pub const PROTOCOL_VERSION: u16 = 32;

/// The major version of PA.
pub const MAJOR: u8 = 12;

/// The minor version of PA.
pub const MINOR: u8 = 0;

/// The micro version of PA (will always be 0 from v1.0 onwards).
pub const MICRO: u8 = 0;

/// Returns the version of the library the current application is linked to.
pub fn get_library_version() -> &'static CStr {
    unsafe { CStr::from_ptr(capi::pa_get_library_version()) }
}

/// Evaluates to true if the PulseAudio library version targeted by this version of the PA binding
/// library is equal or newer than the version specified.
pub fn check_version(major: u8, minor: u8, micro: u8) -> bool {
    ((MAJOR  > major) ||
     (MAJOR == major && MINOR  > minor) ||
     (MAJOR == major && MINOR == minor && MICRO >= micro))
}
