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

use std;
use capi;
use std::cmp::Ordering;
pub use libc::timeval; //Export wanted for use in timer event callbacks, so users don't need to import the libc crate themselves!

/// The number of milliseconds in a second
pub const MSEC_PER_SEC: ::sample::Usecs = 1000;

/// The number of microseconds in a second
pub const USEC_PER_SEC: ::sample::Usecs = 1_000_000;

/// The number of nanoseconds in a second
pub const NSEC_PER_SEC: u64 = 1_000_000_000;

/// The number of microseconds in a millisecond
pub const USEC_PER_MSEC: ::sample::Usecs = 1000;

/// The number of nanoseconds in a millisecond
pub const NSEC_PER_MSEC: u64 = 1_000_000;

/// The number of nanoseconds in a microsecond
pub const NSEC_PER_USEC: u64 = 1000;

/// Invalid time in usec.
pub const USEC_INVALID: ::sample::Usecs = capi::PA_USEC_INVALID;

/// Biggest time in usec.
pub const USEC_MAX: ::sample::Usecs = capi::PA_USEC_MAX;

/// Wrapper for `libc::timeval`, providing trait impls
/// Warning, this must remain directly transmutable with the inner libc::timeval
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval(pub timeval);

impl PartialEq for Timeval {
    fn eq(&self, other: &Self) -> bool {
        self.0.tv_sec == other.0.tv_sec && self.0.tv_usec == other.0.tv_usec
    }
}
impl Eq for Timeval {}

impl Ord for Timeval {
    fn cmp(&self, other: &Self) -> Ordering {
        match unsafe { capi::pa_timeval_cmp(&self.0, &other.0) } {
            0 => Ordering::Equal,
            r if r < 0 => Ordering::Less,
            _ => Ordering::Greater,
        }
    }
}

impl PartialOrd for Timeval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Timeval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "timeval {{ tv_sec: {}, tv_usec: {} }}", self.0.tv_sec, self.0.tv_usec)
    }
}

impl Timeval {
    /// Obtain the current wallclock timestamp, just like UNIX gettimeofday().
    pub fn get_time_of_day(&mut self) -> &mut Self {
        unsafe { capi::pa_gettimeofday(&mut self.0) };
        self
    }

    /// Calculate the difference between the two specified timeval structs.
    pub fn diff(a: &Self, b: &Self) -> ::sample::Usecs {
        unsafe { capi::pa_timeval_diff(&a.0, &b.0) }
    }

    /// Return the time difference between now and self
    pub fn age(&self) -> ::sample::Usecs {
        unsafe { capi::pa_timeval_age(&self.0) }
    }

    /// Add the specified time in microseconds
    pub fn add(&mut self, t: ::sample::Usecs) {
        unsafe { capi::pa_timeval_add(&mut self.0, t); }
    }

    /// Subtract the specified time in microseconds
    pub fn sub(&mut self, t: ::sample::Usecs) {
        unsafe { capi::pa_timeval_sub(&mut self.0, t); }
    }

    /// Set the specified usec value
    pub fn set(&mut self, t: ::sample::Usecs) {
        unsafe { capi::pa_timeval_store(&mut self.0, t); }
    }

    /// Retrieve the specified usec value
    pub fn get(&self) -> ::sample::Usecs {
        unsafe { capi::pa_timeval_load(&self.0) }
    }
}
