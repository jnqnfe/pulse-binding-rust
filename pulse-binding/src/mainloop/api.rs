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
use super::events::io::{IoEvent, IoEventRef, IoEventInternal, IoEventFlagSet};
use super::events::timer::{TimeEvent, TimeEventRef, TimeEventInternal};
use super::events::deferred::{DeferEvent, DeferEventRef, DeferEventInternal};
use time::{Timeval, MicroSeconds, USEC_INVALID};

pub(crate) use capi::pa_mainloop_api as ApiInternal;

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
    fn get_api(&self) -> &MainloopApi;

    /// Returns `true` if the mainloop implementation supports monotonic based time events.
    fn supports_rtclock(&self) -> bool;
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
    pub api: *const MainloopApi,

    /// All implementations must provide a drop method, to be called from an actual drop call.
    pub dropfn: fn(&mut MainloopInner<T>),

    /// Whether or not the implementation supports monotonic based time events. (`true` if so).
    pub supports_rtclock: bool,
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
/// since the trait does not know about the existence of the struct attributes being accessed.
impl<T> MainloopInnerType for MainloopInner<T>
    where T: MainloopInternalType
{
    type I = T;

    /// Return opaque main loop object pointer
    fn get_ptr(&self) -> *mut T {
        self.ptr
    }

    /// Return main loop API object pointer
    fn get_api(&self) -> &MainloopApi {
        assert!(!self.api.is_null());
        unsafe { &*self.api }
    }

    fn supports_rtclock(&self) -> bool {
        self.supports_rtclock
    }
}

pub trait Mainloop {
    type MI: MainloopInnerType;

    fn inner(&self) -> Rc<Self::MI>;

    /// Create a new IO event
    ///
    /// **Note**: You must ensure that the returned event object lives for as long as you want its
    /// event(s) to fire, as its `Drop` implementation destroys the event source. I.e. if you create
    /// a new event, but then immediately drop the object returned here, no event will fire!
    ///
    /// The given callback must accept three parameters, an [`IoEventRef`] object, a copy of the
    /// given file descriptor, and an event flag set, indicating the event(s) that occurred. The
    /// [`DeferEventRef`] object gives you some opportunity to manage the event source from within
    /// it's callback execution.
    ///
    /// [`IoEventRef`]: ../events/io/struct.IoEventRef.html
    fn new_io_event(&mut self, fd: i32, events: IoEventFlagSet,
        mut callback: Box<FnMut(IoEventRef<Self::MI>, i32, IoEventFlagSet) + 'static>
        ) -> Option<IoEvent<Self::MI>>
    {
        let inner_for_wrapper = self.inner();
        let wrapper_cb = Box::new(move |ptr, fd, flags| {
            let ref_obj = IoEventRef::<Self::MI>::from_raw(ptr, Rc::clone(&inner_for_wrapper));
            callback(ref_obj, fd, flags);
        });

        let to_save = super::events::io::EventCb::new(Some(wrapper_cb));
        let (cb_fn, cb_data) = to_save.get_capi_params(super::events::io::event_cb_proxy);

        let inner = self.inner();
        let api = inner.get_api();
        let fn_ptr = api.io_new.unwrap();
        let ptr = fn_ptr(api, fd, events, cb_fn, cb_data);
        if ptr.is_null() {
            return None;
        }
        Some(IoEvent::<Self::MI>::from_raw(ptr, Rc::clone(&inner), to_save))
    }

