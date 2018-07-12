//! Main loop abstraction layer

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
//! [`Standard`]: standard/index.html
//! [`Threaded`]: threaded/index.html
//! [`::mainloop::signal`]: signal/index.html
//! [`::mainloop::api::MainloopApi`]: api/struct.MainloopApi.html

pub mod api;
pub mod events;
pub mod signal;
pub mod standard;
pub mod threaded;

