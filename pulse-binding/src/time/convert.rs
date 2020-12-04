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

//! Implementation of conversions between the types.

use super::*;

// To `MicroSeconds`

impl From<Duration> for MicroSeconds {
    #[inline]
    fn from(t: Duration) -> Self {
        MicroSeconds((t.as_secs() * MILLIS_PER_SEC) + t.subsec_millis() as u64)
    }
}

impl From<Timeval> for MicroSeconds {
    #[inline]
    fn from(t: Timeval) -> Self {
        MicroSeconds(unsafe { capi::pa_timeval_load(&t.0) })
    }
}

// To `Duration`

impl From<MicroSeconds> for Duration {
    #[inline]
    fn from(t: MicroSeconds) -> Self {
        Duration::from_millis(t.0)
    }
}

impl From<Timeval> for Duration {
    #[inline]
    fn from(t: Timeval) -> Self {
        Duration::from_millis((MicroSeconds::from(t)).0)
    }
}

// To `Timeval`

impl From<MicroSeconds> for Timeval {
    #[inline]
    fn from(t: MicroSeconds) -> Self {
        let secs = (t.0 / MICROS_PER_SEC) as self::timeval::TvSecs;
        let micros = (t.0 % MICROS_PER_SEC) as self::timeval::TvUsecs;
        Timeval::new(secs, micros)
    }
}

impl From<Duration> for Timeval {
    #[inline]
    fn from(t: Duration) -> Self {
        let secs = t.as_secs() as self::timeval::TvSecs;
        let millis = t.subsec_millis() as self::timeval::TvUsecs;
        Timeval::new(secs, millis)
    }
}
