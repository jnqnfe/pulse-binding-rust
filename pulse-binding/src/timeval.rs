//! Utility functions for handling timeval calculations.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2017 Lyndon Brown
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

use capi;
use std::cmp::Ordering;
use libc::timeval;

/// The number of milliseconds in a second
pub const MSEC_PER_SEC: ::sample::Usecs = 1000;

/// The number of microseconds in a second
pub const USEC_PER_SEC: ::sample::Usecs = 1000000;

/// The number of nanoseconds in a second
pub const NSEC_PER_SEC: u64 = 1000000000;

/// The number of microseconds in a millisecond
pub const USEC_PER_MSEC: ::sample::Usecs = 1000;

/// The number of nanoseconds in a millisecond
pub const NSEC_PER_MSEC: u64 = 1000000;

/// The number of nanoseconds in a microsecond
pub const NSEC_PER_USEC: u64 = 1000;

/// Invalid time in usec.
pub const USEC_INVALID: ::sample::Usecs = capi::PA_USEC_INVALID;

/// Biggest time in usec.
pub const USEC_MAX: ::sample::Usecs = capi::PA_USEC_MAX;

/// Return the current wallclock timestamp, just like UNIX gettimeofday().
pub fn get_time_of_day(tv: &mut timeval) -> &mut timeval {
    unsafe { capi::pa_gettimeofday(tv) };
    tv
}

/// Calculate the difference between the two specified timeval structs.
pub fn diff(a: &timeval, b: &timeval) -> ::sample::Usecs {
    unsafe { capi::pa_timeval_diff(a, b) }
}

/// Compare one timeval struct with another, returning the ordering.
/// E.g. if self < 'with', 'Less' returned.
pub fn cmp(a: &timeval, b: &timeval) -> Ordering {
    match unsafe { capi::pa_timeval_cmp(a, b) } {
        0 => Ordering::Equal,
        r if r < 0 => Ordering::Less,
        _ => Ordering::Greater,
    }
}

/// Return the time difference between now and the specified timestamp
pub fn age(tv: &timeval) -> ::sample::Usecs {
    unsafe { capi::pa_timeval_age(tv) }
}

/// Add the specified time in microseconds
pub fn add(tv: &mut timeval, v: ::sample::Usecs) {
    unsafe { capi::pa_timeval_add(tv, v); }
}

/// Subtract the specified time in microseconds
pub fn sub(tv: &mut timeval, v: ::sample::Usecs) {
    unsafe { capi::pa_timeval_sub(tv, v); }
}

/// Set the specified usec value
pub fn set(tv: &mut timeval, v: ::sample::Usecs) {
    unsafe { capi::pa_timeval_store(tv, v); }
}

/// Retrieve the specified usec value
pub fn get(tv: &timeval) -> ::sample::Usecs {
    unsafe { capi::pa_timeval_load(tv) }
}
