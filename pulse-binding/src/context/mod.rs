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

//! Connection contexts for asynchronous communication with a server.
//!
//! A `Context` object wraps a connection to a PulseAudio server using its native protocol.
//!
//! # Overview
//!
//! A context is the basic object for a connection to a PulseAudio server. It multiplexes commands,
//! data streams and events through a single channel.
//!
//! There is no need for more than one context per application, unless connections to multiple
//! servers are needed.
//!
//! # Operations
//!
//! All operations on the context are performed asynchronously. I.e. the client will not wait for
//! the server to complete the request. To keep track of all these in-flight operations, the
//! application is given an [`::operation::Operation`] object for each asynchronous operation.
//!
//! There are only two actions (besides reference counting) that can be performed on an
//! [`::operation::Operation`]: querying its state with [`::operation::Operation::get_state`] and
//! aborting it with [`::operation::Operation::cancel`].
//!
//! An [`::operation::Operation`] object is reference counted, so an application must make sure to
//! unreference it, even if it has no intention of using it. This however is taken care of
//! automatically in this Rust binding via the implementation of the `Drop` trait on the object.
//!
//! # Connecting
//!
//! A context must be connected to a server before any operation can be issued. Calling
//! [`Context::connect`] will initiate the connection procedure. Unlike most asynchronous
//! operations, connecting does not result in an [`::operation::Operation`] object. Instead, the
//! application should register a callback using [`Context::set_state_callback`].
//!
//! # Disconnecting
//!
//! When the sound support is no longer needed, the connection needs to be closed using
//! [`Context::disconnect`]. This is an immediate function that works synchronously.
//!
//! Since the context object has references to other objects it must be disconnected after use or
//! there is a high risk of memory leaks. If the connection has terminated by itself, then there is
//! no need to explicitly disconnect the context using [`Context::disconnect`].
//!
//! # Functions
//!
//! The sound server’s functionality can be divided into a number of subsections:
//!
//! * [`::stream`]
//! * [`::context::scache`]
//! * [`::context::introspect`]
//! * [`::context::subscribe`]
//!
//! [`Context::connect`]: struct.Context.html#method.connect
//! [`Context::disconnect`]: struct.Context.html#method.disconnect
//! [`Context::set_state_callback`]: struct.Context.html#method.set_state_callback
//! [`::context::introspect`]: ../context/introspect/index.html 
//! [`::context::scache`]: ../context/scache/index.html
//! [`::context::subscribe`]: ../context/subscribe/index.html
//! [`::operation::Operation::cancel`]: ../operation/struct.Operation.html#method.cancel
//! [`::operation::Operation::get_state`]: ../operation/struct.Operation.html#method.get_state
//! [`::operation::Operation`]: ../operation/struct.Operation.html
//! [`::stream`]: ../stream/index.html

pub mod ext_device_manager;
pub mod ext_device_restore;
pub mod ext_stream_restore;
pub mod introspect;
pub mod scache;
pub mod subscribe;

use std;
use capi;
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::ptr::{null, null_mut};
use std::rc::Rc;
use mainloop::api::MainloopInnerType;
use mainloop::events::timer::{TimeEvent, TimeEventRef};
use operation::Operation;
use error::PAErr;
use time::MonotonicTs;
use proplist::Proplist;
use callbacks::box_closure_get_capi_ptr;
use capi::pa_context as ContextInternal;

/// An opaque connection context to a daemon.
///
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct Context {
    /// The actual C object.
    pub(crate) ptr: *mut ContextInternal,
    /// Used to avoid freeing the internal object when used as a weak wrapper in callbacks.
    weak: bool,
    /// Multi-use callback closure pointers.
    cb_ptrs: CallbackPointers,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

/// Holds copies of callback closure pointers, for those that are “multi-use” (may be fired multiple
/// times), for freeing at the appropriate time.
#[derive(Default)]
struct CallbackPointers {
    set_state: NotifyCb,
    subscribe: self::subscribe::Callback,
    event: EventCb,
}

type NotifyCb = ::callbacks::MultiUseCallback<dyn FnMut(),
    extern "C" fn(*mut ContextInternal, *mut c_void)>;

type EventCb = ::callbacks::MultiUseCallback<dyn FnMut(String, Proplist),
    extern "C" fn(*mut ContextInternal, name: *const c_char, pl: *mut ::proplist::ProplistInternal,
        *mut c_void)>;

type ExtSubscribeCb = ::callbacks::MultiUseCallback<dyn FnMut(),
    extern "C" fn(*mut ContextInternal, *mut c_void)>;

/// The state of a connection context.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    /* NOTE: This enum’s variants and variant values **must** remain identical to the `sys` crate
       (C API) equivalent */
    /// The context hasn’t been connected yet.
    Unconnected,
    /// A connection is being established.
    Connecting,
    /// The client is authorizing itself to the daemon.
    Authorizing,
    /// The client is passing its application name to the daemon.
    SettingName,
    /// The connection is established, the context is ready to execute operations.
    Ready,
    /// The connection failed or was disconnected.
    Failed,
    /// The connection was terminated cleanly.
    Terminated,
}

