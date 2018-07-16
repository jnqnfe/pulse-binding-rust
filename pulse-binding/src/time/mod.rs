//! Time handling functionality.

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
//! which can be obtained via the [`rtclock_now`] function.
//!
//! # Examples
//!
//! ```rust,ignore
//! use pulse::time::{Timeval, MicroSeconds, MICROS_PER_SEC, rtclock_now};
//!
//! // A `Timeval` holding a Unix timestamp, representing the current time-of-day, plus five seconds
//! let unix_tv = Timeval::new_tod().add(MicroSeconds(5 * MICROS_PER_SEC));
//!
//! // Converting to `MicroSeconds`, still a Unix timestamp
//! let unix_usecs = MicroSeconds::from(unix_tv);
//!
//! // A monotonic timestamp, representing the current system time, plus five seconds
//! let rt_usecs = rtclock_now() + MicroSeconds(5 * MICROS_PER_SEC);
//!
//! // Converting to `Timeval`, still a monotonic timestamp
//! let rt_tv = Timeval::from(rt_usecs);
//! ```
//!
//! [`Timeval`]: struct.Timeval.html
//! [`MicroSeconds`]: struct.MicroSeconds.html
//! [`new_tod`]: struct.Timeval.html#method.new_tod
//! [`rtclock_now`]: fn.rtclock_now.html

mod microseconds;
mod timeval;

use capi;

pub use self::microseconds::*;
pub use self::timeval::*;

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
/// 'invalid').
pub const USEC_MAX: MicroSeconds = MicroSeconds(capi::PA_USEC_MAX);

impl From<Timeval> for MicroSeconds {
    fn from(t: Timeval) -> Self {
        MicroSeconds(unsafe { capi::pa_timeval_load(&t.0) })
    }
}
impl From<MicroSeconds> for Timeval {
    fn from(t: MicroSeconds) -> Self {
        let secs = t.0 / MICROS_PER_SEC;
        let usecs = t.0 % MICROS_PER_SEC;
        Timeval::new(secs as i64, usecs as i64)
    }
}

/// Return the current monotonic system time in microseconds.
///
/// Note, if such a clock is not available then this will actually fall back to the wallclock time
/// instead. No indication is available for whether or not this is the case; users need not be
/// concerned and should just treat the value as monotonic in terms of selecting which time related
/// API functions to use it with.
pub fn rtclock_now() -> MicroSeconds {
    MicroSeconds(unsafe { capi::pa_rtclock_now() })
}
