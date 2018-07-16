//! MicroSeconds.

// This file is part of the PulseAudio Rust language binding.
//
// Copyright (c) 2018 Lyndon Brown
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

use std;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// Microseconds. This is an unsigned 64-bit type.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct MicroSeconds(pub u64);

impl MicroSeconds {
    pub fn is_valid(&self) -> bool {
        *self != super::USEC_INVALID
    }

    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).and_then(|i| Some(MicroSeconds(i)))
    }

    pub fn checked_mul(self, rhs: u32) -> Option<Self> {
        self.0.checked_mul(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    pub fn checked_div(self, rhs: u32) -> Option<Self> {
        self.0.checked_div(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }

    pub fn checked_rem(self, rhs: u32) -> Option<Self> {
        self.0.checked_rem(rhs as u64).and_then(|i| Some(MicroSeconds(i)))
    }
}

impl Add for MicroSeconds {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        MicroSeconds(self.0 + other.0)
    }
}
impl AddAssign for MicroSeconds {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for MicroSeconds {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        MicroSeconds(self.0 - other.0)
    }
}
impl SubAssign for MicroSeconds {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for MicroSeconds {
    type Output = Self;

    fn mul(self, rhs: u32) -> Self {
        MicroSeconds(self.0 * rhs as u64)
    }
}
impl MulAssign<u32> for MicroSeconds {
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for MicroSeconds {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        MicroSeconds(self.0 / rhs as u64)
    }
}
impl DivAssign<u32> for MicroSeconds {
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Rem<u32> for MicroSeconds {
    type Output = Self;

    fn rem(self, rhs: u32) -> Self {
        MicroSeconds(self.0 % rhs as u64)
    }
}
impl RemAssign<u32> for MicroSeconds {
    fn rem_assign(&mut self, rhs: u32) {
        *self = *self % rhs;
    }
}

impl std::fmt::Display for MicroSeconds {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} µs", self.0)
    }
}