/// Test size is equal to `sys` equivalent (duplicated here for different documentation)
#[test]
fn state_compare_capi(){
    assert_eq!(std::mem::size_of::<State>(), std::mem::size_of::<capi::pa_context_state_t>());
    assert_eq!(std::mem::align_of::<State>(), std::mem::align_of::<capi::pa_context_state_t>());
}

impl From<State> for capi::pa_context_state_t {
    #[inline]
    fn from(s: State) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}
impl From<capi::pa_context_state_t> for State {
    #[inline]
    fn from(s: capi::pa_context_state_t) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl State {
    /// Checks if the passed state is one of the connected states (returns `true` if so).
    pub fn is_good(self) -> bool {
        self == State::Connecting ||
        self == State::Authorizing ||
        self == State::SettingName ||
        self == State::Ready
    }
}

pub type FlagSet = capi::pa_context_flags_t;

/// Some special flags for contexts.
pub mod flags {
    use capi;
    use super::FlagSet;

    pub const NOFLAGS: FlagSet = capi::PA_CONTEXT_NOFLAGS;
    /// Disable autospawning of the PulseAudio daemon if required.
    pub const NOAUTOSPAWN: FlagSet = capi::PA_CONTEXT_NOAUTOSPAWN;
    /// Don’t fail if the daemon is not available when
    /// [`Context::connect`](../struct.Context.html#method.connect) is called, instead enter
    /// [`State::Connecting`](../enum.State.html#Connecting.v) state and wait for the daemon to
    /// appear.
    pub const NOFAIL: FlagSet = capi::PA_CONTEXT_NOFAIL;
}

impl Context {
    /// Instantiates a new connection context with an abstract mainloop API and an application name.
    ///
    /// It is recommended to use [`new_with_proplist`](#method.new_with_proplist) instead and
    /// specify some initial properties.
    pub fn new(mainloop: &impl ::mainloop::api::Mainloop, name: &str) -> Option<Self> {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let ptr = unsafe { capi::pa_context_new(mainloop.inner().get_api().as_ref(),
            c_name.as_ptr()) };
        match ptr.is_null() { false => Some(Self::from_raw(ptr)), true => None }
    }

    /// Instantiates a new connection context with an abstract mainloop API and an application name,
    /// and specify the initial client property list.
    pub fn new_with_proplist(mainloop: &impl ::mainloop::api::Mainloop, name: &str,
        proplist: &Proplist) -> Option<Self>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let ptr = unsafe { capi::pa_context_new_with_proplist(mainloop.inner().get_api().as_ref(),
            c_name.as_ptr(), proplist.0.ptr) };
        match ptr.is_null() { false => Some(Self::from_raw(ptr)), true => None }
    }

