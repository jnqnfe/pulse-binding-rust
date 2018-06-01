//! PulseAudio Rust language binding library for the 'simple' component.
//!
//! PulseAudio 'simple' provides a simple but limited synchronous playback and recording API. This
//! is a synchronous, simplified wrapper around the standard asynchronous API.

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

//! # About
//!
//! This library is a binding that allows Rust code to connect to the PulseAudio sound server via
//! PulseAudio's existing C API. This binding provides a safe(r) Rust interface which might be
//! preferred over the raw C API provided by the underlying `sys` linking crate.
//!
//! This crate provides an interface to PulseAudio's 'simple' component, and should be used in
//! addition to the general `libpulse_binding` crate.
//!
//! # Introduction
//!
//! The simple API is designed for applications with very basic sound playback or capture needs. It
//! can only support a single stream per connection and has no support for handling of complex
//! features like events, channel mappings and volume control. It is, however, very simple to use
//! and quite sufficient for many programs.
//!
//! # Usage
//!
//! Firstly, add a dependency on the crate in your program's `Cargo.toml` file. Secondly, import the
//! crate along with the general `libpulse_binding` crate to the root of your program:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//! extern crate libpulse_simple_binding as psimple;
//! ```
//!
//! Finally, establish a connection, as below.
//!
//! # Connecting
//!
//! The first step before using the sound system is to connect to the server. This is normally done
//! this way:
//!
//! ```rust
//! # extern crate libpulse_binding as pulse;
//! # extern crate libpulse_simple_binding as psimple;
//! #
//! use psimple::Simple;
//! use pulse::stream::Direction;
//!
//! # fn main() {
//! let spec = pulse::sample::Spec {
//!     format: pulse::sample::SAMPLE_S16NE,
//!     channels: 2,
//!     rate: 44100,
//! };
//! assert!(spec.is_valid());
//!
//! let s = Simple::new(
//!     None,                // Use the default server
//!     "FooApp",            // Our application's name
//!     Direction::Playback, // We want a playback stream
//!     None,                // Use the default device
//!     "Music",             // Description of our stream
//!     &spec,               // Our sample format
//!     None,                // Use default channel map
//!     None                 // Use default buffering attributes
//! ).unwrap();
//! # }
//! ```
//!
//! # Transferring data
//!
//! Once the connection is established to the server, data can start flowing. Using the connection
//! is very similar to the normal read() and write() system calls using [`read`] and [`write`]
//! methods of the [`Simple`] object. Note that these operations always block.
//!
//! # Buffer control
//!
//! * [`Simple::get_latency`]: Will return the total latency of the playback or record pipeline,
//!   respectively.
//! * [`Simple::flush`]: Will throw away all data currently in buffers.
//!
//! If a playback stream is used then the following operation is available:
//!
//! * [`Simple::drain`]: Will wait for all sent data to finish playing.
//!
//! # Cleanup
//!
//! Once playback or capture is complete, the connection should be closed and resources freed. This
//! is done automatically once the object is dropped.
//!
//! [`Simple`]: struct.Simple.html
//! [`read`]: struct.Simple.html#method.read
//! [`write`]: struct.Simple.html#method.write
//! [`Simple::get_latency`]: struct.Simple.html#method.get_latency
//! [`Simple::flush`]: struct.Simple.html#method.flush
//! [`Simple::drain`]: struct.Simple.html#method.drain

#![doc(html_logo_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/logo.png",
       html_favicon_url = "https://github.com/jnqnfe/pulse-binding-rust/raw/master/favicon.ico")]

extern crate libpulse_binding as pulse;
extern crate libpulse_sys as pcapi;
extern crate libpulse_simple_sys as capi;

use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::ptr::null;
use pulse::error::PAErr;

use capi::pa_simple as SimpleInternal;

/// An opaque simple connection object.
/// This acts as a safe Rust wrapper for the actual C object.
pub struct Simple {
    /// The actual C object.
    ptr: *mut SimpleInternal,
}

