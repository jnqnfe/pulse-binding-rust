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

use capi;
use std::os::raw::c_void;
use std::ptr::null_mut;
use error::PAErr;
use super::api::{ApiInternal, MainloopInnerType};
use capi::pa_signal_event as EventInternal;

/// An opaque UNIX signal event source object.
///
/// Note: Saves a copy of the closure callbacks, which it frees on drop.
pub struct Event {
    /// The actual C object.
    ptr: *mut EventInternal,
    /// Saved multi-use state callback closure, for later destruction
    _signal_cb: SignalCb,
}

type SignalCb = ::callbacks::MultiUseCallback<FnMut(i32),
    extern "C" fn(*const ApiInternal, *mut EventInternal, i32, *mut c_void)>;

/// Trait with signal handling, for mainloops
pub trait MainloopSignals : ::mainloop::api::Mainloop {
    /// Initialize the UNIX signal subsystem and bind it to the specified main loop
    fn init_signals(&mut self) -> Result<(), PAErr> {
        let inner = self.inner();
        let api = inner.get_api();
        match unsafe { capi::pa_signal_init(api.into()) } {
            0 => Ok(()),
            e => Err(PAErr(e)),
        }
    }

    /// Cleanup the signal subsystem
    fn signals_done(&self) {
        unsafe { capi::pa_signal_done(); }
    }
}

impl Event {
    /// Create a new UNIX signal event source object
    ///
    /// The callback must accept an integer which represents the signal
    pub fn new<F>(sig: i32, callback: F) -> Self
        where F: FnMut(i32) + 'static
    {
        let saved = SignalCb::new(Some(Box::new(callback)));
        let (cb_fn, cb_data) = saved.get_capi_params(signal_cb_proxy);
        let ptr = unsafe { capi::pa_signal_new(sig, cb_fn, cb_data) };
        Self { ptr: ptr, _signal_cb: saved }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { capi::pa_signal_free(self.ptr) };
        self.ptr = null_mut::<EventInternal>();
    }
}

/// Proxy for signal callbacks.
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn signal_cb_proxy(_api: *const ApiInternal, _e: *mut EventInternal, sig: i32,
    userdata: *mut c_void)
{
    let callback = SignalCb::get_callback(userdata);
    callback(sig);
}
