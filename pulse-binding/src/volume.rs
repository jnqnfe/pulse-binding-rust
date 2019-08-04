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

//! Constants and routines for volume handling.
//!
//! # Overview
//!
//! Sinks, sources, sink inputs, source outputs and samples can all have their own volumes. To deal
//! with these, The PulseAudio library contains a number of functions that ease handling.
//!
//! The basic volume type in PulseAudio is the [`Volume`] type. Most of the time, applications will
//! use the aggregated [`ChannelVolumes`] structure that can store the volume of all channels at
//! once.
//!
//! Volumes commonly span between muted (0%), and normal (100%). It is possible to set volumes to
//! higher than 100%, but clipping might occur.
//!
//! There is no single well-defined meaning attached to the 100% volume for a sink input. In fact,
//! it depends on the server configuration. With flat volumes enabled, it means the maximum volume
//! that the sound hardware is capable of, which is usually so high that you absolutely must not set
//! sink input volume to 100% unless the the user explicitly requests that (note that usually you
//! shouldn’t set the volume anyway if the user doesn’t explicitly request it, instead, let
//! PulseAudio decide the volume for the sink input). With flat volumes disabled the sink input
//! volume is relative to the sink volume, so 100% sink input volume means that the sink input is
//! played at the current sink volume level. In this case 100% is often a good default volume for a
//! sink input, although you still should let PulseAudio decide the default volume. It is possible
//! to figure out whether flat volume mode is in effect for a given sink by calling
//! [`::context::introspect::Introspector::get_sink_info_by_name`].
//!
//! # Calculations
//!
//! The [`Volume`]s in PulseAudio are cubic in nature and applications should not perform
//! calculations with them directly. Instead, they should be converted to and from either dB or a
//! linear scale.
//!
//! The [`VolumeDB`] type represents decibel (dB) converted values, and [`VolumeLinear`], linear.
//! The `From` trait has been implemented for your convenience, allowing such conversions.
//!
//! For simple multiplication, [`Volume::multiply`] and [`ChannelVolumes::sw_multiply`] can be used.
//!
//! It’s often unknown what scale hardware volumes relate to. Don’t use the above functions on sink
//! and source volumes, unless the sink or source in question has the
//! [`::def::sink_flags::DECIBEL_VOLUME`] or [`::def::source_flags::DECIBEL_VOLUME`] flag set. The
//! conversion functions are rarely needed anyway, most of the time it’s sufficient to treat all
//! volumes as opaque with a range from [`VOLUME_MUTED`] \(0%) to [`VOLUME_NORM`] \(100%).
//!
//! [`Volume`]: struct.Volume.html
//! [`VolumeDB`]: struct.VolumeDB.html
//! [`VolumeLinear`]: struct.VolumeLinear.html
//! [`ChannelVolumes`]: struct.ChannelVolumes.html
//! [`::context::introspect::Introspector::get_sink_info_by_name`]:
//! ../context/introspect/struct.Introspector.html#method.get_sink_info_by_name
//! [`Volume::multiply`]: struct.Volume.html#method.multiply
//! [`ChannelVolumes::sw_multiply`]: struct.ChannelVolumes.html#method.sw_multiply
//! [`VOLUME_MUTED`]: constant.VOLUME_MUTED.html
//! [`VOLUME_NORM`]: constant.VOLUME_NORM.html
//! [`::def::sink_flags::DECIBEL_VOLUME`]: ../def/sink_flags/constant.DECIBEL_VOLUME.html
//! [`::def::source_flags::DECIBEL_VOLUME`]: ../def/source_flags/constant.DECIBEL_VOLUME.html

use std;
use capi;
use std::ffi::CStr;
use std::ptr::null;

pub const VOLUME_NORM: Volume = Volume(capi::PA_VOLUME_NORM);
pub const VOLUME_MUTED: Volume = Volume(capi::PA_VOLUME_MUTED);
pub const VOLUME_MAX: Volume = Volume(capi::PA_VOLUME_MAX);
pub const VOLUME_INVALID: Volume = Volume(capi::PA_VOLUME_INVALID);

/// Minus Infinity.
///
/// This floor value is used / can be used, when using converting between integer software volume
/// and decibel (dB, floating point) software volume.
pub const DECIBEL_MINUS_INFINITY: VolumeDB = VolumeDB(capi::PA_DECIBEL_MININFTY);

