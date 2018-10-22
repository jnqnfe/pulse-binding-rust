// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
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

//! Memory allocation functions.

use std;
use std::os::raw::{c_char, c_void};

/// Allocate `n` new structures of the specified type.
#[inline(always)]
pub unsafe fn pa_xnew(n: usize, k: usize) -> *mut c_void {
    assert!(n < (std::i32::MAX as usize / k));
    pa_xmalloc(n*k)
}

/// Same as [`pa_xnew`](fn.pa_xnew.html) but set the memory to zero.
#[inline(always)]
pub unsafe fn pa_xnew0(n: usize, k: usize) -> *mut c_void {
    assert!(n < (std::i32::MAX as usize / k));
    pa_xmalloc0(n*k)
}

/// Same as [`pa_xnew`](fn.pa_xnew.html) but duplicate the specified data.
#[inline(always)]
pub unsafe fn pa_xnewdup(p: *const c_void, n: usize, k: usize) -> *mut c_void {
    assert!(n < (std::i32::MAX as usize / k));
    pa_xmemdup(p, n*k)
}

/// Reallocate `n` new structures of the specified type.
#[inline(always)]
pub unsafe fn pa_xrenew(p: *mut c_void, n: usize, k: usize) -> *mut c_void {
    assert!(n < (std::i32::MAX as usize / k));
    pa_xrealloc(p, n*k)
}

extern "C" {
    /// Allocate the specified number of bytes, just like `malloc()` does.
    /// However, in case of OOM, terminate.
    pub fn pa_xmalloc(l: usize) -> *mut c_void;

    /// Same as [`pa_xmalloc`](fn.pa_xmalloc.html) , but initialize allocated memory to 0.
    pub fn pa_xmalloc0(l: usize) -> *mut c_void;

    ///  The combination of [`pa_xmalloc`](fn.pa_xmalloc.html) and `realloc()`.
    pub fn pa_xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;

    /// Free allocated memory.
    pub fn pa_xfree(p: *mut c_void);

    /// Duplicate the specified string, allocating memory with [`pa_xmalloc`](fn.pa_xmalloc.html).
    pub fn pa_xstrdup(s: *const c_char) -> *mut c_char;

    /// Duplicate the specified string, but truncate after `l` characters.
    pub fn pa_xstrndup(s: *const c_char, l: usize) -> *mut c_char;

    /// Duplicate the specified memory block.
    pub fn pa_xmemdup(p: *const c_void, l: usize) -> *mut c_void;
}
