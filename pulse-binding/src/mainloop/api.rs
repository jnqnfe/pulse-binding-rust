//! Main loop abstraction layer API.

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

use std;
use capi;
use std::os::raw::c_void;
use std::rc::Rc;
use libc::timeval;
use super::events::io::*;
use super::events::timer::*;
use super::events::deferred::*;
use timeval::Timeval;

pub use capi::pa_mainloop_api as ApiInternal;

/// This enables generic type enforcement with the opaque C objects.
pub trait MainloopInternalType {}

/// This enables generic type enforcement with MainloopInner objects, and describes mandatory
/// accessors for the internal pointers, allowing access to these pointers across the generic
/// implementations to work.
pub trait MainloopInnerType {
    type I: MainloopInternalType;

    /// Return opaque main loop object pointer
    fn get_ptr(&self) -> *mut Self::I;

    /// Return main loop API object pointer
    fn get_api(&self) -> &mut MainloopApi;
}

/// Mainloop inner wrapper.
///
/// This contains the actual main loop object pointers, holding both the pointer to the actual
/// opaque main loop C object, and the pointer to the associated API vtable.
///
/// An instance of this type will be held, in an `Rc` ref counted wrapper both in an outer Mainloop
/// wrapper, and by all event objects. With event objects holding a ref-counted copy, this both
/// gives event objects access to the API pointer, which they need, and also it allows us to ensure
/// that event objects do not outlive the main loop object (which internally owns the API object),
/// and thus ensures correct destruction order of event and main loop objects.
pub struct MainloopInner<T>
    where T: MainloopInternalType
{
    /// An opaque main loop object
    pub ptr: *mut T,

    /// The abstract main loop API vtable for the GLIB main loop object. No need to free this API as
    /// it is owned by the loop and is destroyed when the loop is freed.
    pub api: *mut MainloopApi,

    /// All implementations must provide a drop method, to be called from an actual drop call.
    pub dropfn: fn(&mut MainloopInner<T>),
}

impl<T> Drop for MainloopInner<T>
    where T: MainloopInternalType
{
    fn drop(&mut self) {
        (self.dropfn)(self);
    }
}

/// This is the actual implementation of the 'inner type' trait.
///
/// It is not possible to replace this with 'default' method implementations within the trait itself
/// since the trait does not know about the existance of the struct attributes being accessed.
impl<T> MainloopInnerType for MainloopInner<T>
    where T: MainloopInternalType
{
    type I = T;

    /// Return opaque main loop object pointer
    fn get_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Return main loop API object pointer
    fn get_api(&self) -> &mut MainloopApi {
        assert!(!self.api.is_null());
        unsafe { &mut *self.api }
    }
}

pub trait Mainloop {
    type MI: MainloopInnerType;

    fn inner(&self) -> Rc<Self::MI>;

    /// Create a new IO event
    fn new_io_event(&mut self, fd: i32, events: IoEventFlagSet, cb: (IoEventCb, *mut c_void)
        ) -> Option<IoEvent<Self::MI>>
    {
        let fn_ptr = self.inner().get_api().io_new.unwrap();
        let ptr = fn_ptr(self.inner().get_api(), fd, events, Some(cb.0), cb.1);
        if ptr.is_null() {
            return None;
        }
        Some(IoEvent::<Self::MI>::from_raw(ptr, self.inner().clone()))
    }

    /// Create a new timer event
    fn new_timer_event(&mut self, tv: &Timeval, cb: (TimeEventCb, *mut c_void)
        ) -> Option<TimeEvent<Self::MI>>
    {
        let fn_ptr = self.inner().get_api().time_new.unwrap();
        let ptr = fn_ptr(self.inner().get_api(), &tv.0, Some(cb.0), cb.1);
        if ptr.is_null() {
            return None;
        }
        Some(TimeEvent::<Self::MI>::from_raw(ptr, self.inner().clone()))
    }

    /// Create a new deferred event
    fn new_deferred_event(&mut self, cb: (DeferEventCb, *mut c_void)
        ) -> Option<DeferEvent<Self::MI>>
    {
        let fn_ptr = self.inner().get_api().defer_new.unwrap();
        let ptr = fn_ptr(self.inner().get_api(), Some(cb.0), cb.1);
        if ptr.is_null() {
            return None;
        }
        Some(DeferEvent::<Self::MI>::from_raw(ptr, self.inner().clone()))
    }