    /// Create a new timer event
    ///
    /// **Note**: You must ensure that the returned event object lives for as long as you want its
    /// event(s) to fire, as its `Drop` implementation destroys the event source. I.e. if you create
    /// a new event, but then immediately drop the object returned here, no event will fire!
    ///
    /// The callback must take a [`TimeEventRef`] object, which gives you some opportunity to
    /// manage the event source from within it's callback execution.
    ///
    /// Example event set to fire in five seconds time:
    ///
    /// ```rust,ignore
    /// use pulse::time::{Timeval, MicroSeconds, MICROS_PER_SEC};
    /// let _t_event = mainloop.new_timer_event(
    ///     &(Timeval::new_tod().add(MicroSeconds(5 * MICROS_PER_SEC))),
    ///     Box::new(|| { println!("Timer event fired!"); }));
    /// ```
    ///
    /// [`TimeEventRef`]: ../events/timer/struct.TimeEventRef.html
    fn new_timer_event(&mut self, tv: &Timeval,
        mut callback: Box<FnMut(TimeEventRef<Self::MI>) + 'static>) -> Option<TimeEvent<Self::MI>>
    {
        let inner_for_wrapper = self.inner();
        let wrapper_cb = Box::new(move |ptr| {
            let ref_obj = TimeEventRef::<Self::MI>::from_raw(ptr, Rc::clone(&inner_for_wrapper));
            callback(ref_obj);
        });

        let to_save = super::events::timer::EventCb::new(Some(wrapper_cb));
        let (cb_fn, cb_data) = to_save.get_capi_params(super::events::timer::event_cb_proxy);

        let inner = self.inner();
        let api = inner.get_api();
        let fn_ptr = api.time_new.unwrap();
        let ptr = fn_ptr(api, &tv.0, cb_fn, cb_data);
        if ptr.is_null() {
            return None;
        }
        Some(TimeEvent::<Self::MI>::from_raw(ptr, Rc::clone(&inner), to_save))
    }

    /// Create a new monotonic-based timer event
    ///
    /// Asserts that `t` is not `USEC_INVALID`
    ///
    /// This is an alternative to the `new_timer_event` method, taking a monotonic based time value.
    /// Note that this takes the time value as a `MicroSeconds` value, rather than `&Timeval`,
    /// however beware that simply converting between the two representations is **not** enough to
    /// also convert the value between monotonic and non-monotonic.
    ///
    /// **Note**: You must ensure that the returned event object lives for as long as you want its
    /// event(s) to fire, as its `Drop` implementation destroys the event source. I.e. if you create
    /// a new event, but then immediately drop the object returned here, no event will fire!
    ///
    /// The callback must take a [`TimeEventRef`] object, which gives you some opportunity to
    /// manage the event source from within it's callback execution.
    ///
    /// Example event set to fire in five seconds time:
    ///
    /// ```rust,ignore
    /// use pulse::time::{MicroSeconds, MICROS_PER_SEC, rtclock_now};
    /// let _t_event = mainloop.new_timer_event_rt(
    ///     rtclock_now() + MicroSeconds(5 * MICROS_PER_SEC),
    ///     Box::new(|| { println!("Timer event fired!"); }));
    /// ```
    ///
    /// [`TimeEventRef`]: ../events/timer/struct.TimeEventRef.html
    fn new_timer_event_rt(&mut self, t: MicroSeconds,
        mut callback: Box<FnMut(TimeEventRef<Self::MI>) + 'static>) -> Option<TimeEvent<Self::MI>>
    {
        assert_ne!(t, USEC_INVALID);

        let inner_for_wrapper = self.inner();
        let wrapper_cb = Box::new(move |ptr| {
            let ref_obj = TimeEventRef::<Self::MI>::from_raw(ptr, Rc::clone(&inner_for_wrapper));
            callback(ref_obj);
        });

        let to_save = super::events::timer::EventCb::new(Some(wrapper_cb));
        let (cb_fn, cb_data) = to_save.get_capi_params(super::events::timer::event_cb_proxy);

        let inner = self.inner();

        let mut tv = Timeval::new_zero();
        tv.set_rt(t, inner.supports_rtclock());

        let api = inner.get_api();
        let fn_ptr = api.time_new.unwrap();
        let ptr = fn_ptr(api, &tv.0, cb_fn, cb_data);
        if ptr.is_null() {
            return None;
        }
        Some(TimeEvent::<Self::MI>::from_raw(ptr, Rc::clone(&inner), to_save))
    }

    /// Create a new deferred event
    ///
    /// **Note**: You must ensure that the returned event object lives for as long as you want its
    /// event(s) to fire, as its `Drop` implementation destroys the event source. I.e. if you create
    /// a new event, but then immediately drop the object returned here, no event will fire!
    ///
    /// The callback must take a [`DeferEventRef`] object, which gives you some opportunity to
    /// manage the event source from within it's callback execution.
    ///
    /// [`DeferEventRef`]: ../events/deferred/struct.DeferEventRef.html
    fn new_deferred_event(&mut self, mut callback: Box<FnMut(DeferEventRef<Self::MI>) + 'static>
        ) -> Option<DeferEvent<Self::MI>>
    {
        let inner_for_wrapper = self.inner();
        let wrapper_cb = Box::new(move |ptr| {
            let ref_obj = DeferEventRef::<Self::MI>::from_raw(ptr, Rc::clone(&inner_for_wrapper));
            callback(ref_obj);
        });

        let to_save = super::events::deferred::EventCb::new(Some(wrapper_cb));
        let (cb_fn, cb_data) = to_save.get_capi_params(super::events::deferred::event_cb_proxy);

        let inner = self.inner();
        let api = inner.get_api();
        let fn_ptr = api.defer_new.unwrap();
        let ptr = fn_ptr(api, cb_fn, cb_data);
        if ptr.is_null() {
            return None;
        }
        Some(DeferEvent::<Self::MI>::from_raw(ptr, Rc::clone(&inner), to_save))
    }

    /// Run the specified callback once from the main loop using an anonymous defer event.
    /// If the mainloop runs in a different thread, you need to follow the mainloop implementation's
    /// rules regarding how to safely create defer events. In particular, if you're using
    /// [`::mainloop::threaded`](../threaded/index.html), you must lock the mainloop before calling
    /// this function.
    fn once_event(&mut self, callback: Box<FnMut() + 'static>) {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(Some(callback), once_cb_proxy);

        let inner = self.inner();
        let api = inner.get_api();
        unsafe { capi::pa_mainloop_api_once(std::mem::transmute(api), cb_fn, cb_data) };
    }

    /// Call quit
    fn quit(&mut self, retval: ::def::Retval) {
        let inner = self.inner();
        let api = inner.get_api();
        let fn_ptr = api.quit.unwrap();
        fn_ptr(api, retval.0);
    }
}

/// An IO event callback prototype
pub type IoEventCb = extern "C" fn(a: *const MainloopApi, e: *mut IoEventInternal, fd: i32,
    events: IoEventFlagSet, userdata: *mut c_void);
/// A IO event destroy callback prototype
pub type IoEventDestroyCb = extern "C" fn(a: *const MainloopApi, e: *mut IoEventInternal,
    userdata: *mut c_void);

/// A time event callback prototype
pub type TimeEventCb = extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal,
    tv: *const timeval, userdata: *mut c_void);
/// A time event destroy callback prototype
pub type TimeEventDestroyCb = extern "C" fn(a: *const MainloopApi, e: *mut TimeEventInternal,
    userdata: *mut c_void);

/// A defer event callback prototype
pub type DeferEventCb = extern "C" fn(a: *const MainloopApi, e: *mut DeferEventInternal,
    userdata: *mut c_void);
/// A defer event destroy callback prototype
pub type DeferEventDestroyCb = extern "C" fn(a: *const MainloopApi, e: *mut DeferEventInternal,
    userdata: *mut c_void);

/// An abstract mainloop API vtable
#[repr(C)]
pub struct MainloopApi {
    /// A pointer to some private, arbitrary data of the main loop implementation
    pub userdata: *mut c_void,

