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
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::INVALID, MicroSeconds(std::u64::MAX));
    /// ```
    pub const INVALID: Self = Self(capi::PA_USEC_INVALID);

    /// One second in microseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::SECOND, MicroSeconds(1_000_000));
    /// ```
    pub const SECOND: Self = Self(super::MICROS_PER_SEC);

    /// One millisecond in microseconds.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::MILLISECOND, MicroSeconds(1_000));
    /// ```
    pub const MILLISECOND: Self = Self(super::MICROS_PER_MILLI);

    /// Zero value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::ZERO, MicroSeconds(0));
    /// ```
    pub const ZERO: Self = Self(0);

    /// Smallest _valid_ time value (zero).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::MIN, MicroSeconds(0));
    /// ```
    pub const MIN: Self = Self(0);

    /// Largest _valid_ time value (largest integer value is reserved for representing ‘invalid’).
    ///
    /// Roughly equal to 5,124,095,576 hours, 213,503,982 days, or 584,542 years.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::MAX, MicroSeconds(std::u64::MAX - 1));
    /// ```
    pub const MAX: Self = Self(capi::PA_USEC_MAX);

    /// Returns `true` so long as inner value is not `Self::INVALID`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// assert_eq!(MicroSeconds::MIN.is_valid(), true);
    /// assert_eq!(MicroSeconds::MAX.is_valid(), true);
    /// assert_eq!(MicroSeconds::INVALID.is_valid(), false);
    /// assert_eq!(MicroSeconds::ZERO.is_valid(), true);
    /// assert_eq!(MicroSeconds(60 * MICROS_PER_SEC).is_valid(), true);
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }

    /// Returns `true` so long as inner value is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::ZERO.is_zero(), true);
    /// assert_eq!(MicroSeconds(0).is_zero(), true);
    /// assert_eq!(MicroSeconds(1).is_zero(), false);
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Creates a new `MicroSeconds` from the specified number of whole seconds. Returns `None` on
    /// overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::from_secs(2), Some(MicroSeconds(2_000_000)));
    /// assert_eq!(MicroSeconds::from_secs(0xffff_ffff_0000_0000), None);
    /// ```
    #[inline]
    pub fn from_secs(secs: u64) -> Option<Self> {
        secs.checked_mul(super::MICROS_PER_SEC).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Creates a new `MicroSeconds` from the specified number of whole milliseconds. Returns `None`
    /// on overflow.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::MicroSeconds;
    /// assert_eq!(MicroSeconds::from_millis(23), Some(MicroSeconds(23_000)));
    /// assert_eq!(MicroSeconds::from_millis(0xffff_ffff_0000_0000), None);
    /// ```
    #[inline]
    pub fn from_millis(millis: u64) -> Option<Self> {
        millis.checked_mul(super::MICROS_PER_MILLI).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_add()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// let quater_minute = MicroSeconds(15 * MICROS_PER_SEC);
    /// let half_minute = MicroSeconds(30 * MICROS_PER_SEC);
    /// let three_quater_minute = MicroSeconds(45 * MICROS_PER_SEC);
    ///
    /// assert_eq!(half_minute.checked_add(quater_minute), Some(three_quater_minute));
    /// assert_eq!(MicroSeconds::MAX.checked_add(half_minute), None);
    /// ```
    #[inline]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer addition. Computes `self + rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_add()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC, NANOS_PER_MICRO};
    /// let half_minute = MicroSeconds(30 * MICROS_PER_SEC);
    /// let duration1 = Duration::new(2, 5 * NANOS_PER_MICRO + 20); // 2s + 5us + 20ns
    /// let duration2 = Duration::new(MicroSeconds::MAX.0 / MICROS_PER_SEC, 0);
    ///
    /// assert_eq!(half_minute.checked_add_duration(duration1), Some(MicroSeconds(32_000_005)));
    /// assert_eq!(half_minute.checked_add_duration(duration2), None);
    /// ```
    #[inline]
    pub fn checked_add_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_add(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_sub()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// let quater_minute = MicroSeconds(15 * MICROS_PER_SEC);
    /// let three_quater_minute = MicroSeconds(45 * MICROS_PER_SEC);
    /// let whole_minute = MicroSeconds(60 * MICROS_PER_SEC);
    ///
    /// assert_eq!(whole_minute.checked_sub(quater_minute), Some(three_quater_minute));
    /// assert_eq!(quater_minute.checked_sub(whole_minute), None);
    /// ```
    #[inline]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer subtraction. Computes `self - rhs`, returning `None` if overflow occurred,
    /// using the inner integer’s `checked_sub()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::time::Duration;
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC, NANOS_PER_MICRO};
    /// let half_minute = MicroSeconds(30 * MICROS_PER_SEC);
    /// let duration1 = Duration::new(2, 5 * NANOS_PER_MICRO + 20); // 2s + 5us + 20ns
    /// let duration2 = Duration::new(45, 0);
    ///
    /// assert_eq!(half_minute.checked_sub_duration(duration1), Some(MicroSeconds(27_999_995)));
    /// assert_eq!(half_minute.checked_sub_duration(duration2), None);
    /// ```
    #[inline]
    pub fn checked_sub_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_sub(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer multiplication. Computes `self * rhs`, returning `None` if overflow
    /// occurred, using the inner integer’s `checked_mul()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// let quater_minute = MicroSeconds(15 * MICROS_PER_SEC);
    /// let whole_minute = MicroSeconds(60 * MICROS_PER_SEC);
    ///
    /// assert_eq!(quater_minute.checked_mul(4), Some(whole_minute));
    /// assert_eq!(MicroSeconds::MAX.checked_mul(2), None);
    /// ```
    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer division. Computes `self / rhs`, returning `None` if `rhs == 0`, using the
    /// inner integer’s `checked_div()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// let quater_minute = MicroSeconds(15 * MICROS_PER_SEC);
    /// let whole_minute = MicroSeconds(60 * MICROS_PER_SEC);
    ///
    /// assert_eq!(whole_minute.checked_div(4), Some(quater_minute));
    /// assert_eq!(whole_minute.checked_div(0), None);
    /// ```
    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    /// Checked integer remainder. Computes `self % rhs`, returning `None` if `rhs == 0`, using the
    /// inner integer’s `checked_rem()` method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use libpulse_binding::time::{MicroSeconds, MICROS_PER_SEC};
    /// let quater_minute = MicroSeconds(15 * MICROS_PER_SEC);
    /// let whole_minute = MicroSeconds(60 * MICROS_PER_SEC);
    ///
    /// assert_eq!(whole_minute.checked_rem(4), Some(MicroSeconds::ZERO));
    /// assert_eq!(whole_minute.checked_rem(7), Some(MicroSeconds(4)));
    /// assert_eq!(whole_minute.checked_rem(0), None);
    /// ```
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

#[test]
fn primatives() {
    assert_eq!(MicroSeconds::SECOND * 2, MicroSeconds(2 * super::MICROS_PER_SEC));
    //assert_eq!(2 * MicroSeconds::SECOND, MicroSeconds(2 * super::MICROS_PER_SEC)); not implemented
    assert_eq!(MicroSeconds::MILLISECOND * 2, MicroSeconds(2 * super::MICROS_PER_MILLI));
    //assert_eq!(2 * MicroSeconds::MILLISECOND, MicroSeconds(2 * super::MICROS_PER_MILLI)); not implemented
}
