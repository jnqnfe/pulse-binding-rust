// Copyright 2017 Lyndon Brown
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

//! Asynchronous operations.

use capi;
use std::os::raw::c_void;
use std::ptr::null_mut;

use capi::pa_operation as OperationInternal;
pub use capi::pa_operation_state_t as State;

/// An asynchronous operation object.
///
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct Operation<ClosureProto: ?Sized> {
    /// The actual C object.
    ptr: *mut OperationInternal,
    /// The operation’s associated closure callback.
    /// This is a copy of the callback userdata pointer given in the C API function call that
    /// generated the operation instance (except not cast to void). It is saved here in case the
    /// user tries to cancel execution of the callback (with the `cancel` method), in which case we
    /// need this in order to release the memory.
    saved_cb: Option<*mut Box<ClosureProto>>,
    /// Saved multi-use state callback closure, for later destruction
    state_cb: NotifyCb,
}

type NotifyCb = ::callbacks::MultiUseCallback<FnMut(),
    extern "C" fn(*mut OperationInternal, *mut c_void)>;

impl<ClosureProto: ?Sized> Operation<ClosureProto> {
    /// Create a new `Operation` from an existing [`OperationInternal`](enum.OperationInternal.html)
    /// pointer. We also take a copy of the closure callback pointer, in order to free the memory
    /// on cancellation.
    pub(crate) fn from_raw(ptr: *mut OperationInternal, saved_cb: *mut Box<ClosureProto>)
        -> Self
    {
        assert_eq!(false, ptr.is_null());
        let saved_cb_actual = match saved_cb.is_null() {
            true => Some(saved_cb),
            false => None,
        };
        Self { ptr: ptr, saved_cb: saved_cb_actual, state_cb: Default::default() }
    }

    /// Cancel the operation.
    ///
    /// Beware! This will not necessarily cancel the execution of the operation on the server side.
    /// However it will make sure that the callback associated with this operation will not be
    /// called any more, effectively disabling the operation from the client side’s view.
    ///
    /// **Warning**, you should **never** attempt to use this to cancel a callback from within the
    /// execution of that callback itself. This should go without saying, since it makes absolutely
    /// no sense to try and do this, but be aware that this is not supported by the C API and
    /// **will** break things.
    pub fn cancel(&mut self) {
        unsafe { capi::pa_operation_cancel(self.ptr); }
        // Release the memory allocated for the closure.
        // Note, we `take()` here to help avoid issues if this function is mistakenly called more
        // than once.
        let callback = self.saved_cb.take();
        if let Some(ptr) = callback {
            if !ptr.is_null() {
                drop(unsafe { Box::from_raw(ptr as *mut Box<ClosureProto>) });
            }
        }
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
        let saved = &mut self.state_cb;
        *saved = NotifyCb::new(callback);
        let (cb_fn, cb_data) = saved.get_capi_params(notify_cb_proxy);
        unsafe { capi::pa_operation_set_state_callback(self.ptr, cb_fn, cb_data); }
    }
}

impl<ClosureProto: ?Sized> Drop for Operation<ClosureProto> {
    fn drop(&mut self) {
        // Note, we deliberately do not destroy the `saved_cb` closure here. That should only be
        // destroyed either separately by a callback proxy, or by the `Operation`’s `cancel` method.
        unsafe { capi::pa_operation_unref(self.ptr) };
        self.ptr = null_mut::<OperationInternal>();
    }
}

/// Proxy for notification callbacks.
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn notify_cb_proxy(_: *mut OperationInternal, userdata: *mut c_void) {
    let callback = NotifyCb::get_callback(userdata);
    callback();
}
