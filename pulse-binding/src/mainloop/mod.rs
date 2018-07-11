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

//! # Overview
//!
//! Both the PulseAudio core and the PulseAudio client library use a main loop abstraction layer.
//! Due to this it is possible to embed PulseAudio into other applications easily. Three main loop
//! implementations are currently available:
//!
//! * A minimal implementation based on the C library's poll() function. (See
//!   [`::mainloop::standard`]).
//! * A variation of the previous implementation, where it runs in a separate thread. (See
//!   [`::mainloop::threaded`]).
//! * A wrapper around the GLIB main loop. Use this to embed PulseAudio into your GLIB/GTK+/GNOME
//!   programs. (See the separate `libpulse_glib_binding` crate).
//!
//! The structure [`::mainloop::api::MainloopApi`] is used as a 'vtable' for the main loop
//! abstraction.
//!
//! This mainloop abstraction layer has no direct support for UNIX signals. Generic, mainloop
//! implementation agnostic support is available through [`::mainloop::signal`].
//!
//! [`::mainloop::standard`]: standard/index.html
//! [`::mainloop::threaded`]: threaded/index.html
//! [`::mainloop::api::MainloopApi`]: api/struct.MainloopApi.html
//! [`::mainloop::signal`]: signal/index.html

pub mod api;
pub mod events;
pub mod signal;
pub mod standard;
pub mod threaded;

