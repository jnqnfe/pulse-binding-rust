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

//! MicroSeconds.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::time::Duration;

/// Microseconds. Represents a span of time like `std::time::Duration`.
///
/// This is an unsigned 64-bit type, and thus represents absolute values only.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    /// `MicroSeconds` value representing an ‘invalid’ time.
    pub const INVALID: Self = Self(capi::PA_USEC_INVALID);

    /// Largest valid time value (largest integer value is reserved for representing ‘invalid’).
    pub const MAX: Self = Self(capi::PA_USEC_MAX);

    /// Returns `true` so long as inner value is not `Self::INVALID`.
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_add()` method.
    #[inline]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_add()` method.
    #[inline]
    pub fn checked_add_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_add(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_sub()` method.
    #[inline]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_sub()` method.
    #[inline]
    pub fn checked_sub_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_sub(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None` if overflow
    /// occurred, using the inner integer’s `checked_mul()` method.
    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0`, using the
    /// inner integer’s `checked_div()` method.
    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs == 0`, using the
    /// inner integer’s `checked_rem()` method.
    #[inline]
    pub fn checked_rem(self, rhs: u32) -> Option<Self> {
        self.0.checked_rem(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
}

impl std::fmt::Display for MicroSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} µs", self.0)
    }
}

impl Add for MicroSeconds {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        self.checked_add(other).unwrap()
    }
}
impl AddAssign for MicroSeconds {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for MicroSeconds {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        self.checked_sub(other).unwrap()
    }
}
impl SubAssign for MicroSeconds {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Operations with `Duration`
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Add<Duration> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Duration) -> Self {
        self.checked_add_duration(rhs).unwrap()
    }
}
impl AddAssign<Duration> for MicroSeconds {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = self.checked_add_duration(rhs).unwrap();
    }
}

impl Sub<Duration> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Duration) -> Self {
        self.checked_sub_duration(rhs).unwrap()
    }
}
impl SubAssign<Duration> for MicroSeconds {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self.checked_sub_duration(rhs).unwrap();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Operations with primatives
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Mul<u32> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self {
        MicroSeconds(self.0.checked_mul(rhs as u64).unwrap())
    }
}
impl MulAssign<u32> for MicroSeconds {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u32) -> Self {
        MicroSeconds(self.0.checked_div(rhs as u64).unwrap())
    }
}
impl DivAssign<u32> for MicroSeconds {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Rem<u32> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: u32) -> Self {
        MicroSeconds(self.0.checked_rem(rhs as u64).unwrap())
    }
}
impl RemAssign<u32> for MicroSeconds {
    #[inline]
    fn rem_assign(&mut self, rhs: u32) {
        *self = *self % rhs;
    }
}
