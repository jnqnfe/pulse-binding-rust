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
//! The constants here mostly relate to those provided in the PulseAudio (PA) C headers.
//!
//! - They are typically only updated following a new **major** release of PA.
//! - Some values are dynamic, depending upon the level of PA compatibility support selected at
//!   compile time via Cargo feature flags. For instance if you enable support for PA <= v12 then
//!   they will indicate v12, whereas if you exclude v12 support, they will indicate v11.
//!
//! Note that:
//!
//! - The minimum supported version of PA is v5.0.
//! - Where a new major version of PA introduces API changes, such as new function symbols or new
//!   enum variants, for instance, we add a Cargo feature to allow selective control over inclusion
//!   of those changed, and thus control over the level of compatibility with newer releases that
//!   the crate is compiled with.
//!
//! The [`pa_get_library_version`] function always obtains at runtime the version of the actual PA
//! library in use.
//!
//! The [`get_compatibility`] function gives an indication of the level of compatibility support
//! built in at compile time, per Cargo feature flags.
//!
//! [`pa_get_library_version`]: fn.pa_get_library_version.html
//! [`get_compatibility`]: fn.get_compatibility.html

use std::os::raw::c_char;

/// PulseAudio version compatibility.
///
/// Used for indicating what level of PA version compatibility was selected at compile time via
/// Cargo feature flags.
///
/// Note that PA v5 is the oldest supported.
pub enum Compatibility {
    /// Support for latest compatible version.
    Latest,
    /// Support for PA versions <= 11 selected.
    PreV12,
    /// Support for PA versions <= 7 selected.
    PreV8,
    /// Support for PA versions <= 5 selected.
    PreV6,
}

// Current
#[cfg(feature="pa_v12_compatibility")]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::Latest;
    pub const TARGET_VERSION_STRING: &str = "12.0.0";
    pub const TARGET_VERSION: (u8, u8) = (12, 0);
    pub const PA_PROTOCOL_VERSION: u16 = 32;
}

// Pre-v12
#[cfg(all(not(feature="pa_v12_compatibility"), feature="pa_v8_compatibility"))]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::PreV12;
    pub const TARGET_VERSION_STRING: &str = "11.0.0";
    pub const TARGET_VERSION: (u8, u8) = (11, 0);
    pub const PA_PROTOCOL_VERSION: u16 = 32;
}

// Pre-v8
#[cfg(all(not(feature="pa_v8_compatibility"), feature="pa_v6_compatibility"))]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::PreV8;
    pub const TARGET_VERSION_STRING: &str = "7.0.0";
    pub const TARGET_VERSION: (u8, u8) = (7, 0);
    pub const PA_PROTOCOL_VERSION: u16 = 30;
}

// Pre-v6
#[cfg(not(feature="pa_v6_compatibility"))]
mod actual {
    pub const COMPATIBILITY: super::Compatibility = super::Compatibility::PreV6;
    pub const TARGET_VERSION_STRING: &str = "5.0.0";
    pub const TARGET_VERSION: (u8, u8) = (5, 0);
    pub const PA_PROTOCOL_VERSION: u16 = 29;
}

/// The newest version of the PulseAudio client library this linking library is known to be
/// compatible with.
pub const TARGET_VERSION_STRING: &str = actual::TARGET_VERSION_STRING;

/// The major and minor components of the newest version of the PulseAudio client library this
/// linking library is known to be compatible with.
pub const TARGET_VERSION: (u8, u8) = actual::TARGET_VERSION;

/// The current protocol version.
pub const PA_PROTOCOL_VERSION: u16 = actual::PA_PROTOCOL_VERSION;

// Note, this seems to be constant, as of c95d0d7dcbca0c531b972ece1004caad95c92936
pub const PA_API_VERSION: u8 = 12;

/// Returns indication of the level of PulseAudio version compatibility selected at compie time via
/// Cargo feature flags.
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
