// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.
//
// Portions of documentation are copied from the LGPL 2.1+ licensed PulseAudio C headers on a
// fair-use basis, as discussed in the overall project readme (available in the git repository).

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
//! The [`get_library_version()`] function obtains at runtime the version of the actual PA client
//! library in use.
//!
//! # Dynamic constants
//!
//! The version constants defined here mostly relate to those provided in the PA C headers, and are
//! likely of little use to most projects. They are set dynamically, depending upon the feature
//! flags used, or in other words the level of minimum compatibility support selected. Note that PA
//! version feature flags are only introduced when new versions of PA introduce changes to its API
//! that would require one. The version numbers associated with each PA version feature flag are
//! those from the PA version that required introduction of that feature flag.
//!
//! As an example to clarify, if the “newest” PA version feature flag enabled is `pa_v8` (which
//! obviously corresponds to a minimum compatibility level of PA version 8.0), then the
//! [`TARGET_VERSION`] constant is set to `(8, 0)`. The “next-newest” feature flag is `pa_v11`,
//! which if enabled would bump it up to `(11, 0)`.
//!
//! [`get_library_version()`]: fn.get_library_version.html

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
