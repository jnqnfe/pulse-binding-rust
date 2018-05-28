//! Sample cache mechanism.

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
//! The sample cache provides a simple way of overcoming high network latencies and reducing
//! bandwidth. Instead of streaming a sound precisely when it should be played, it is stored on the
//! server and only the command to start playing it needs to be sent.
//!
//! # Creation
//!
//! To create a sample, the normal stream API is used (see [`::stream`]). The function
//! [`::stream::Stream::connect_upload`] will make sure the stream is stored as a sample on the
//! server.
//!
//! To complete the upload, [`::stream::Stream::finish_upload`] is called and the sample will
//! receive the same name as the stream. If the upload should be aborted, simply call
//! [`::stream::Stream::disconnect`].
//!
//! # Playing samples
//!
//! To play back a sample, simply call [`::context::Context::play_sample`]:
//!
//! ```rust,ignore
//! extern crate libpulse_binding as pulse;
//!
//! use pulse::volume;
//!
//! //...
//!
//! let o = my_context.play_sample(
//!     "sample2",            // Name of my sample
//!     None,                 // Use default sink
//!     volume::VOLUME_NORM,  // Full volume
//!     None                  // Don't need a callback
//! );
//! ```
//!
//! # Removing samples
//!
//! When a sample is no longer needed, it should be removed on the server to save resources. The
//! sample is deleted using [`::context::Context::remove_sample`].
//!
//! [`::stream`]: ../../stream/index.html
//! [`::stream::Stream::connect_upload`]: ../../stream/struct.Stream.html#method.connect_upload
//! [`::stream::Stream::finish_upload`]: ../../stream/struct.Stream.html#method.finish_upload
//! [`::stream::Stream::disconnect`]: ../../stream/struct.Stream.html#method.disconnect
//! [`::context::Context::play_sample`]: ../struct.Context.html#method.play_sample
//! [`::context::Context::remove_sample`]: ../struct.Context.html#method.remove_sample

use capi;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::ptr::null;
use super::{ContextInternal, Context, ContextSuccessCb};
use ::util::unwrap_optional_callback;

/// Callback prototype for [`::context::Context::play_sample_with_proplist`].
/// The `idx` value is the index of the sink input object, or [`::def::INVALID_INDEX`] on failure.
///
/// [`::context::Context::play_sample_with_proplist`]: ../struct.Context.html#method.play_sample_with_proplist
/// [`::def::INVALID_INDEX`]: ../../def/constant.INVALID_INDEX.html
pub type ContextPlaySampleCb = extern "C" fn(c: *mut ContextInternal, idx: u32, userdata: *mut c_void);

impl Context {
    /// Remove a sample from the sample cache.
    ///
    /// Returns an operation object which may be used to cancel the operation while it is running.
    pub fn remove_sample(&self, name: &str, cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> Option<::operation::Operation>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let ptr = unsafe { capi::pa_context_remove_sample(self.ptr, c_name.as_ptr(), Some(cb.0), cb.1) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Play a sample from the sample cache to the specified device.
    ///
    /// If the sepecified device is `None` use the default sink.
    ///
    /// # Params
    ///
    /// * `name`: Name of the sample to play.
    /// * `dev`: Sink to play this sample on, or `None` for default.
    /// * `volume`: Volume to play this sample with. Starting with 0.9.15 you may pass here
    ///   [`::volume::VOLUME_INVALID`] which will leave the decision about the volume to the server
    ///   side which is a good idea.
    /// * `cb`: Call this function after successfully starting playback, or `None`.
    /// * `userdata`: Userdata to pass to the callback.
    ///
    /// [`::volume::VOLUME_INVALID`]: ../volume/constant.VOLUME_INVALID.html
    pub fn play_sample(&self, name: &str, dev: Option<&str>, volume: ::volume::Volume,
        cb: Option<(ContextSuccessCb, *mut c_void)>) -> Option<::operation::Operation>
    {
        let (cb_f, cb_d) = unwrap_optional_callback::<ContextSuccessCb>(cb);
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_dev = match dev {
            Some(dev) => CString::new(dev.clone()).unwrap(),
            None => CString::new("").unwrap(),
        };

        let p_dev: *const c_char = match dev {
            Some(_) => c_dev.as_ptr(),
            None => null::<c_char>(),
        };

        let ptr = unsafe { capi::pa_context_play_sample(self.ptr, c_name.as_ptr(), p_dev, volume,
            cb_f, cb_d) };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }

    /// Play a sample from the sample cache to the specified device, allowing specification of a
    /// property list for the playback stream.
    ///
    /// If the device is `None` use the default sink.
    ///
    /// # Params
    ///
    /// * `name`: Name of the sample to play.
    /// * `dev`: Sink to play this sample on, or `None` for default.
    /// * `volume`: Volume to play this sample with. Starting with 0.9.15 you may pass here
    ///   [`::volume::VOLUME_INVALID`] which will leave the decision about the volume to the server
    ///   side which is a good idea.
    /// * `proplist`: Property list for this sound. The property list of the cached entry will have
    ///   this merged into it.
    /// * `cb`: Call this function after successfully starting playback, or `None`.
    /// * `userdata`: Userdata to pass to the callback.
    ///
    /// [`::volume::VOLUME_INVALID`]: ../volume/constant.VOLUME_INVALID.html
    pub fn play_sample_with_proplist(&self, name: &str, dev: Option<&str>, volume: ::volume::Volume,
        proplist: &mut ::proplist::Proplist, cb: Option<(ContextPlaySampleCb, *mut c_void)>
        ) -> Option<::operation::Operation>
    {
        let (cb_f, cb_d) = unwrap_optional_callback::<ContextPlaySampleCb>(cb);
        // Warning: New CStrings will be immediately freed if not bound to a
        // variable, leading to as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_dev = match dev {
            Some(dev) => CString::new(dev.clone()).unwrap(),
            None => CString::new("").unwrap(),
        };

        let p_dev: *const c_char = match dev {
            Some(_) => c_dev.as_ptr(),
            None => null::<c_char>(),
        };

        let ptr = unsafe {
            capi::pa_context_play_sample_with_proplist(self.ptr, c_name.as_ptr(), p_dev, volume,
                proplist.ptr, cb_f, cb_d)
        };
        if ptr.is_null() {
            return None;
        }
        Some(::operation::Operation::from_raw(ptr))
    }
}
