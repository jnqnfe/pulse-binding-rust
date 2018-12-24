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

//! Version constants and functions.
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
//!
//! When feature flags are used for backwards compatibility, the versions defined here (as of
//! version `1.4`) are adjusted to return the newest compatible major version.
//!
//! The `get_library_version` function always obtains at runtime the version of the actual PA
//! library in use.

use std::os::raw::c_char;
pub use self::actual::{TARGET_VERSION_STRING, TARGET_VERSION};

/// PA version compatibility selection.
///
/// Used for indicating PA version compatibility support, which can vary depending upon feature
/// flags.
pub enum Compatibility {
    /// Support for latest compatible version.
    Latest,
    /// Support for PA versions < 12 selected.
    PreV12,
}

// Current
#[cfg(feature="pa_v12_compatibility")]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::Latest;

    /// The newest version of the PulseAudio client library this linking library is known to be
    /// compatible with.
    pub const TARGET_VERSION_STRING: &str = "12.0.0";

    /// The major and minor components of the newest version of the PulseAudio client library this
    /// linking library is known to be compatible with.
    pub const TARGET_VERSION: (u8, u8) = (12, 0);
}

// Pre-v12
#[cfg(not(feature="pa_v12_compatibility"))]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::PreV12;

    /// The newest version of the PulseAudio client library this linking library is known to be
    /// compatible with.
    pub const TARGET_VERSION_STRING: &str = "11.0.0";

    /// The major and minor components of the newest version of the PulseAudio client library this
    /// linking library is known to be compatible with.
    pub const TARGET_VERSION: (u8, u8) = (11, 0);
}

pub const PA_API_VERSION: u8 = 12;
pub const PA_PROTOCOL_VERSION: u16 = 32;

/// Returns indication of PA version compatibility support, depending upon feature flags used.
#[inline(always)]
pub const fn get_compatibility() -> Compatibility {
    actual::COMPATIBILITY
}

#[inline(always)]
pub fn pa_check_version(major: u8, minor: u8, _micro: u8) -> bool {
    // Note, defined micro version is always zero as of PA v1.0, thus ignored here
    (TARGET_VERSION.0  > major) ||
    (TARGET_VERSION.0 == major && TARGET_VERSION.1  > minor)
}

#[link(name="pulse")]
extern "C" {
    pub fn pa_get_library_version() -> *const c_char;
}