    /// Creates a new `Context` from an existing [`ContextInternal`](enum.ContextInternal.html)
    /// pointer.
    #[inline]
    pub(crate) fn from_raw(ptr: *mut ContextInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr: ptr, weak: false, cb_ptrs: Default::default() }
    }

    /// Sets a callback function that is called whenever the context status changes.
    pub fn set_state_callback(&mut self, callback: Option<Box<dyn FnMut() + 'static>>) {
        let saved = &mut self.cb_ptrs.set_state;
        *saved = NotifyCb::new(callback);
        let (cb_fn, cb_data) = saved.get_capi_params(notify_cb_proxy_multi);
        unsafe { capi::pa_context_set_state_callback(self.ptr, cb_fn, cb_data); }
    }

    /// Sets a callback function that is called whenever a meta/policy control event is received.
    ///
    /// The callback is given a name which represents what event occurred. The set of defined events
    /// can be extended at any time. Also, server modules may introduce additional message types so
    /// make sure that your callback function ignores messages it doesn’t know. It is also given an
    /// (owned) property list.
    pub fn set_event_callback(&mut self,
        callback: Option<Box<dyn FnMut(String, Proplist) + 'static>>)
    {
        let saved = &mut self.cb_ptrs.event;
        *saved = EventCb::new(callback);
        let (cb_fn, cb_data) = saved.get_capi_params(event_cb_proxy);
        unsafe { capi::pa_context_set_event_callback(self.ptr, cb_fn, cb_data); }
    }

    /// Gets the error number of the last failed operation.
    #[inline]
    pub fn errno(&self) -> PAErr {
        PAErr(unsafe { capi::pa_context_errno(self.ptr) })
    }

    /// Checks if some data is pending to be written to the connection (returns `true` if so).
    #[inline]
    pub fn is_pending(&self) -> bool {
        unsafe { capi::pa_context_is_pending(self.ptr) != 0 }
    }

    /// Gets the current context status.
    #[inline]
    pub fn get_state(&self) -> State {
        unsafe { capi::pa_context_get_state(self.ptr).into() }
    }

    /// Connects the context to the specified server.
    ///
    /// If server is `None`, connect to the default server. This routine may but will not always
    /// return synchronously on error. Use [`set_state_callback`](#method.set_state_callback) to be
    /// notified when the connection is established. If `flags` doesn’t have
    /// [`flags::NOAUTOSPAWN`](flags/constant.NOAUTOSPAWN.html) set and no specific server is
    /// specified or accessible, a new daemon is spawned. If `api` is not `None`, the functions
    /// specified in the structure are used when forking a new child process.
    pub fn connect(&mut self, server: Option<&str>, flags: FlagSet, api: Option<&::def::SpawnApi>)
        -> Result<(), PAErr>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_server = match server {
            Some(server) => CString::new(server.clone()).unwrap(),
            None => CString::new("").unwrap(),
        };

        let p_api = api.map_or(null::<capi::pa_spawn_api>(), |a| a.as_ref());
        let p_server = server.map_or(null::<c_char>(), |_| c_server.as_ptr() as *const c_char);

        match unsafe { capi::pa_context_connect(self.ptr, p_server, flags, p_api) } {
            0 => Ok(()),
            e => Err(PAErr(e)),
        }
    }

    /// Terminates the context connection immediately.
    #[inline]
    pub fn disconnect(&mut self) {
        unsafe { capi::pa_context_disconnect(self.ptr); }
    }

    /// Drains the context.
    ///
    /// If there is nothing to drain, the function returns `None`.
    ///
    /// Note that it can also return `None` under other conditions. Many functions in the C API
    /// perform internal state validation checks and return a null pointer if they detect a problem,
    /// just as they return a null pointer on invalid input. Other functions panic on getting a null
    /// pointer return, however this function is unique in a null pointer also signalling something
    /// useful, and it is not possible to tell the difference. However, while I feel the need to be
    /// clear about the possibility, I believe that such invalid state conditions should only occur
    /// if there were a serious bug within PA, thus you are probably safe to just ignore this and
    /// always take a `None` return to indicate only that there is nothing to drain.
    pub fn drain<F>(&mut self, callback: F) -> Option<Operation<dyn FnMut()>>
        where F: FnMut() + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut()>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_drain(self.ptr, Some(notify_cb_proxy_single), cb_data) };
        // NOTE: this function is unique in NEEDING the `Option` wrapper on the return value, since
        // a null pointer may be returned if there is nothing to drain! Do not remove it!
        match ptr.is_null() {
            false => Some(Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut()>)),
            true => None,
        }
    }

    /// Tells the daemon to exit.
    ///
    /// The returned operation is unlikely to complete successfully, since the daemon probably died
    /// before returning a success notification.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn exit_daemon<F>(&mut self, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_exit_daemon(self.ptr, Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Sets the name of the default sink.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn set_default_sink<F>(&mut self, name: &str, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_set_default_sink(self.ptr, c_name.as_ptr(),
            Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Sets the name of the default source.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn set_default_source<F>(&mut self, name: &str, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_set_default_source(self.ptr, c_name.as_ptr(),
            Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Checks if this is a connection to a local daemon.
    ///
    /// Returns `true` when the connection is to a local daemon. Returns `None` on error, for
    /// instance when no connection has been made yet.
    pub fn is_local(&self) -> Option<bool> {
        match unsafe { capi::pa_context_is_local(self.ptr) } {
            1 => Some(true),
            0 => Some(false),
            _ => None,
        }
    }

    /// Sets a different application name for context on the server.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn set_name<F>(&mut self, name: &str, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_set_name(self.ptr, c_name.as_ptr(),
            Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Gets the server name this context is connected to.
    pub fn get_server(&self) -> Option<String> {
        let ptr = unsafe { capi::pa_context_get_server(self.ptr) };
        match ptr.is_null() {
            false => Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }),
            true => None,
        }
    }

    /// Gets the protocol version of the library.
    #[inline]
    pub fn get_protocol_version(&self) -> u32 {
        unsafe { capi::pa_context_get_protocol_version(self.ptr) }
    }

    /// Gets the protocol version of the connected server.
    ///
    /// Returns `None` on error.
    pub fn get_server_protocol_version(&self) -> Option<u32> {
        match unsafe { capi::pa_context_get_server_protocol_version(self.ptr) } {
            ::def::INVALID_INDEX => None,
            r => Some(r),
        }
    }

    /// Updates the property list of the client, adding new entries.
    ///
    /// Please note that it is highly recommended to set as many properties initially via
    /// [`new_with_proplist`](#method.new_with_proplist) as possible instead a posteriori with this
    /// function, since that information may then be used to route streams of the client to the
    /// right device.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn proplist_update<F>(&mut self, mode: ::proplist::UpdateMode, pl: &Proplist, callback: F)
        -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_proplist_update(self.ptr, mode, pl.0.ptr,
            Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Updates the property list of the client, remove entries.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn proplist_remove<F>(&mut self, keys: &[&str], callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let mut c_keys: Vec<CString> = Vec::with_capacity(keys.len());
        for key in keys {
            c_keys.push(CString::new(key.clone()).unwrap());
        }

        // Capture array of pointers to the above CString values.
        // We also add a NULL pointer entry on the end, as expected by the C function called here.
        let mut c_key_ptrs: Vec<*const c_char> = Vec::with_capacity(c_keys.len() + 1);
        for c_key in c_keys {
            c_key_ptrs.push(c_key.as_ptr());
        }
        c_key_ptrs.push(null());

        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_proplist_remove(self.ptr, c_key_ptrs.as_ptr(),
            Some(success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Gets the client index this context is identified in the server with.
    ///
    /// This is useful for usage with the introspection functions, such as
    /// [`::introspect::Introspector::get_client_info`].
    ///
    /// Returns `None` on error.
    ///
    /// [`::introspect::Introspector::get_client_info`]: introspect/struct.Introspector.html#method.get_client_info
    pub fn get_index(&self) -> Option<u32> {
        match unsafe { capi::pa_context_get_index(self.ptr) } {
            ::def::INVALID_INDEX => None,
            r => Some(r),
        }
    }

    /// Creates a new timer event source for the specified time.
    ///
    /// This is an alternative to the mainloop `new_timer_event_rt` method.
    ///
    /// A reference to the mainloop object is needed, in order to associate the event object with
    /// it. The association is done to ensure the event does not outlive the mainloop.
    ///
    /// If pointer returned by underlying C function is `NULL`, `None` will be returned, otherwise a
    /// [`::mainloop::events::timer::TimeEvent`] object will be returned.
    ///
    /// Example event set to fire in five seconds time:
    ///
    /// ```rust,ignore
    /// use pulse::time::{MonotonicTs, MicroSeconds, MICROS_PER_SEC};
    /// let _t_event = context.rttime_new::<Mainloop, _>(&mainloop,
    ///     MonotonicTs::now() + MicroSeconds(5 * MICROS_PER_SEC),
    ///     || { println!("Timer event fired!"); });
    /// ```
    ///
    /// **Note**: You must ensure that the returned event object lives for as long as you want its
    /// event(s) to fire, as its `Drop` implementation destroys the event source. I.e. if you create
    /// a new event, but then immediately drop the object returned here, no event will fire!
    ///
    /// [`::mainloop::events::timer::TimeEvent`]: ../mainloop/events/timer/struct.TimeEvent.html
    pub fn rttime_new<T, F>(&self, mainloop: &impl (::mainloop::api::Mainloop<MI=T::MI>),
        time: MonotonicTs, mut callback: F) -> Option<TimeEvent<T::MI>>
        where T: ::mainloop::api::Mainloop + 'static,
              F: FnMut(TimeEventRef<T::MI>) + 'static
    {
        let inner_for_wrapper = mainloop.inner();
        let wrapper_cb = Box::new(move |ptr| {
            let ref_obj = TimeEventRef::<T::MI>::from_raw(ptr, Rc::clone(&inner_for_wrapper));
            callback(ref_obj);
        });

        let to_save = ::mainloop::events::timer::EventCb::new(Some(wrapper_cb));
        let (cb_fn, cb_data) = to_save.get_capi_params(::mainloop::events::timer::event_cb_proxy);

        let ptr = unsafe { capi::pa_context_rttime_new(self.ptr, (time.0).0,
            std::mem::transmute(cb_fn), cb_data) };
        match ptr.is_null() {
            false => Some(TimeEvent::<T::MI>::from_raw(ptr, mainloop.inner(), to_save)),
            true => None,
        }
    }

    /// Gets the optimal block size for passing around audio buffers.
    ///
    /// It is recommended to allocate buffers of the size returned here when writing audio data to
    /// playback streams, if the latency constraints permit this. It is not recommended writing
    /// larger blocks than this because usually they will then be split up internally into chunks of
    /// this size. It is not recommended writing smaller blocks than this (unless required due to
    /// latency demands) because this increases CPU usage.
    ///
    /// If `ss` is invalid, returns `None`, else returns tile size rounded down to multiple of the
    /// frame size. This is supposed to be used in a construct such as:
    ///
    /// ```rust,ignore
    /// let ss = stream.get_sample_spec().unwrap();
    /// let size = stream.get_context().get_tile_size(ss).unwrap();
    /// ```
    pub fn get_tile_size(&self, ss: &::sample::Spec) -> Option<usize> {
        // Note: C function doc comments mention possibility of passing in a NULL pointer for ss.
        // We do not allow this, since 
        match unsafe { capi::pa_context_get_tile_size(self.ptr, ss.as_ref()) } {
            std::usize::MAX => None,
            r => Some(r),
        }
    }

    /// Loads the authentication cookie from a file.
    ///
    /// This function is primarily meant for PulseAudio’s own tunnel modules, which need to load the
    /// cookie from a custom location. Applications don’t usually need to care about the cookie at
    /// all, but if it happens that you know what the authentication cookie is and your application
    /// needs to load it from a non-standard location, feel free to use this function.
    pub fn load_cookie_from_file(&mut self, cookie_file_path: &str) -> Result<(), PAErr> {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_path = CString::new(cookie_file_path.clone()).unwrap();
        match unsafe { capi::pa_context_load_cookie_from_file(self.ptr, c_path.as_ptr()) } {
            0 => Ok(()),
            e => Err(PAErr(e)),
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if !self.weak {
            unsafe { capi::pa_context_unref(self.ptr) };
        }
        self.ptr = null_mut::<ContextInternal>();
    }
}

/// Proxy for completion success callbacks.
///
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn success_cb_proxy(_: *mut ContextInternal, success: i32, userdata: *mut c_void) {
    let success_actual = match success { 0 => false, _ => true };
    let _ = std::panic::catch_unwind(|| {
        assert!(!userdata.is_null());
        // Note, destroys closure callback after use - restoring outer box means it gets dropped
        let mut callback = unsafe { Box::from_raw(userdata as *mut Box<dyn FnMut(bool)>) };
        (callback)(success_actual);
    });
}

/// Proxy for notification callbacks (single use).
///
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn notify_cb_proxy_single(_: *mut ContextInternal, userdata: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        assert!(!userdata.is_null());
        // Note, destroys closure callback after use - restoring outer box means it gets dropped
        let mut callback = unsafe { Box::from_raw(userdata as *mut Box<dyn FnMut()>) };
        (callback)();
    });
}

/// Proxy for notification callbacks (multi use).
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn notify_cb_proxy_multi(_: *mut ContextInternal, userdata: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        let callback = NotifyCb::get_callback(userdata);
        (callback)();
    });
}

