// Copyright 2018 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
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

//! Monotonic timestamps.

use std;
use capi;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::MicroSeconds;

/// A monotonic timestamp
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicTs(pub(crate) MicroSeconds);

impl MonotonicTs {
    /// Return the current monotonic system time in microseconds.
    ///
    /// Note, if such a clock is not available then this will actually fall back to the wallclock
    /// time instead. No indication is available for whether or not this is the case, and the
    /// return value is still a `MonotonicTs` type.
    pub fn now() -> Self {
        MonotonicTs(MicroSeconds(unsafe { capi::pa_rtclock_now() }))
    }

    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    pub fn checked_add(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_add(other).and_then(|us| Some(MonotonicTs(us)))
    }

    pub fn checked_sub(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_sub(other).and_then(|us| Some(MonotonicTs(us)))
    }
}

impl Add<MicroSeconds> for MonotonicTs {
    type Output = Self;

    fn add(self, other: MicroSeconds) -> Self {
        MonotonicTs(self.0 + other)
    }
}
impl AddAssign<MicroSeconds> for MonotonicTs {
    fn add_assign(&mut self, rhs: MicroSeconds) {
        *self = *self + rhs;
    }
}

impl Sub<MicroSeconds> for MonotonicTs {
    type Output = Self;

    fn sub(self, other: MicroSeconds) -> Self {
        MonotonicTs(self.0 - other)
    }
}
impl SubAssign<MicroSeconds> for MonotonicTs {
    fn sub_assign(&mut self, rhs: MicroSeconds) {
        *self = *self - rhs;
    }
}

impl std::fmt::Display for MonotonicTs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