/// Software volume expressed as an integer.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Volume(pub capi::pa_volume_t);

impl Default for Volume {
    fn default() -> Self { VOLUME_NORM }
}

/// Software volume expressed in decibels (dBs).
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct VolumeDB(pub f64);

impl Default for VolumeDB {
    fn default() -> Self { VolumeDB(0.0) }
}

/// Software volume expressed as linear factor.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct VolumeLinear(pub f64);

impl Default for VolumeLinear {
    fn default() -> Self { VolumeLinear(0.0) }
}

/// A structure encapsulating a per-channel volume
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ChannelVolumes {
    /* NOTE: This struct must be directly usable by the C API, thus same attributes/layout/etc */
    /// Number of channels.
    pub channels: u8,
    /// Per-channel volume.
    pub values: [Volume; ::sample::CHANNELS_MAX],
}

/// Test size is equal to `sys` equivalent (duplicated here for different documentation)
#[test]
fn set_compare_capi(){
    assert_eq!(std::mem::size_of::<ChannelVolumes>(), std::mem::size_of::<capi::pa_cvolume>());
    assert_eq!(std::mem::align_of::<ChannelVolumes>(), std::mem::align_of::<capi::pa_cvolume>());
}

impl AsRef<capi::pa_cvolume> for ChannelVolumes {
    #[inline]
    fn as_ref(&self) -> &capi::pa_cvolume {
        unsafe { &*(self as *const Self as *const capi::pa_cvolume) }
    }
}
impl AsMut<capi::pa_cvolume> for ChannelVolumes {
    #[inline]
    fn as_mut(&mut self) -> &mut capi::pa_cvolume {
        unsafe { &mut *(self as *mut Self as *mut capi::pa_cvolume) }
    }
}

impl From<capi::pa_cvolume> for ChannelVolumes {
    #[inline]
    fn from(cv: capi::pa_cvolume) -> Self {
        unsafe { std::mem::transmute(cv) }
    }
}

impl PartialEq for ChannelVolumes {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self.channels == other.channels {
            true => self.values[..self.channels as usize] == other.values[..other.channels as usize],
            false => false,
        }
    }
}

/// Converts a decibel value to a volume (amplitude, not power).
///
/// This is only valid for software volumes!
impl From<VolumeDB> for Volume {
    #[inline]
    fn from(v: VolumeDB) -> Self {
        Volume(unsafe { capi::pa_sw_volume_from_dB(v.0) })
    }
}
/// Converts a volume to a decibel value (amplitude, not power).
///
/// This is only valid for software volumes!
impl From<Volume> for VolumeDB {
    #[inline]
    fn from(v: Volume) -> Self {
        VolumeDB(unsafe { capi::pa_sw_volume_to_dB(v.0) })
    }
}

/// Converts a linear factor to a volume.
///
/// `0.0` and less is muted while `1.0` is [`VOLUME_NORM`](constant.VOLUME_NORM.html).
/// This is only valid for software volumes!
impl From<VolumeLinear> for Volume {
    #[inline]
    fn from(v: VolumeLinear) -> Self {
        Volume(unsafe { capi::pa_sw_volume_from_linear(v.0) })
    }
}
/// Converts a volume to a linear factor.
///
/// This is only valid for software volumes!
impl From<Volume> for VolumeLinear {
    #[inline]
    fn from(v: Volume) -> Self {
        VolumeLinear(unsafe { capi::pa_sw_volume_to_linear(v.0) })
    }
}

/// Converts a linear factor to a decibel value (amplitude, not power).
///
/// `0.0` and less is muted while `1.0` is [`VOLUME_NORM`](constant.VOLUME_NORM.html).
///
/// This is only valid for software volumes!
impl From<VolumeLinear> for VolumeDB {
    #[inline]
    fn from(v: VolumeLinear) -> Self {
        VolumeDB::from(Volume::from(v))
    }
}
/// Converts a decibel value (amplitude, not power) to a linear factor.
///
/// This is only valid for software volumes!
impl From<VolumeDB> for VolumeLinear {
    #[inline]
    fn from(v: VolumeDB) -> Self {
        VolumeLinear::from(Volume::from(v))
    }
}

impl VolumeLinear {
    #[inline]
    pub fn is_muted(&self) -> bool {
        self.0 <= 0.0
    }

