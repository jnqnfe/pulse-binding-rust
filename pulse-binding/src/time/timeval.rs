//! Timeval.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2018 Lyndon Brown
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
use super::{rtclock_now, USEC_INVALID, MicroSeconds};

/// Bit to set in `timeval`'s `tv_usec` attribute to mark that the `timeval` is in monotonic time
const PA_TIMEVAL_RTCLOCK: i64 = 1 << 30;

/// Wrapper for `libc::timeval`, attaching various methods and trait implementations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Timeval(pub libc::timeval); // Warning, this must remain directly transmutable with the inner libc::timeval

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
        Timeval(libc::timeval { tv_sec: sec, tv_usec: usec })
    }

    /// Create a new instance, with value of zero.
    pub fn new_zero() -> Self {
        Timeval::new(0, 0)
    }

    /// Create a new instance, with value of 'time of day'.
    pub fn new_tod() -> Self {
        let mut tv = Timeval(libc::timeval { tv_sec: 0, tv_usec: 0 });
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

    /// Set to the specified value, given in microseconds
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
        let rt_now = Timeval::from(rtclock_now());

        match rt_now.cmp(self) {
            Ordering::Less => { wc_now.add(Timeval::diff(self, &rt_now)); },
            _              => { wc_now.sub(Timeval::diff(&rt_now, self)); },
        }

        *self = wc_now;
        self
    }
}
