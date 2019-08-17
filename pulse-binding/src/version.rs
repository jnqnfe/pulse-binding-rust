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

//! Version constants and functions.
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
//! - We have backwards compatibility with PA version `11.x` (and down to `8.x`) if the
//!   `pa_encoding_from_string` feature flag is disabled.
//!
//! When feature flags are used for backwards compatibility, the versions defined here (as of
//! version `2.3`) are adjusted to return the newest compatible major version.
//!
//! The `get_library_version` function always obtains at runtime the version of the actual PA
//! library in use.

use capi;
use std::ffi::CStr;
pub use capi::version::Compatibility;

/// The newest version of the PulseAudio client library this binding is known to be compatible with.
pub const TARGET_VERSION_STRING: &str = capi::version::TARGET_VERSION_STRING;

/// The major and minor components of the newest version of the PulseAudio client library this
/// binding is known to be compatible with.
pub const TARGET_VERSION: (u8, u8) = capi::version::TARGET_VERSION;

/// The current API version, from the PA C header. Note, this seems to be separate from the PA
/// version number, where is was `12` for the v0.9.11 release, and has not been changed since
/// (c95d0d7dcbca0c531b972ece1004caad95c92936).
pub const API_VERSION: u8 = capi::version::PA_API_VERSION;

/// The current protocol version.
pub const PROTOCOL_VERSION: u16 = capi::version::PA_PROTOCOL_VERSION;

/// Gets an indication of PA version compatibility support, depending upon feature flags used.
#[inline(always)]
pub const fn get_compatibility() -> Compatibility {
    capi::version::get_compatibility()
}

/// Gets the version of the library actually in use at runtime.
#[inline]
pub fn get_library_version() -> &'static CStr {
    unsafe { CStr::from_ptr(capi::pa_get_library_version()) }
}

/// Evaluates to `true` if the PulseAudio library version targeted by this version of the PA binding
/// library is equal or newer than the version specified.
pub fn check_version(major: u8, minor: u8, _micro: u8) -> bool {
    // Note, defined micro version is always zero as of PA v1.0, thus ignored here
    (TARGET_VERSION.0  > major) ||
    (TARGET_VERSION.0 == major && TARGET_VERSION.1  > minor)
}
