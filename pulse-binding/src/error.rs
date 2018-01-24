//! Error management

// This file is part of the PulseAudio Rust language binding.
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

use capi;
use std::ffi::CStr;

/// These represent the `i32` error codes returned by many of the underlying PulseAudio C functions.
/// Beware, these enum values are positive values, whilst PA functions return them in negative form,
/// i.e. the `Invalid` variant here has a value of `3`, while functions returning this error code
/// return `-3`. This is identical to the enum provided in the PA C API.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Code {
    /// No error
    Ok = 0,
    /// Access failure
    Access,
    /// Unknown command
    Command,
    /// Invalid argument
    Invalid,
    /// Entity exists
    Exist,
    /// No such entity
    NoEntity,
    /// Connection refused
    ConnectionRefused,
    /// Protocol error
    Protocol,
    Timeout,
    /// No authentication key
    AuthKey,
    Internal,
    ConnectionTerminated,
    /// Entity killed
    Killed,
    InvalidServer,
    ModInitFailed,
    BadState,
    NoData,
    /// Incompatible protocol version
    Version,
    /// Data too large
    TooLarge,
    /// Operation not supported
    NotSupported,
    /// The error code was unknown to the client
    Unknown,
    /// Extension does not exist.
    NoExtension,
    /// Obsolete functionality.
    Obsolete,
    /// Missing implementation.
    NotImplemented,
    /// The caller forked without calling execve() and tried to reuse the context.
    Forked,
    /// An IO error happened.
    Io,
    /// Device or resource busy.
    Busy,
}

impl Code {
    /// Convert a Code to a human readable string.
    pub fn strerror(self) -> Option<&'static CStr> {
        let ptr = unsafe { capi::pa_strerror(-(self as i32)) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr) })
    }
}

/// Convert an integer error value, as returned by many PA C API functions, to a human readable
/// string.
pub fn strerror(error: i32) -> Option<&'static CStr> {
    let ptr = unsafe { capi::pa_strerror(error) };
    if ptr.is_null() {
        return None;
    }
    Some(unsafe { CStr::from_ptr(ptr) })
}
