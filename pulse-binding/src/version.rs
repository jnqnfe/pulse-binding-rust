// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

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

// Re-export from sys
pub use capi::version::{Compatibility, get_compatibility};
pub use capi::version::{TARGET_VERSION_STRING, TARGET_VERSION};
pub use capi::version::{PA_API_VERSION as API_VERSION, PA_PROTOCOL_VERSION as PROTOCOL_VERSION};
pub use capi::version::pa_check_version as check_version;

/// Gets the version of the (PulseAudio client system) library actually in use at runtime.
#[inline]
pub fn get_library_version() -> &'static CStr {
    unsafe { CStr::from_ptr(capi::pa_get_library_version()) }
}
