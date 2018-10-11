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

//! Routines for daemon introspection.
//!
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
use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::borrow::Cow;
use std::ptr::null_mut;
use super::{Context, ContextInternal};
use time::MicroSeconds;
use callbacks::{ListResult, box_closure_get_capi_ptr, callback_for_list_instance, ListInstanceCallback};
use operation::Operation;

use capi::pa_sink_port_info as SinkPortInfoInternal;
use capi::pa_sink_info as SinkInfoInternal;
use capi::pa_source_port_info as SourcePortInfoInternal;
use capi::pa_source_info as SourceInfoInternal;
use capi::pa_server_info as ServerInfoInternal;
use capi::pa_module_info as ModuleInfoInternal;
use capi::pa_client_info as ClientInfoInternal;
use capi::pa_card_profile_info2 as CardProfileInfo2Internal;
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
#[derive(Debug)]
pub struct SinkPortInfo<'a> {
    /// Name of this port.
    pub name: Option<Cow<'a, str>>,
    /// Description of this port.
    pub description: Option<Cow<'a, str>>,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// A flag indicating availability status of this port.
    pub available: ::def::PortAvailable,
}

impl<'a> SinkPortInfo<'a> {
    fn new_from_raw(p: *const SinkPortInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            SinkPortInfo {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                priority: src.priority,
                available: std::mem::transmute(src.available),
            }
        }
    }
}

/// Stores information about sinks.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SinkInfo<'a> {
    /// Name of the sink.
    pub name: Option<Cow<'a, str>>,
    /// Index of the sink.
    pub index: u32,
    /// Description of this sink.
    pub description: Option<Cow<'a, str>>,
    /// Sample spec of this sink.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// Index of the owning module of this sink, or `None` if is invalid.
    pub owner_module: Option<u32>,
    /// Volume of the sink.
    pub volume: ::volume::ChannelVolumes,
    /// Mute switch of the sink.
    pub mute: bool,
    /// Index of the monitor source connected to this sink.
    pub monitor_source: u32,
    /// The name of the monitor source.
    pub monitor_source_name: Option<Cow<'a, str>>,
    /// Length of queued audio in the output buffer.
    pub latency: MicroSeconds,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Flags.
    pub flags: ::def::SinkFlagSet,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// The latency this device has been configured to.
    pub configured_latency: MicroSeconds,
    /// Some kind of "base" volume that refers to unamplified/unattenuated volume in the context of
    /// the output device.
    pub base_volume: ::volume::Volume,
    /// State.
    pub state: ::def::SinkState,
    /// Number of volume steps for sinks which do not support arbitrary volumes.
    pub n_volume_steps: u32,
    /// Card index, or `None` if invalid.
    pub card: Option<u32>,
    /// Set of available ports.
    pub ports: Vec<SinkPortInfo<'a>>,
    /// Pointer to active port in the set, or `None`.
    pub active_port: Option<Box<SinkPortInfo<'a>>>,
    /// Set of formats supported by the sink.
    pub formats: Vec<::format::Info>,
}

impl<'a> SinkInfo<'a> {
    fn new_from_raw(p: *const SinkInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };

        let mut port_vec = Vec::with_capacity(src.n_ports as usize);
        assert!(src.n_ports == 0 || !src.ports.is_null());
        for i in 0..src.n_ports as isize {
            let indexed_ptr = unsafe { (*src.ports.offset(i)) as *mut SinkPortInfoInternal };
            if !indexed_ptr.is_null() {
                port_vec.push(SinkPortInfo::new_from_raw(indexed_ptr));
            }
        }
        let mut formats_vec = Vec::with_capacity(src.n_formats as usize);
        assert!(src.n_formats == 0 || !src.formats.is_null());
        for i in 0..src.n_formats as isize {
            let indexed_ptr = unsafe { (*src.formats.offset(i)) as *mut ::format::InfoInternal };
            if !indexed_ptr.is_null() {
                formats_vec.push(::format::Info::from_raw_weak(indexed_ptr));
            }
        }

        unsafe {
            SinkInfo {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                index: src.index,
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                sample_spec: std::mem::transmute(src.sample_spec),
                channel_map: std::mem::transmute(src.channel_map),
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                volume: std::mem::transmute(src.volume),
                mute: match src.mute { 0 => false, _ => true },
                monitor_source: src.monitor_source,
                monitor_source_name: match src.monitor_source_name.is_null() {
                    false => Some(CStr::from_ptr(src.monitor_source_name).to_string_lossy()),
                    true => None,
                },
                latency: MicroSeconds(src.latency),
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                flags: src.flags,
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                configured_latency: MicroSeconds(src.configured_latency),
                base_volume: ::volume::Volume(src.base_volume),
                state: std::mem::transmute(src.state),
                n_volume_steps: src.n_volume_steps,
                card: match src.card {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                ports: port_vec,
                active_port: match src.active_port.is_null() {
                    true => None,
                    false => Some(Box::new(SinkPortInfo::new_from_raw(src.active_port))),
                },
                formats: formats_vec,
            }
        }
    }
}

