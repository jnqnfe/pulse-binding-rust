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

//! Main loop abstraction layer
//!
//! # Main Loop Abstraction
//!
//! Both the PulseAudio core and the PulseAudio client library use a main loop abstraction layer.
//! Due to this it is possible to embed PulseAudio into other applications easily.
//!
//! This abstraction contains three basic elements:
//!
//! * Deferred events: Events that will trigger as soon as possible. Note that some implementations
//!   may block all other events when a deferred event is active.
//! * I/O events: Events that trigger on file descriptor activities.
//! * Timer events: Events that trigger after a fixed amount of time.
//!
//! The abstraction is represented as a number of function pointers in the
//! [`::mainloop::api::MainloopApi`] structure.
//!
//! To actually be able to use these functions, an implementation needs to be coupled to the
//! abstraction. There are three of these shipped with PulseAudio, but any other can be used with a
//! minimal amount of work, provided it supports the three basic events listed above.
//!
//! The implementations shipped with PulseAudio are:
//!
//! * [`Standard`]: A minimal but fast implementation based on the C library's poll() function.
//! * [`Threaded`]: A special version of the previous implementation where all of PulseAudio's
//!   internal handling runs in a separate thread.
//! * 'Glib': A wrapper around GLib's main loop. This is provided in the separate
//!   `libpulse_glib_binding` crate.
//!
//! UNIX signals may be hooked to a main loop using the functionality from [`::mainloop::signal`].
//! This relies only on the main loop abstraction and can therefore be used with any of the
//! implementations.
//!
//! # Callback Notes
//!
//! ## Execution
//!
//! As described in the [standard mainloop documentation], there are three phases to mainloop
//! execution, and the third - 'dispatch' - is when user callbacks get executed.
//!
//! It is important to understand that while it is *typical* that user callbacks are executed
//! by the mainloop's dispatcher, callback execution is not exclusively done there; in some cases
//! callbacks get executed directly in synchronous function execution. For instance, if you set up
//! a context state change callback, then try to connect the context object, execution of the
//! 'connect' function call involves (internally within the PulseAudio client library) direct
//! execution of this callback in setting the initial connection state. After returning, the
//! callback is then on only executed asynchronously from the mainloop's dispatcher.
//!
//! While execution using the [`Standard`] mainloop is entirely synchronous, the [`Threaded`]
//! mainloop implementation runs the standard mainloop in a separate thread and callback execution
//! occurs asynchronously, requiring careful use of the mainloop's `lock` method. When writing
//! callbacks with the [`Threaded`] mainloop, users must beware the potential that in a few cases
//! the callback may be executed in two different scenarios, and with different threads. Note that
//! the threaded mainloop has an [`in_thread`] method for determining whether or not the thread it
//! it is executed from is the special event loop thread.
//!
//! ## Queued Events and Changing Callbacks
//!
//! It is also worth understanding that any events that get queued for dispatch do **not** hold
//! cached copies of user callback parameters. Where applicable, you can thus freely and safely
//! change the set callback, with that change taking effect immediately to all future event
//! dispatching.
//!
//! ## Threading and `Rc`
//!
//! Normally when holding multiple references to objects across threads in Rust you would use an
//! `Arc` wrapper. However, with the [`Threaded`] mainloop, you may be able to get away with using
//! just an `Rc` wrapper. Remember that with the [`Threaded`] mainloop you **must** use it's `lock`
//! method to synchronise access to objects, and so you know that at any one moment either your
//! thread (when you take the lock) **or** the event loop thread hold the lock, never both, and thus
//! only one thread is ever working with objects at any one time, and since Rust actually has no
//! idea that more than one thread is involved (hidden in the C library's implementation), you can
//! safely get away with using `Rc`.
//!
//! [`Standard`]: standard/index.html
//! [`Threaded`]: threaded/index.html
//! [`::mainloop::signal`]: signal/index.html
//! [`::mainloop::api::MainloopApi`]: api/struct.MainloopApi.html
//! [standard mainloop documentation]: standard/index.html
//! [`in_thread`]: threaded/struct.Mainloop.html#method.in_thread

pub mod api;
pub mod events;
pub mod signal;
pub mod standard;
pub mod threaded;

