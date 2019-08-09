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

//! Main loop timer events.
//!
//! # Notes
//!
//! Time events may be created (or reset) with either Unix time values or real-time (monotonic)
//! based values (though if the mainloop does not support monotonic time value, they may be silently
//! converted to unix time).
//!
//! Note that time events created with one form of time value can be freely restarted with the other
//! form of time value.

use std;
use std::os::raw::c_void;
use std::rc::Rc;
use libc::timeval;
use super::super::api::{MainloopApi, MainloopInnerType};
use time::{UnixTs, MonotonicTs, Timeval, USEC_INVALID};

pub use capi::pa_time_event as TimeEventInternal;

/// A timer event source
pub struct TimeEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut TimeEventInternal,
    /// Source mainloop.
    owner: Rc<T>,
    /// Saved callback closure, for later destruction.
    _saved_cb: EventCb,
}

/// A reference to a timer event source, provided to the callback, allowing modification within the
/// callback itself.
pub struct TimeEventRef<T: 'static>
    where T: MainloopInnerType
{
    ptr: *mut TimeEventInternal,
    /// Source mainloop
    owner: Rc<T>,
}

pub(crate) type EventCb = ::callbacks::MultiUseCallback<dyn FnMut(*mut TimeEventInternal),
    extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal, tv: *const timeval,
    userdata: *mut c_void)>;

impl<T> TimeEvent<T>
    where T: MainloopInnerType
{
    #[inline]
    pub(crate) fn from_raw(ptr: *mut TimeEventInternal, mainloop_inner: Rc<T>, callback: EventCb)
        -> Self
    {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner, _saved_cb: callback }
    }

    /// Restarts this timer event source (whether still running or already expired) with a new Unix
    /// time.
    #[inline]
    pub fn restart(&mut self, t: &UnixTs) {
        let fn_ptr = (*self.owner).get_api().time_restart.unwrap();
        fn_ptr(self.ptr, &(t.0).0);
    }

    /// Restarts this timer event source (whether still running or already expired) with a new
    /// monotonic time.
    pub fn restart_rt(&mut self, t: MonotonicTs) {
        assert_ne!(t.0, USEC_INVALID);
        let mut tv = Timeval::new_zero();
        tv.set_rt(t.0, (*self.owner).supports_rtclock());

        let fn_ptr = (*self.owner).get_api().time_restart.unwrap();
        fn_ptr(self.ptr, &tv.0);
    }
}

impl<T> TimeEventRef<T>
    where T: MainloopInnerType
{
    pub(crate) fn from_raw(ptr: *mut TimeEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Restarts this timer event source (whether still running or already expired) with a new Unix
    /// time.
    #[inline]
    pub fn restart(&mut self, t: &UnixTs) {
        let fn_ptr = (*self.owner).get_api().time_restart.unwrap();
        fn_ptr(self.ptr, &(t.0).0);
    }

    /// Restarts this timer event source (whether still running or already expired) with a new
    /// monotonic time.
    pub fn restart_rt(&mut self, t: MonotonicTs) {
        assert_ne!(t.0, USEC_INVALID);
        let mut tv = Timeval::new_zero();
        tv.set_rt(t.0, (*self.owner).supports_rtclock());

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
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
pub(crate)
extern "C"
fn event_cb_proxy(_: *const MainloopApi, e: *mut TimeEventInternal, _: *const timeval,
    userdata: *mut c_void)
{
    let _ = std::panic::catch_unwind(|| {
        let callback = EventCb::get_callback(userdata);
        (callback)(e);
    });
}
