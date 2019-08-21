// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language binding.
//
// Licensed under the MIT license or the Apache license (version 2.0), at your option. You may not
// copy, modify, or distribute this file except in compliance with said license. You can find copies
// of these licenses either in the LICENSE-MIT and LICENSE-APACHE files, or alternatively at
// <http://opensource.org/licenses/MIT> and <http://www.apache.org/licenses/LICENSE-2.0>
// respectively.
//
// Portions of documentation are copied from the LGPL 2.1+ licensed PulseAudio C headers on a
// fair-use basis, as discussed in the overall project readme (available in the git repository).

//! PulseAudio Rust language binding library for the ‘simple’ component.
//!
//! PulseAudio ‘simple’ provides a simple but limited synchronous playback and recording API. This
//! is a synchronous, simplified wrapper around the standard asynchronous API.
//!
//! # About
//!
//! This library is a binding that allows Rust code to connect to the PulseAudio sound server via
//! PulseAudio’s existing C API. This binding provides a safe(r) Rust interface which might be
//! preferred over the raw C API provided by the underlying `sys` linking crate.
//!
//! This crate provides an interface to PulseAudio’s ‘simple’ component, and should be used in
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
//! Firstly, add a dependency on the crate in your program’s `Cargo.toml` file. Secondly, import the
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
//! use pulse::sample;
//!
//! # fn main() {
//! let spec = sample::Spec {
//!     format: sample::SAMPLE_S16NE,
//!     channels: 2,
//!     rate: 44100,
//! };
//! assert!(spec.is_valid());
//!
//! let s = Simple::new(
//!     None,                // Use the default server
//!     "FooApp",            // Our application’s name
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
use std::{ffi::CString, ptr::null};
use std::mem;
use pulse::{error::PAErr, time::MicroSeconds};
use pulse::{stream, sample, channelmap, def};

use capi::pa_simple as SimpleInternal;

/// An opaque simple connection object.
pub struct Simple {
    /// The actual C object.
    ptr: *mut SimpleInternal,
}

unsafe impl Send for Simple {}
unsafe impl Sync for Simple {}

impl Simple {
    /// Creates a new connection to the server.
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
    pub fn new(server: Option<&str>, name: &str, dir: stream::Direction, dev: Option<&str>,
        stream_name: &str, ss: &sample::Spec, map: Option<&channelmap::Map>,
        attr: Option<&def::BufferAttr>) -> Result<Self, PAErr>
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

        let p_map = map.map_or(null::<pcapi::pa_channel_map>(), |m| m.as_ref());
        let p_attr = attr.map_or(null::<pcapi::pa_buffer_attr>(), |a| a.as_ref());
        let p_server = server.map_or(null::<c_char>(), |_| c_server.as_ptr() as *const c_char);
        let p_dev = dev.map_or(null::<c_char>(), |_| c_dev.as_ptr() as *const c_char);
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
                mem::transmute(ss),
                p_map,
                p_attr,
                &mut error
            )
        };
        match ptr.is_null() {
            false => Ok(Self::from_raw(ptr)),
            true => Err(PAErr(error)),
        }
    }

    /// Creates a new `Simple` from an existing [`SimpleInternal`](capi/enum.pa_simple.html) pointer.
    fn from_raw(ptr: *mut SimpleInternal) -> Self {
        assert_eq!(false, ptr.is_null());
        Self { ptr }
    }

    /// Writes some data to the server.
    pub fn write(&self, data: &[u8]) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_write(self.ptr, data.as_ptr() as *mut c_void, data.len(),
            &mut error) }
        {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }

    /// Waits until all data already written is played by the daemon.
    pub fn drain(&self) -> Result<(), PAErr> {
        let mut error: i32 = 0;
        match unsafe { capi::pa_simple_drain(self.ptr, &mut error) } {
            0 => Ok(()),
            _ => Err(PAErr(error)),
        }
    }

    /// Reads some data from the server.
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

    /// Gets the playback or record latency.
    pub fn get_latency(&self) -> Option<MicroSeconds> {
        let mut error: i32 = 0;
        let ret = unsafe { capi::pa_simple_get_latency(self.ptr, &mut error) };
        if error != 0 {
            return None;
        }
        Some(MicroSeconds(ret))
    }

    /// Flushes the playback or record buffer.
    ///
    /// This discards any audio in the buffer.
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
