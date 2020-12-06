// Copyright 2018 Lyndon Brown
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

//! Monotonic timestamps.

use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::{MicroSeconds, op_err};

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

    /// Returns `true` so long as inner value is not `MicroSeconds::INVALID`.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred,
    /// using the inner `MicroSeconds`’s `checked_add()` method.
    #[inline]
    pub fn checked_add(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_add(other).and_then(|us| Some(MonotonicTs(us)))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred,
    /// using the inner `MicroSeconds`’s `checked_sub()` method.
    #[inline]
    pub fn checked_sub(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_sub(other).and_then(|us| Some(MonotonicTs(us)))
    }
}

impl std::fmt::Display for MonotonicTs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Add<MicroSeconds> for MonotonicTs {
    type Output = Self;

    #[inline]
    fn add(self, other: MicroSeconds) -> Self {
        self.checked_add(other).expect(op_err::ADD)
    }
}
impl AddAssign<MicroSeconds> for MonotonicTs {
    #[inline]
    fn add_assign(&mut self, rhs: MicroSeconds) {
        *self = self.add(rhs);
    }
}

impl Sub<MicroSeconds> for MonotonicTs {
    type Output = Self;

    #[inline]
    fn sub(self, other: MicroSeconds) -> Self {
        self.checked_sub(other).expect(op_err::SUB)
    }
}
impl SubAssign<MicroSeconds> for MonotonicTs {
    #[inline]
    fn sub_assign(&mut self, rhs: MicroSeconds) {
        *self = self.sub(rhs);
    }
}
