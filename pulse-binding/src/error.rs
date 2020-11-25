// Copyright 2017 Lyndon Brown
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

//! Error management.

use std::ffi::CStr;

type ErrorInt = i32;

/// A wrapper around integer errors returned by PulseAudio. Can be converted to a `Code` variant for
/// comparison purposes if desired.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PAErr(pub ErrorInt);

/// These represent errors returned by many of the underlying PulseAudio C functions.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Code {
    /* NOTE: This enum’s variants and variant values **must** remain identical to the `sys` crate
       (C API) equivalent */
    /// No error.
    Ok = 0,
    /// Access failure.
    Access,
    /// Unknown command.
    Command,
    /// Invalid argument.
    Invalid,
    /// Entity exists.
    Exist,
    /// No such entity.
    NoEntity,
    /// Connection refused.
    ConnectionRefused,
    /// Protocol error.
    Protocol,
    Timeout,
    /// No authentication key.
    AuthKey,
    Internal,
    ConnectionTerminated,
    /// Entity killed.
    Killed,
    InvalidServer,
    ModInitFailed,
    BadState,
    NoData,
    /// Incompatible protocol version.
    Version,
    /// Data too large.
    TooLarge,
    /// Operation not supported.
    NotSupported,
    /// The error code was unknown to the client.
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
    IO,
    /// Device or resource busy.
    Busy,
}

/// Test size is equal to `sys` equivalent
#[test]
fn code_compare_capi(){
    assert_eq!(std::mem::size_of::<Code>(), std::mem::size_of::<capi::pa_error_code_t>());
    assert_eq!(std::mem::align_of::<Code>(), std::mem::align_of::<capi::pa_error_code_t>());
}

impl PAErr {
    /// Converts an integer error value, as returned by many PA C API functions, to a human readable
    /// string.
    pub fn to_string(&self) -> Option<String> {
        let ptr = unsafe { capi::pa_strerror(self.0) };
        match ptr.is_null() {
            false => Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }),
            true => None,
        }
    }
}

impl std::fmt::Display for PAErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.to_string() {
            Some(s) => write!(f, "{}", s),
            None => write!(f, ""),
        }
    }
}

impl Code {
    /// Converts a `Code` to a human readable string.
    pub fn to_string(self) -> Option<String> {
        PAErr::from(self).to_string()
    }
}

impl From<Code> for PAErr {
    #[inline]
    fn from(c: Code) -> Self {
        // Error codes are negative, `Code` enum variants are positive
        PAErr(-(c as ErrorInt))
    }
}
impl From<PAErr> for Code {
    fn from(e: PAErr) -> Self {
        // Error codes are negative, `Code` enum variants are positive
        // Note, avoid transmute - likely different sizes!
        let abs = -(e.0);
        match abs {
            x if x == Code::Ok                   as ErrorInt => Code::Ok,
            x if x == Code::Access               as ErrorInt => Code::Access,
            x if x == Code::Command              as ErrorInt => Code::Command,
            x if x == Code::Invalid              as ErrorInt => Code::Invalid,
            x if x == Code::Exist                as ErrorInt => Code::Exist,
            x if x == Code::NoEntity             as ErrorInt => Code::NoEntity,
            x if x == Code::ConnectionRefused    as ErrorInt => Code::ConnectionRefused,
            x if x == Code::Protocol             as ErrorInt => Code::Protocol,
            x if x == Code::Timeout              as ErrorInt => Code::Timeout,
            x if x == Code::AuthKey              as ErrorInt => Code::AuthKey,
            x if x == Code::Internal             as ErrorInt => Code::Internal,
            x if x == Code::ConnectionTerminated as ErrorInt => Code::ConnectionTerminated,
            x if x == Code::Killed               as ErrorInt => Code::Killed,
            x if x == Code::InvalidServer        as ErrorInt => Code::InvalidServer,
            x if x == Code::ModInitFailed        as ErrorInt => Code::ModInitFailed,
            x if x == Code::BadState             as ErrorInt => Code::BadState,
            x if x == Code::NoData               as ErrorInt => Code::NoData,
            x if x == Code::Version              as ErrorInt => Code::Version,
            x if x == Code::TooLarge             as ErrorInt => Code::TooLarge,
            x if x == Code::NotSupported         as ErrorInt => Code::NotSupported,
            x if x == Code::Unknown              as ErrorInt => Code::Unknown,
            x if x == Code::NoExtension          as ErrorInt => Code::NoExtension,
            x if x == Code::Obsolete             as ErrorInt => Code::Obsolete,
            x if x == Code::NotImplemented       as ErrorInt => Code::NotImplemented,
            x if x == Code::Forked               as ErrorInt => Code::Forked,
            x if x == Code::IO                   as ErrorInt => Code::IO,
            x if x == Code::Busy                 as ErrorInt => Code::Busy,
            _                                                => Code::Unknown,
        }
    }
}
