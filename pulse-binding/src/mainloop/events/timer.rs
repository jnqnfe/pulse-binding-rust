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
use timeval::{Timeval, MicroSeconds, USEC_INVALID};

pub use capi::pa_time_event as TimeEventInternal;

/// A timer event source object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct TimeEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut TimeEventInternal,
    /// Source mainloop
    owner: Rc<T>,
    /// Saved callback closure, for later destruction
    _saved_cb: EventCb,
}

pub(crate) type EventCb = ::callbacks::MultiUseCallback<FnMut(),
    extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal, tv: *const timeval,
    userdata: *mut c_void)>;

impl<T> TimeEvent<T>
    where T: MainloopInnerType
{
    pub(crate) fn from_raw(ptr: *mut TimeEventInternal, mainloop_inner: Rc<T>, callback: EventCb
        ) -> Self
    {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner, _saved_cb: callback }
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

    /// Restart this timer event source (whether still running or already expired) with a new
    /// monotonic time.
    pub fn restart_rt(&mut self, t: MicroSeconds) {
        assert_ne!(t, USEC_INVALID);
        let mut tv = Timeval::new_zero();
        tv.set_rt(t, (*self.owner).supports_rtclock());

        let fn_ptr = (*self.owner).get_api().time_restart.unwrap();
        fn_ptr(self.ptr, &tv.0);
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

/// Proxy for the event callback.
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
pub(crate)
extern "C"
fn event_cb_proxy(_: *const MainloopApi, _: *mut TimeEventInternal, _: *const timeval,
    userdata: *mut c_void)
{
    let callback = EventCb::get_callback(userdata);
    callback();
}
