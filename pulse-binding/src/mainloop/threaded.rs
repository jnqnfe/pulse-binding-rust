//! A variation of the standard main loop implementation, using a background thread.

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

//! # Overview
//!
//! The threaded main loop implementation is a special version of the standard main loop
//! implementation. For the basic design, see the standard main loop documentation
//! ([`::mainloop::standard`]).
//!
//! The added feature in the threaded main loop is that it spawns a new thread that runs the real
//! main loop in the background. This allows a synchronous application to use the asynchronous API
//! without risking stalling the PulseAudio library. A few synchronization primitives are available
//! to access the objects attached to the event loop safely.
//!
//! # Creation
//!
//! A [`Mainloop`] object is created using [`Mainloop::new`]. This will only allocate the required
//! structures though, so to use it the thread must also be started. This is done through
//! [`Mainloop::start`], after which you can start using the main loop.
//!
//! # Destruction
//!
//! When the PulseAudio connection has been terminated, the thread must be stopped and the
//! resources freed. Stopping the thread is done using [`Mainloop::stop`], which must be called
//! without the lock (see below) held. When that function returns, the thread is stopped and the
//! [`Mainloop`] object can be destroyed.
//!
//! Destruction of the [`Mainloop`] object is done automatically when the object falls out of scope.
//! (Rust's `Drop` trait has been implemented and takes care of it).
//!
//! # Locking
//!
//! Since the PulseAudio API doesn't allow concurrent accesses to objects, a locking scheme must be
//! used to guarantee safe usage. The threaded main loop API provides such a scheme through the
//! functions [`Mainloop::lock`] and [`Mainloop::unlock`].
//!
//! The lock is recursive, so it's safe to use it multiple times from the same thread. Just make
//! sure you call [`Mainloop::unlock`] the same number of times you called [`Mainloop::lock`].
//!
//! The lock needs to be held whenever you call any PulseAudio function that uses an object
//! associated with this main loop. Make sure you do not hold on to the lock more than necessary
//! though, as the threaded main loop stops while the lock is held.
//!
//! Example:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//!
//! use pulse::mainloop::threaded::Mainloop;
//! use pulse::stream:{Stream, State};
//!
//! fn my_check_stream_func(m: &Mainloop, s: &Stream) {
//!     m.lock();
//!
//!     let state = s.get_state();
//!
//!     m.unlock();
//!
//!     match state {
//!         State::Ready => { printf!("Stream is ready!"); },
//!         _ => { printf!("Stream is not ready!"); },
//!     }
//! }
//! ```
//!
//! # Callbacks
//!
//! Callbacks in PulseAudio are asynchronous, so they require extra care when using them together
//! with a threaded main loop.
//!
//! The easiest way to turn the callback based operations into synchronous ones, is to simply wait
//! for the callback to be called and continue from there. This is the approach chosen in
//! PulseAudio's threaded API.
//!
//! ## Basic callbacks
//!
//! For the basic case, where all that is required is to wait for the callback to be invoked, the
//! code should look something like this:
//!
//! Example:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//!
//! use std::os::raw::c_void;
//! use std::mem::transmute;
//! use pulse::mainloop::threaded::Mainloop;
//! use pulse::operation::State;
//! use pulse::stream:Stream;
//!
//! fn my_drain_stream_func(m: &mut Mainloop, s: &Stream) {
//!     m.lock();
//!
//!     let o = s.drain(Some((my_drain_callback,
//!         unsafe { transmute(m) }))).unwrap();
//!
//!     while o.get_state() == State::Running {
//!         m.wait();
//!     }
//!
//!     m.unlock();
//! }
//!
//! extern "C"
//! fn my_drain_callback(_: *mut pulse::stream::StreamInternal, _: i32, userdata: *mut c_void) {
//!     assert!(!userdata.is_null());
//!     let m: &Mainloop = unsafe { transmute(userdata) };
//!     m.signal(false);
//! }
//! ```
//!
//! The function `my_drain_stream_func` will wait for the callback to be called using
//! [`Mainloop::wait`].
//!
//! If your application is multi-threaded, then this waiting must be done inside a while loop. The
//! reason for this is that multiple threads might be using [`Mainloop::wait`] at the same time.
//! Each thread must therefore verify that it was its callback that was invoked. Also the underlying
//! OS synchronization primitives are usually not free of spurious wake-ups, so a [`Mainloop::wait`]
//! must be called within a loop even if you have only one thread waiting.
//!
//! The callback `my_drain_callback` indicates to the main function that it has been called using
//! [`Mainloop::signal`].
//!
//! As you can see, [`Mainloop::wait`] may only be called with the lock held. The same thing is true
//! for [`Mainloop::signal`], but as the lock is held before the callback is invoked, you do not
//! have to deal with that.
//!
//! The functions will not dead lock because the wait function will release the lock before waiting
//! and then regrab it once it has been signalled. For those of you familiar with threads, the
//! behaviour is that of a condition variable.
//!
//! ## Data callbacks
//!
//! For many callbacks, simply knowing that they have been called is insufficient. The callback also
//! receives some data that is desired. To access this data safely, we must extend our example a
//! bit:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//!
//! use std::os::raw::c_void;
//! use std::mem::transmute;
//! use pulse::mainloop::threaded::Mainloop;
//! use pulse::stream:Stream;
//!
//! struct DrainCbData<'a>(&'a Mainloop, Option<&mut i32>);
//!
//! fn my_drain_stream_func(m: &Mainloop, s: &mut Stream) {
//!     m.lock();
//!
//!     let mut data = DrainCbData(m, None);
//!
//!     let o = s.drain(Some((my_drain_callback,
//!         unsafe { transmute(&mut data) }))).unwrap();
//!
//!     while o.get_state() == State::Running {
//!         m.wait();
//!     }
//!
//!     assert!(!data.1.is_none());
//!     let success = *(data.1.take());
//!     m.accept(); // Allow callback to continue now
//!
//!     match success {
//!         0 => { println!("Bitter defeat..."); },
//!         _ => { println!("Success!"); },
//!     }
//!
//!     m.unlock();
//! }
//!
//! extern "C"
//! fn my_drain_callback(_: *mut pulse::stream::StreamInternal,
//!     success: mut i32, userdata: *mut c_void)
//! {
//!     assert!(!userdata.is_null());
//!     let data: &mut DrainCbData = unsafe { transmute(userdata) };
//!     data.1 = Some(&mut success);
//!     data.0.signal(true); // Signal and wait
//! }
//! ```
//!
//! The example is a bit silly as it would have been more simple to just copy the contents of
//! `success`, but for larger data structures this can be wasteful.
//!
//! The difference here compared to the basic callback is the value `true` passed to
//! [`Mainloop::signal`] and the call to [`Mainloop::accept`]. What will happen is that
//! [`Mainloop::signal`] will signal the main function and then wait. The main function is then free
//! to use the data in the callback until [`Mainloop::accept`] is called, which will allow the
//! callback to continue.
//!
//! Note that [`Mainloop::accept`] must be called some time between exiting the while loop and
//! unlocking the main loop! Failure to do so will result in a race condition. I.e. it is not okay
//! to release the lock and regrab it before calling [`Mainloop::accept`].
//!
//! ## Asynchronous callbacks
//!
//! PulseAudio also has callbacks that are completely asynchronous, meaning that they can be called
//! at any time. The threaded main loop API provides the locking mechanism to handle concurrent
//! accesses, but nothing else. Applications will have to handle communication from the callback to
//! the main program through their own mechanisms.
//!
//! The callbacks that are completely asynchronous are:
//!
//! * State callbacks for contexts, streams, etc.
//! * Subscription notifications.
//!
//! # Example
//!
//! An example program using the threaded mainloop:
//!
//! ```rust
//! extern crate libpulse_binding as pulse;
//!
//! use std::os::raw::c_void;
//! use std::mem::transmute;
//! use pulse::mainloop::threaded::Mainloop;
//! use pulse::mainloop::api::Mainloop as MainloopTrait; //Needs to be in scope
//!
//! fn main() {
//!     let spec = pulse::sample::Spec {
//!         format: pulse::sample::SAMPLE_S16NE,
//!         channels: 2,
//!         rate: 44100,
//!     };
//!     assert!(spec.is_valid());
//!
//!     let mut proplist = pulse::proplist::Proplist::new().unwrap();
//!     proplist.sets(pulse::proplist::properties::APPLICATION_NAME, "FooApp")
//!         .unwrap();
//!
//!     let mut mainloop = Mainloop::new().unwrap();
//!
//!     let mut context = pulse::context::Context::new_with_proplist(
//!         mainloop.get_api(),
//!         "FooAppContext",
//!         &mut proplist
//!         ).unwrap();
//!
//!     context.set_state_callback(Some((context_state_change_cb,
//!         unsafe { transmute(&mut mainloop) })));
//!
//!     context.connect(None, pulse::context::flags::NOFLAGS, None).unwrap();
//!
//!     mainloop.lock();
//!     mainloop.start().unwrap();
//!
//!     // Wait for context to be ready
//!     loop {
//!         match context.get_state() {
//!             pulse::context::State::Ready => { break; },
//!             pulse::context::State::Failed |
//!             pulse::context::State::Terminated => {
//!                 eprintln!("context state failed/terminated, quitting...");
//!                 mainloop.unlock();
//!                 mainloop.stop();
//!                 return;
//!             },
//!             _ => { mainloop.wait(); },
//!         }
//!     }
//!     context.set_state_callback(None);
//!
//!     let stream = pulse::stream::Stream::new(&mut context, "Music", &spec,
//!         None).unwrap();
//!
//!     stream.set_state_callback(Some((stream_state_change_cb,
//!         unsafe { transmute(&mut mainloop) })));
//!
//!     stream.connect_playback(None, None, pulse::stream::flags::START_CORKED,
//!         None, None).unwrap();
//!
//!     // Wait for stream to be ready
//!     loop {
//!         match stream.get_state() {
//!             pulse::stream::State::Ready => { break; },
//!             pulse::stream::State::Failed |
//!             pulse::stream::State::Terminated => {
//!                 eprintln!("stream state failed/terminated, quitting...");
//!                 mainloop.unlock();
//!                 mainloop.stop();
//!                 return;
//!             },
//!             _ => { mainloop.wait(); },
//!         }
//!     }
//!     stream.set_state_callback(None);
//!
//!     mainloop.unlock();
//!
//!     // Our main loop
//! #   let mut count = 0; // For automatic unit tests, we'll spin a few times
//!     loop {
//!         mainloop.lock();
//!
//!         // Write some data with stream.write()
//!
//!         if stream.is_corked().unwrap() {
//!             stream.uncork(None);
//!         }
//!
//!         let o = stream.drain(Some((drain_cb,
//!             unsafe { transmute(&mainloop) }))).unwrap();
//!         while o.get_state() == pulse::operation::State::Running {
//!             mainloop.wait();
//!         }
//!
//!         mainloop.unlock();
//!
//!         // If done writing data, call mainloop.stop() (with lock released), then break!
//! #       // Stop test getting stuck in infinite loop!
//! #       count += 1;
//! #       if count == 3 {
//! #           mainloop.stop();
//! #           break;
//! #       }
//!     }
//!
//!     // Clean shutdown
//!     mainloop.lock();
//!     stream.disconnect().unwrap();
//!     mainloop.unlock();
//! }
//!
//! extern "C"
//! fn context_state_change_cb(context: *mut pulse::context::ContextInternal, data: *mut c_void) {
//!     assert_eq!(false, data.is_null());
//!     let context = pulse::context::Context::from_raw_weak(context);
//!     let state = context.get_state();
//!     let mainloop: &Mainloop = unsafe { transmute(data) };
//!     match state {
//!         pulse::context::State::Ready |
//!         pulse::context::State::Failed |
//!         pulse::context::State::Terminated => {
//!             mainloop.signal(false);
//!         },
//!         _ => {},
//!     }
//! }
//!
//! extern "C"
//! fn stream_state_change_cb(stream: *mut pulse::stream::StreamInternal, data: *mut c_void) {
//!     assert_eq!(false, data.is_null());
//!     let stream = pulse::stream::Stream::from_raw_weak(stream);
//!     let state = stream.get_state();
//!     let mainloop: &Mainloop = unsafe { transmute(data) };
//!     match state {
//!         pulse::stream::State::Ready |
//!         pulse::stream::State::Failed |
//!         pulse::stream::State::Terminated => {
//!             mainloop.signal(false);
//!         },
//!         _ => {},
//!     }
//! }
//!
//! extern "C"
//! fn drain_cb(_: *mut pulse::stream::StreamInternal, _: i32, data: *mut c_void) {
//!     assert_eq!(false, data.is_null());
//!     let mainloop: &Mainloop = unsafe { transmute(data) };
//!     mainloop.signal(false);
//! }
//! ```
//!
//! [`::mainloop::standard`]: ../standard/index.html
//! [`Mainloop`]: struct.Mainloop.html
//! [`Mainloop::new`]: struct.Mainloop.html#method.new
//! [`Mainloop::start`]: struct.Mainloop.html#method.start
//! [`Mainloop::stop`]: struct.Mainloop.html#method.stop
//! [`Mainloop::lock`]: struct.Mainloop.html#method.lock
//! [`Mainloop::unlock`]: struct.Mainloop.html#method.unlock
//! [`Mainloop::wait`]: struct.Mainloop.html#method.wait
//! [`Mainloop::signal`]: struct.Mainloop.html#method.signal
//! [`Mainloop::accept`]: struct.Mainloop.html#method.accept

