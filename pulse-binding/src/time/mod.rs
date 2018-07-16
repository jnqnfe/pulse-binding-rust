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

mod microseconds;
mod monotonic;
mod timeval;
mod unix;

use capi;

pub use self::microseconds::*;
pub use self::monotonic::*;
pub use self::timeval::*;
pub use self::unix::*;

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
