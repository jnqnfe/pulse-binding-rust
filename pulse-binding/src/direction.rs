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

//! Utility functions for Direction.

#[cfg(any(feature = "pa_v6", feature = "dox"))]
use std::ffi::CStr;

pub type FlagSet = capi::direction::pa_direction_t;

pub mod flags {
    use capi;
    use super::FlagSet;

    pub const OUTPUT: FlagSet = capi::PA_DIRECTION_OUTPUT;
    pub const INPUT: FlagSet = capi::PA_DIRECTION_INPUT;
}

/// Checks whether direction is valid (either input, output or bidirectional).
///
/// Available since PA version 6.
#[inline]
#[cfg(any(feature = "pa_v6", feature = "dox"))]
pub fn is_valid(f: FlagSet) -> bool {
    unsafe { capi::pa_direction_valid(f) != 0 }
}

/// Gets a textual representation of the direction.
///
/// Available since PA version 6.
#[inline]
#[cfg(any(feature = "pa_v6", feature = "dox"))]
pub fn to_string(f: FlagSet) -> String {
    unsafe { CStr::from_ptr(capi::pa_direction_to_string(f)).to_string_lossy().into_owned() }
}
