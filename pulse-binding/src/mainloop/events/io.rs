//! Main loop IO events.

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
use std::rc::Rc;
use super::super::api::{MainloopApi, MainloopInnerType};

pub use capi::pa_io_event as IoEventInternal;

/// A bitmask for IO events
pub type IoEventFlagSet = capi::mainloop::pa_io_event_flags_t;

pub mod flags {
    use capi;
    use super::IoEventFlagSet;

    /// No event
    pub const NULL: IoEventFlagSet = capi::PA_IO_EVENT_NULL;
    /// Input event
    pub const INPUT: IoEventFlagSet = capi::PA_IO_EVENT_INPUT;
    /// Output event
    pub const OUTPUT: IoEventFlagSet = capi::PA_IO_EVENT_OUTPUT;
    /// Hangup event
    pub const HANGUP: IoEventFlagSet = capi::PA_IO_EVENT_HANGUP;
    /// Error event
    pub const ERROR: IoEventFlagSet = capi::PA_IO_EVENT_ERROR;
}

/// An IO event source object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct IoEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut IoEventInternal,
    owner: Rc<T>,
}

/// An IO event callback prototype
pub type IoEventCb = extern "C" fn(a: *mut MainloopApi, e: *mut IoEventInternal, fd: i32,
    events: IoEventFlagSet, userdata: *mut c_void);
/// A IO event destroy callback prototype
pub type IoEventDestroyCb = extern "C" fn(a: *mut MainloopApi, e: *mut IoEventInternal,
    userdata: *mut c_void);

impl<T> IoEvent<T>
    where T: MainloopInnerType
{
    pub(crate) fn from_raw(ptr: *mut IoEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Enable or disable IO events on this object.
    pub fn enable(&mut self, events: IoEventFlagSet) {
        let fn_ptr = (*self.owner).get_api().io_enable.unwrap();
        fn_ptr(self.ptr, events);
    }

    /// Set a function that is called when the IO event source is destroyed.
    /// Use this to free the userdata argument if required.
    pub fn set_destroy(&mut self, cb: IoEventDestroyCb) {
        let fn_ptr = (*self.owner).get_api().io_set_destroy.unwrap();
        fn_ptr(self.ptr, Some(cb));
    }
}

impl<T> Drop for IoEvent<T>
    where T: MainloopInnerType
{
    fn drop(&mut self) {
        let fn_ptr = (*self.owner).get_api().io_free.unwrap();
        fn_ptr(self.ptr);
    }
}
