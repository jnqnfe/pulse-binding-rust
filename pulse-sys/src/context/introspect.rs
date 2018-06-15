//! Routines for daemon introspection.

// This file is part of the PulseAudio Rust language linking library.
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

use std::os::raw::{c_char, c_void};
use super::{pa_context, pa_context_success_cb_t};

#[repr(C)]
pub struct pa_sink_port_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub priority: u32,
    pub available: i32,
}

#[repr(C)]
pub struct pa_sink_info {
    pub name: *const c_char,
    pub index: u32,
    pub description: *const c_char,
    pub sample_spec: ::sample::pa_sample_spec,
    pub channel_map: ::channelmap::pa_channel_map,
    pub owner_module: u32,
    pub volume: ::volume::pa_cvolume,
    pub mute: i32,
    pub monitor_source: u32,
    pub monitor_source_name: *const c_char,
    pub latency: ::sample::pa_usec_t,
    pub driver: *const c_char,
    pub flags: ::def::pa_sink_flags_t,
    pub proplist: *mut ::proplist::pa_proplist,
    pub configured_latency: ::sample::pa_usec_t,
    pub base_volume: ::volume::pa_volume_t,
    pub state: ::def::pa_sink_state_t,
    pub n_volume_steps: u32,
    pub card: u32,
    pub n_ports: u32,
    pub ports: *mut *mut pa_sink_port_info,
    pub active_port: *mut pa_sink_port_info,
    pub n_formats: u8,
    pub formats: *mut *mut ::format::pa_format_info,
}

pub type pa_sink_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_sink_info, eol: i32, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_source_port_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub priority: u32,
    pub available: i32,
}

#[repr(C)]
pub struct pa_source_info {
    pub name: *const c_char,
    pub index: u32,
    pub description: *const c_char,
    pub sample_spec: ::sample::pa_sample_spec,
    pub channel_map: ::channelmap::pa_channel_map,
    pub owner_module: u32,
    pub volume: ::volume::pa_cvolume,
    pub mute: i32,
    pub monitor_of_sink: u32,
    pub monitor_of_sink_name: *const c_char,
    pub latency: ::sample::pa_usec_t,
    pub driver: *const c_char,
    pub flags: ::def::pa_source_flags_t,
    pub proplist: *mut ::proplist::pa_proplist,
    pub configured_latency: ::sample::pa_usec_t,
    pub base_volume: ::volume::pa_volume_t,
    pub state: ::def::pa_source_state_t,
    pub n_volume_steps: u32,
    pub card: u32,
    pub n_ports: u32,
    pub ports: *mut *mut pa_source_port_info,
    pub active_port: *mut pa_source_port_info,
    pub n_formats: u8,
    pub formats: *mut *mut ::format::pa_format_info,
}

pub type pa_source_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_source_info, eol: i32, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_server_info {
    pub user_name: *const c_char,
    pub host_name: *const c_char,
    pub server_version: *const c_char,
    pub server_name: *const c_char,
    pub sample_spec: ::sample::pa_sample_spec,
    pub default_sink_name: *const c_char,
    pub default_source_name: *const c_char,
    pub cookie: u32,
    pub channel_map: ::channelmap::pa_channel_map,
}

pub type pa_server_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_server_info, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_module_info {
    pub index: u32,
    pub name: *const c_char,
    pub argument: *const c_char,
    pub n_used: u32,
    #[deprecated]
    pub auto_unload: i32,
    pub proplist: *mut ::proplist::pa_proplist,
}

pub type pa_module_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_module_info, eol: i32, userdata: *mut c_void)>;

pub type pa_context_index_cb_t = Option<extern "C" fn(c: *mut pa_context, idx: u32, userdata: *mut c_void)>;

/// Stores information about clients.
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct pa_client_info {
    /// Index of this client.
    pub index: u32,
    /// Name of this client.
    pub name: *const c_char,
    /// Index of the owning module, or
    /// [`::def::PA_INVALID_INDEX`](../../def/constant.PA_INVALID_INDEX.html).
    pub owner_module: u32,
    /// Driver name.
    pub driver: *const c_char,
    /// Property list.
    pub proplist: *mut ::proplist::pa_proplist,
}

pub type pa_client_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_client_info, eol: i32, userdata: *mut c_void)>;

#[repr(C)]
#[deprecated]
pub struct pa_card_profile_info {
    /// Name of this profile.
    pub name: *const c_char,
    /// Description of this profile.
    pub description: *const c_char,
    /// Number of sinks this profile would create.
    pub n_sinks: u32,
    /// Number of sources this profile would create.
    pub n_sources: u32,
    /// The higher this value is, the more useful this profile is as a default.
    pub priority: u32,
}

