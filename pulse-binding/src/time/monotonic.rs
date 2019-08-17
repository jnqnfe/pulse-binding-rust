// Copyright 2018 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Monotonic timestamps.

use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::MicroSeconds;

/// A monotonic timestamp.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicTs(pub(crate) MicroSeconds);

impl MonotonicTs {
    /// Gets the current monotonic system time in microseconds.
    ///
    /// Note, if such a clock is not available then this will actually fall back to the wallclock
    /// time instead. No indication is available for whether or not this is the case, and the
    /// return value is still a `MonotonicTs` type.
    #[inline]
    pub fn now() -> Self {
        MonotonicTs(MicroSeconds(unsafe { capi::pa_rtclock_now() }))
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    #[inline]
    pub fn checked_add(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_add(other).and_then(|us| Some(MonotonicTs(us)))
    }

    #[inline]
    pub fn checked_sub(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_sub(other).and_then(|us| Some(MonotonicTs(us)))
    }
}

impl Add<MicroSeconds> for MonotonicTs {
    type Output = Self;

    #[inline]
    fn add(self, other: MicroSeconds) -> Self {
        MonotonicTs(self.0 + other)
    }
}
impl AddAssign<MicroSeconds> for MonotonicTs {
    #[inline]
    fn add_assign(&mut self, rhs: MicroSeconds) {
        *self = *self + rhs;
    }
}

impl Sub<MicroSeconds> for MonotonicTs {
    type Output = Self;

    #[inline]
    fn sub(self, other: MicroSeconds) -> Self {
        MonotonicTs(self.0 - other)
    }
}
impl SubAssign<MicroSeconds> for MonotonicTs {
    #[inline]
    fn sub_assign(&mut self, rhs: MicroSeconds) {
        *self = *self - rhs;
    }
}

impl std::fmt::Display for MonotonicTs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
