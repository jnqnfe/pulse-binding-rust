//! Routines for daemon introspection.

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
//! Sometimes it is necessary to query and modify global settings in the server. For this,
//! PulseAudio has the introspection API. It can list sinks, sources, samples and other aspects of
//! the server. It can also modify the attributes of the server that will affect operations on a
//! global level, and not just the application's context.
//!
//! # Usage
//!
//! The introspection routines are exposed as methods on an [`Introspector`] object held by the
//! [`Context`] object, and can be accessed via the [`Context`] object's [`introspect`] method.
//!
//! # Querying
//!
//! All querying is done through callbacks. This approach is necessary to maintain an asynchronous
//! design. The client will request the information and some time later, the server will respond
//! with the desired data.
//!
//! Some objects can have multiple instances on the server. When requesting all of these at once,
//! the callback will be called multiple times, each time with an [`ListResult`] variant. It will be
//! called once for each item in turn, using the `Item` variant, and then once more time with the
//! `End` variant to signal that the end of the list has been reached. If an error occurs, then
//! the `Error` variant will be given.
//!
//! Note that even if a single object is requested, and not the entire list, the terminating call
//! will still be made.
//!
//! Data members in the information structures are only valid during the duration of the callback.
//! If they are required after the callback is finished, a deep copy of the information structure
//! must be performed.
//!
//! # Server Information
//!
//! The server can be queried about its name, the environment it's running on and the currently
//! active global defaults. Calling [`Introspector::get_server_info`] provides access to a
//! [`ServerInfo`] structure containing all of these.
//!
//! # Memory Usage
//!
//! Statistics about memory usage can be fetched using [`Introspector::stat`], giving a [`StatInfo`]
//! structure.
//!
//! # Sinks and Sources
//!
//! The server can have an arbitrary number of sinks and sources. Each sink and source have both an
//! index and a name associated with it. As such, there are three ways to get access to them:
//!
//! * By index: [`Introspector::get_sink_info_by_index`], [`Introspector::get_source_info_by_index`]
//! * By name:  [`Introspector::get_sink_info_by_name`], [`Introspector::get_source_info_by_name`]
//! * All:      [`Introspector::get_sink_info_list`], [`Introspector::get_source_info_list`]
//!
//! All three methods use the same callback and will provide a [`SinkInfo`] or [`SourceInfo`]
//! structure.
//!
//! # Sink Inputs and Source Outputs
//!
//! Sink inputs and source outputs are the representations of the client ends of streams inside the
//! server. I.e. they connect a client stream to one of the global sinks or sources.
//!
//! Sink inputs and source outputs only have an index to identify them. As such, there are only two
//! ways to get information about them:
//!
//! * By index: [`Introspector::get_sink_input_info`], [`Introspector::get_source_output_info`]
//! * All:      [`Introspector::get_sink_input_info_list`],
//!             [`Introspector::get_source_output_info_list`]
//!
//! The structure returned is the [`SinkInputInfo`] or [`SourceOutputInfo`] structure.
//!
//! # Samples
//!
//! The list of cached samples can be retrieved from the server. Three methods exist for querying
//! the sample cache list:
//!
//! * By index: [`Introspector::get_sample_info_by_index`]
//! * By name:  [`Introspector::get_sample_info_by_name`]
//! * All:      [`Introspector::get_sample_info_list`]
//!
//! Note that this only retrieves information about the sample, not the sample data itself.
//!
//! # Driver Modules
//!
//! PulseAudio driver modules are identified by index and are retrieved using either
//! [`Introspector::get_module_info`] or [`Introspector::get_module_info_list`]. The information
//! structure is called [`ModuleInfo`].
//!
//! # Clients
//!
//! PulseAudio clients are also identified by index and are retrieved using either
//! [`Introspector::get_client_info`] or [`Introspector::get_client_info_list`]. The information
//! structure is called [`ClientInfo`].
//!
//! # Control
//!
//! Some parts of the server are only possible to read, but most can also be modified in different
//! ways. Note that these changes will affect all connected clients and not just the one issuing the
//! request.
//!
//! # Sinks and Sources
//!
//! The most common change one would want to apply to sinks and sources is to modify the volume of
//! the audio. Identically to how sinks and sources can be queried, there are two ways of
//! identifying them:
//!
//! * By index: [`Introspector::set_sink_volume_by_index`],
//!             [`Introspector::set_source_volume_by_index`]
//! * By name:  [`Introspector::set_sink_volume_by_name`],
//!             [`Introspector::set_source_volume_by_name`]
//!
//! It is also possible to mute a sink or source:
//!
//! * By index: [`Introspector::set_sink_mute_by_index`], [`Introspector::set_source_mute_by_index`]
//! * By name:  [`Introspector::set_sink_mute_by_name`], [`Introspector::set_source_mute_by_name`]
//!
//! # Sink Inputs and Source Outputs
//!
//! If an application desires to modify the volume of just a single stream (commonly one of its own
//! streams), this can be done by setting the volume of its associated sink input or source output,
//! using [`Introspector::set_sink_input_volume`] or [`Introspector::set_source_output_volume`].
//!
//! It is also possible to remove sink inputs and source outputs, terminating the streams associated
//! with them:
//!
//! * Sink input: [`Introspector::kill_sink_input`]
//! * Source output: [`Introspector::kill_source_output`]
//!
//! It is strongly recommended that all volume changes are done as a direct result of user input.
//! With automated requests, such as those resulting from misguided attempts of crossfading,
//! PulseAudio can store the stream volume at an inappropriate moment and restore it later. Besides,
//! such attempts lead to OSD popups in some desktop environments.
//!
//! As a special case of the general rule above, it is recommended that your application leaves the
//! task of saving and restoring the volume of its streams to PulseAudio and does not attempt to do
//! it by itself. PulseAudio really knows better about events such as stream moving or headphone
//! plugging that would make the volume stored by the application inapplicable to the new
//! configuration.
//!
//! Another important case where setting a sink input volume may be a bad idea is related to
//! interpreters that interpret potentially untrusted scripts. PulseAudio relies on your application
//! not making malicious requests (such as repeatedly setting the volume to 100%). Thus, script
//! interpreters that represent a security boundary must sandbox volume-changing requests coming
//! from their scripts. In the worst case, it may be necessary to apply the script-requested volume
//! to the script-produced sounds by altering the samples in the script interpreter and not touching
//! the sink or sink input volume as seen by PulseAudio.
//!
//! If an application changes any volume, it should also listen to changes of the same volume
//! originating from outside the application (e.g., from the system mixer application) and update
//! its user interface accordingly. Use [`::subscribe`] to get such notifications.
//!
//! # Modules
//!
//! Server modules can be remotely loaded and unloaded using [`Introspector::load_module`] and
//! [`Introspector::unload_module`].
//!
//! # Clients
//!
//! The only operation supported on clients is the possibility of kicking them off the server using
//! [`Introspector::kill_client`].
//!
//! [`::subscribe`]: ../subscribe/index.html
//!
//! [`Context`]: ../struct.Context.html
//! [`ListResult`]: ../../callbacks/enum.ListResult.html
//! [`Introspector`]: struct.Introspector.html
//! [`ClientInfo`]: struct.ClientInfo.html
//! [`ModuleInfo`]: struct.ModuleInfo.html
//! [`ServerInfo`]: struct.ServerInfo.html
//! [`SinkInfo`]: struct.SinkInfo.html
//! [`SinkInputInfo`]: struct.SinkInputInfo.html
//! [`SourceInfo`]: struct.SourceInfo.html
//! [`SourceOutputInfo`]: struct.SourceOutputInfo.html
//! [`StatInfo`]: struct.StatInfo.html 
//!
//! [`introspect`]: ../struct.Context.html#method.introspect
//! [`Introspector::get_client_info_list`]: struct.Introspector.html#method.get_client_info_list
//! [`Introspector::get_client_info`]: struct.Introspector.html#method.get_client_info
//! [`Introspector::get_module_info_list`]: struct.Introspector.html#method.get_module_info_list
//! [`Introspector::get_module_info`]: struct.Introspector.html#method.get_module_info
//! [`Introspector::get_sample_info_by_index`]: struct.Introspector.html#method.get_sample_info_by_index
//! [`Introspector::get_sample_info_by_name`]: struct.Introspector.html#method.get_sample_info_by_name
//! [`Introspector::get_sample_info_list`]: struct.Introspector.html#method.get_sample_info_list
//! [`Introspector::get_server_info`]: struct.Introspector.html#method.get_server_info
//! [`Introspector::get_sink_info_by_index`]: struct.Introspector.html#method.get_sink_info_by_index
//! [`Introspector::get_sink_info_by_name`]: struct.Introspector.html#method.get_sink_info_by_name
//! [`Introspector::get_sink_info_list`]: struct.Introspector.html#method.get_sink_info_list
//! [`Introspector::get_sink_input_info_list`]: struct.Introspector.html#method.get_sink_input_info_list
//! [`Introspector::get_sink_input_info`]: struct.Introspector.html#method.get_sink_input_info
//! [`Introspector::get_source_info_by_index`]: struct.Introspector.html#method.get_source_info_by_index
//! [`Introspector::get_source_info_by_name`]: struct.Introspector.html#method.get_source_info_by_name
//! [`Introspector::get_source_info_list`]: struct.Introspector.html#method.get_source_info_list
//! [`Introspector::get_source_output_info_list`]: struct.Introspector.html#method.get_source_output_info_list
//! [`Introspector::get_source_output_info`]: struct.Introspector.html#method.get_source_output_info
//! [`Introspector::kill_client`]: struct.Introspector.html#method.kill_client
//! [`Introspector::kill_sink_input`]: struct.Introspector.html#method.kill_sink_input
//! [`Introspector::kill_source_output`]: struct.Introspector.html#method.kill_source_output
//! [`Introspector::load_module`]: struct.Introspector.html#method.load_module
//! [`Introspector::set_sink_input_volume`]: struct.Introspector.html#method.set_sink_input_volume
//! [`Introspector::set_sink_mute_by_index`]: struct.Introspector.html#method.set_sink_mute_by_index
//! [`Introspector::set_sink_mute_by_name`]: struct.Introspector.html#method.set_sink_mute_by_name
//! [`Introspector::set_sink_volume_by_index`]: struct.Introspector.html#method.set_sink_volume_by_index
//! [`Introspector::set_sink_volume_by_name`]: struct.Introspector.html#method.set_sink_volume_by_name
//! [`Introspector::set_source_mute_by_index`]: struct.Introspector.html#method.set_source_mute_by_index
//! [`Introspector::set_source_mute_by_name`]: struct.Introspector.html#method.set_source_mute_by_name
//! [`Introspector::set_source_output_volume`]: struct.Introspector.html#method.set_source_output_volume
//! [`Introspector::set_source_volume_by_index`]: struct.Introspector.html#method.set_source_volume_by_index
//! [`Introspector::set_source_volume_by_name`]: struct.Introspector.html#method.set_source_volume_by_name
//! [`Introspector::stat`]: struct.Introspector.html#method.stat
//! [`Introspector::unload_module`]: struct.Introspector.html#method.unload_module

