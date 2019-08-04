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

//! Unix timestamps.

use std;
use capi;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use super::{Timeval, MicroSeconds};

/// A Unix timestamp
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct UnixTs(pub(crate) Timeval);

impl UnixTs {
    /// Current ‘time of day’
    pub fn now() -> Self {
        let mut tv = Timeval::new_zero();
        unsafe { capi::pa_gettimeofday(&mut tv.0) };
        UnixTs(tv)
    }

    /// Calculate the difference between the two specified timestamps.
    #[inline]
    pub fn diff(a: &Self, b: &Self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_diff(&(a.0).0, &(b.0).0) })
    }

    /// Return the time difference between now and self
    #[inline]
    pub fn age(&self) -> MicroSeconds {
        MicroSeconds(unsafe { capi::pa_timeval_age(&(self.0).0) })
    }

    #[inline]
    pub fn checked_add(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_add_us(other).and_then(|us| Some(UnixTs(us)))
    }

    #[inline]
    pub fn checked_sub(self, other: MicroSeconds) -> Option<Self> {
        self.0.checked_sub_us(other).and_then(|us| Some(UnixTs(us)))
    }
}

impl Add<MicroSeconds> for UnixTs {
    type Output = Self;

    #[inline]
    fn add(self, other: MicroSeconds) -> Self {
        UnixTs(self.0 + other)
    }
}
impl AddAssign<MicroSeconds> for UnixTs {
    #[inline]
    fn add_assign(&mut self, rhs: MicroSeconds) {
        *self = *self + rhs;
    }
}

impl Sub<MicroSeconds> for UnixTs {
    type Output = Self;

    #[inline]
    fn sub(self, other: MicroSeconds) -> Self {
        UnixTs(self.0 - other)
    }
}
impl SubAssign<MicroSeconds> for UnixTs {
    #[inline]
    fn sub_assign(&mut self, rhs: MicroSeconds) {
        *self = *self - rhs;
    }
}

impl std::fmt::Display for UnixTs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
