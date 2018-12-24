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

//! Main loop deferred events.

use std::os::raw::c_void;
use std::rc::Rc;
use crate::mainloop::api::{MainloopApi, MainloopInnerType};
use crate::callbacks::MultiUseCallback;

pub use capi::pa_defer_event as DeferEventInternal;

/// A deferred event source.
pub struct DeferEvent<T>
    where T: MainloopInnerType
{
    ptr: *mut DeferEventInternal,
    /// Source mainloop.
    owner: Rc<T>,
    /// Saved callback closure, for later destruction.
    _saved_cb: EventCb,
}

/// A reference to a deferred event source, provided to the callback, allowing modification within
/// the callback itself.
pub struct DeferEventRef<T: 'static>
    where T: MainloopInnerType
{
    ptr: *mut DeferEventInternal,
    /// Source mainloop.
    owner: Rc<T>,
}

pub(crate) type EventCb = MultiUseCallback<dyn FnMut(*mut DeferEventInternal),
    extern "C" fn(a: *const MainloopApi, e: *mut DeferEventInternal, userdata: *mut c_void)>;

impl<T> DeferEvent<T>
    where T: MainloopInnerType
{
    #[inline]
    pub(crate) fn from_raw(ptr: *mut DeferEventInternal, mainloop_inner: Rc<T>, callback: EventCb)
        -> Self
    {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner, _saved_cb: callback }
    }

    /// Enables this event source temporarily.
    #[inline]
    pub fn enable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 1);
    }

    /// Disables this event source temporarily.
    #[inline]
    pub fn disable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 0);
    }
}

impl<T> DeferEventRef<T>
    where T: MainloopInnerType
{
    #[inline]
    pub(crate) fn from_raw(ptr: *mut DeferEventInternal, mainloop_inner: Rc<T>) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, owner: mainloop_inner }
    }

    /// Enables this event source temporarily.
    #[inline]
    pub fn enable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 1);
    }

    /// Disables this event source temporarily.
    #[inline]
    pub fn disable(&mut self) {
        let fn_ptr = (*self.owner).get_api().defer_enable.unwrap();
        fn_ptr(self.ptr, 0);
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

/// Proxy for the event callback.
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
pub(crate)
extern "C"
fn event_cb_proxy(_: *const MainloopApi, e: *mut DeferEventInternal, userdata: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        let callback = EventCb::get_callback(userdata);
        (callback)(e);
    });
}