use std;
use capi;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
use std::ptr::null_mut;
use super::{Context, ContextInternal};
use timeval::MicroSeconds;
use callbacks::ListResult;

use capi::pa_sink_port_info as SinkPortInfoInternal;
use capi::pa_sink_info as SinkInfoInternal;
use capi::pa_source_port_info as SourcePortInfoInternal;
use capi::pa_source_info as SourceInfoInternal;
use capi::pa_server_info as ServerInfoInternal;
use capi::pa_module_info as ModuleInfoInternal;
use capi::pa_client_info as ClientInfoInternal;
#[allow(deprecated)]
use capi::pa_card_profile_info as CardProfileInfo;
use capi::pa_card_profile_info2 as CardProfileInfo2;
use capi::pa_card_port_info as CardPortInfoInternal;
use capi::pa_card_info as CardInfoInternal;
use capi::pa_sink_input_info as SinkInputInfoInternal;
use capi::pa_source_output_info as SourceOutputInfoInternal;
use capi::pa_sample_info as SampleInfoInternal;

pub use capi::pa_stat_info as StatInfo;

/// A wrapper object providing introspection routines to a context.
pub struct Introspector {
    context: *mut super::ContextInternal,
}

impl Context {
    /// Returns an introspection object linked to the current context, giving access to
    /// introspection routines. See [`::context::introspect`](introspect/index.html).
    pub fn introspect(&self) -> Introspector {
        unsafe { capi::pa_context_ref(self.ptr) };
        Introspector::from_raw(self.ptr)
    }
}

