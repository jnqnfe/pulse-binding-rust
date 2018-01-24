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
use ::util::unwrap_optional_callback;

pub use capi::pa_operation as OperationInternal;
pub use capi::pa_operation_state_t as State;

/// An asynchronous operation object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct Operation {
    /// The actual C object.
    ptr: *mut OperationInternal,
    /// Used to avoid freeing the internal object when used as a weak wrapper in callbacks
    weak: bool,
}

/// A callback for operation state changes
pub type NotifyCb = extern "C" fn(o: *mut OperationInternal, userdata: *mut c_void);

impl Operation {
    /// Create a new `Operation` from an existing [`OperationInternal`](enum.OperationInternal.html)
    /// pointer.
    pub fn from_raw(ptr: *mut OperationInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, weak: false }
    }

    /// Create a new `Operation` from an existing [`OperationInternal`](enum.OperationInternal.html)
    /// pointer. This is the 'weak' version, for use in callbacks, which avoids destroying the
    /// internal object when dropped.
    pub fn from_raw_weak(ptr: *mut OperationInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, weak: true }
    }

    /// Cancel the operation.
    ///
    /// Beware! This will not necessarily cancel the execution of the operation on the server side.
    /// However it will make sure that the callback associated with this operation will not be
    /// called anymore, effectively disabling the operation from the client side's view.
    pub fn cancel(&self) {
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
    pub fn set_state_callback(&self, cb: Option<(NotifyCb, *mut c_void)>) {
        let (cb_f, cb_d) = unwrap_optional_callback::<NotifyCb>(cb);
        unsafe { capi::pa_operation_set_state_callback(self.ptr, cb_f, cb_d); }
    }
}

impl Drop for Operation {
    fn drop(&mut self) {
        if !self.weak {
            unsafe { capi::pa_operation_unref(self.ptr) };
        }
        self.ptr = null_mut::<OperationInternal>();
    }
}