    #[inline]
    pub fn is_normal(&self) -> bool {
        self.0 == 1.0
    }
}

impl Volume {
    #[inline]
    pub fn is_muted(&self) -> bool {
        *self == VOLUME_MUTED
    }

    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == VOLUME_NORM
    }

    #[inline]
    pub fn is_max(&self) -> bool {
        *self == VOLUME_MAX
    }

    /// Get the recommended maximum volume to show in user facing UIs.
    ///
    /// Note: UIs should deal gracefully with volumes greater than this value and not cause feedback
    /// loops etc. - i.e. if the volume is more than this, the UI should not limit it and push the
    /// limited value back to the server.
    #[inline]
    pub fn ui_max() -> Self {
        Volume(capi::pa_volume_ui_max())
    }

    /// Checks if volume is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        capi::pa_volume_is_valid(self.0)
    }

    /// Clamps volume to the permitted range.
    #[inline]
    pub fn clamp(&mut self) {
        self.0 = capi::pa_clamp_volume(self.0)
    }

    /// Multiplies two software volumes, returning the result.
    ///
    /// This uses [`VOLUME_NORM`](constant.VOLUME_NORM.html) as neutral element of multiplication.
    ///
    /// This is only valid for software volumes!
    #[inline]
    pub fn multiply(a: Self, b: Self) -> Self {
        Volume(unsafe { capi::pa_sw_volume_multiply(a.0, b.0) })
    }

    /// Divides two software volumes, returning the result.
    ///
    /// This uses [`VOLUME_NORM`](constant.VOLUME_NORM.html) as neutral element of division. If a
    /// division by zero is tried the result will be `0`.
    ///
    /// This is only valid for software volumes!
    #[inline]
    pub fn divide(a: Self, b: Self) -> Self {
        Volume(unsafe { capi::pa_sw_volume_divide(a.0, b.0) })
    }

    /// Pretty prints a volume.
    pub fn print(&self) -> String {
        const PRINT_MAX: usize = capi::PA_VOLUME_SNPRINT_MAX;
        let mut tmp = Vec::with_capacity(PRINT_MAX);
        unsafe {
            capi::pa_volume_snprint(tmp.as_mut_ptr(), PRINT_MAX, self.0);
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Pretty prints a volume but showing dB values.
    pub fn print_db(&self) -> String {
        const PRINT_DB_MAX: usize = capi::PA_SW_VOLUME_SNPRINT_DB_MAX;
        let mut tmp = Vec::with_capacity(PRINT_DB_MAX);
        unsafe {
            capi::pa_sw_volume_snprint_dB(tmp.as_mut_ptr(), PRINT_DB_MAX, self.0);
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Pretty prints a volume in a verbose way.
    ///
    /// The volume is printed in several formats: the raw volume value, percentage, and if
    /// `print_db` is true, also the dB value.
    pub fn print_verbose(&self, print_db: bool) -> String {
        const PRINT_VERBOSE_MAX: usize = capi::PA_VOLUME_SNPRINT_VERBOSE_MAX;
        let mut tmp = Vec::with_capacity(PRINT_VERBOSE_MAX);
        unsafe {
            capi::pa_volume_snprint_verbose(tmp.as_mut_ptr(), PRINT_VERBOSE_MAX, self.0,
                print_db as i32);
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }
}

impl std::fmt::Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.print())
    }
}

impl ChannelVolumes {
    /// Initializes the specified volume and return a pointer to it.
    ///
    /// The sample spec will have a defined state but [`is_valid`](#method.is_valid) will fail for
    /// it.
    #[inline]
    pub fn init(&mut self) -> &Self {
        unsafe { capi::pa_cvolume_init(self.as_mut()) };
        self
    }

    /// Sets the volume of the specified number of channels to the supplied volume.
    #[inline]
    pub fn set(&mut self, channels: u32, v: Volume) -> &Self {
        unsafe { capi::pa_cvolume_set(self.as_mut(), channels, v.0) };
        self
    }

    /// Sets the volume of the first n channels to [`VOLUME_NORM`](constant.VOLUME_NORM.html).
    #[inline]
    pub fn reset(&mut self, channels: u32) -> &Self {
        self.set(channels, VOLUME_NORM)
    }

    /// Sets the volume of the first n channels to [`VOLUME_MUTED`](constant.VOLUME_MUTED.html).
    #[inline]
    pub fn mute(&mut self, channels: u32) -> &Self {
        self.set(channels, VOLUME_MUTED)
    }

    /// Checks if self is equal to `to`.
    ///
    /// This checks that the number of channels in self equals the number in `to` and that the
    /// channels volumes in self equal those in `to`.
    #[inline]
    pub fn equal_to(&self, to: &Self) -> bool {
        unsafe { capi::pa_cvolume_equal(self.as_ref(), to.as_ref()) != 0 }
    }

    /// Checks if all channels are muted.
    #[inline]
    pub fn is_muted(&self) -> bool {
        self.channels_equal_to(VOLUME_MUTED)
    }

    /// Checks if all channels are at normal volume level.
    #[inline]
    pub fn is_norm(&self) -> bool {
        self.channels_equal_to(VOLUME_NORM)
    }

    /// Gets the average volume of all channels.
    #[inline]
    pub fn avg(&self) -> Volume {
        Volume(unsafe { capi::pa_cvolume_avg(self.as_ref()) })
    }

    /// Returns the average volume of all channels that are included in the specified channel map
    /// with the specified channel position mask.
    ///
    /// If no channel is selected the returned value will be
    /// [`VOLUME_MUTED`](constant.VOLUME_MUTED.html). If `mask` is `None`, has the same effect as
    /// passing [`::channelmap::POSITION_MASK_ALL`](../channelmap/constant.POSITION_MASK_ALL.html).
    #[inline]
    pub fn avg_mask(&self, cm: &::channelmap::Map, mask: Option<::channelmap::PositionMask>)
        -> Volume
    {
        let mask_actual = mask.unwrap_or(::channelmap::POSITION_MASK_ALL);
        Volume(unsafe { capi::pa_cvolume_avg_mask(self.as_ref(), cm.as_ref(), mask_actual) })
    }

    /// Gets the maximum volume of all channels.
    #[inline]
    pub fn max(&self) -> Volume {
        Volume(unsafe { capi::pa_cvolume_max(self.as_ref()) })
    }

    /// Gets the maximum volume of all channels that are included in the specified channel map
    /// with the specified channel position mask.
    ///
    /// If no channel is selected the returned value will be
    /// [`VOLUME_MUTED`](constant.VOLUME_MUTED.html). If `mask` is `None`, has the same effect as
    /// passing [`::channelmap::POSITION_MASK_ALL`](../channelmap/constant.POSITION_MASK_ALL.html).
    #[inline]
    pub fn max_mask(&self, cm: &::channelmap::Map, mask: Option<::channelmap::PositionMask>)
        -> Volume
    {
        let mask_actual = mask.unwrap_or(::channelmap::POSITION_MASK_ALL);
        Volume(unsafe { capi::pa_cvolume_max_mask(self.as_ref(), cm.as_ref(), mask_actual) })
    }

    /// Gets the minimum volume of all channels.
    #[inline]
    pub fn min(&self) -> Volume {
        Volume(unsafe { capi::pa_cvolume_min(self.as_ref()) })
    }

    /// Gets the minimum volume of all channels that are included in the specified channel map
    /// with the specified channel position mask.
    ///
    /// If no channel is selected the returned value will be
    /// [`VOLUME_MUTED`](constant.VOLUME_MUTED.html). If `mask` is `None`, has the same effect as
    /// passing [`::channelmap::POSITION_MASK_ALL`](../channelmap/constant.POSITION_MASK_ALL.html).
    #[inline]
    pub fn min_mask(&self, cm: &::channelmap::Map, mask: Option<::channelmap::PositionMask>)
        -> Volume
    {
        let mask_actual = mask.unwrap_or(::channelmap::POSITION_MASK_ALL);
        Volume(unsafe { capi::pa_cvolume_min_mask(self.as_ref(), cm.as_ref(), mask_actual) })
    }

    /// Checks if the `ChannelVolumes` structure is valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { capi::pa_cvolume_valid(self.as_ref()) != 0 }
    }

    /// Checks if the volume of all channels are equal to the specified value.
    #[inline]
    pub fn channels_equal_to(&self, v: Volume) -> bool {
        unsafe { capi::pa_cvolume_channels_equal_to(self.as_ref(), v.0) != 0 }
    }

    /// Multiplies two per-channel volumes.
    ///
    /// If `with` is `None`, multiplies with itself. This is only valid for software volumes!
    /// Returns pointer to self.
    #[inline]
    pub fn sw_multiply(&mut self, with: Option<&Self>) -> &mut Self {
        unsafe { capi::pa_sw_cvolume_multiply(self.as_mut(), self.as_mut(),
            with.unwrap_or(self).as_ref()) };
        self
    }

    /// Multiplies a per-channel volume with a scalar volume.
    ///
    /// This is only valid for software volumes! Returns pointer to self.
    #[inline]
    pub fn sw_multiply_scalar(&mut self, with: Volume) -> &mut Self {
        unsafe { capi::pa_sw_cvolume_multiply_scalar(self.as_mut(), self.as_ref(), with.0) };
        self
    }

    /// Divides two per-channel volumes.
    ///
    /// If `with` is `None`, divides with itself. This is only valid for software volumes! Returns
    /// pointer to self.
    #[inline]
    pub fn sw_divide(&mut self, with: Option<&Self>) -> &mut Self {
        unsafe { capi::pa_sw_cvolume_divide(self.as_mut(), self.as_mut(),
            with.unwrap_or(self).as_ref()) };
        self
    }

    /// Divides a per-channel volume by a scalar volume.
    ///
    /// This is only valid for software volumes! Returns pointer to self.
    #[inline]
    pub fn sw_divide_scalar(&mut self, with: Volume) -> &mut Self {
        unsafe { capi::pa_sw_cvolume_divide_scalar(self.as_mut(), self.as_ref(), with.0) };
        self
    }

    /// Remaps a volume from one channel mapping to a different channel mapping.
    ///
    /// Returns pointer to self.
    #[inline]
    pub fn remap(&mut self, from: &::channelmap::Map, to: &::channelmap::Map) -> &mut Self {
        unsafe { capi::pa_cvolume_remap(self.as_mut(), from.as_ref(), to.as_ref()) };
        self
    }

    /// Checks if the specified volume is compatible with the specified sample spec.
    #[inline]
    pub fn is_compatible_with_ss(&self, ss: &::sample::Spec) -> bool {
        unsafe { capi::pa_cvolume_compatible(self.as_ref(), ss.as_ref()) != 0 }
    }

    /// Checks if the specified volume is compatible with the specified channel map.
    #[inline]
    pub fn is_compatible_with_cm(&self, cm: &::channelmap::Map) -> bool {
        unsafe { capi::pa_cvolume_compatible_with_channel_map(self.as_ref(), cm.as_ref()) != 0 }
    }

    /// Calculates a ‘balance’ value for the specified volume with the specified channel map.
    ///
    /// The return value will range from `-1.0` (left) to `+1.0` (right). If no balance value is
    /// applicable to this channel map the return value will always be `0.0`. See
    /// [`::channelmap::Map::can_balance`].
    ///
    /// [`::channelmap::Map::can_balance`]: ../channelmap/struct.Map.html#method.can_balance
    #[inline]
    pub fn get_balance(&self, map: &::channelmap::Map) -> f32 {
        unsafe { capi::pa_cvolume_get_balance(self.as_ref(), map.as_ref()) }
    }

    /// Adjusts the ‘balance’ value for the specified volume with the specified channel map.
    ///
    /// The balance is a value between `-1.0` and `+1.0`. This operation might not be reversible!
    /// Also, after this call [`get_balance`] is not guaranteed to actually return the requested
    /// balance value (e.g. when the input volume was zero anyway for all channels). If no balance
    /// value is applicable to this channel map the volume will not be modified. See
    /// [`::channelmap::Map::can_balance`].
    ///
    /// Returns pointer to self, or `None` on error.
    ///
    /// [`get_balance`]: #method.get_balance
    /// [`::channelmap::Map::can_balance`]: ../channelmap/struct.Map.html#method.can_balance
    #[inline]
    pub fn set_balance(&mut self, map: &::channelmap::Map, new_balance: f32) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_set_balance(self.as_mut(), map.as_ref(), new_balance) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Calculates a ‘fade’ value (i.e. ‘balance’ between front and rear) for the specified volume
    /// with the specified channel map.
    ///
    /// The return value will range from -1.0f (rear) to +1.0f (left). If no fade value is
    /// applicable to this channel map the return value will always be `0.0`. See
    /// [`::channelmap::Map::can_fade`].
    ///
    /// [`::channelmap::Map::can_fade`]: ../channelmap/struct.Map.html#method.can_fade
    #[inline]
    pub fn get_fade(&self, map: &::channelmap::Map) -> f32 {
        unsafe { capi::pa_cvolume_get_fade(self.as_ref(), map.as_ref()) }
    }

    /// Adjusts the ‘fade’ value (i.e. ‘balance’ between front and rear) for the specified volume
    /// with the specified channel map.
    ///
    /// The balance is a value between `-1.0` and `+1.0`. This operation might not be reversible!
    /// Also, after this call [`get_fade`] is not guaranteed to actually return the requested fade
    /// value (e.g. when the input volume was zero anyway for all channels). If no fade value is
    /// applicable to this channel map the volume will not be modified. See
    /// [`::channelmap::Map::can_fade`].
    ///
    /// Returns pointer to self, or `None` on error.
    ///
    /// [`get_fade`]: #method.get_fade
    /// [`::channelmap::Map::can_fade`]: ../channelmap/struct.Map.html#method.can_fade
    #[inline]
    pub fn set_fade(&mut self, map: &::channelmap::Map, new_fade: f32) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_set_fade(self.as_mut(), map.as_ref(), new_fade) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Calculates a ‘lfe balance’ value for the specified volume with the specified channel map.
    ///
    /// The return value will range from `-1.0` (no lfe) to `+1.0` (only lfe), where `0.0` is
    /// balanced. If no value is applicable to this channel map the return value will always be
    /// `0.0`. See [`::channelmap::Map::can_lfe_balance`].
    ///
    /// [`::channelmap::Map::can_lfe_balance`]:
    /// ../channelmap/struct.Map.html#method.can_lfe_balance
    #[inline]
    pub fn get_lfe_balance(&self, map: &::channelmap::Map) -> f32 {
        unsafe { capi::pa_cvolume_get_lfe_balance(self.as_ref(), map.as_ref()) }
    }

    /// Adjusts the ‘LFE balance’ value for the specified volume with the specified channel map.
    ///
    /// The balance is a value between `-1.0` (no lfe) and `+1.0` (only lfe). This operation might
    /// not be reversible! Also, after this call [`get_lfe_balance`] is not guaranteed to actually
    /// return the requested value (e.g. when the input volume was zero anyway for all channels). If
    /// no lfe balance value is applicable to this channel map the volume will not be modified. See
    /// [`::channelmap::Map::can_lfe_balance`].
    ///
    /// Returns pointer to self, or `None` on error.
    ///
    /// [`get_lfe_balance`]: #method.get_lfe_balance
    /// [`::channelmap::Map::can_lfe_balance`]: ../channelmap/struct.Map.html#method.can_lfe_balance
    #[inline]
    pub fn set_lfe_balance(&mut self, map: &::channelmap::Map, new_balance: f32)
        -> Option<&mut Self>
    {
        let ptr = unsafe { capi::pa_cvolume_set_lfe_balance(self.as_mut(), map.as_ref(),
            new_balance) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Scales so that the maximum volume of all channels equals `max`.
    ///
    /// The proportions between the channel volumes are kept.
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn scale(&mut self, max: Volume) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_scale(self.as_mut(), max.0) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Scales so that the maximum volume of all channels selected via `cm`/`mask` equals `max`.
    ///
    /// This also modifies the volume of those channels that are unmasked. The proportions between
    /// the channel volumes are kept.
    ///
    /// If `mask` is `None`, has the same effect as passing
    /// [`::channelmap::POSITION_MASK_ALL`](../channelmap/constant.POSITION_MASK_ALL.html).
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn scale_mask(&mut self, max: Volume, cm: &mut ::channelmap::Map,
        mask: Option<::channelmap::PositionMask>) -> Option<&mut Self>
    {
        let mask_actual = mask.unwrap_or(::channelmap::POSITION_MASK_ALL);
        let ptr = unsafe { capi::pa_cvolume_scale_mask(self.as_mut(), max.0, cm.as_ref(),
            mask_actual) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Sets the passed volume to all channels at the specified channel position.
    ///
    /// Returns `None` if either invalid data was provided, or if there is no channel at the
    /// position specified. You can check if a channel map includes a specific position by calling
    /// [`::channelmap::Map::has_position`]. On success, returns pointer to self.
    ///
    /// [`::channelmap::Map::has_position`]: ../channelmap/struct.Map.html#method.has_position
    #[inline]
    pub fn set_position(&mut self, map: &::channelmap::Map, t: ::channelmap::Position, v: Volume)
        -> Option<&mut Self>
    {
        // Note: C function returns NULL on invalid data or no channel at position specified (no
        // change needed). We could ignore failure and always return self ptr, but it does not seem
        // ideal to leave callers unaware should they be passing in invalid data.
        let ptr = unsafe { capi::pa_cvolume_set_position(self.as_mut(), map.as_ref(), t.into(),
            v.0) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Gets the maximum volume of all channels at the specified channel position.
    ///
    /// Will return `0` if there is no channel at the position specified. You can check if a channel
    /// map includes a specific position by calling [`::channelmap::Map::has_position`].
    ///
    /// [`::channelmap::Map::has_position`]: ../channelmap/struct.Map.html#method.has_position
    #[inline]
    pub fn get_position(&self, map: &::channelmap::Map, t: ::channelmap::Position) -> Volume {
        Volume(unsafe { capi::pa_cvolume_get_position(self.as_ref(), map.as_ref(), t.into()) })
    }

    /// Merges one set of channel volumes with another.
    ///
    /// The channel count is set to the minimum between that of self and that of `with`. Only this
    /// number of channels are processed. For each channel processed, volume is set to the greatest
    /// of the values from self and from `with`. I.e if one set has three channels and the other has
    /// two, the number of channels will be set to two, and only the first two channels will be
    /// compared, with the greatest values of these two channels being stored. The third channel in
    /// the one set will be ignored.
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn merge(&mut self, with: &Self) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_merge(self.as_mut(), self.as_ref(), with.as_ref()) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Increases the volume passed in by `inc`, but not exceeding `limit`.
    ///
    /// The proportions between the channels are kept.
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn inc_clamp(&mut self, inc: Volume, limit: Volume) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_inc_clamp(self.as_mut(), inc.0, limit.0) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Increases the volume passed in by `inc`.
    ///
    /// The proportions between the channels are kept.
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn increase(&mut self, inc: Volume) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_inc(self.as_mut(), inc.0) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Decreases the volume passed in by `dec`.
    ///
    /// The proportions between the channels are kept.
    ///
    /// Returns pointer to self, or `None` on error.
    #[inline]
    pub fn decrease(&mut self, dec: Volume) -> Option<&mut Self> {
        let ptr = unsafe { capi::pa_cvolume_dec(self.as_mut(), dec.0) };
        if ptr.is_null() {
            return None;
        }
        Some(self)
    }

    /// Pretty prints a volume structure.
    pub fn print(&self) -> String {
        const PRINT_MAX: usize = capi::PA_CVOLUME_SNPRINT_MAX;
        let mut tmp = Vec::with_capacity(PRINT_MAX);
        unsafe {
            capi::pa_cvolume_snprint(tmp.as_mut_ptr(), PRINT_MAX, self.as_ref());
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Pretty prints a volume structure but show dB values.
    pub fn print_db(&self) -> String {
        const PRINT_DB_MAX: usize = capi::PA_SW_CVOLUME_SNPRINT_DB_MAX;
        let mut tmp = Vec::with_capacity(PRINT_DB_MAX);
        unsafe {
            capi::pa_sw_cvolume_snprint_dB(tmp.as_mut_ptr(), PRINT_DB_MAX, self.as_ref());
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Pretty prints a volume structure in a verbose way.
    ///
    /// The volume for each channel is printed in several formats: the raw volume value,
    /// percentage, and if `print_db` is non-zero, also the dB value. If `map` is provided, the
    /// channel names will be printed.
    pub fn print_verbose(&self, map: Option<&::channelmap::Map>, print_db: bool) -> String {
        const PRINT_VERBOSE_MAX: usize = capi::PA_CVOLUME_SNPRINT_VERBOSE_MAX;

        let p_map = map.map_or(null::<capi::pa_channel_map>(), |m| m.as_ref());

        let mut tmp = Vec::with_capacity(PRINT_VERBOSE_MAX);
        unsafe {
            capi::pa_cvolume_snprint_verbose(tmp.as_mut_ptr(), PRINT_VERBOSE_MAX, self.as_ref(),
                p_map, print_db as i32);
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }
}

impl std::fmt::Display for ChannelVolumes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.print())
    }
}