use std;
use capi;
use std::rc::Rc;
use std::ffi::CString;
use std::ptr::null_mut;

pub use capi::pa_threaded_mainloop as MainloopInternal;

impl super::api::MainloopInternalType for MainloopInternal {}

/// This acts as a safe interface to the internal PA Mainloop.
///
/// The mainloop object pointers are further enclosed here in a ref counted wrapper, allowing this
/// outer wrapper to have clean methods for creating event objects, which can cleanly pass a copy of
/// the inner ref counted mainloop object to them. Giving this to events serves two purposes,
/// firstly because they need the API pointer, secondly, it ensures that event objects do not
/// outlive the mainloop object.
pub struct Mainloop {
    /// The ref-counted inner data
    pub _inner: Rc<super::api::MainloopInner<MainloopInternal>>,
}

impl super::api::Mainloop for Mainloop {
    type MI = super::api::MainloopInner<MainloopInternal>;

    fn inner(&self) -> Rc<super::api::MainloopInner<MainloopInternal>> {
        self._inner.clone()
    }
}

impl super::api::MainloopInner<MainloopInternal> {
    fn drop_actual(&mut self) {
        unsafe { capi::pa_threaded_mainloop_free(self.ptr) };
        self.ptr = null_mut::<MainloopInternal>();
        self.api = null_mut::<::mainloop::api::MainloopApi>();
    }
}

