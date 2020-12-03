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

//! Utility functions for Direction.

#[cfg(any(doc, feature = "pa_v6"))]
use std::ffi::CStr;

pub type FlagSet = capi::direction::pa_direction_t;

pub mod flags {
    use capi;
    use super::FlagSet;

    pub const OUTPUT: FlagSet = capi::PA_DIRECTION_OUTPUT;
    pub const INPUT:  FlagSet = capi::PA_DIRECTION_INPUT;
}

/// Checks whether direction is valid (either input, output or bidirectional).
///
/// Available since PA version 6.
#[inline]
#[cfg(any(doc, feature = "pa_v6"))]
#[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
pub fn is_valid(f: FlagSet) -> bool {
    unsafe { capi::pa_direction_valid(f) != 0 }
}

/// Gets a textual representation of the direction.
///
/// Available since PA version 6.
#[inline]
#[cfg(any(doc, feature = "pa_v6"))]
#[cfg_attr(docsrs, doc(cfg(feature = "pa_v6")))]
pub fn to_string(f: FlagSet) -> String {
    unsafe { CStr::from_ptr(capi::pa_direction_to_string(f)).to_string_lossy().into_owned() }
}
