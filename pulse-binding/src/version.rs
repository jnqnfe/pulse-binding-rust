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
//! This module contains functions and constants relating to PulseAudio (PA) client API version
//! compatibility.
//!
//! # Dynamic compatibility
//!
//! As discussed in the project `COMPATIBILITY.md` file, support is offered for multiple versions of
//! the PA client library, with changes made in newer versions being guarded with Cargo feature
//! flags.
//!
//! Note that the minimum supported version of PA is v4.0.
//!
//! # Runtime check
//!
//! The [`get_library_version`] function obtains at runtime the version of the actual PA client
//! library in use.
//!
//! # Dynamic constants
//!
//! The version constants defined here mostly relate to those provided in the PA C headers. They are
//! typically only updated following a new **major** release of PA. They are also dynamic, depending
//! upon the level of compatibility support selected at compile time via the available Cargo feature
//! flags.
//!
//! Note that there is **not** a one-to-one mapping of major PA version to compatibility feature. A
//! new such feature is only introduced where actual changes in the PA client API require one
//! because they introduce changes such as the addition of new functions.
//!
//! It is not obvious how the constants relate to version compatibility levels without reading the
//! code, so this needs explaining. Simply put, version numbers are updated typically only on
//! release of a new major version of PA, and update the existing version number associated with the
//! current latest compatibility level selector when there are no changes requiring a new
//! compatibility feature.
//!
//! Thus, for instance, PA versions 8 and 12 introduced additions to the API and have corresponding
//! compatibility features to control the inclusion of those additions on top of the minimum level
//! of support offered. If you have v8 compatibility enabled but not v12, then the version number
//! indicated will be v11.
//!
//! [`get_library_version`]: fn.get_library_version.html
//! [`get_compatibility`]: fn.get_compatibility.html

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

/// Get selected compatibility level.
///
/// Returns indication of the level of PulseAudio version compatibility selected at compile time via
/// Cargo feature flags.
#[inline(always)]
pub const fn get_compatibility() -> Compatibility {
    capi::version::get_compatibility()
}

/// Gets the version of the (PulseAudio client system) library actually in use at runtime.
#[inline]
pub fn get_library_version() -> &'static CStr {
    unsafe { CStr::from_ptr(capi::pa_get_library_version()) }
}

/// Compare a given version with the targetted version.
///
/// Returns `true` if newer or equal to [`TARGET_VERSION`](constant.TARGET_VERSION.html).
///
/// Note that as of PulseAudio v1.0 the `micro` component is always zero and so is ignored.
pub fn check_version(major: u8, minor: u8, _micro: u8) -> bool {
    // Note, defined micro version is always zero as of PA v1.0, thus ignored here
    (TARGET_VERSION.0  > major) ||
    (TARGET_VERSION.0 == major && TARGET_VERSION.1  > minor)
}