impl Introspector {
    /// Create a new `Introspector` from an existing
    /// [`ContextInternal`](../struct.ContextInternal.html) pointer.
    fn from_raw(context: *mut ContextInternal) -> Self {
        Self { context: context }
    }
}

impl Drop for Introspector {
    fn drop(&mut self) {
        unsafe { capi::pa_context_unref(self.context) };
        self.context = null_mut::<super::ContextInternal>();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sink info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about a specific port of a sink.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SinkPortInfo {
    /// Name of this port.
    pub name: *const c_char,
    /// Description of this port.
    pub description: *const c_char,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// A flag indicating availability status of this port.
    pub available: ::def::PortAvailable,
}

impl From<SinkPortInfoInternal> for SinkPortInfo {
    fn from(p: SinkPortInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// Stores information about sinks.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SinkInfo {
    /// Name of the sink.
    pub name: *const c_char,
    /// Index of the sink.
    pub index: u32,
    /// Description of this sink.
    pub description: *const c_char,
    /// Sample spec of this sink.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// Index of the owning module of this sink, or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub owner_module: u32,
    /// Volume of the sink.
    pub volume: ::volume::ChannelVolumes,
    /// Mute switch of the sink.
    pub mute: i32,
    /// Index of the monitor source connected to this sink.
    pub monitor_source: u32,
    /// The name of the monitor source.
    pub monitor_source_name: *const c_char,
    /// Length of queued audio in the output buffer.
    pub latency: MicroSeconds,
    /// Driver name.
    pub driver: *const c_char,
    /// Flags.
    pub flags: ::def::SinkFlagSet,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// The latency this device has been configured to.
    pub configured_latency: MicroSeconds,
    /// Some kind of "base" volume that refers to unamplified/unattenuated volume in the context of
    /// the output device.
    pub base_volume: ::volume::Volume,
    /// State.
    pub state: ::def::SinkState,
    /// Number of volume steps for sinks which do not support arbitrary volumes.
    pub n_volume_steps: u32,
    /// Card index, or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub card: u32,
    /// Number of entries in port array.
    pub n_ports: u32,
    /// Array of available ports, or `NULL`. Array is terminated by an entry set to `NULL`. The
    /// number of entries is stored in `n_ports`.
    pub ports: *mut *mut SinkPortInfo,
    /// Pointer to active port in the array, or `NULL`.
    pub active_port: *mut SinkPortInfo,
    /// Number of formats supported by the sink.
    pub n_formats: u8,
    /// Array of formats supported by the sink.
    pub formats: *mut *mut ::format::InfoInternal,
}

impl From<SinkInfoInternal> for SinkInfo {
    fn from(p: SinkInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a sink by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_by_name<F>(&self, name: &str, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SinkInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SinkInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sink_info_by_name(self.context, c_name.as_ptr(),
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get information about a sink by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_by_index<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SinkInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SinkInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sink_info_by_index(self.context, index,
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete sink list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SinkInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SinkInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sink_info_list(self.context,
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a sink device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_volume_by_index(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_volume_by_index(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a sink device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_volume_by_name(&mut self, name: &str, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_volume_by_name(self.context, c_name.as_ptr(),
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a sink device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_mute_by_index(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_mute_by_index(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a sink device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_mute_by_name(&mut self, name: &str, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_mute_by_name(self.context, c_name.as_ptr(),
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Suspend/Resume a sink.
    /// 
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_sink_by_name(&mut self, sink_name: &str, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(sink_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_suspend_sink_by_name(self.context, c_name.as_ptr(),
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Suspend/Resume a sink.
    ///
    /// If `index` is [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) all sinks will
    /// be suspended. Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_sink_by_index(&mut self, index: u32, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_suspend_sink_by_index(self.context, index,
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_port_by_index(&mut self, index: u32, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_port_by_index(self.context, index,
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_port_by_name(&mut self, name: &str, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_port_by_name(self.context, c_name.as_ptr(),
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get sink info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sink_info_list_cb_proxy(_: *mut ContextInternal, i: *const SinkInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const SinkInfo>)>)
            };
            callback(ListResult::Item(i as *const SinkInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const SinkInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about a specific port of a source.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SourcePortInfo {
    /// Name of this port.
    pub name: *const c_char,
    /// Description of this port.
    pub description: *const c_char,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// A flag indicating availability status of this port.
    pub available: ::def::PortAvailable,
}

impl From<SourcePortInfoInternal> for SourcePortInfo {
    fn from(p: SourcePortInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// Stores information about sources.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SourceInfo {
    /// Name of the source.
    pub name: *const c_char,
    /// Index of the source.
    pub index: u32,
    /// Description of this source.
    pub description: *const c_char,
    /// Sample spec of this source.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// Owning module index, or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub owner_module: u32,
    /// Volume of the source.
    pub volume: ::volume::ChannelVolumes,
    /// Mute switch of the sink.
    pub mute: i32,
    /// If this is a monitor source, the index of the owning sink, otherwise
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub monitor_of_sink: u32,
    /// Name of the owning sink, or `NULL`.
    pub monitor_of_sink_name: *const c_char,
    /// Length of filled record buffer of this source.
    pub latency: MicroSeconds,
    /// Driver name.
    pub driver: *const c_char,
    /// Flags.
    pub flags: ::def::SourceFlagSet,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// The latency this device has been configured to.
    pub configured_latency: MicroSeconds,
    /// Some kind of "base" volume that refers to unamplified/unattenuated volume in the context of
    /// the input device.
    pub base_volume: ::volume::Volume,
    /// State.
    pub state: ::def::SourceState,
    /// Number of volume steps for sources which do not support arbitrary volumes.
    pub n_volume_steps: u32,
    /// Card index, or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html)
    pub card: u32,
    /// Number of entries in port array.
    pub n_ports: u32,
    /// Array of available ports, or `NULL`. Array is terminated by an entry set to `NULL`. The
    /// number of entries is stored in `n_ports`.
    pub ports: *mut *mut SourcePortInfo,
    /// Pointer to active port in the array, or `NULL`.
    pub active_port: *mut SourcePortInfo,
    /// Number of formats supported by the source.
    pub n_formats: u8,
    /// Array of formats supported by the source.
    pub formats: *mut *mut ::format::InfoInternal,
}

impl From<SourceInfoInternal> for SourceInfo {
    fn from(p: SourceInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a source by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_by_name<F>(&self, name: &str, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SourceInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SourceInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_source_info_by_name(self.context, c_name.as_ptr(),
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get information about a source by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_by_index<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SourceInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SourceInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_source_info_by_index(self.context, index,
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete source list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SourceInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SourceInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_source_info_list(self.context,
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a source device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_volume_by_index(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_volume_by_index(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a source device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_volume_by_name(&mut self, name: &str, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_volume_by_name(self.context, c_name.as_ptr(),
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a source device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_mute_by_index(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_mute_by_index(self.context, index,
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a source device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_mute_by_name(&mut self, name: &str, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_mute_by_name(self.context, c_name.as_ptr(),
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Suspend/Resume a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_source_by_name(&mut self, name: &str, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_suspend_source_by_name(self.context, c_name.as_ptr(),
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Suspend/Resume a source.
    ///
    /// If `index` is [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html), all sources
    /// will be suspended. Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_source_by_index(&mut self, index: u32, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_suspend_source_by_index(self.context, index,
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_port_by_index(&mut self, index: u32, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_port_by_index(self.context, index,
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_port_by_name(&mut self, name: &str, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_port_by_name(self.context, c_name.as_ptr(),
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get source info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_source_info_list_cb_proxy(_: *mut ContextInternal, i: *const SourceInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const SourceInfo>)>)
            };
            callback(ListResult::Item(i as *const SourceInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const SourceInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Server info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Server information.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct ServerInfo {
    /// User name of the daemon process.
    pub user_name: *const c_char,
    /// Host name the daemon is running on.
    pub host_name: *const c_char,
    /// Version string of the daemon.
    pub server_version: *const c_char,
    /// Server package name (usually "pulseaudio").
    pub server_name: *const c_char,
    /// Default sample specification.
    pub sample_spec: ::sample::Spec,
    /// Name of default sink.
    pub default_sink_name: *const c_char,
    /// Name of default source.
    pub default_source_name: *const c_char,
    /// A random cookie for identifying this instance of PulseAudio.
    pub cookie: u32,
    /// Default channel map.
    pub channel_map: ::channelmap::Map,
}

impl From<ServerInfoInternal> for ServerInfo {
    fn from(p: ServerInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get some information about the server.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_server_info<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(*const ServerInfo) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(*const ServerInfo)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_server_info(self.context,
            Some(get_server_info_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get server info callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn get_server_info_cb_proxy(_: *mut ContextInternal, i: *const ServerInfoInternal,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null() && !i.is_null());
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = unsafe { Box::from_raw(userdata as *mut Box<FnMut(*const ServerInfo)>) };
    callback(i as *const ServerInfo);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Module info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about modules.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct ModuleInfo {
    /// Index of the module.
    pub index: u32,
    /// Name of the module.
    pub name: *const c_char,
    /// Argument string of the module.
    pub argument: *const c_char,
    /// Usage counter or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub n_used: u32,
    /// Non-zero if this is an autoloaded module.
    #[deprecated]
    pub auto_unload: i32,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
}

impl From<ModuleInfoInternal> for ModuleInfo {
    fn from(p: ModuleInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get some information about a module by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_module_info<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const ModuleInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const ModuleInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_module_info(self.context, index,
            Some(mod_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete list of currently loaded modules.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_module_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const ModuleInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const ModuleInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_module_info_list(self.context,
            Some(mod_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Load a module.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state. The callback is provided with the
    /// index.
    pub fn load_module<F>(&mut self, name: &str, argument: &str, callback: F
        ) -> ::operation::Operation
        where F: FnMut(u32) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_arg = CString::new(argument.clone()).unwrap();

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(u32)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_load_module(self.context, c_name.as_ptr(),
            c_arg.as_ptr(), Some(context_index_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Unload a module.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn unload_module<F>(&mut self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_unload_module(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get module info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn mod_info_list_cb_proxy(_: *mut ContextInternal, i: *const ModuleInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const ModuleInfo>)>)
            };
            callback(ListResult::Item(i as *const ModuleInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const ModuleInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

/// Proxy for context index callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn context_index_cb_proxy(_: *mut ContextInternal, index: u32, userdata: *mut c_void) {
    assert!(!userdata.is_null());
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = unsafe { Box::from_raw(userdata as *mut Box<FnMut(u32)>) };
    callback(index);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Client info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about clients.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct ClientInfo {
    /// Index of this client.
    pub index: u32,
    /// Name of this client.
    pub name: *const c_char,
    /// Index of the owning module, or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub owner_module: u32,
    /// Driver name.
    pub driver: *const c_char,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
}

impl From<ClientInfoInternal> for ClientInfo {
    fn from(p: ClientInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a client by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_client_info<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const ClientInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const ClientInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_client_info(self.context, index,
            Some(get_client_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete client list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_client_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const ClientInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const ClientInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_client_info_list(self.context,
            Some(get_client_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Kill a client.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_client<F>(&mut self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_kill_client(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get sink info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_client_info_list_cb_proxy(_: *mut ContextInternal, i: *const ClientInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const ClientInfo>)>)
            };
            callback(ListResult::Item(i as *const ClientInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const ClientInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Card info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about a specific port of a card.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct CardPortInfo {
    /// Name of this port.
    pub name: *const c_char,
    /// Description of this port.
    pub description: *const c_char,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// Availability status of this port.
    pub available: ::def::PortAvailable,
    /// The direction of this port.
    pub direction: ::direction::FlagSet,
    /// Number of entries in profile array.
    pub n_profiles: u32,
    /// Superseded by `profiles2`.
    #[deprecated]
    #[allow(deprecated)]
    pub profiles: *mut *mut CardProfileInfo,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// Latency offset of the port that gets added to the sink/source latency when the port is
    /// active.
    pub latency_offset: i64,
    /// Array of pointers to available profiles, or `NULL`. Array is terminated by an entry set to
    /// `NULL`.
    pub profiles2: *mut *mut CardProfileInfo2,
}

impl From<CardPortInfoInternal> for CardPortInfo {
    fn from(p: CardPortInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// Stores information about cards.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct CardInfo {
    /// Index of this card.
    pub index: u32,
    /// Name of this card.
    pub name: *const c_char,
    /// Index of the owning module, or [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html).
    pub owner_module: u32,
    /// Driver name.
    pub driver: *const c_char,
    /// Number of entries in profile array.
    pub n_profiles: u32,
    /// Superseded by `profiles2`.
    #[deprecated]
    #[allow(deprecated)]
    pub profiles: *mut CardProfileInfo,
    /// Superseded by `active_profile2`.
    #[deprecated]
    #[allow(deprecated)]
    pub active_profile: *mut CardProfileInfo,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// Number of entries in port array.
    pub n_ports: u32,
    /// Array of pointers to ports, or `NULL`. Array is terminated by an entry set to `NULL`.
    pub ports: *mut *mut CardPortInfo,
    /// Array of pointers to available profiles, or `NULL`. Array is terminated by an entry set to
    /// `NULL`.
    pub profiles2: *mut *mut CardProfileInfo2,
    /// Pointer to active profile in the array, or `NULL`.
    pub active_profile2: *mut CardProfileInfo2,
}

impl From<CardInfoInternal> for CardInfo {
    fn from(p: CardInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a card by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_by_index<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const CardInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const CardInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_card_info_by_index(self.context, index,
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get information about a card by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_by_name<F>(&self, name: &str, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const CardInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const CardInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_card_info_by_name(self.context, c_name.as_ptr(),
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete card list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const CardInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const CardInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_card_info_list(self.context,
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a card.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_card_profile_by_index(&mut self, index: u32, profile: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_profile = CString::new(profile.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_card_profile_by_index(self.context, index,
            c_profile.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Change the profile of a card.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_card_profile_by_name(&mut self, name: &str, profile: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_profile = CString::new(profile.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_card_profile_by_name(self.context, c_name.as_ptr(),
            c_profile.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the latency offset of a port.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_port_latency_offset(&mut self, card_name: &str, port_name: &str, offset: i64,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(card_name.clone()).unwrap();
        let c_port = CString::new(port_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_port_latency_offset(self.context, c_name.as_ptr(),
            c_port.as_ptr(), offset, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get card info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_card_info_list_cb_proxy(_: *mut ContextInternal, i: *const CardInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const CardInfo>)>)
            };
            callback(ListResult::Item(i as *const CardInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const CardInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sink input info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about sink inputs.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SinkInputInfo {
    /// Index of the sink input.
    pub index: u32,
    /// Name of the sink input.
    pub name: *const c_char,
    /// Index of the module this sink input belongs to, or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) when it does not belong to
    /// any module.
    pub owner_module: u32,
    /// Index of the client this sink input belongs to, or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) when it does not belong to
    /// any client.
    pub client: u32,
    /// Index of the connected sink.
    pub sink: u32,
    /// The sample specification of the sink input.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// The volume of this sink input.
    pub volume: ::volume::ChannelVolumes,
    /// Latency due to buffering in sink input, see
    /// [`::def::TimingInfo`](../../def/struct.TimingInfo.html) for details.
    pub buffer_usec: MicroSeconds,
    /// Latency of the sink device, see
    /// [`::def::TimingInfo`](../../def/struct.TimingInfo.html) for details.
    pub sink_usec: MicroSeconds,
    /// The resampling method used by this sink input.
    pub resample_method: *const c_char,
    /// Driver name.
    pub driver: *const c_char,
    /// Stream muted.
    pub mute: i32,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// Stream corked.
    pub corked: i32,
    /// Stream has volume. If not set, then the meaning of this struct's volume member is unspecified.
    pub has_volume: i32,
    /// The volume can be set. If not set, the volume can still change even though clients can't
    /// control the volume.
    pub volume_writable: i32,
    /// Stream format information.
    pub format: *mut ::format::InfoInternal,
}

impl From<SinkInputInfoInternal> for SinkInputInfo {
    fn from(p: SinkInputInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get some information about a sink input by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_input_info<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SinkInputInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SinkInputInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sink_input_info(self.context, index,
            Some(get_sink_input_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete sink input list.
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_input_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SinkInputInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SinkInputInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sink_input_info_list(self.context,
            Some(get_sink_input_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Move the specified sink input to a different sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_sink_input_by_name(&mut self, index: u32, sink_name: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(sink_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_move_sink_input_by_name(self.context, index,
            c_name.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Move the specified sink input to a different sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_sink_input_by_index(&mut self, index: u32, sink_index: u32,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_move_sink_input_by_index(self.context, index,
            sink_index, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a sink input stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_input_volume(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_input_volume(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a sink input stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_input_mute(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_sink_input_mute(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Kill a sink input.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_sink_input<F>(&mut self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_kill_sink_input(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get sink input info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sink_input_info_list_cb_proxy(_: *mut ContextInternal, i: *const SinkInputInfoInternal,
    eol: i32, userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const SinkInputInfo>)>)
            };
            callback(ListResult::Item(i as *const SinkInputInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const SinkInputInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source output info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about source outputs.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SourceOutputInfo {
    /// Index of the source output.
    pub index: u32,
    /// Name of the source output.
    pub name: *const c_char,
    /// Index of the module this source output belongs to, or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) when it does not belong to
    /// any module.
    pub owner_module: u32,
    /// Index of the client this source output belongs to, or
    /// [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) when it does not belong to
    /// any client.
    pub client: u32,
    /// Index of the connected source.
    pub source: u32,
    /// The sample specification of the source output.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// Latency due to buffering in the source output, see
    /// [`::def::TimingInfo`](../../def/struct.TimingInfo.html) for details.
    pub buffer_usec: MicroSeconds,
    /// Latency of the source device, see [`::def::TimingInfo`](../../def/struct.TimingInfo.html)
    /// for details.
    pub source_usec: MicroSeconds,
    /// The resampling method used by this source output.
    pub resample_method: *const c_char,
    /// Driver name.
    pub driver: *const c_char,
    /// Property list.
    pub proplist: *mut ::proplist::ProplistInternal,
    /// Stream corked.
    pub corked: i32,
    /// The volume of this source output.
    pub volume: ::volume::ChannelVolumes,
    /// Stream muted.
    pub mute: i32,
    /// Stream has volume. If not set, then the meaning of this struct's volume member is unspecified.
    pub has_volume: i32,
    /// The volume can be set. If not set, the volume can still change even though clients can't
    /// control the volume.
    pub volume_writable: i32,
    /// Stream format information.
    pub format: *mut ::format::InfoInternal,
}

impl From<SourceOutputInfoInternal> for SourceOutputInfo {
    fn from(p: SourceOutputInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a source output by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_output_info<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SourceOutputInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SourceOutputInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_source_output_info(self.context, index,
            Some(get_source_output_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete list of source outputs.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_output_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SourceOutputInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SourceOutputInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_source_output_info_list(self.context,
            Some(get_source_output_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Move the specified source output to a different source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_source_output_by_name(&mut self, index: u32, source_name: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(source_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_move_source_output_by_name(self.context, index,
            c_name.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Move the specified source output to a different source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_source_output_by_index(&mut self, index: u32, source_index: u32,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_move_source_output_by_index(self.context, index,
            source_index, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the volume of a source output stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_output_volume(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_output_volume(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Set the mute switch of a source output stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_output_mute(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> ::operation::Operation
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, *mut c_void) = match callback {
            Some(f) => {
                // WARNING: Type must be explicit here, else compiles but seg faults :/
                let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(f));
                (Some(super::success_cb_proxy), boxed as *mut c_void)
            },
            None => (None, std::ptr::null_mut::<c_void>()),
        };
        let ptr = unsafe { capi::pa_context_set_source_output_mute(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Kill a source output.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_source_output<F>(&mut self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(bool) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(bool)> = Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_kill_source_output(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get source output info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_source_output_info_list_cb_proxy(_: *mut ContextInternal, i: *const SourceOutputInfoInternal,
    eol: i32, userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const SourceOutputInfo>)>)
            };
            callback(ListResult::Item(i as *const SourceOutputInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const SourceOutputInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stat info
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Introspector {
    /// Get daemon memory block statistics.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn stat<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(*const StatInfo) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(*const StatInfo)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_stat(self.context, Some(get_stat_info_cb_proxy),
            cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get stat info callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn get_stat_info_cb_proxy(_: *mut ContextInternal, i: *const StatInfo, userdata: *mut c_void) {
    assert!(!userdata.is_null() && !i.is_null());
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = unsafe { Box::from_raw(userdata as *mut Box<FnMut(*const StatInfo)>) };
    callback(i);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sample info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about sample cache entries.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct SampleInfo {
    /// Index of this entry.
    pub index: u32,
    /// Name of this entry.
    pub name: *const c_char,
    /// Default volume of this entry.
    pub volume: ::volume::ChannelVolumes,
    /// Sample specification of the sample.
    pub sample_spec: ::sample::Spec,
    /// The channel map.
    pub channel_map: ::channelmap::Map,
    /// Duration of this entry.
    pub duration: MicroSeconds,
    /// Length of this sample in bytes.
    pub bytes: u32,
    /// Non-zero when this is a lazy cache entry.
    pub lazy: i32,
    /// In case this is a lazy cache entry, the filename for the sound file to be loaded on demand.
    pub filename: *const c_char,
    /// Property list for this sample.
    pub proplist: *mut ::proplist::ProplistInternal,
}

impl From<SampleInfoInternal> for SampleInfo {
    fn from(p: SampleInfoInternal) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

impl Introspector {
    /// Get information about a sample by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_by_name<F>(&self, name: &str, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SampleInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SampleInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sample_info_by_name(self.context, c_name.as_ptr(),
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get information about a sample by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_by_index<F>(&self, index: u32, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SampleInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SampleInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sample_info_by_index(self.context, index,
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }

    /// Get the complete list of samples stored in the daemon.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_list<F>(&self, callback: F) -> ::operation::Operation
        where F: FnMut(ListResult<*const SampleInfo>) + 'static
    {
        let cb_data: *mut c_void = {
            // WARNING: Type must be explicit here, else compiles but seg faults :/
            let boxed: *mut Box<FnMut(ListResult<*const SampleInfo>)> =
                Box::into_raw(Box::new(Box::new(callback)));
            boxed as *mut c_void
        };
        let ptr = unsafe { capi::pa_context_get_sample_info_list(self.context,
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        ::operation::Operation::from_raw(ptr)
    }
}

/// Proxy for get sample info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sample_info_list_cb_proxy(_: *mut ContextInternal, i: *const SampleInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    assert!(!userdata.is_null());
    match eol {
        0 => { // Give item to real callback, do NOT destroy it
            assert!(!i.is_null());
            let callback = unsafe {
                &mut *(userdata as *mut Box<FnMut(ListResult<*const SampleInfo>)>)
            };
            callback(ListResult::Item(i as *const SampleInfo));
        },
        _ => { // End-of-list or failure, signal to real callback, do now destroy it
            let mut callback = unsafe {
                Box::from_raw(userdata as *mut Box<FnMut(ListResult<*const SampleInfo>)>)
            };
            callback(match eol < 0 { false => ListResult::End, true => ListResult::Error });
        },
    }
}
