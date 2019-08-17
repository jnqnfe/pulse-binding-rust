// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.

//! Main loop IO events.

use std::os::raw::c_void;
use std::rc::Rc;
use crate::mainloop::api::{MainloopApi, MainloopInnerType};
use crate::callbacks::MultiUseCallback;

pub use capi::pa_io_event as IoEventInternal;

/// A bitmask for IO events.
pub type IoEventFlagSet = capi::mainloop::pa_io_event_flags_t;

pub mod flags {
    use capi;
    use super::IoEventFlagSet;

    /// No event.
    pub const NULL: IoEventFlagSet = capi::PA_IO_EVENT_NULL;
    /// Input event.
    pub const INPUT: IoEventFlagSet = capi::PA_IO_EVENT_INPUT;
    /// Output event.
    pub const OUTPUT: IoEventFlagSet = capi::PA_IO_EVENT_OUTPUT;
    /// Hangup event.
    pub const HANGUP: IoEventFlagSet = capi::PA_IO_EVENT_HANGUP;
    /// Error event.
    pub const ERROR: IoEventFlagSet = capi::PA_IO_EVENT_ERROR;
}

/// An IO event source
pub struct IoEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut IoEventInternal,
    /// Source mainloop.
    owner: Rc<T>,
    /// Saved callback closure, for later destruction.
    _saved_cb: EventCb,
}

/// A reference to an IO event source, provided to the callback, allowing modification within the
/// callback itself.
pub struct IoEventRef<T: 'static>
    where T: MainloopInnerType
{
    ptr: *mut IoEventInternal,
    /// Source mainloop.
    owner: Rc<T>,
}

pub(crate) type EventCb = MultiUseCallback<dyn FnMut(*mut IoEventInternal, i32, IoEventFlagSet),
    extern "C" fn(a: *const MainloopApi, e: *mut IoEventInternal, fd: i32, events: IoEventFlagSet,
    userdata: *mut c_void)>;

impl<T> IoEvent<T>
    where T: MainloopInnerType
{
    #[inline]
    pub(crate) fn from_raw(ptr: *mut IoEventInternal, mainloop_inner: Rc<T>, callback: EventCb)
        -> Self
    {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner, _saved_cb: callback }
    }

    /// Enables or disables IO events on this object.
    #[inline]
    pub fn enable(&mut self, events: IoEventFlagSet) {
        let fn_ptr = (*self.owner).get_api().io_enable.unwrap();
        fn_ptr(self.ptr, events);
    }
}

impl<T> IoEventRef<T>
    where T: MainloopInnerType
{
    #[inline]
    pub(crate) fn from_raw(ptr: *mut IoEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Enables or disables IO events on this object.
    #[inline]
    pub fn enable(&mut self, events: IoEventFlagSet) {
        let fn_ptr = (*self.owner).get_api().io_enable.unwrap();
        fn_ptr(self.ptr, events);
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

/// Proxy for the event callback.
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
pub(crate)
extern "C"
fn event_cb_proxy(_: *const MainloopApi, e: *mut IoEventInternal, fd: i32, events: IoEventFlagSet,
    userdata: *mut c_void)
{
    let _ = std::panic::catch_unwind(|| {
        let callback = EventCb::get_callback(userdata);
        (callback)(e, fd, events);
    });
}