    /// Set the userdata pointer held in the api vtable object
    fn set_api_userdata(&mut self, userdata: *mut c_void) {
        self.inner().get_api().userdata = userdata;
    }

    /// Get the userdata pointer held in the api vtable object
    fn get_api_userdata(&self) -> *mut c_void {
        self.inner().get_api().userdata
    }

    /// Call quit
    fn quit(&mut self, retval: i32) {
        let fn_ptr = self.inner().get_api().quit.unwrap();
        fn_ptr(self.inner().get_api(), retval);
    }
}

/// An abstract mainloop API vtable
#[repr(C)]
pub struct MainloopApi {
    /// A pointer to some private, arbitrary data of the main loop implementation
    pub userdata: *mut c_void,

    /// Create a new IO event source object
    pub io_new: Option<extern "C" fn(a: *mut MainloopApi, fd: i32, events: IoEventFlagSet,
        cb: Option<IoEventCb>, userdata: *mut c_void) -> *mut IoEventInternal>,
    /// Enable or disable IO events on this object
    pub io_enable: Option<extern "C" fn(e: *mut IoEventInternal, events: IoEventFlagSet)>,
    /// Free a IO event source object
    pub io_free: Option<extern "C" fn(e: *mut IoEventInternal)>,
    /// Set a function that is called when the IO event source is destroyed. Use this to free the
    /// `userdata` argument if required.
    pub io_set_destroy: Option<extern "C" fn(e: *mut IoEventInternal, cb: Option<IoEventDestroyCb>)>,

    /// Create a new timer event source object for the specified Unix time
    pub time_new: Option<extern "C" fn(a: *mut MainloopApi, tv: *const timeval,
        cb: Option<TimeEventCb>, userdata: *mut c_void) -> *mut TimeEventInternal>,
    /// Restart a running or expired timer event source with a new Unix time
    pub time_restart: Option<extern "C" fn(e: *mut TimeEventInternal, tv: *const timeval)>,
    /// Free a deferred timer event source object
    pub time_free: Option<extern "C" fn(e: *mut TimeEventInternal)>,
    /// Set a function that is called when the timer event source is destroyed. Use this to free the
    /// `userdata` argument if required.
    pub time_set_destroy: Option<extern "C" fn(e: *mut TimeEventInternal,
        cb: Option<TimeEventDestroyCb>)>,

    /// Create a new deferred event source object
    pub defer_new: Option<extern "C" fn(a: *mut MainloopApi, cb: Option<DeferEventCb>,
        userdata: *mut c_void) -> *mut DeferEventInternal>,
    /// Enable or disable a deferred event source temporarily
    pub defer_enable: Option<extern "C" fn(e: *mut DeferEventInternal, b: i32)>,
    /// Free a deferred event source object
    pub defer_free: Option<extern "C" fn(e: *mut DeferEventInternal)>,
    /// Set a function that is called when the deferred event source is
    /// destroyed. Use this to free the `userdata` argument if required.
    pub defer_set_destroy: Option<extern "C" fn(e: *mut DeferEventInternal,
        cb: Option<DeferEventDestroyCb>)>,

    /// Exit the main loop and return the specified retval
    pub quit: Option<extern "C" fn(a: *mut MainloopApi, retval: i32)>,
}

pub type MainloopApiOnceCallback = extern "C" fn(m: *mut ApiInternal,
    userdata: *mut c_void);

impl MainloopApi {
    /// Run the specified callback function once from the main loop using an anonymous defer event.
    /// If the mainloop runs in a different thread, you need to follow the mainloop implementation's
    /// rules regarding how to safely create defer events. In particular, if you're using
    /// [`::mainloop::threaded`](../threaded/index.html), you must lock the mainloop before calling
    /// this function.
    pub fn mainloop_api_once(&mut self, cb: (MainloopApiOnceCallback, *mut c_void)) {
        unsafe { capi::pa_mainloop_api_once(std::mem::transmute(self), Some(cb.0), cb.1) };
    }
}
