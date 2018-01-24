//! Utility functions for handling timeval calculations.

// This file is part of the PulseAudio Rust language linking library.
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

use std;
use libc::timeval;

pub const PA_MSEC_PER_SEC: ::sample::pa_usec_t = 1000;
pub const PA_USEC_PER_SEC: ::sample::pa_usec_t = 1000000;
pub const PA_NSEC_PER_SEC: u64 = 1000000000;
pub const PA_USEC_PER_MSEC: ::sample::pa_usec_t = 1000;
pub const PA_NSEC_PER_MSEC: u64 = 1000000;
pub const PA_NSEC_PER_USEC: u64 = 1000;

pub const PA_USEC_INVALID: ::sample::pa_usec_t = std::u64::MAX;

pub const PA_USEC_MAX: ::sample::pa_usec_t = std::u64::MAX - 1;

#[link(name="pulse")]
extern "C" {
    pub fn pa_gettimeofday(tv: *mut timeval) -> *mut timeval;
    pub fn pa_timeval_diff(a: *const timeval, b: *const timeval) -> ::sample::pa_usec_t;
    pub fn pa_timeval_cmp(a: *const timeval, b: *const timeval) -> i32;
    pub fn pa_timeval_age(tv: *const timeval) -> ::sample::pa_usec_t;
    pub fn pa_timeval_add(tv: *mut timeval, v: ::sample::pa_usec_t) -> *mut timeval;
    pub fn pa_timeval_sub(tv: *mut timeval, v: ::sample::pa_usec_t) -> *mut timeval;
    pub fn pa_timeval_store(tv: *mut timeval, v: ::sample::pa_usec_t) -> *mut timeval;
    pub fn pa_timeval_load(tv: *const timeval) -> ::sample::pa_usec_t;
}