impl Introspector {
    /// Get information about a sink by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_by_name<F>(&self, name: &str, callback: F
        ) -> Operation<FnMut(ListResult<&SinkInfo>)>
        where F: FnMut(ListResult<&SinkInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SinkInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sink_info_by_name(self.context, c_name.as_ptr(),
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SinkInfo>)>)
    }

    /// Get information about a sink by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_by_index<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&SinkInfo>)>
        where F: FnMut(ListResult<&SinkInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SinkInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sink_info_by_index(self.context, index,
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SinkInfo>)>)
    }

    /// Get the complete sink list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&SinkInfo>)>
        where F: FnMut(ListResult<&SinkInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SinkInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sink_info_list(self.context,
            Some(get_sink_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SinkInfo>)>)
    }

    /// Set the volume of a sink device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_volume_by_index(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_volume_by_index(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the volume of a sink device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_volume_by_name(&mut self, name: &str, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_volume_by_name(self.context, c_name.as_ptr(),
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a sink device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_mute_by_index(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_mute_by_index(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a sink device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_mute_by_name(&mut self, name: &str, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_mute_by_name(self.context, c_name.as_ptr(),
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Suspend/Resume a sink.
    /// 
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_sink_by_name(&mut self, sink_name: &str, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(sink_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_suspend_sink_by_name(self.context, c_name.as_ptr(),
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Suspend/Resume a sink.
    ///
    /// If `index` is [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html) all sinks will
    /// be suspended. Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_sink_by_index(&mut self, index: u32, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_suspend_sink_by_index(self.context, index,
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Change the profile of a sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_port_by_index(&mut self, index: u32, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_port_by_index(self.context, index,
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Change the profile of a sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_port_by_name(&mut self, name: &str, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_port_by_name(self.context, c_name.as_ptr(),
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get sink info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sink_info_list_cb_proxy(_: *mut ContextInternal, i: *const SinkInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&SinkInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = SinkInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about a specific port of a source.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SourcePortInfo<'a> {
    /// Name of this port.
    pub name: Option<Cow<'a, str>>,
    /// Description of this port.
    pub description: Option<Cow<'a, str>>,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// A flag indicating availability status of this port.
    pub available: ::def::PortAvailable,
}

impl<'a> SourcePortInfo<'a> {
    fn new_from_raw(p: *const SourcePortInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            SourcePortInfo {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                description: match src.description.is_null() {
                    false=> Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                priority: src.priority,
                available: std::mem::transmute(src.available),
            }
        }
    }
}

/// Stores information about sources.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SourceInfo<'a> {
    /// Name of the source.
    pub name: Option<Cow<'a, str>>,
    /// Index of the source.
    pub index: u32,
    /// Description of this source.
    pub description: Option<Cow<'a, str>>,
    /// Sample spec of this source.
    pub sample_spec: ::sample::Spec,
    /// Channel map.
    pub channel_map: ::channelmap::Map,
    /// Owning module index, or `None`.
    pub owner_module: Option<u32>,
    /// Volume of the source.
    pub volume: ::volume::ChannelVolumes,
    /// Mute switch of the sink.
    pub mute: bool,
    /// If this is a monitor source, the index of the owning sink, otherwise `None`.
    pub monitor_of_sink: Option<u32>,
    /// Name of the owning sink, or `None`.
    pub monitor_of_sink_name: Option<Cow<'a, str>>,
    /// Length of filled record buffer of this source.
    pub latency: MicroSeconds,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Flags.
    pub flags: ::def::SourceFlagSet,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// The latency this device has been configured to.
    pub configured_latency: MicroSeconds,
    /// Some kind of "base" volume that refers to unamplified/unattenuated volume in the context of
    /// the input device.
    pub base_volume: ::volume::Volume,
    /// State.
    pub state: ::def::SourceState,
    /// Number of volume steps for sources which do not support arbitrary volumes.
    pub n_volume_steps: u32,
    /// Card index, or `None`.
    pub card: Option<u32>,
    /// Set of available ports.
    pub ports: Vec<SourcePortInfo<'a>>,
    /// Pointer to active port in the set, or `None`.
    pub active_port: Option<Box<SourcePortInfo<'a>>>,
    /// Set of formats supported by the sink.
    pub formats: Vec<::format::Info>,
}

impl<'a> SourceInfo<'a> {
    fn new_from_raw(p: *const SourceInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };

        let mut port_vec = Vec::with_capacity(src.n_ports as usize);
        assert!(src.n_ports == 0 || !src.ports.is_null());
        for i in 0..src.n_ports as isize {
            let indexed_ptr = unsafe { (*src.ports.offset(i)) as *mut SourcePortInfoInternal };
            if !indexed_ptr.is_null() {
                port_vec.push(SourcePortInfo::new_from_raw(indexed_ptr));
            }
        }
        let mut formats_vec = Vec::with_capacity(src.n_formats as usize);
        assert!(src.n_formats == 0 || !src.formats.is_null());
        for i in 0..src.n_formats as isize {
            let indexed_ptr = unsafe { (*src.formats.offset(i)) as *mut ::format::InfoInternal };
            if !indexed_ptr.is_null() {
                formats_vec.push(::format::Info::from_raw_weak(indexed_ptr));
            }
        }

        unsafe {
            SourceInfo {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                index: src.index,
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                sample_spec: std::mem::transmute(src.sample_spec),
                channel_map: std::mem::transmute(src.channel_map),
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                volume: std::mem::transmute(src.volume),
                mute: match src.mute { 0 => false, _ => true },
                monitor_of_sink: match src.monitor_of_sink {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                monitor_of_sink_name: match src.monitor_of_sink_name.is_null() {
                    false => Some(CStr::from_ptr(src.monitor_of_sink_name).to_string_lossy()),
                    true => None,
                },
                latency: MicroSeconds(src.latency),
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                flags: src.flags,
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                configured_latency: MicroSeconds(src.configured_latency),
                base_volume: ::volume::Volume(src.base_volume),
                state: src.state.into(),
                n_volume_steps: src.n_volume_steps,
                card: match src.card {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                ports: port_vec,
                active_port: match src.active_port.is_null() {
                    true => None,
                    false => Some(Box::new(SourcePortInfo::new_from_raw(src.active_port))),
                },
                formats: formats_vec,
            }
        }
    }
}

impl Introspector {
    /// Get information about a source by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_by_name<F>(&self, name: &str, callback: F
        ) -> Operation<FnMut(ListResult<&SourceInfo>)>
        where F: FnMut(ListResult<&SourceInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SourceInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_source_info_by_name(self.context, c_name.as_ptr(),
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SourceInfo>)>)
    }

    /// Get information about a source by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_by_index<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&SourceInfo>)>
        where F: FnMut(ListResult<&SourceInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SourceInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_source_info_by_index(self.context, index,
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SourceInfo>)>)
    }

    /// Get the complete source list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&SourceInfo>)>
        where F: FnMut(ListResult<&SourceInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SourceInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_source_info_list(self.context,
            Some(get_source_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SourceInfo>)>)
    }

    /// Set the volume of a source device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_volume_by_index(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_volume_by_index(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the volume of a source device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_volume_by_name(&mut self, name: &str, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_volume_by_name(self.context, c_name.as_ptr(),
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a source device specified by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_mute_by_index(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_mute_by_index(self.context, index,
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a source device specified by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_mute_by_name(&mut self, name: &str, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_mute_by_name(self.context, c_name.as_ptr(),
            mute as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Suspend/Resume a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_source_by_name(&mut self, name: &str, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_suspend_source_by_name(self.context, c_name.as_ptr(),
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Suspend/Resume a source.
    ///
    /// If `index` is [`::def::INVALID_INDEX`](../../def/constant.INVALID_INDEX.html), all sources
    /// will be suspended. Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn suspend_source_by_index(&mut self, index: u32, suspend: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_suspend_source_by_index(self.context, index,
            suspend as i32, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Change the profile of a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_port_by_index(&mut self, index: u32, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_port_by_index(self.context, index,
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Change the profile of a source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_port_by_name(&mut self, name: &str, port: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_port = CString::new(port.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_port_by_name(self.context, c_name.as_ptr(),
            c_port.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get source info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_source_info_list_cb_proxy(_: *mut ContextInternal, i: *const SourceInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&SourceInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = SourceInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Server info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Server information.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct ServerInfo<'a> {
    /// User name of the daemon process.
    pub user_name: Option<Cow<'a, str>>,
    /// Host name the daemon is running on.
    pub host_name: Option<Cow<'a, str>>,
    /// Version string of the daemon.
    pub server_version: Option<Cow<'a, str>>,
    /// Server package name (usually "pulseaudio").
    pub server_name: Option<Cow<'a, str>>,
    /// Default sample specification.
    pub sample_spec: ::sample::Spec,
    /// Name of default sink.
    pub default_sink_name: Option<Cow<'a, str>>,
    /// Name of default source.
    pub default_source_name: Option<Cow<'a, str>>,
    /// A random cookie for identifying this instance of PulseAudio.
    pub cookie: u32,
    /// Default channel map.
    pub channel_map: ::channelmap::Map,
}

impl<'a> ServerInfo<'a> {
    fn new_from_raw(p: *const ServerInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            ServerInfo {
                user_name: match src.user_name.is_null() {
                    false => Some(CStr::from_ptr(src.user_name).to_string_lossy()),
                    true => None,
                },
                host_name: match src.host_name.is_null() {
                    false => Some(CStr::from_ptr(src.host_name).to_string_lossy()),
                    true => None,
                },
                server_version: match src.server_version.is_null() {
                    false => Some(CStr::from_ptr(src.server_version).to_string_lossy()),
                    true => None,
                },
                server_name: match src.server_name.is_null() {
                    false => Some(CStr::from_ptr(src.server_name).to_string_lossy()),
                    true => None,
                },
                sample_spec: std::mem::transmute(src.sample_spec),
                default_sink_name: match src.default_sink_name.is_null() {
                    false => Some(CStr::from_ptr(src.default_sink_name).to_string_lossy()),
                    true => None,
                },
                default_source_name: match src.default_source_name.is_null() {
                    false => Some(CStr::from_ptr(src.default_source_name).to_string_lossy()),
                    true => None,
                },
                cookie: src.cookie,
                channel_map: std::mem::transmute(src.channel_map),
            }
        }
    }
}

impl Introspector {
    /// Get some information about the server.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_server_info<F>(&self, callback: F) -> Operation<FnMut(&ServerInfo)>
        where F: FnMut(&ServerInfo) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(&ServerInfo)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_server_info(self.context,
            Some(get_server_info_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(&ServerInfo)>)
    }
}

/// Proxy for get server info callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn get_server_info_cb_proxy(_: *mut ContextInternal, i: *const ServerInfoInternal,
    userdata: *mut c_void)
{
    assert!(!i.is_null());
    let obj = ServerInfo::new_from_raw(i);

    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = ::callbacks::get_su_callback::<FnMut(&ServerInfo)>(userdata);
    callback(&obj);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Module info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about modules.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct ModuleInfo<'a> {
    /// Index of the module.
    pub index: u32,
    /// Name of the module.
    pub name: Option<Cow<'a, str>>,
    /// Argument string of the module.
    pub argument: Option<Cow<'a, str>>,
    /// Usage counter or `None` if invalid.
    pub n_used: Option<u32>,
    /// Property list.
    pub proplist: ::proplist::Proplist,
}

impl<'a> ModuleInfo<'a> {
    fn new_from_raw(p: *const ModuleInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            ModuleInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                argument: match src.argument.is_null() {
                    false => Some(CStr::from_ptr(src.argument).to_string_lossy()),
                    true => None,
                },
                n_used: match src.n_used {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
            }
        }
    }
}

impl Introspector {
    /// Get some information about a module by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_module_info<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&ModuleInfo>)>
        where F: FnMut(ListResult<&ModuleInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&ModuleInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_module_info(self.context, index,
            Some(mod_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&ModuleInfo>)>)
    }

    /// Get the complete list of currently loaded modules.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_module_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&ModuleInfo>)>
        where F: FnMut(ListResult<&ModuleInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&ModuleInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_module_info_list(self.context,
            Some(mod_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&ModuleInfo>)>)
    }

    /// Load a module.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state. The callback is provided with the
    /// index.
    pub fn load_module<F>(&mut self, name: &str, argument: &str, callback: F
        ) -> Operation<FnMut(u32)>
        where F: FnMut(u32) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_arg = CString::new(argument.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(u32)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_load_module(self.context, c_name.as_ptr(),
            c_arg.as_ptr(), Some(context_index_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(u32)>)
    }

    /// Unload a module.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn unload_module<F>(&mut self, index: u32, callback: F) -> Operation<FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_unload_module(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get module info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn mod_info_list_cb_proxy(_: *mut ContextInternal, i: *const ModuleInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&ModuleInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = ModuleInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

/// Proxy for context index callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn context_index_cb_proxy(_: *mut ContextInternal, index: u32, userdata: *mut c_void) {
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = ::callbacks::get_su_callback::<FnMut(u32)>(userdata);
    callback(index);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Client info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about clients.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct ClientInfo<'a> {
    /// Index of this client.
    pub index: u32,
    /// Name of this client.
    pub name: Option<Cow<'a, str>>,
    /// Index of the owning module, or `None`.
    pub owner_module: Option<u32>,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Property list.
    pub proplist: ::proplist::Proplist,
}

impl<'a> ClientInfo<'a> {
    fn new_from_raw(p: *const ClientInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            ClientInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
            }
        }
    }
}

impl Introspector {
    /// Get information about a client by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_client_info<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&ClientInfo>)>
        where F: FnMut(ListResult<&ClientInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&ClientInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_client_info(self.context, index,
            Some(get_client_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&ClientInfo>)>)
    }

    /// Get the complete client list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_client_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&ClientInfo>)>
        where F: FnMut(ListResult<&ClientInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&ClientInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_client_info_list(self.context,
            Some(get_client_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&ClientInfo>)>)
    }

    /// Kill a client.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_client<F>(&mut self, index: u32, callback: F) -> Operation<FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_kill_client(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get sink info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_client_info_list_cb_proxy(_: *mut ContextInternal, i: *const ClientInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&ClientInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = ClientInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Card info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about a specific profile of a card.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct CardProfileInfo2<'a> {
    /// Name of this profile.
    pub name: Option<Cow<'a, str>>,
    /// Description of this profile.
    pub description: Option<Cow<'a, str>>,
    /// Number of sinks this profile would create.
    pub n_sinks: u32,
    /// Number of sources this profile would create.
    pub n_sources: u32,
    /// The higher this value is, the more useful this profile is as a default.
    pub priority: u32,

    /// Is this profile available? If this is `false`, meaning "unavailable", then it makes no sense
    /// to try to activate this profile. If this is `true`, it's still not a guarantee that
    /// activating the profile will result in anything useful, it just means that the server isn't
    /// aware of any reason why the profile would definitely be useless.
    pub available: bool,
}

impl<'a> CardProfileInfo2<'a> {
    fn new_from_raw(p: *const CardProfileInfo2Internal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            CardProfileInfo2 {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                n_sinks: src.n_sinks,
                n_sources: src.n_sources,
                priority: src.priority,
                available: match src.available { 0 => false, _ => true },
            }
        }
    }
}

/// Stores information about a specific port of a card.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct CardPortInfo<'a> {
    /// Name of this port.
    pub name: Option<Cow<'a, str>>,
    /// Description of this port.
    pub description: Option<Cow<'a, str>>,
    /// The higher this value is, the more useful this port is as a default.
    pub priority: u32,
    /// Availability status of this port.
    pub available: ::def::PortAvailable,
    /// The direction of this port.
    pub direction: ::direction::FlagSet,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// Latency offset of the port that gets added to the sink/source latency when the port is
    /// active.
    pub latency_offset: i64,
    /// Set of available profiles.
    pub profiles: Vec<CardProfileInfo2<'a>>,
}

impl<'a> CardPortInfo<'a> {
    fn new_from_raw(p: *const CardPortInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };

        let mut profiles_vec = Vec::with_capacity(src.n_profiles as usize);
        assert!(src.n_profiles == 0 || !src.profiles2.is_null());
        for i in 0..src.n_profiles as isize {
            let indexed_ptr = unsafe { (*src.profiles2.offset(i)) as *mut CardProfileInfo2Internal };
            if !indexed_ptr.is_null() {
                profiles_vec.push(CardProfileInfo2::new_from_raw(indexed_ptr));
            }
        }

        unsafe {
            CardPortInfo {
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                description: match src.description.is_null() {
                    false => Some(CStr::from_ptr(src.description).to_string_lossy()),
                    true => None,
                },
                priority: src.priority,
                available: std::mem::transmute(src.available),
                direction: src.direction,
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                latency_offset: src.latency_offset,
                profiles: profiles_vec,
            }
        }
    }
}

/// Stores information about cards.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct CardInfo<'a> {
    /// Index of this card.
    pub index: u32,
    /// Name of this card.
    pub name: Option<Cow<'a, str>>,
    /// Index of the owning module, or `None`.
    pub owner_module: Option<u32>,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// Set of ports.
    pub ports: Vec<CardPortInfo<'a>>,
    /// Set of available profiles.
    pub profiles: Vec<CardProfileInfo2<'a>>,
    /// Pointer to active profile in the set, or `None`.
    pub active_profile: Option<Box<CardProfileInfo2<'a>>>,
}

impl<'a> CardInfo<'a> {
    fn new_from_raw(p: *const CardInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };

        let mut ports_vec = Vec::with_capacity(src.n_ports as usize);
        assert!(src.n_ports == 0 || !src.ports.is_null());
        for i in 0..src.n_ports as isize {
            let indexed_ptr = unsafe { (*src.ports.offset(i)) as *mut CardPortInfoInternal };
            if !indexed_ptr.is_null() {
                ports_vec.push(CardPortInfo::new_from_raw(indexed_ptr));    
            }
        }
        let mut profiles_vec = Vec::with_capacity(src.n_profiles as usize);
        assert!(src.n_profiles == 0 || !src.profiles2.is_null());
        for i in 0..src.n_profiles as isize {
            let indexed_ptr = unsafe { (*src.profiles2.offset(i)) as *mut CardProfileInfo2Internal };
            if !indexed_ptr.is_null() {
                profiles_vec.push(CardProfileInfo2::new_from_raw(indexed_ptr));
            }
        }

        unsafe {
            CardInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                ports: ports_vec,
                profiles: profiles_vec,
                active_profile: match src.active_profile2.is_null() {
                    true => None,
                    false => Some(Box::new(CardProfileInfo2::new_from_raw(src.active_profile2))),
                },
            }
        }
    }
}

impl Introspector {
    /// Get information about a card by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_by_index<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&CardInfo>)>
        where F: FnMut(ListResult<&CardInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&CardInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_card_info_by_index(self.context, index,
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&CardInfo>)>)
    }

    /// Get information about a card by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_by_name<F>(&self, name: &str, callback: F
        ) -> Operation<FnMut(ListResult<&CardInfo>)>
        where F: FnMut(ListResult<&CardInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&CardInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_card_info_by_name(self.context, c_name.as_ptr(),
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&CardInfo>)>)
    }

    /// Get the complete card list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_card_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&CardInfo>)>
        where F: FnMut(ListResult<&CardInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&CardInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_card_info_list(self.context,
            Some(get_card_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&CardInfo>)>)
    }

    /// Change the profile of a card.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_card_profile_by_index(&mut self, index: u32, profile: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_profile = CString::new(profile.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_card_profile_by_index(self.context, index,
            c_profile.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Change the profile of a card.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_card_profile_by_name(&mut self, name: &str, profile: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();
        let c_profile = CString::new(profile.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_card_profile_by_name(self.context, c_name.as_ptr(),
            c_profile.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the latency offset of a port.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_port_latency_offset(&mut self, card_name: &str, port_name: &str, offset: i64,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(card_name.clone()).unwrap();
        let c_port = CString::new(port_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_port_latency_offset(self.context, c_name.as_ptr(),
            c_port.as_ptr(), offset, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get card info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_card_info_list_cb_proxy(_: *mut ContextInternal, i: *const CardInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&CardInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = CardInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sink input info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about sink inputs.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SinkInputInfo<'a> {
    /// Index of the sink input.
    pub index: u32,
    /// Name of the sink input.
    pub name: Option<Cow<'a, str>>,
    /// Index of the module this sink input belongs to, or `None` when it does not belong to any
    /// module.
    pub owner_module: Option<u32>,
    /// Index of the client this sink input belongs to, or invalid when it does not belong to any
    /// client.
    pub client: Option<u32>,
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
    pub resample_method: Option<Cow<'a, str>>,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Stream muted.
    pub mute: bool,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// Stream corked.
    pub corked: bool,
    /// Stream has volume. If not set, then the meaning of this struct's volume member is unspecified.
    pub has_volume: bool,
    /// The volume can be set. If not set, the volume can still change even though clients can't
    /// control the volume.
    pub volume_writable: bool,
    /// Stream format information.
    pub format: ::format::Info,
}

impl<'a> SinkInputInfo<'a> {
    fn new_from_raw(p: *const SinkInputInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            SinkInputInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                client: match src.client {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                sink: src.sink,
                sample_spec: std::mem::transmute(src.sample_spec),
                channel_map: std::mem::transmute(src.channel_map),
                volume: std::mem::transmute(src.volume),
                buffer_usec: MicroSeconds(src.buffer_usec),
                sink_usec: MicroSeconds(src.sink_usec),
                resample_method: match src.resample_method.is_null() {
                    false => Some(CStr::from_ptr(src.resample_method).to_string_lossy()),
                    true => None,
                },
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                mute: match src.mute { 0 => false, _ => true },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                corked: match src.corked { 0 => false, _ => true },
                has_volume: match src.has_volume { 0 => false, _ => true },
                volume_writable: match src.volume_writable { 0 => false, _ => true },
                format: ::format::Info::from_raw_weak(src.format as *mut ::format::InfoInternal),
            }
        }
    }
}

impl Introspector {
    /// Get some information about a sink input by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_input_info<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&SinkInputInfo>)>
        where F: FnMut(ListResult<&SinkInputInfo>) + 'static
    {
        let cb_data =
            box_closure_get_capi_ptr::<FnMut(ListResult<&SinkInputInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sink_input_info(self.context, index,
            Some(get_sink_input_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SinkInputInfo>)>)
    }

    /// Get the complete sink input list.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sink_input_info_list<F>(&self, callback: F
        ) -> Operation<FnMut(ListResult<&SinkInputInfo>)>
        where F: FnMut(ListResult<&SinkInputInfo>) + 'static
    {
        let cb_data =
            box_closure_get_capi_ptr::<FnMut(ListResult<&SinkInputInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sink_input_info_list(self.context,
            Some(get_sink_input_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SinkInputInfo>)>)
    }

    /// Move the specified sink input to a different sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_sink_input_by_name(&mut self, index: u32, sink_name: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(sink_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_move_sink_input_by_name(self.context, index,
            c_name.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Move the specified sink input to a different sink.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_sink_input_by_index(&mut self, index: u32, sink_index: u32,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_move_sink_input_by_index(self.context, index,
            sink_index, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the volume of a sink input stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_input_volume(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_input_volume(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a sink input stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_sink_input_mute(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_sink_input_mute(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Kill a sink input.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_sink_input<F>(&mut self, index: u32, callback: F) -> Operation<FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_kill_sink_input(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get sink input info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sink_input_info_list_cb_proxy(_: *mut ContextInternal, i: *const SinkInputInfoInternal,
    eol: i32, userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&SinkInputInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = SinkInputInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Source output info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about source outputs.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SourceOutputInfo<'a> {
    /// Index of the source output.
    pub index: u32,
    /// Name of the source output.
    pub name: Option<Cow<'a, str>>,
    /// Index of the module this source output belongs to, or `None` when it does not belong to any
    /// module.
    pub owner_module: Option<u32>,
    /// Index of the client this source output belongs to, or `None` when it does not belong to any
    /// client.
    pub client: Option<u32>,
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
    pub resample_method: Option<Cow<'a, str>>,
    /// Driver name.
    pub driver: Option<Cow<'a, str>>,
    /// Property list.
    pub proplist: ::proplist::Proplist,
    /// Stream corked.
    pub corked: bool,
    /// The volume of this source output.
    pub volume: ::volume::ChannelVolumes,
    /// Stream muted.
    pub mute: bool,
    /// Stream has volume. If not set, then the meaning of this struct's volume member is unspecified.
    pub has_volume: bool,
    /// The volume can be set. If not set, the volume can still change even though clients can't
    /// control the volume.
    pub volume_writable: bool,
    /// Stream format information.
    pub format: ::format::Info,
}

impl<'a> SourceOutputInfo<'a> {
    fn new_from_raw(p: *const SourceOutputInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            SourceOutputInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                owner_module: match src.owner_module {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                client: match src.client {
                    ::def::INVALID_INDEX => None,
                    i => Some(i),
                },
                source: src.source,
                sample_spec: std::mem::transmute(src.sample_spec),
                channel_map: std::mem::transmute(src.channel_map),
                buffer_usec: MicroSeconds(src.buffer_usec),
                source_usec: MicroSeconds(src.source_usec),
                resample_method: match src.resample_method.is_null() {
                    false => Some(CStr::from_ptr(src.resample_method).to_string_lossy()),
                    true => None,
                },
                driver: match src.driver.is_null() {
                    false => Some(CStr::from_ptr(src.driver).to_string_lossy()),
                    true => None,
                },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
                corked: match src.corked { 0 => false, _ => true },
                volume: std::mem::transmute(src.volume),
                mute: match src.mute { 0 => false, _ => true },
                has_volume: match src.has_volume { 0 => false, _ => true },
                volume_writable: match src.volume_writable { 0 => false, _ => true },
                format: ::format::Info::from_raw_weak(src.format as *mut ::format::InfoInternal),
            }
        }
    }
}

impl Introspector {
    /// Get information about a source output by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_output_info<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&SourceOutputInfo>)>
        where F: FnMut(ListResult<&SourceOutputInfo>) + 'static
    {
        let cb_data =
            box_closure_get_capi_ptr::<FnMut(ListResult<&SourceOutputInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_source_output_info(self.context, index,
            Some(get_source_output_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SourceOutputInfo>)>)
    }

    /// Get the complete list of source outputs.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_source_output_info_list<F>(&self, callback: F
        ) -> Operation<FnMut(ListResult<&SourceOutputInfo>)>
        where F: FnMut(ListResult<&SourceOutputInfo>) + 'static
    {
        let cb_data =
            box_closure_get_capi_ptr::<FnMut(ListResult<&SourceOutputInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_source_output_info_list(self.context,
            Some(get_source_output_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SourceOutputInfo>)>)
    }

    /// Move the specified source output to a different source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_source_output_by_name(&mut self, index: u32, source_name: &str,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(source_name.clone()).unwrap();

        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_move_source_output_by_name(self.context, index,
            c_name.as_ptr(), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Move the specified source output to a different source.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn move_source_output_by_index(&mut self, index: u32, source_index: u32,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_move_source_output_by_index(self.context, index,
            source_index, cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the volume of a source output stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_output_volume(&mut self, index: u32, volume: &::volume::ChannelVolumes,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_output_volume(self.context, index,
            std::mem::transmute(volume), cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Set the mute switch of a source output stream.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The optional callback must accept a `bool`, which indicates success.
    pub fn set_source_output_mute(&mut self, index: u32, mute: bool,
        callback: Option<Box<FnMut(bool) + 'static>>) -> Operation<FnMut(bool)>
    {
        let (cb_fn, cb_data): (Option<extern "C" fn(_, _, _)>, _) =
            ::callbacks::get_su_capi_params::<_, _>(callback, super::success_cb_proxy);
        let ptr = unsafe { capi::pa_context_set_source_output_mute(self.context, index, mute as i32,
            cb_fn, cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }

    /// Kill a source output.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    ///
    /// The callback must accept a `bool`, which indicates success.
    pub fn kill_source_output<F>(&mut self, index: u32, callback: F) -> Operation<FnMut(bool)>
        where F: FnMut(bool) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(bool)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_kill_source_output(self.context, index,
            Some(super::success_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(bool)>)
    }
}

/// Proxy for get source output info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_source_output_info_list_cb_proxy(_: *mut ContextInternal, i: *const SourceOutputInfoInternal,
    eol: i32, userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&SourceOutputInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = SourceOutputInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stat info
////////////////////////////////////////////////////////////////////////////////////////////////////

impl Introspector {
    /// Get daemon memory block statistics.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn stat<F>(&self, callback: F) -> Operation<FnMut(&StatInfo)>
        where F: FnMut(&StatInfo) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(&StatInfo)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_stat(self.context, Some(get_stat_info_cb_proxy),
            cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(&StatInfo)>)
    }
}

/// Proxy for get stat info callbacks.
/// Warning: This is for single-use cases only! It destroys the actual closure callback.
extern "C"
fn get_stat_info_cb_proxy(_: *mut ContextInternal, i: *const StatInfo, userdata: *mut c_void) {
    assert!(!i.is_null());
    // Note, destroys closure callback after use - restoring outer box means it gets dropped
    let mut callback = ::callbacks::get_su_callback::<FnMut(&StatInfo)>(userdata);
    callback(unsafe { i.as_ref().unwrap() });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sample info
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores information about sample cache entries.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[derive(Debug)]
pub struct SampleInfo<'a> {
    /// Index of this entry.
    pub index: u32,
    /// Name of this entry.
    pub name: Option<Cow<'a, str>>,
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
    pub lazy: bool,
    /// In case this is a lazy cache entry, the filename for the sound file to be loaded on demand.
    pub filename: Option<Cow<'a, str>>,
    /// Property list for this sample.
    pub proplist: ::proplist::Proplist,
}

impl<'a> SampleInfo<'a> {
    fn new_from_raw(p: *const SampleInfoInternal) -> Self {
        assert!(!p.is_null());
        let src = unsafe { p.as_ref().unwrap() };
        unsafe {
            SampleInfo {
                index: src.index,
                name: match src.name.is_null() {
                    false => Some(CStr::from_ptr(src.name).to_string_lossy()),
                    true => None,
                },
                volume: std::mem::transmute(src.volume),
                sample_spec: std::mem::transmute(src.sample_spec),
                channel_map: std::mem::transmute(src.channel_map),
                duration: MicroSeconds(src.duration),
                bytes: src.bytes,
                lazy: match src.lazy { 0 => false, _ => true },
                filename: match src.filename.is_null() {
                    false => Some(CStr::from_ptr(src.filename).to_string_lossy()),
                    true => None,
                },
                proplist: ::proplist::Proplist::from_raw_weak(src.proplist),
            }
        }
    }
}

impl Introspector {
    /// Get information about a sample by its name.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_by_name<F>(&self, name: &str, callback: F
        ) -> Operation<FnMut(ListResult<&SampleInfo>)>
        where F: FnMut(ListResult<&SampleInfo>) + 'static
    {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_name = CString::new(name.clone()).unwrap();

        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SampleInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sample_info_by_name(self.context, c_name.as_ptr(),
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SampleInfo>)>)
    }

    /// Get information about a sample by its index.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_by_index<F>(&self, index: u32, callback: F
        ) -> Operation<FnMut(ListResult<&SampleInfo>)>
        where F: FnMut(ListResult<&SampleInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SampleInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sample_info_by_index(self.context, index,
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SampleInfo>)>)
    }

    /// Get the complete list of samples stored in the daemon.
    ///
    /// Returns `None` on error, i.e. invalid arguments or state.
    pub fn get_sample_info_list<F>(&self, callback: F) -> Operation<FnMut(ListResult<&SampleInfo>)>
        where F: FnMut(ListResult<&SampleInfo>) + 'static
    {
        let cb_data = box_closure_get_capi_ptr::<FnMut(ListResult<&SampleInfo>)>(Box::new(callback));
        let ptr = unsafe { capi::pa_context_get_sample_info_list(self.context,
            Some(get_sample_info_list_cb_proxy), cb_data) };
        assert!(!ptr.is_null());
        Operation::from_raw(ptr, cb_data as *mut Box<FnMut(ListResult<&SampleInfo>)>)
    }
}

/// Proxy for get sample info list callbacks.
/// Warning: This is for list cases only! On EOL it destroys the actual closure callback.
extern "C"
fn get_sample_info_list_cb_proxy(_: *mut ContextInternal, i: *const SampleInfoInternal, eol: i32,
    userdata: *mut c_void)
{
    match callback_for_list_instance::<FnMut(ListResult<&SampleInfo>)>(eol, userdata) {
        ListInstanceCallback::Entry(callback) => {
            assert!(!i.is_null());
            let obj = SampleInfo::new_from_raw(i);
            callback(ListResult::Item(&obj));
        },
        ListInstanceCallback::End(mut callback) => { callback(ListResult::End); },
        ListInstanceCallback::Error(mut callback) => { callback(ListResult::Error); },
    }
}
