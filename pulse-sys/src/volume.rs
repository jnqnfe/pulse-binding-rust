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

//! Constants and routines for volume handling.

use std;
use std::os::raw::c_char;

/// The basic volume type
pub type pa_volume_t = u32;

/// Normal volume (100%, 0 dB)
pub const PA_VOLUME_NORM: pa_volume_t = 0x10000;

/// Muted (minimal valid) volume (0%, -inf dB)
pub const PA_VOLUME_MUTED: pa_volume_t = 0;

/// Maximum valid volume we can store
pub const PA_VOLUME_MAX: pa_volume_t = std::u32::MAX / 2;

#[inline(always)]
pub fn pa_volume_ui_max() -> pa_volume_t {
    unsafe { pa_sw_volume_from_dB(11.0) }
}

/// Special 'invalid' volume
pub const PA_VOLUME_INVALID: pa_volume_t = std::u32::MAX;

/// This floor value is used as minus infinity when using
/// [`pa_sw_volume_to_dB`](fn.pa_sw_volume_to_dB.html) /
/// [`pa_sw_volume_from_dB`](fn.pa_sw_volume_from_dB.html).
pub const PA_DECIBEL_MININFTY: f64 = -std::f64::INFINITY;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct pa_cvolume {
    pub channels: u8,
    pub values: [pa_volume_t; ::sample::PA_CHANNELS_MAX],
}

/// The maximum length of strings returned by [`pa_cvolume_snprint`]. Please note that this value
/// can change with any release without warning and without being considered API or ABI breakage.
/// You should not use this definition anywhere where it might become part of an ABI.
///
/// [`pa_cvolume_snprint`]: fn.pa_cvolume_snprint.html
pub const PA_CVOLUME_SNPRINT_MAX: usize = 320;

/// The maximum length of strings returned by [`pa_sw_cvolume_snprint_dB`]. Please note that this
/// value can change with any release without warning and without being considered API or ABI
/// breakage. You should not use this definition anywhere where it might become part of an ABI.
///
/// [`pa_sw_cvolume_snprint_dB`]: fn.pa_sw_cvolume_snprint_dB.html
pub const PA_SW_CVOLUME_SNPRINT_DB_MAX: usize = 448;

/// The maximum length of strings returned by [`pa_cvolume_snprint_verbose`]. Please note that this
/// value can change with any release without warning and without being considered API or ABI
/// breakage. You should not use this definition anywhere where it might become part of an ABI.
///
/// [`pa_cvolume_snprint_verbose`]: fn.pa_cvolume_snprint_verbose.html
pub const PA_CVOLUME_SNPRINT_VERBOSE_MAX: usize = 1984;

/// The maximum length of strings returned by [`pa_volume_snprint`](fn.pa_volume_snprint.html).
/// Please note that this value can change with any release without warning and without being
/// considered API or ABI breakage. You should not use this definition anywhere where it might
/// become part of an ABI.
pub const PA_VOLUME_SNPRINT_MAX: usize = 10;

/// The maximum length of strings returned by [`pa_sw_volume_snprint_dB`]. Please note that this
/// value can change with any release without warning and without being considered API or ABI
/// breakage. You should not use this definition anywhere where it might become part of an ABI.
///
/// [`pa_sw_volume_snprint_dB`]: fn.pa_sw_volume_snprint_dB.html
pub const PA_SW_VOLUME_SNPRINT_DB_MAX: usize = 11;

/// The maximum length of strings returned by [`pa_volume_snprint_verbose`](fn.print_verbose.html),
/// as per the underlying C function. Please note that this value can change with any release
/// without warning and without being considered API or ABI breakage. You should not use this
/// definition anywhere where it might become part of an ABI.
///
/// [`pa_volume_snprint_verbose`]: fn.pa_volume_snprint_verbose.html
pub const PA_VOLUME_SNPRINT_VERBOSE_MAX: usize = 35;

#[inline(always)]
pub fn pa_volume_is_valid(v: pa_volume_t) -> bool {
    v <= PA_VOLUME_MAX
}

pub fn pa_clamp_volume(v: pa_volume_t) -> pa_volume_t {
    if v < PA_VOLUME_MUTED {
        return PA_VOLUME_MUTED;
    }
    if v > PA_VOLUME_MAX {
        return PA_VOLUME_MAX;
    }
    v
}

/// Set the volume of the first n channels to `PA_VOLUME_NORM`
#[inline(always)]
pub unsafe fn pa_cvolume_reset(a: *mut pa_cvolume, n: u32) -> *mut pa_cvolume {
    pa_cvolume_set(a, n, PA_VOLUME_NORM)
}

/// Set the volume of the first n channels to `PA_VOLUME_MUTED`
#[inline(always)]
pub unsafe fn pa_cvolume_mute(a: *mut pa_cvolume, n: u32) -> *mut pa_cvolume {
    pa_cvolume_set(a, n, PA_VOLUME_MUTED)
}

