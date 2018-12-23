// Copyright 2017 Lyndon Brown
//
// This file is part of the PulseAudio Rust language linking library.
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

//! Utility functions for handling a stream or sink format.

use std::os::raw::c_char;
use crate::sample::{pa_sample_spec, pa_sample_format_t};
use crate::{proplist::pa_proplist, channelmap::pa_channel_map};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum pa_encoding_t {
    Any,
    PCM,
    AC3_IEC61937,
    EAC3_IEC61937,
    MPEG_IEC61937,
    DTS_IEC61937,
    MPEG2_AAC_IEC61937,

    Invalid = -1,
}

pub const PA_ENCODING_MAX: usize = 7;

pub const PA_ENCODING_ANY: pa_encoding_t = pa_encoding_t::Any;
pub const PA_ENCODING_PCM: pa_encoding_t = pa_encoding_t::PCM;
pub const PA_ENCODING_AC3_IEC61937: pa_encoding_t = pa_encoding_t::AC3_IEC61937;
pub const PA_ENCODING_EAC3_IEC61937: pa_encoding_t = pa_encoding_t::EAC3_IEC61937;
pub const PA_ENCODING_MPEG_IEC61937: pa_encoding_t = pa_encoding_t::MPEG_IEC61937;
pub const PA_ENCODING_DTS_IEC61937: pa_encoding_t = pa_encoding_t::DTS_IEC61937;
pub const PA_ENCODING_MPEG2_AAC_IEC61937: pa_encoding_t = pa_encoding_t::MPEG2_AAC_IEC61937;
pub const PA_ENCODING_INVALID: pa_encoding_t = pa_encoding_t::Invalid;

impl Default for pa_encoding_t {
    fn default() -> Self {
        pa_encoding_t::Invalid
    }
}

/// Represents the format of data provided in a stream or processed by a sink.
#[repr(C)]
pub struct pa_format_info {
    pub encoding: pa_encoding_t,
    pub plist: *mut pa_proplist,
}

/// The maximum length of strings returned by [`pa_format_info_snprint`](fn.pa_format_info_snprint.html).
///
/// Please note that this value can change with any release without warning and without being
/// considered API or ABI breakage. You should not use this definition anywhere where it might
/// become part of an ABI.
pub const PA_FORMAT_INFO_SNPRINT_MAX: usize = 256;

/// Represents the type of value of a property.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum pa_prop_type_t {
    Int,
    IntRange,
    IntArray,
    String,
    StringArray,

    Invalid = -1,
}

pub const PA_PROP_TYPE_INT: pa_prop_type_t = pa_prop_type_t::Int;
pub const PA_PROP_TYPE_INT_RANGE: pa_prop_type_t = pa_prop_type_t::IntRange;
pub const PA_PROP_TYPE_INT_ARRAY: pa_prop_type_t = pa_prop_type_t::IntArray;
pub const PA_PROP_TYPE_STRING: pa_prop_type_t = pa_prop_type_t::String;
pub const PA_PROP_TYPE_STRING_ARRAY: pa_prop_type_t = pa_prop_type_t::StringArray;
pub const PA_PROP_TYPE_INVALID: pa_prop_type_t = pa_prop_type_t::Invalid;

impl Default for pa_prop_type_t {
    fn default() -> Self {
        pa_prop_type_t::Invalid
    }
}

#[link(name="pulse")]
extern "C" {
    pub fn pa_encoding_to_string(e: pa_encoding_t) -> *const c_char;

    // Symbol was missing from PA’s symbol file prior to PA v12
    #[cfg(feature = "pa_v12_compatibility")]
    pub fn pa_encoding_from_string(encoding: *const c_char) -> pa_encoding_t;

    pub fn pa_format_info_new() -> *mut pa_format_info;
    pub fn pa_format_info_copy(src: *const pa_format_info) -> *mut pa_format_info;
    pub fn pa_format_info_free(f: *mut pa_format_info);
    pub fn pa_format_info_valid(f: *const pa_format_info) -> i32;
    pub fn pa_format_info_is_pcm(f: *const pa_format_info) -> i32;
    pub fn pa_format_info_is_compatible(first: *const pa_format_info, second: *const pa_format_info) -> i32;
    pub fn pa_format_info_snprint(s: *mut c_char, l: usize, f: *const pa_format_info) -> *mut c_char;
    pub fn pa_format_info_from_string(s: *const c_char) -> *mut pa_format_info;
    pub fn pa_format_info_from_sample_spec(ss: *const pa_sample_spec, map: *const pa_channel_map) -> *mut pa_format_info;
    pub fn pa_format_info_to_sample_spec(f: *const pa_format_info, ss: *mut pa_sample_spec, map: *mut pa_channel_map) -> i32;
    pub fn pa_format_info_get_prop_type(f: *const pa_format_info, key: *const c_char) -> pa_prop_type_t;
    pub fn pa_format_info_get_prop_int(f: *const pa_format_info, key: *const c_char, v: *mut i32) -> i32;
    pub fn pa_format_info_get_prop_int_range(f: *const pa_format_info, key: *const c_char, min: *mut i32, max: *mut i32) -> i32;
    pub fn pa_format_info_get_prop_int_array(f: *const pa_format_info, key: *const c_char, values: *mut *mut i32, n_values: *mut i32) -> i32;
    pub fn pa_format_info_get_prop_string(f: *const pa_format_info, key: *const c_char, v: *mut *mut c_char) -> i32;
    pub fn pa_format_info_get_prop_string_array(f: *const pa_format_info, key: *const c_char, values: *mut *mut *mut c_char, n_values: *mut i32) -> i32;
    pub fn pa_format_info_free_string_array(values: *mut *mut c_char, n_values: i32);
    pub fn pa_format_info_set_prop_int(f: *mut pa_format_info, key: *const c_char, value: i32);
    pub fn pa_format_info_set_prop_int_array(f: *mut pa_format_info, key: *const c_char, values: *const i32, n_values: i32);
    pub fn pa_format_info_set_prop_int_range(f: *mut pa_format_info, key: *const c_char, min: i32, max: i32);
    pub fn pa_format_info_set_prop_string(f: *mut pa_format_info, key: *const c_char, value: *const c_char);
    pub fn pa_format_info_set_prop_string_array(f: *mut pa_format_info, key: *const c_char, values: *const *const c_char, n_values: i32);
    pub fn pa_format_info_set_sample_format(f: *mut pa_format_info, sf: pa_sample_format_t);
    pub fn pa_format_info_set_rate(f: *mut pa_format_info, rate: i32);
    pub fn pa_format_info_set_channels(f: *mut pa_format_info, channels: i32);
    pub fn pa_format_info_set_channel_map(f: *mut pa_format_info, map: *const pa_channel_map);
}
