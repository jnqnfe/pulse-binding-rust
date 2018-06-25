//! Asynchronous operations.

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
use std::os::raw::c_void;
use std::ptr::null_mut;

use capi::pa_operation as OperationInternal;
pub use capi::pa_operation_state_t as State;

/// An asynchronous operation object.
/// This acts as a safe Rust wrapper for the actual C object.
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct Operation {
    /// The actual C object.
    ptr: *mut OperationInternal,
    /// Multi-use callback closure pointers
    cb_ptrs: CallbackPointers,
}

/// Holds copies of callback closure pointers, for those that are "multi-use" (may be fired multiple
/// times), for freeing at the appropriate time.
#[derive(Default)]
struct CallbackPointers {
    state: NotifyCb,
}

type NotifyCb = ::callbacks::MultiUseCallback<FnMut(),
    extern "C" fn(*mut OperationInternal, *mut c_void)>;

impl Operation {
    /// Create a new `Operation` from an existing [`OperationInternal`](enum.OperationInternal.html)
    /// pointer.
    pub(crate) fn from_raw(ptr: *mut OperationInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, cb_ptrs: Default::default() }
    }

    /// Cancel the operation.
    ///
    /// Beware! This will not necessarily cancel the execution of the operation on the server side.
    /// However it will make sure that the callback associated with this operation will not be
    /// called anymore, effectively disabling the operation from the client side's view.
    ///
    /// **Warning**, cancelling operations with *single-use* callbacks (those that are fired only
    /// once) **will** result in a memory leak. (In such cases the closure is transfered to the
    /// callback via a raw pointer, and when the callback is fired, it is reconstructed and dropped
    /// after use; cancelling callback execution means this will not happen, thus a leak occurs).
    pub fn cancel(&mut self) {
        unsafe { capi::pa_operation_cancel(self.ptr); }
    }

    /// Return the current status of the operation
    pub fn get_state(&self) -> State {
        unsafe { capi::pa_operation_get_state(self.ptr) }
    }

    /// Set the callback function that is called when the operation state changes.
    ///
    /// Usually this is not necessary, since the functions that create `Operation` objects already
    /// take a callback that is called when the operation finishes. Registering a state change
    /// callback is mainly useful, if you want to get called back also if the operation gets
    /// cancelled.
    pub fn set_state_callback(&mut self, callback: Option<Box<FnMut() + 'static>>) {
        let saved = &mut self.cb_ptrs.state;
        *saved = NotifyCb::new(callback);
        let (cb_fn, cb_data) = saved.get_capi_params(notify_cb_proxy);
        unsafe { capi::pa_operation_set_state_callback(self.ptr, cb_fn, cb_data); }
    }
}

impl Drop for Operation {
    fn drop(&mut self) {
        unsafe { capi::pa_operation_unref(self.ptr) };
        self.ptr = null_mut::<OperationInternal>();
    }
}

/// Proxy for notification callbacks.
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn notify_cb_proxy(_: *mut OperationInternal, userdata: *mut c_void) {
    assert!(!userdata.is_null());
    // Note, does NOT destroy closure callback after use - only handles pointer
    let callback = unsafe { &mut *(userdata as *mut Box<FnMut()>) };
    callback();
}
