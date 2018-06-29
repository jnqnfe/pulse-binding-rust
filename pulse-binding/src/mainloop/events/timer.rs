//! Main loop timer events.

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
use libc::timeval;
use super::super::api::{MainloopApi, MainloopInnerType};
use timeval::Timeval;

pub use capi::pa_time_event as TimeEventInternal;

/// A timer event source object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct TimeEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut TimeEventInternal,
    /// Source mainloop
    owner: Rc<T>,
}

/// A time event callback prototype
pub type TimeEventCb = extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal,
    tv: *const timeval, userdata: *mut c_void);
/// A time event destroy callback prototype
pub type TimeEventDestroyCb = extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal,
    userdata: *mut c_void);

impl<T> TimeEvent<T>
    where T: MainloopInnerType
{
    pub(crate) fn from_raw(ptr: *mut TimeEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Needed to support
    /// [`::context::Context::rttime_restart`](../../../context/struct.Context.html#method.rttime_restart).
    pub(crate) fn get_ptr(&self) -> *mut TimeEventInternal {
        self.ptr
    }

    /// Restart this timer event source (whether still running or already expired) with a new Unix
    /// time.
    pub fn restart(&mut self, tv: &Timeval) {
        let fn_ptr = (*self.owner).get_api().time_restart.unwrap();
        fn_ptr(self.ptr, &tv.0);
    }

    /// Set a function that is called when the timer event source is destroyed.
    /// Use this to free the userdata argument if required.
    pub fn set_destroy(&mut self, cb: TimeEventDestroyCb) {
        let fn_ptr = (*self.owner).get_api().time_set_destroy.unwrap();
        fn_ptr(self.ptr, Some(cb));
    }
}

impl<T> Drop for TimeEvent<T>
    where T: MainloopInnerType
{
    fn drop(&mut self) {
        let fn_ptr = (*self.owner).get_api().time_free.unwrap();
        fn_ptr(self.ptr);
    }
}
