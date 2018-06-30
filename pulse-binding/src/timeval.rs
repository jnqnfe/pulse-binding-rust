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
use libc;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
pub use libc::timeval; //Export wanted for use in timer event callbacks, so users don't need to import the libc crate themselves!

// (Copied constants from rust's std/time/duration.rs)
pub const NANOS_PER_SEC: u32 = 1_000_000_000;
pub const NANOS_PER_MILLI: u32 = 1_000_000;
pub const NANOS_PER_MICRO: u32 = 1_000;
pub const MICROS_PER_SEC: u64 = 1_000_000;
pub const MICROS_PER_MILLI: u64 = 1_000;
pub const MILLIS_PER_SEC: u64 = 1_000;

/// Invalid time. Microseconds value representing 'invalid'.
pub const USEC_INVALID: MicroSeconds = MicroSeconds(capi::PA_USEC_INVALID);

/// Largest valid time value in microseconds (largest integer value is reserved for representing
/// 'invalid'.
pub const USEC_MAX: MicroSeconds = MicroSeconds(capi::PA_USEC_MAX);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    pub fn is_valid(&self) -> bool {
        *self != USEC_INVALID
    }
}

/// Wrapper for `libc::timeval`, providing trait impls
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval(pub timeval); // Warning, this must remain directly transmutable with the inner libc::timeval

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
    /// Create a new instance, with values provided.
    pub fn new(sec: libc::time_t, usec: libc::suseconds_t) -> Self {
        Timeval(timeval { tv_sec: sec, tv_usec: usec })
    }

    /// Create a new instance, with value of zero.
    pub fn new_zero() -> Self {
        Timeval::new(0, 0)
    }

    /// Create a new instance, with value of 'time of day'.
    pub fn new_tod() -> Self {
        let mut tv = Timeval(timeval { tv_sec: 0, tv_usec: 0 });
        tv.get_time_of_day();
        tv
    }

    /// Obtain the current wallclock timestamp, just like UNIX gettimeofday().
    pub fn get_time_of_day(&mut self) -> &mut Self {
        unsafe { capi::pa_gettimeofday(&mut self.0) };
        self
    }

    /// Calculate the difference between the two specified timeval structs.
    pub fn diff(a: &Self, b: &Self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_diff(&a.0, &b.0) })
    }

    /// Return the time difference between now and self
    pub fn age(&self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_age(&self.0) })
    }

    /// Add the specified time in microseconds
    pub fn add(&mut self, t: MicroSeconds) -> &mut Self {
        unsafe { capi::pa_timeval_add(&mut self.0, t.0); }
        self
    }

    /// Subtract the specified time in microseconds
    pub fn sub(&mut self, t: MicroSeconds) -> &mut Self {
        unsafe { capi::pa_timeval_sub(&mut self.0, t.0); }
        self
    }

    /// Set the specified usec value
    pub fn set(&mut self, t: MicroSeconds) -> &mut Self {
        unsafe { capi::pa_timeval_store(&mut self.0, t.0); }
        self
    }
}

impl From<Timeval> for MicroSeconds {
    fn from(t: Timeval) -> Self {
        MicroSeconds(unsafe { capi::pa_timeval_load(&t.0) })
    }
}
impl From<MicroSeconds> for Timeval {
    fn from(t: MicroSeconds) -> Self {
        let mut tmp = Timeval(timeval { tv_sec: 0, tv_usec: 0 });
        tmp.set(t);
        tmp
    }
}

impl MicroSeconds {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|i| Some(MicroSeconds(i)))
    }
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).and_then(|i| Some(MicroSeconds(i)))
    }
    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
    pub fn checked_rem(self, rhs: u32) -> Option<Self> {
        self.0.checked_rem(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
}

impl Add for MicroSeconds {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MicroSeconds(self.0 + other.0)
    }
}
impl AddAssign for MicroSeconds {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for MicroSeconds {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        MicroSeconds(self.0 - other.0)
    }
}
impl SubAssign for MicroSeconds {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for MicroSeconds {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        MicroSeconds(self.0 * rhs as u64)
    }
}
impl MulAssign<u32> for MicroSeconds {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for MicroSeconds {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        MicroSeconds(self.0 / rhs as u64)
    }
}
impl DivAssign<u32> for MicroSeconds {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Rem<u32> for MicroSeconds {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self {
        MicroSeconds(self.0 % rhs as u64)
    }
}
impl RemAssign<u32> for MicroSeconds {
    fn rem_assign(&mut self, rhs: u32) {
        *self = *self % rhs;
    }
}

impl std::fmt::Display for MicroSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} µs", self.0)
    }
}