impl Mainloop {
    /// Allocate a new threaded main loop object.
    ///
    /// You have to call [`start`](#method.start) before the event loop thread starts running.
    pub fn new() -> Option<Self> {
        let ptr = unsafe { capi::pa_threaded_mainloop_new() };
        if ptr.is_null() {
            return None;
        }
        let api_ptr = unsafe { capi::pa_threaded_mainloop_get_api(ptr) };
        assert!(!api_ptr.is_null());
        Some(
            Self {
                _inner: Rc::new(
                    super::api::MainloopInner::<MainloopInternal> {
                        ptr: ptr,
                        api: unsafe { std::mem::transmute(api_ptr) },
                        dropfn: super::api::MainloopInner::<MainloopInternal>::drop_actual,
                    }
                ),
            }
        )
    }

    /// Start the event loop thread.
    pub fn start(&self) -> Result<(), i32> {
        match unsafe { capi::pa_threaded_mainloop_start((*self._inner).ptr) } {
            0 => Ok(()),
            e => Err(e),
        }
    }

    /// Terminate the event loop thread cleanly. Make sure to unlock the mainloop object before
    /// calling this function.
    pub fn stop(&self) {
        unsafe { capi::pa_threaded_mainloop_stop((*self._inner).ptr); }
    }

    /// Lock the event loop object, effectively blocking the event loop thread from processing
    /// events. You can use this to enforce exclusive access to all objects attached to the event
    /// loop. This lock is recursive. This function may not be called inside the event loop thread.
    /// Events that are dispatched from the event loop thread are executed with this lock held.
    pub fn lock(&self) {
        unsafe { capi::pa_threaded_mainloop_lock((*self._inner).ptr); }
    }