/// Stores information about a specific profile of a card.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
pub struct pa_card_profile_info2 {
    /// Name of this profile.
    pub name: *const c_char,
    /// Description of this profile.
    pub description: *const c_char,
    /// Number of sinks this profile would create.
    pub n_sinks: u32,
    /// Number of sources this profile would create.
    pub n_sources: u32,
    /// The higher this value is, the more useful this profile is as a default.
    pub priority: u32,

    /// Is this profile available? If this is zero, meaning "unavailable", then it makes no sense to
    /// try to activate this profile. If this is non-zero, it's still not a guarantee that
    /// activating the profile will result in anything useful, it just means that the server isn't
    /// aware of any reason why the profile would definitely be useless.
    pub available: i32,
}

#[repr(C)]
pub struct pa_card_port_info {
    pub name: *const c_char,
    pub description: *const c_char,
    pub priority: u32,
    pub available: i32,
    pub direction: i32,
    pub n_profiles: u32,
    #[deprecated]
    #[allow(deprecated)]
    pub profiles: *mut *mut pa_card_profile_info,
    pub proplist: *mut ::proplist::pa_proplist,
    pub latency_offset: i64,
    pub profiles2: *mut *mut pa_card_profile_info2,
}

#[repr(C)]
pub struct pa_card_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub driver: *const c_char,
    pub n_profiles: u32,
    #[deprecated]
    #[allow(deprecated)]
    pub profiles: *mut pa_card_profile_info,
    #[deprecated]
    #[allow(deprecated)]
    pub active_profile: *mut pa_card_profile_info,
    pub proplist: *mut ::proplist::pa_proplist,
    pub n_ports: u32,
    pub ports: *mut *mut pa_card_port_info,
    pub profiles2: *mut *mut pa_card_profile_info2,
    pub active_profile2: *mut pa_card_profile_info2,
}

pub type pa_card_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_card_info, eol: i32, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_sink_input_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub client: u32,
    pub sink: u32,
    pub sample_spec: ::sample::pa_sample_spec,
    pub channel_map: ::channelmap::pa_channel_map,
    pub volume: ::volume::pa_cvolume,
    pub buffer_usec: ::sample::pa_usec_t,
    pub sink_usec: ::sample::pa_usec_t,
    pub resample_method: *const c_char,
    pub driver: *const c_char,
    pub mute: i32,
    pub proplist: *mut ::proplist::pa_proplist,
    pub corked: i32,
    pub has_volume: i32,
    pub volume_writable: i32,
    pub format: *mut ::format::pa_format_info,
}

pub type pa_sink_input_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_sink_input_info, eol: i32, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_source_output_info {
    pub index: u32,
    pub name: *const c_char,
    pub owner_module: u32,
    pub client: u32,
    pub source: u32,
    pub sample_spec: ::sample::pa_sample_spec,
    pub channel_map: ::channelmap::pa_channel_map,
    pub buffer_usec: ::sample::pa_usec_t,
    pub source_usec: ::sample::pa_usec_t,
    pub resample_method: *const c_char,
    pub driver: *const c_char,
    pub proplist: *mut ::proplist::pa_proplist,
    pub corked: i32,
    pub volume: ::volume::pa_cvolume,
    pub mute: i32,
    pub has_volume: i32,
    pub volume_writable: i32,
    pub format: *mut ::format::pa_format_info,
}

pub type pa_source_output_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_source_output_info, eol: i32, userdata: *mut c_void)>;

/// Memory block statistics.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
#[repr(C)]
#[derive(Debug)]
pub struct pa_stat_info {
    /// Currently allocated memory blocks.
    pub memblock_total: u32,
    /// Current total size of allocated memory blocks.
    pub memblock_total_size: u32,
    /// Allocated memory blocks during the whole lifetime of the daemon.
    pub memblock_allocated: u32,
    /// Total size of all memory blocks allocated during the whole lifetime of the daemon.
    pub memblock_allocated_size: u32,
    /// Total size of all sample cache entries.
    pub scache_size: u32,
}

pub type pa_stat_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_stat_info, userdata: *mut c_void)>;

#[repr(C)]
pub struct pa_sample_info {
    pub index: u32,
    pub name: *const c_char,
    pub volume: ::volume::pa_cvolume,
    pub sample_spec: ::sample::pa_sample_spec,
    pub channel_map: ::channelmap::pa_channel_map,
    pub duration: ::sample::pa_usec_t,
    pub bytes: u32,
    pub lazy: i32,
    pub filename: *const c_char,
    pub proplist: *mut ::proplist::pa_proplist,
}

