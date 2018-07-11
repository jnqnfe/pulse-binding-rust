//! Monotonic clock utilities.

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
use time::MicroSeconds;

/// Return the current monotonic system time in usecs, if such a clock is available. If it is not
/// available this will return the wallclock time instead.
pub fn now() -> MicroSeconds {
    MicroSeconds(unsafe { capi::pa_rtclock_now() })
}
