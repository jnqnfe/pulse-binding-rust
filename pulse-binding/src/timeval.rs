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

//! # Expressing Time
//!
//! Time can be expressed to PulseAudio in two forms, either Unix time ("wallclock"), or real-time
//! (monotonic). Furthermore we have two different structs for representing time values, [`Timeval`]
//! and [`MicroSeconds`].
//!
//! Just like the PulseAudio C API, in this binding we tend to use [`Timeval`] for expressing Unix
//! time, and [`MicroSeconds`] for monotonic time. It is important to understand that the two
//! different structs are **not** what distinguishes between the two time value types.
//!
//! The `From` trait has been implemented for convenience allowing easy conversion between these two
//! structs, however these do **not** convert the values they hold between Unix time and monotonic
//! time.
//!
//! You must be careful when supplying a time value to a function that you are supplying a time
//! value of the correct type. Note that functions taking monotonic time typically have `rt` in
//! their name.
//!
//! Unix based times will typically be an offset from the current wall-clock time (time-of-day).
//! You can get a new [`Timeval`] object set with this value using it's [`new_tod`] associated
//! function. Monotonic based times should be an offset from the current monotonic system time,
//! which can be obtained via the [`::rtclock::now`] function.
//!
//! # Examples
//!
//! ```rust,ignore
//! use pulse::timeval::{Timeval, MicroSeconds, MICROS_PER_SEC};
//!
//! // A `Timeval` holding a Unix timestamp, representing the current time-of-day, plus five seconds
//! let unix_tv = Timeval::new_tod().add(MicroSeconds(5 * MICROS_PER_SEC));
//!
//! // Converting to `MicroSeconds`, still a Unix timestamp
//! let unix_usecs = MicroSeconds::from(unix_tv);
//!
//! // A monotonic timestamp, representing the current system time, plus five seconds
//! let rt_usecs = pulse::rtclock::now() + MicroSeconds(5 * MICROS_PER_SEC);
//!
//! // Converting to `Timeval`, still a monotonic timestamp
//! let rt_tv = Timeval::from(rt_usecs);
//! ```
//!
//! [`Timeval`]: struct.Timeval.html
//! [`MicroSeconds`]: struct.MicroSeconds.html
//! [`new_tod`]: struct.Timeval.html#method.new_tod
//! [`::rtclock::now`]: ../rtclock/fn.now.html

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

/// Bit to set in `timeval`'s `tv_usec` attribute to mark that the `timeval` is in monotonic time
const PA_TIMEVAL_RTCLOCK: i64 = 1 << 30;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    pub fn is_valid(&self) -> bool {
        *self != USEC_INVALID
    }
}

/// Wrapper for `libc::timeval`, attaching various methods and trait implementations
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

    /// Set to the specified (monotonic) value
    ///
    /// The `rtclock` boolean is used for indicating support of the rtclock (monotonic time). If
    /// `true` then the conversion from `MicroSeconds` to `Timeval` is done, and a special 'rt' flag
    /// bit is set in `Timeval`'s inner `tv_usec` attribute. If `false`, then instead the timestamp
    /// is converted to a Unix wallclock timestamp.
    ///
    /// Asserts that `v` is not `USEC_INVALID`
    pub(crate) fn set_rt(&mut self, v: MicroSeconds, rtclock: bool) -> &mut Self {
        /* This is a copy of PA's internal `pa_timeval_rtstore()` function */

        assert_ne!(v, USEC_INVALID);

        self.set(v);

        match rtclock {
            true => { self.0.tv_usec |= PA_TIMEVAL_RTCLOCK; },
            false => { self.wallclock_from_rtclock(); },
        }
        self
    }

    pub(crate) fn wallclock_from_rtclock(&mut self) -> &mut Self {
        /* This is a copy of PA's internal `wallclock_from_rtclock()` function */

        let mut wc_now = Timeval::new_tod();
        let rt_now = Timeval::from(::rtclock::now());

        match rt_now.cmp(self) {
            Ordering::Less => { wc_now.add(Timeval::diff(self, &rt_now)); },
            _              => { wc_now.sub(Timeval::diff(&rt_now, self)); },
        }

        *self = wc_now;
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