pub type pa_sample_info_cb_t = Option<extern "C" fn(c: *mut pa_context, i: *const pa_sample_info, eol: i32, userdata: *mut c_void)>;

#[link(name="pulse")]
extern "C" {
    pub fn pa_context_get_sink_info_by_name(c: *mut pa_context, name: *const c_char, cb: pa_sink_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_sink_info_by_index(c: *mut pa_context, idx: u32, cb: pa_sink_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_sink_info_list(c: *mut pa_context, cb: pa_sink_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_volume_by_index(c: *mut pa_context, idx: u32, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_volume_by_name(c: *mut pa_context, name: *const c_char, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_mute_by_index(c: *mut pa_context, idx: u32, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_mute_by_name(c: *mut pa_context, name: *const c_char, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_suspend_sink_by_name(c: *mut pa_context, sink_name: *const c_char, suspend: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_suspend_sink_by_index(c: *mut pa_context, idx: u32, suspend: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_port_by_index(c: *mut pa_context, idx: u32, port: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_port_by_name(c: *mut pa_context, name: *const c_char, port: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_source_info_by_name(c: *mut pa_context, name: *const c_char, cb: pa_source_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_source_info_by_index(c: *mut pa_context, idx: u32, cb: pa_source_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_source_info_list(c: *mut pa_context, cb: pa_source_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_volume_by_index(c: *mut pa_context, idx: u32, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_volume_by_name(c: *mut pa_context, name: *const c_char, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_mute_by_index(c: *mut pa_context, idx: u32, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_mute_by_name(c: *mut pa_context, name: *const c_char, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_suspend_source_by_name(c: *mut pa_context, source_name: *const c_char, suspend: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_suspend_source_by_index(c: *mut pa_context, idx: u32, suspend: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_port_by_index(c: *mut pa_context, idx: u32, port: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_port_by_name(c: *mut pa_context, name: *const c_char, port: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_server_info(c: *mut pa_context, cb: pa_server_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_module_info(c: *mut pa_context, idx: u32, cb: pa_module_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_module_info_list(c: *mut pa_context, cb: pa_module_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_load_module(c: *mut pa_context, name: *const c_char, argument: *const c_char, cb: pa_context_index_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_unload_module(c: *mut pa_context, idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_client_info(c: *mut pa_context, idx: u32, cb: pa_client_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_client_info_list(c: *mut pa_context, cb: pa_client_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_kill_client(c: *mut pa_context, idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_card_info_by_index(c: *mut pa_context, idx: u32, cb: pa_card_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_card_info_by_name(c: *mut pa_context, name: *const c_char, cb: pa_card_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_card_info_list(c: *mut pa_context, cb: pa_card_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_card_profile_by_index(c: *mut pa_context, idx: u32, profile: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_card_profile_by_name(c: *mut pa_context, name: *const c_char, profile: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_port_latency_offset(c: *mut pa_context, card_name: *const c_char, port_name: *const c_char, offset: i64, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_sink_input_info(c: *mut pa_context, idx: u32, cb: pa_sink_input_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_sink_input_info_list(c: *mut pa_context, cb: pa_sink_input_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_move_sink_input_by_name(c: *mut pa_context, idx: u32, sink_name: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_move_sink_input_by_index(c: *mut pa_context, idx: u32, sink_idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_input_volume(c: *mut pa_context, idx: u32, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_sink_input_mute(c: *mut pa_context, idx: u32, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_kill_sink_input(c: *mut pa_context, idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_source_output_info(c: *mut pa_context, idx: u32, cb: pa_source_output_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_source_output_info_list(c: *mut pa_context, cb: pa_source_output_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_move_source_output_by_name(c: *mut pa_context, idx: u32, source_name: *const c_char, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_move_source_output_by_index(c: *mut pa_context, idx: u32, source_idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_output_volume(c: *mut pa_context, idx: u32, volume: *const ::volume::pa_cvolume, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_set_source_output_mute(c: *mut pa_context, idx: u32, mute: i32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_kill_source_output(c: *mut pa_context, idx: u32, cb: pa_context_success_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_stat(c: *mut pa_context, cb: pa_stat_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;

    pub fn pa_context_get_sample_info_by_name(c: *mut pa_context, name: *const c_char, cb: pa_sample_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_sample_info_by_index(c: *mut pa_context, idx: u32, cb: pa_sample_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
    pub fn pa_context_get_sample_info_list(c: *mut pa_context, cb: pa_sample_info_cb_t, userdata: *mut c_void) -> *mut ::operation::pa_operation;
}
