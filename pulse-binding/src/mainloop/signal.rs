//! UNIX signal support for main loops.

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

//! # Overview
//!
//! In contrast to other main loop event sources such as timer and IO events, UNIX signal support
//! requires modification of the global process environment. Due to this the generic main loop
//! abstraction layer as defined in [`::mainloop::api`](../api/index.html) doesn't have direct
//! support for UNIX signals. However, you may hook signal support into an abstract main loop via
//! the routines defined herein.

use std;
use capi;
use std::os::raw::c_void;
use std::ptr::null_mut;

pub use capi::pa_signal_event as EventInternal;

/// An opaque UNIX signal event source object
/// This acts as a safe Rust wrapper for the actual C object.
pub struct Event {
    /// The actual C object.
    ptr: *mut EventInternal,
}

/// Callback prototype for signal events
pub type SignalCb = extern "C" fn(api: *mut capi::pa_mainloop_api, e: *mut EventInternal, sig: i32,
    userdata: *mut c_void);

/// Destroy callback prototype for signal events
pub type DestroyCb = extern "C" fn(api: *mut capi::pa_mainloop_api, e: *mut EventInternal,
    userdata: *mut c_void);

impl ::mainloop::api::MainloopApi {
    /// Initialize the UNIX signal subsystem and bind it to the specified main loop
    pub fn init_signals(&mut self) -> Result<(), i32> {
        match unsafe { capi::pa_signal_init(std::mem::transmute(self)) } {
            0 => Ok(()),
            e => Err(e),
        }
    }

    /// Cleanup the signal subsystem
    pub fn signals_done(&self) {
        unsafe { capi::pa_signal_done(); }
    }
}

impl Event {
    /// Create a new UNIX signal event source object
    pub fn new(sig: i32, cb: (SignalCb, *mut c_void)) -> Self {
        Self {
            ptr: unsafe { capi::pa_signal_new(sig, Some(cb.0), cb.1) },
        }
    }

    /// Set a function that is called when the signal event source is destroyed. Use this to free
    /// the userdata argument if required
    pub fn signal_set_destroy(&mut self, callback: DestroyCb) {
        unsafe { capi::pa_signal_set_destroy(self.ptr, Some(callback)); }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { capi::pa_signal_free(self.ptr) };
        self.ptr = null_mut::<EventInternal>();
    }
}
