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
//! The constants here mostly relate to those provided in the `sys` crate and thus the PulseAudio
//! (PA) C headers.
//!
//! - They are typically only updated following a new **major** release of PA.
//! - Some values are dynamic, depending upon the level of PA compatibility support selected at
//!   compile time via Cargo feature flags. For instance if you enable support for PA <= v12 then
//!   they will indicate v12, whereas if you exclude v12 support, they will indicate v11.
//!
//! Note that:
//!
//! - The minimum supported version of PA is v4.0.
//! - Where a new major version of PA introduces API changes, such as new function symbols or new
//!   enum variants, for instance, we add a Cargo feature to allow selective control over inclusion
//!   of those changed, and thus control over the level of compatibility with newer releases that
//!   the crate is compiled with.
//!
//! The [`get_library_version`] function always obtains at runtime the version of the actual PA
//! library in use.
//!
//! The [`get_compatibility`] function gives an indication of the level of compatibility support
//! built in at compile time, per Cargo feature flags.
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

/// Returns indication of the level of PulseAudio version compatibility selected at compie time via
/// Cargo feature flags.
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
