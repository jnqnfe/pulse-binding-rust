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

//! Routines for controlling module-stream-restore.

use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::ptr::{null, null_mut};
use std::mem;
use capi::pa_ext_stream_restore_info as InfoInternal;
use super::{ContextInternal, Context};
use crate::{channelmap, proplist};
use crate::callbacks::{ListResult, box_closure_get_capi_ptr, callback_for_list_instance, ListInstanceCallback};
use crate::{operation::Operation, volume::ChannelVolumes};

/// Stores information about one entry in the stream database that is maintained by
/// module-stream-restore.
#[derive(Debug)]
pub struct Info<'a> {
    /// Identifier string of the stream. A string like “sink-input-by-role:” or similar followed by
    /// some arbitrary property value.
    pub name: Option<Cow<'a, str>>,
    /// The channel map for the volume field, if applicable.
    pub channel_map: channelmap::Map,
    /// The volume of the stream when it was seen last, if applicable and saved.
    pub volume: ChannelVolumes,
    /// The sink/source of the stream when it was last seen, if applicable and saved.
    pub device: Option<Cow<'a, str>>,
    /// The boolean mute state of the stream when it was last seen, if applicable and saved.
    pub mute: bool,
}

impl<'a> Info<'a> {
    fn new_from_raw(p: *const InfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            Info {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                channel_map: mem::transmute(src.channel_map),
                volume: mem::transmute(src.volume),
                device: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.device).to_string_lossy()),
                    true => None,
                },
                mute: match src.mute { 0 => false, _ => true },
            }
        }
    }
}

/// A wrapper object providing stream restore routines to a context.
///
/// Note: Saves a copy of active multi-use closure callbacks, which it frees on drop.
pub struct StreamRestore {
    context: *mut ContextInternal,
    /// Multi-use callback closure pointers
    cb_ptrs: CallbackPointers,
}

unsafe impl Send for StreamRestore {}
unsafe impl Sync for StreamRestore {}

/// Holds copies of callback closure pointers, for those that are “multi-use” (may be fired multiple
/// times), for freeing at the appropriate time.
#[derive(Default)]
struct CallbackPointers {
    subscribe: super::ExtSubscribeCb,
}

impl Context {
    /// Gets a stream restore object linked to the current context, giving access to stream restore
    /// routines.
    ///
    /// See [`context::ext_stream_restore`](ext_stream_restore/index.html).
    pub fn stream_restore(&self) -> StreamRestore {
        unsafe { capi::pa_context_ref(self.ptr) };
        StreamRestore::from_raw(self.ptr)
    }
}

impl StreamRestore {
    /// Creates a new `DeviceManager` from an existing
    /// [`ContextInternal`](../../../libpulse_sys/context/struct.pa_context.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context, cb_ptrs: Default::default() }
    }

    /// Tests if this extension module is available in the server.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn test<F>(&mut self, callback: F) -> Operation<dyn FnMut(u32)>
        where F: FnMut(u32) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(u32)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_stream_restore_test(self.context,
            Some(super::ext_test_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(u32)>)
    }

    /// Reads all entries from the stream database.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn read<F>(&mut self, callback: F) -> Operation<dyn FnMut(ListResult<&Info>)>
        where F: FnMut(ListResult<&Info>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(ListResult<&Info>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_stream_restore_read(self.context, Some(read_list_cb_proxy),
            cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(ListResult<&Info>)>)
    }

    /// Stores entries in the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn write<F>(&mut self, mode: proplist::UpdateMode, data: &[&Info],
        apply_immediately: bool, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe {
            capi::pa_ext_stream_restore_write(self.context, mode, mem::transmute(data.as_ptr()),
                data.len() as u32, apply_immediately as i32, Some(super::success_cb_proxy),
                cb_data)
        };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Deletes entries from the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn delete<F>(&mut self, streams: &[&str], callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
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
        for c_stream in &c_streams {
            c_stream_ptrs.push(c_stream.as_ptr());
        }
        c_stream_ptrs.push(null());

        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_stream_restore_delete(self.context, c_stream_ptrs.as_ptr(),
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Subscribes to changes in the stream database.
    ///
    /// The callback must accept a `bool`, which indicates success.
    ///
    /// Panics if the underlying C function returns a null pointer.
    pub fn subscribe<F>(&mut self, enable: bool, callback: F) -> Operation<dyn FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<dyn FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_ext_stream_restore_subscribe(self.context, enable as i32,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<dyn FnMut(bool)>)
    }

    /// Sets the subscription callback that is called when [`subscribe`](#method.subscribe) was
    /// called.
    pub fn set_subscribe_cb<F>(&mut self, callback: F)
        where F: FnMut() + 'static
    {
        let saved = &mut self.cb_ptrs.subscribe;
        *saved = super::ExtSubscribeCb::new(Some(Box::new(callback)));
        let (cb_fn, cb_data) = saved.get_capi_params(super::ext_subscribe_cb_proxy);
        unsafe { capi::pa_ext_stream_restore_set_subscribe_cb(self.context, cb_fn, cb_data); }
    }
}

impl Drop for StreamRestore {
    fn drop(&mut self) {
        unsafe { capi::pa_context_unref(self.context) };
        self.context = null_mut::<ContextInternal>();
    }
}

/// Proxy for read list callbacks.
///
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn read_list_cb_proxy(_: *mut ContextInternal, i: *const InfoInternal, eol: i32,
    userdata: *mut c_void)
{
    let _ = std::panic::catch_unwind(|| {
        match callback_for_list_instance::<dyn FnMut(ListResult<&Info>)>(eol, userdata) {
            ListInstanceCallback::Entry(callback) => {
                assert!(!i.is_null());
                let obj = Info::new_from_raw(i);
                (callback)(ListResult::Item(&obj));
            },
            ListInstanceCallback::End(mut callback) => { (callback)(ListResult::End); },
            ListInstanceCallback::Error(mut callback) => { (callback)(ListResult::Error); },
        }
    });
}
