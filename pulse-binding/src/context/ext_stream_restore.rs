//! Routines for controlling module-stream-restore.

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
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::ptr::{null, null_mut};
use super::{ContextInternal, Context};

pub use capi::pa_ext_stream_restore_info as InfoInternal;

/// Stores information about one entry in the stream database that is maintained by
/// module-stream-restore.
#[repr(C)]
pub struct Info {
    /// Identifier string of the stream. A string like "sink-input-by-role:" or similar followed by
    /// some arbitrary property value.
    pub name: *const c_char,
    /// The channel map for the volume field, if applicable.
    pub channel_map: ::channelmap::Map,
    /// The volume of the stream when it was seen last, if applicable and saved.
    pub volume: ::volume::CVolume,
    /// The sink/source of the stream when it was last seen, if applicable and saved.
    pub device: *const c_char,
    /// The boolean mute state of the stream when it was last seen, if applicable and saved.
    pub mute: i32,
}

impl From<InfoInternal> for Info {
    fn from(p: InfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// A wrapper object providing stream restore routines to a context.
pub struct StreamRestore {
    context: *mut ContextInternal,
    /// Used to avoid freeing the internal object when used as a weak wrapper in callbacks
    weak: bool,
}

impl Context {
    /// Returns a stream restore object linked to the current context, giving access to stream
    /// restore routines. See [`::context::ext_stream_restore`](ext_stream_restore/index.html).
    pub fn stream_restore(&self) -> StreamRestore {
        unsafe { capi::pa_context_ref(self.ptr) };
        StreamRestore::from_raw(self.ptr)
    }
}

/// Callback prototype for [`test`](struct.StreamRestore.html#method.test).
pub type TestCb = extern "C" fn(c: *mut ContextInternal, version: u32, userdata: *mut c_void);

/// Callback prototype for [`read`](struct.StreamRestore.html#method.read).
pub type ReadCb = extern "C" fn(c: *mut ContextInternal, info: *const InfoInternal, eol: i32,
    userdata: *mut c_void);

/// Callback prototype for [`set_subscribe_cb`](struct.StreamRestore.html#method.set_subscribe_cb).
pub type SubscribeCb = extern "C" fn(c: *mut ContextInternal, userdata: *mut c_void);

impl StreamRestore {
    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context, weak: false }
    }

    /// Create a new `DeviceManager` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer. This is the 'weak' version, for
    /// use in callbacks, which avoids destroying the internal object when dropped.
    pub fn from_raw_weak(context: *mut ContextInternal) -> Self {
        Self { context: context, weak: true }
    }

    /// Test if this extension module is available in the server.
    pub fn test(&mut self, cb: (TestCb, *mut c_void)) -> ::operation::Operation {
        let ptr = unsafe { capi::pa_ext_stream_restore_test(self.context, Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Read all entries from the stream database.
    pub fn read(&mut self, cb: (ReadCb, *mut c_void)) -> ::operation::Operation {
        let ptr = unsafe { capi::pa_ext_stream_restore_read(self.context, Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Store entries in the stream database.
    pub fn write(&mut self, mode: ::proplist::UpdateMode, data: &[&Info], apply_immediately: bool,
        cb: (::context::ContextSuccessCb, *mut c_void)) -> ::operation::Operation
    {
        let ptr = unsafe {
            capi::pa_ext_stream_restore_write(self.context, mode,
                std::mem::transmute(data.as_ptr()), data.len() as u32, apply_immediately as i32,
                Some(cb.0), cb.1)
        };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Delete entries from the stream database.
    pub fn delete(&mut self, streams: &[&str], cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let mut c_streams: Vec<CString> = Vec::with_capacity(streams.len());
        for stream in streams {
            c_streams.push(CString::new(stream.clone()).unwrap());
        }

        // Capture array of pointers to the above CString values.
        // We also add a `NULL` pointer entry on the end, as expected by the C function called here.
        let mut c_stream_ptrs: Vec<*const c_char> = Vec::with_capacity(c_streams.len() + 1);
        for c_stream in c_streams {
            c_stream_ptrs.push(c_stream.as_ptr());
        }
        c_stream_ptrs.push(null());

        let ptr = unsafe { capi::pa_ext_stream_restore_delete(self.context, c_stream_ptrs.as_ptr(),
            Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Subscribe to changes in the stream database.
    pub fn subscribe(&mut self, enable: bool, cb: (::context::ContextSuccessCb, *mut c_void)
        ) -> ::operation::Operation
    {
        let ptr = unsafe { capi::pa_ext_stream_restore_subscribe(self.context, enable as i32,
            Some(cb.0), cb.1) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb(&mut self, cb: (SubscribeCb, *mut c_void)) {
        unsafe { capi::pa_ext_stream_restore_set_subscribe_cb(self.context, Some(cb.0), cb.1); }
    }
}

impl Drop for StreamRestore {
    fn drop(&mut self) {
        if !self.weak {
            unsafe { capi::pa_context_unref(self.context) };
        }
        self.context = null_mut::<ContextInternal>();
    }
}