    /// Unlock the event loop object, inverse of [`lock`](#method.lock).
    pub fn unlock(&self) {
        unsafe { capi::pa_threaded_mainloop_unlock((*self._inner).ptr); }
    }

    /// Wait for an event to be signalled by the event loop thread. You can use this to pass data
    /// from the event loop thread to the main thread in a synchronized fashion. This function may
    /// not be called inside the event loop thread. Prior to this call the event loop object needs
    /// to be locked using [`lock`](#method.lock). While waiting the lock will be released.
    /// Immediately before returning it will be acquired again. This function may spuriously wake up
    /// even without [`signal`](#method.signal) being called. You need to make sure to handle that!
    pub fn wait(&self) {
        unsafe { capi::pa_threaded_mainloop_wait((*self._inner).ptr); }
    }

    /// Signal all threads waiting for a signalling event in [`wait`](#method.wait). If
    /// `wait_for_accept` is non-zero, do not return before the signal was accepted by an
    /// [`accept`](#method.accept) call. While waiting for that condition the event loop object is
    /// unlocked.
    pub fn signal(&self, wait_for_accept: bool) {
        unsafe { capi::pa_threaded_mainloop_signal((*self._inner).ptr, wait_for_accept as i32); }
    }

    /// Accept a signal from the event thread issued with [`signal`].
    ///
    /// This call should only be used in conjunction with [`signal`] with `wait_for_accept` as
    /// `true`.
    ///
    /// [`signal`]: #method.signal
    pub fn accept(&self) {
        unsafe { capi::pa_threaded_mainloop_accept((*self._inner).ptr); }
    }