    /// Create a new IO event source object
    pub io_new: Option<extern "C" fn(a: *const MainloopApi, fd: i32, events: IoEventFlagSet,
        cb: Option<IoEventCb>, userdata: *mut c_void) -> *mut IoEventInternal>,
    /// Enable or disable IO events on this object
    pub io_enable: Option<extern "C" fn(e: *mut IoEventInternal, events: IoEventFlagSet)>,
    /// Free a IO event source object
    pub io_free: Option<extern "C" fn(e: *mut IoEventInternal)>,
    /// Set a function that is called when the IO event source is destroyed. Use this to free the
    /// `userdata` argument if required.
    pub io_set_destroy: Option<extern "C" fn(e: *mut IoEventInternal, cb: Option<IoEventDestroyCb>)>,

    /// Create a new timer event source object for the specified Unix time
    pub time_new: Option<extern "C" fn(a: *const MainloopApi, tv: *const timeval,
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
    pub defer_new: Option<extern "C" fn(a: *const MainloopApi, cb: Option<DeferEventCb>,
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
    pub quit: Option<extern "C" fn(a: *const MainloopApi, retval: ::def::RetvalActual)>,
}

impl<'a> From<*const ApiInternal> for &'a MainloopApi {
    fn from(a: *const ApiInternal) -> Self {
        unsafe { std::mem::transmute(a) }
    }
}

impl<'a> From<&'a MainloopApi> for *const ApiInternal {
    fn from(a: &'a MainloopApi) -> Self {
        unsafe { std::mem::transmute(a) }
    }
}

/// Proxy for anonymous 'once' deferred event callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn once_cb_proxy(_: *const ApiInternal, userdata: *mut c_void) {
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = ::callbacks::get_su_callback::<FnMut()>(userdata);
    callback();
}