/// Proxy for event callbacks.
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn event_cb_proxy(_: *mut ContextInternal, name: *const c_char,
    proplist: *mut ::proplist::ProplistInternal, userdata: *mut c_void)
{
    let _ = std::panic::catch_unwind(|| {
        assert!(!name.is_null());
        let n = {
            let tmp = unsafe { CStr::from_ptr(name) };
            tmp.to_string_lossy().into_owned()
        };
        let pl = Proplist::from_raw_weak(proplist);

        let callback = EventCb::get_callback(userdata);
        (callback)(n, pl);
    });
}

/// Proxy for extension test callbacks.
///
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn ext_test_cb_proxy(_: *mut ContextInternal, version: u32, userdata: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        // Note, destroys closure callback after use - restoring outer box means it gets dropped
        let mut callback = ::callbacks::get_su_callback::<dyn FnMut(u32)>(userdata);
        (callback)(version);
    });
}

/// Proxy for extension subscribe callbacks.
///
/// Warning: This is for multi-use cases! It does **not** destroy the actual closure callback, which
/// must be accomplished separately to avoid a memory leak.
extern "C"
fn ext_subscribe_cb_proxy(_: *mut ContextInternal, userdata: *mut c_void) {
    let _ = std::panic::catch_unwind(|| {
        let callback = ExtSubscribeCb::get_callback(userdata);
        (callback)();
    });
}