impl Simple {
    /// Create a new connection to the server.
    ///
    /// # Params
    ///
    /// * `server`: Server name, or `None` for default.
    /// * `name`: A descriptive name for this client (application name, ...).
    /// * `dir`: Open this stream for recording or playback?
    /// * `dev`: Sink (resp. source) name, or `None` for default.
    /// * `stream_name`: A descriptive name for this stream (application name, song title, ...).
    /// * `ss`: The sample type to use.
    /// * `map`: The channel map to use, or `None` for default.
    /// * `attr`: Buffering attributes, or `None` for default.
    pub fn new(server: Option<&str>, name: &str, dir: pulse::stream::Direction,
        dev: Option<&str>, stream_name: &str, ss: &pulse::sample::Spec,
        map: Option<&pulse::channelmap::Map>, attr: Option<&pulse::def::BufferAttr>
        ) -> Result<Self, PAErr>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_server = match server {
            Some(server) => CString::new(server.clone()).unwrap(),
            None => CString::new("").unwrap(),
        };
        let c_dev = match dev {
            Some(dev) => CString::new(dev.clone()).unwrap(),
            None => CString::new("").unwrap(),
        };

        let p_map: *const pcapi::pa_channel_map = match map {
            Some(map) => unsafe { std::mem::transmute(map) },
            None => null::<pcapi::pa_channel_map>(),
        };
        let p_attr: *const pcapi::pa_buffer_attr = match attr {
            Some(attr) => unsafe { std::mem::transmute(attr) },
            None => null::<pcapi::pa_buffer_attr>(),
        };
        let p_server: *const c_char = match server {
            Some(_) => c_server.as_ptr(),
            None => null::<c_char>(),
        };
        let p_dev: *const c_char = match dev {
            Some(_) => c_dev.as_ptr(),
            None => null::<c_char>(),
        };
        let c_name = CString::new(name.clone()).unwrap();
        let c_stream_name = CString::new(stream_name.clone()).unwrap();

        let mut error: i32 = 0;
        let ptr = unsafe {
            capi::pa_simple_new(
                p_server,
                c_name.as_ptr(),
                dir,
                p_dev,
                c_stream_name.as_ptr(),
                std::mem::transmute(ss),
                p_map,
                p_attr,
                &mut error
            )
        };
        if ptr.is_null() {
            return Err(PAErr(error));
        }
        Ok(Self::from_raw(ptr))
    }

    /// Create a new `Simple` from an existing [`SimpleInternal`](capi/enum.pa_simple.html) pointer.
    fn from_raw(ptr: *mut SimpleInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr }
    }

    /// Write some data to the server.
    pub fn write(&self, data: &[u8]) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_write(self.ptr, data.as_ptr() as *mut c_void, data.len(),
            &mut error) }
        {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }

    /// Wait until all data already written is played by the daemon.
    pub fn drain(&self) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_drain(self.ptr, &mut error) } {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }

    /// Read some data from the server.
    ///
    /// This function blocks until `data.len()` amount of data has been received from the server,
    /// or until an error occurs.
    pub fn read(&self, data: &mut [u8]) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_read(self.ptr, data.as_mut_ptr() as *mut c_void, data.len(),
            &mut error) }
        {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }

    /// Return the playback or record latency.
    pub fn get_latency(&self) -> Option<pulse::timeval::MicroSeconds> {
        let mut error: i32 = 0;
        let ret = unsafe { capi::pa_simple_get_latency(self.ptr, &mut error) };
        if error != 0 {
            return None;
        }
        Some(pulse::timeval::MicroSeconds(ret))
    }

    /// Flush the playback or record buffer. This discards any audio in the buffer.
    pub fn flush(&self) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_flush(self.ptr, &mut error) } {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }
}

impl Drop for Simple {
    fn drop(&mut self) {
        // Close and free the connection to the server.
        unsafe { capi::pa_simple_free(self.ptr) };
        self.ptr = null::<SimpleInternal>() as *mut SimpleInternal;
    }
}