extern "C" {
    pub fn pa_cvolume_equal(a: *const pa_cvolume, b: *const pa_cvolume) -> i32;
    pub fn pa_cvolume_init(a: *mut pa_cvolume) -> *mut pa_cvolume;
    pub fn pa_cvolume_set(a: *mut pa_cvolume, channels: u32, v: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_snprint(s: *mut c_char, l: usize, c: *const pa_cvolume) -> *mut c_char;
    pub fn pa_sw_cvolume_snprint_dB(s: *mut c_char, l: usize, c: *const pa_cvolume) -> *mut c_char;
    pub fn pa_cvolume_snprint_verbose(s: *mut c_char, l: usize, c: *const pa_cvolume, map: *const ::channelmap::pa_channel_map, print_dB: i32) -> *mut c_char;

    pub fn pa_volume_snprint(s: *mut c_char, l: usize, v: pa_volume_t) -> *mut c_char;
    pub fn pa_sw_volume_snprint_dB(s: *mut c_char, l: usize, v: pa_volume_t) -> *mut c_char;
    pub fn pa_volume_snprint_verbose(s: *mut c_char, l: usize, v: pa_volume_t, print_dB: i32) -> *mut c_char;

    pub fn pa_cvolume_avg(a: *const pa_cvolume) -> pa_volume_t;
    pub fn pa_cvolume_avg_mask(a: *const pa_cvolume, cm: *const ::channelmap::pa_channel_map, mask: ::channelmap::pa_channel_position_mask_t) -> pa_volume_t;
    pub fn pa_cvolume_max(a: *const pa_cvolume) -> pa_volume_t;
    pub fn pa_cvolume_max_mask(a: *const pa_cvolume, cm: *const ::channelmap::pa_channel_map, mask: ::channelmap::pa_channel_position_mask_t) -> pa_volume_t;
    pub fn pa_cvolume_min(a: *const pa_cvolume) -> pa_volume_t;
    pub fn pa_cvolume_min_mask(a: *const pa_cvolume, cm: *const ::channelmap::pa_channel_map, mask: ::channelmap::pa_channel_position_mask_t) -> pa_volume_t;
    pub fn pa_cvolume_valid(v: *const pa_cvolume) -> i32;
    pub fn pa_cvolume_channels_equal_to(a: *const pa_cvolume, v: pa_volume_t) -> i32;

    pub fn pa_sw_volume_multiply(a: pa_volume_t, b: pa_volume_t) -> pa_volume_t;
    pub fn pa_sw_cvolume_multiply(dest: *mut pa_cvolume, a: *const pa_cvolume, b: *const pa_cvolume) -> *mut pa_cvolume;
    pub fn pa_sw_cvolume_multiply_scalar(dest: *mut pa_cvolume, a: *const pa_cvolume, b: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_sw_volume_divide(a: pa_volume_t, b: pa_volume_t) -> pa_volume_t;
    pub fn pa_sw_cvolume_divide(dest: *mut pa_cvolume, a: *const pa_cvolume, b: *const pa_cvolume) -> *mut pa_cvolume;
    pub fn pa_sw_cvolume_divide_scalar(dest: *mut pa_cvolume, a: *const pa_cvolume, b: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_sw_volume_from_dB(f: f64) -> pa_volume_t;
    pub fn pa_sw_volume_to_dB(v: pa_volume_t) -> f64;
    pub fn pa_sw_volume_from_linear(v: f64) -> pa_volume_t;
    pub fn pa_sw_volume_to_linear(v: pa_volume_t) -> f64;

    pub fn pa_cvolume_remap(v: *mut pa_cvolume, from: *const ::channelmap::pa_channel_map, to: *const ::channelmap::pa_channel_map) -> *mut pa_cvolume;
    pub fn pa_cvolume_compatible(v: *const pa_cvolume, ss: *const ::sample::pa_sample_spec) -> i32;
    pub fn pa_cvolume_compatible_with_channel_map(v: *const pa_cvolume, cm: *const ::channelmap::pa_channel_map) -> i32;
    pub fn pa_cvolume_get_balance(v: *const pa_cvolume, map: *const ::channelmap::pa_channel_map) -> f32;
    pub fn pa_cvolume_set_balance(v: *mut pa_cvolume, map: *const ::channelmap::pa_channel_map, new_balance: f32) -> *mut pa_cvolume;
    pub fn pa_cvolume_get_fade(v: *const pa_cvolume, map: *const ::channelmap::pa_channel_map) -> f32;
    pub fn pa_cvolume_set_fade(v: *mut pa_cvolume, map: *const ::channelmap::pa_channel_map, new_fade: f32) -> *mut pa_cvolume;
    pub fn pa_cvolume_get_lfe_balance(v: *const pa_cvolume, map: *const ::channelmap::pa_channel_map) -> f32;
    pub fn pa_cvolume_set_lfe_balance(v: *mut pa_cvolume, map: *const ::channelmap::pa_channel_map, new_balance: f32) -> *mut pa_cvolume;
    pub fn pa_cvolume_scale(v: *mut pa_cvolume, max: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_scale_mask(v: *mut pa_cvolume, max: pa_volume_t, cm: *const ::channelmap::pa_channel_map, mask: ::channelmap::pa_channel_position_mask_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_set_position(cv: *mut pa_cvolume, map: *const ::channelmap::pa_channel_map, t: ::channelmap::pa_channel_position_t, v: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_get_position(cv: *const pa_cvolume, map: *const ::channelmap::pa_channel_map, t: ::channelmap::pa_channel_position_t) -> pa_volume_t;
    pub fn pa_cvolume_merge(dest: *mut pa_cvolume, a: *const pa_cvolume, b: *const pa_cvolume) -> *mut pa_cvolume;
    pub fn pa_cvolume_inc_clamp(v: *mut pa_cvolume, inc: pa_volume_t, limit: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_inc(v: *mut pa_cvolume, inc: pa_volume_t) -> *mut pa_cvolume;
    pub fn pa_cvolume_dec(v: *mut pa_cvolume, dec: pa_volume_t) -> *mut pa_cvolume;
}
