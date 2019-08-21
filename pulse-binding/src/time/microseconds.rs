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

/// Microseconds. This is an unsigned 64-bit type.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    #[inline]
    pub fn is_valid(&self) -> bool {
        *self != super::USEC_INVALID
    }

    #[inline]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_add_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_add(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_sub_duration(self, rhs: Duration) -> Option<Self> {
        let usecs = MicroSeconds::from(rhs);
        self.0.checked_sub(usecs.0).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    #[inline]
    pub fn checked_rem(self, rhs: u32) -> Option<Self> {
        self.0.checked_rem(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
}

impl Add for MicroSeconds {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        MicroSeconds(self.0 + other.0)
    }
}
impl AddAssign for MicroSeconds {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

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

impl Sub for MicroSeconds {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        MicroSeconds(self.0 - other.0)
    }
}
impl SubAssign for MicroSeconds {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
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

impl Mul<u32> for MicroSeconds {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u32) -> Self {
        MicroSeconds(self.0 * rhs as u64)
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
        MicroSeconds(self.0 / rhs as u64)
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
        MicroSeconds(self.0 % rhs as u64)
    }
}
impl RemAssign<u32> for MicroSeconds {
    #[inline]
    fn rem_assign(&mut self, rhs: u32) {
        *self = *self % rhs;
    }
}

impl std::fmt::Display for MicroSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} µs", self.0)
    }
}
