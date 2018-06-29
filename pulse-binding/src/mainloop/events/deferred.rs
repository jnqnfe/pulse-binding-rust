//! Main loop deferred events.

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

use std::os::raw::c_void;
use std::rc::Rc;
use super::super::api::{MainloopApi, MainloopInnerType};

pub use capi::pa_defer_event as DeferEventInternal;

/// A deferred event source object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct DeferEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut DeferEventInternal,
    /// Source mainloop
    owner: Rc<T>,
}

/// A defer event callback prototype
pub type DeferEventCb = extern "C" fn(a: *const MainloopApi, e: *mut DeferEventInternal,
    userdata: *mut c_void);
/// A defer event destroy callback prototype
pub type DeferEventDestroyCb = extern "C" fn(a: *const MainloopApi, e: *mut DeferEventInternal,
    userdata: *mut c_void);

impl<T> DeferEvent<T>
    where T: MainloopInnerType
{
    pub(crate) fn from_raw(ptr: *mut DeferEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Enable this event source temporarily.
    pub fn enable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 1);
    }

    /// Disable this event source temporarily.
    pub fn disable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 0);
    }

    /// Set a function that is called when the deferred event source is destroyed. Use this to free
    /// the userdata argument if required.
    pub fn set_destroy_cb(&mut self, cb: DeferEventDestroyCb) {
        let fn_ptr = (*self.owner).get_api().defer_set_destroy.unwrap();
        fn_ptr(self.ptr, Some(cb));
    }
}

impl<T> Drop for DeferEvent<T>
    where T: MainloopInnerType
{
    fn drop(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_free.unwrap();
        fn_ptr(self.ptr);
    }
}