    /// Return the return value as specified with the main loop's `quit` routine (used internally by
    /// threaded mainloop).
    pub fn get_retval(&self) -> i32 {
        unsafe { capi::pa_threaded_mainloop_get_retval((*self._inner).ptr) }
    }

    /// Return the main loop abstraction layer vtable for this main loop.
    ///
    /// There is no need to free this object as it is owned by the loop and is destroyed when the
    /// loop is freed.
    ///
    /// Talking to PA directly with C requires fetching this pointer explicitly via this function.
    /// This is actually unecessary through this binding. The pointer is retrieved automatically
    /// upon Mainloop creation, stored internally, and automatically obtained from it by functions
    /// that need it.
    pub fn get_api(&self) -> &mut ::mainloop::api::MainloopApi {
        let ptr = (*self._inner).api;
        assert_eq!(false, ptr.is_null());
        unsafe { &mut *ptr }
    }

    /// Returns `true` when called from within the event loop thread.
    pub fn in_thread(&self) -> bool {
        unsafe { capi::pa_threaded_mainloop_in_thread((*self._inner).ptr) != 0 }
    }

    /// Sets the name of the thread.
    pub fn set_name(&self, name: &str) {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        unsafe { capi::pa_threaded_mainloop_set_name((*self._inner).ptr, c_name.as_ptr()); }
    }
}
