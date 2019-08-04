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

//! Constants and routines for handing channel mapping.
//!
//! # Overview
//!
//! Channel maps provide a way to associate channels in a stream with a specific speaker position.
//! This relieves applications of having to make sure their channel order is identical to the final
//! output.
//!
//! # Initialisation
//!
//! A channel map consists of an array of [`Position`] values, one for each channel. This array is
//! stored together with a channel count in a [`Map`] structure.
//!
//! Before filling the structure, the application must initialise it using [`Map::init`]. There are
//! also a number of convenience functions for standard channel mappings:
//!
//! * [`Map::init_mono`]: Create a channel map with only mono audio.
//! * [`Map::init_stereo`]: Create a standard stereo mapping.
//! * [`Map::init_auto`]: Create a standard channel map for a specific number of channels.
//! * [`Map::init_extend`]: Similar to [`Map::init_auto`] but synthesize a channel map if no
//!   predefined one is known for the specified number of channels.
//!
//! [`Position`]: enum.Position.html
//! [`Map`]: struct.Map.html
//! [`Map::init`]: struct.Map.html#method.init
//! [`Map::init_mono`]: struct.Map.html#method.init_mono
//! [`Map::init_stereo`]: struct.Map.html#method.init_stereo
//! [`Map::init_auto`]: struct.Map.html#method.init_auto
//! [`Map::init_extend`]: struct.Map.html#method.init_extend

use std;
use capi;
use std::ffi::{CStr, CString};
use std::borrow::Cow;

pub use capi::pa_channel_map_def_t as MapDef;

/// A mask of channel positions.
pub type PositionMask = capi::channelmap::pa_channel_position_mask_t;

/// Position mask covering all positions.
pub const POSITION_MASK_ALL: PositionMask = 0xffffffffffffffffu64;

/// A list of channel labels.
///
/// Note, certain aliases, specifically `Left`, `Right`, `Center` and `Subwoofer`, available in the
/// equivalent C enum are not provided here, since Rust does not allow aliases.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Position {
    /* NOTE: This enum’s variants and variant values **must** remain identical to the `sys` crate
       (C API) equivalent */

    Invalid = -1,
    Mono = 0,

    /// Apple, Dolby call this ‘Left’.
    FrontLeft,
    /// Apple, Dolby call this ‘Right’.
    FrontRight,
    /// Apple, Dolby call this ‘Center’.
    FrontCenter,

    /// Microsoft calls this ‘Back Center’, Apple calls this ‘Center Surround’,
    /// Dolby calls this ‘Surround Rear Center’.
    RearCenter,
    /// Microsoft calls this ‘Back Left’, Apple calls this ‘Left Surround’,
    /// Dolby calls this ‘Surround Rear Left’.
    RearLeft,
    /// Microsoft calls this ‘Back Right’, Apple calls this ‘Right Surround’,
    /// Dolby calls this ‘Surround Rear Right’.
    RearRight,

    /// Aka subwoofer. Microsoft calls this ‘Low Frequency’,
    /// Apple calls this ‘LFEScreen’.
    Lfe,

    /// Apple, Dolby call this ‘Left Center’.
    FrontLeftOfCenter,
    /// Apple, Dolby call this ‘Right Center’.
    FrontRightOfCenter,

    /// Apple calls this ‘Left Surround Direct’,
    /// Dolby calls this ‘Surround Left’.
    SideLeft,
    /// Apple calls this ‘Right Surround Direct’,
    /// Dolby calls this ‘Surround Right’.
    SideRight,

    Aux0,
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    Aux5,
    Aux6,
    Aux7,
    Aux8,
    Aux9,
    Aux10,
    Aux11,
    Aux12,
    Aux13,
    Aux14,
    Aux15,
    Aux16,
    Aux17,
    Aux18,
    Aux19,
    Aux20,
    Aux21,
    Aux22,
    Aux23,
    Aux24,
    Aux25,
    Aux26,
    Aux27,
    Aux28,
    Aux29,
    Aux30,
    Aux31,

    /// Apple calls this ‘Top Center Surround’.
    TopCenter,

    /// Apple calls this ‘Vertical Height Left’.
    TopFrontLeft,
    /// Apple calls this ‘Vertical Height Right’.
    TopFrontRight,
    /// Apple calls this ‘Vertical Height Center’.
    TopFrontCenter,

    /// Microsoft and Apple call this ‘Top Back Left’.
    TopRearLeft,
    /// Microsoft and Apple call this ‘Top Back Right’.
    TopRearRight,
    /// Microsoft and Apple call this ‘Top Back Center’.
    TopRearCenter,
}

impl Default for Position {
    #[inline(always)]
    fn default() -> Self {
        Position::Invalid
    }
}

/// Test size is equal to `sys` equivalent (duplicated here for different documentation)
#[test]
fn pos_compare_capi(){
    assert_eq!(std::mem::size_of::<Position>(), std::mem::size_of::<capi::pa_channel_position_t>());
    assert_eq!(std::mem::align_of::<Position>(), std::mem::align_of::<capi::pa_channel_position_t>());
}

impl From<Position> for capi::pa_channel_position_t {
    #[inline]
    fn from(p: Position) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}
impl From<capi::pa_channel_position_t> for Position {
    #[inline]
    fn from(p: capi::pa_channel_position_t) -> Self {
        unsafe { std::mem::transmute(p) }
    }
}

/// A channel map which can be used to attach labels to specific channels of a stream.
///
/// These values are relevant for conversion and mixing of streams.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Map {
    /* NOTE: This struct must be directly usable by the C API, thus same attributes/layout/etc */
    /// Number of channels mapped.
    pub channels: u8,
    /// Channel labels.
    pub map: [Position; ::sample::CHANNELS_MAX],
}

/// Test size is equal to `sys` equivalent (duplicated here for different documentation)
#[test]
fn map_compare_capi(){
    assert_eq!(std::mem::size_of::<Map>(), std::mem::size_of::<capi::pa_channel_map>());
    assert_eq!(std::mem::align_of::<Map>(), std::mem::align_of::<capi::pa_channel_map>());
}

impl AsRef<capi::pa_channel_map> for Map {
    #[inline]
    fn as_ref(&self) -> &capi::pa_channel_map {
        unsafe { &*(self as *const Self as *const capi::pa_channel_map) }
    }
}
impl AsMut<capi::pa_channel_map> for Map {
    #[inline]
    fn as_mut(&mut self) -> &mut capi::pa_channel_map {
        unsafe { &mut *(self as *mut Self as *mut capi::pa_channel_map) }
    }
}
impl AsRef<Map> for capi::pa_channel_map {
    #[inline]
    fn as_ref(&self) -> &Map {
        unsafe { &*(self as *const Self as *const Map) }
    }
}

impl From<capi::pa_channel_map> for Map {
    #[inline]
    fn from(m: capi::pa_channel_map) -> Self {
        unsafe { std::mem::transmute(m) }
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            channels: 0,
            map: [Position::Invalid; ::sample::CHANNELS_MAX],
        }
    }
}

impl PartialEq for Map {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        match self.channels == other.channels {
            true => self.map[..self.channels as usize] == other.map[..other.channels as usize],
            false => false,
        }
    }
}

impl Position {
    /// Makes a bit mask from a channel position.
    pub fn to_mask(self) -> PositionMask {
        if self == Position::Invalid {
            return 0;
        }
        (1 as PositionMask) << (self as PositionMask)
    }

    /// Return a text label for the specified channel position.
    pub fn to_string(pos: Self) -> Option<Cow<'static, str>> {
        let ptr = unsafe { capi::pa_channel_position_to_string(pos.into()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy() })
    }

    /// Return a human readable text label for the specified channel position.
    pub fn to_pretty_string(pos: Self) -> Option<String> {
        let ptr = unsafe { capi::pa_channel_position_to_pretty_string(pos.into()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }

    /// The inverse of [`to_string`](#method.to_string).
    pub fn from_string(s: &str) -> Self {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_str = CString::new(s.clone()).unwrap();
        unsafe { capi::pa_channel_position_from_string(c_str.as_ptr()).into() }
    }
}

impl Map {
    /// Parse a channel position list or well-known mapping name into a channel map structure.
    ///
    /// This turns the output of [`print`](#method.print) and [`to_name`](#method.to_name) back into
    /// a `Map`.
    pub fn new_from_string(s: &str) -> Result<Self, ()> {
        // Warning: New CStrings will be immediately freed if not bound to a variable, leading to
        // as_ptr() giving dangling pointers!
        let c_str = CString::new(s.clone()).unwrap();
        let mut map: Self = Self::default();
        unsafe {
            if capi::pa_channel_map_parse((&mut map).as_mut(), c_str.as_ptr()).is_null() {
                return Err(());
            }
        }
        Ok(map)
    }

    /// Initialize the specified channel map and return a pointer to it.
    ///
    /// The map will have a defined state but [`is_valid`](#method.is_valid) will fail for it.
    #[inline]
    pub fn init(&mut self) -> &mut Self {
        unsafe { capi::pa_channel_map_init(self.as_mut()) };
        self
    }

    /// Initialize the specified channel map for monaural audio and return a pointer to it.
    #[inline]
    pub fn init_mono(&mut self) -> &mut Self {
        unsafe { capi::pa_channel_map_init_mono(self.as_mut()) };
        self
    }

    /// Initialize the specified channel map for stereophonic audio and return a pointer to it.
    #[inline]
    pub fn init_stereo(&mut self) -> &mut Self {
        unsafe { capi::pa_channel_map_init_stereo(self.as_mut()) };
        self
    }

    /// Initialize the specified channel map for the specified number of channels using default
    /// labels and return a pointer to it.
    ///
    /// This call will fail (return `None`) if there is no default channel map known for this
    /// specific number of channels and mapping.
    pub fn init_auto(&mut self, channels: u32, def: MapDef) -> Option<&mut Self> {
        debug_assert!(channels as usize <= ::sample::CHANNELS_MAX);
        unsafe {
            if capi::pa_channel_map_init_auto(self.as_mut(), channels, def).is_null() {
                return None;
            }
        }
        Some(self)
    }

    /// Similar to [`init_auto`](#method.init_auto) but instead of failing if no default mapping is
    /// known with the specified parameters it will synthesize a mapping based on a known mapping
    /// with fewer channels and fill up the rest with AUX0...AUX31 channels.
    pub fn init_extend(&mut self, channels: u32, def: MapDef) -> &mut Self {
        debug_assert!(channels as usize <= ::sample::CHANNELS_MAX);
        unsafe { capi::pa_channel_map_init_extend(self.as_mut(), channels, def) };
        self
    }

    /// Make a human readable string from the map.
    pub fn print(&self) -> String {
        const PRINT_MAX: usize = capi::PA_CHANNEL_MAP_SNPRINT_MAX;
        let mut tmp = Vec::with_capacity(PRINT_MAX);
        unsafe {
            capi::pa_channel_map_snprint(tmp.as_mut_ptr(), PRINT_MAX, self.as_ref());
            CStr::from_ptr(tmp.as_mut_ptr()).to_string_lossy().into_owned()
        }
    }

    /// Compare whether or not two maps are equal.
    #[inline]
    pub fn is_equal_to(&self, to: &Self) -> bool {
        unsafe { capi::pa_channel_map_equal(self.as_ref(), to.as_ref()) == 1 }
    }

    /// Check whether or not the map is considered valid.
    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { capi::pa_channel_map_valid(self.as_ref()) != 0 }
    }

    /// Checks whether or not the specified map is compatible with the specified sample spec.
    #[inline]
    pub fn is_compatible_with_sample_spec(&self, ss: &::sample::Spec) -> bool {
        unsafe { capi::pa_channel_map_compatible(self.as_ref(), ss.as_ref()) != 0 }
    }

    /// Checks whether every channel defined in `of` is also defined in self.
    #[inline]
    pub fn is_superset_of(&self, of: &Self) -> bool {
        unsafe { capi::pa_channel_map_superset(self.as_ref(), of.as_ref()) != 0 }
    }

    /// Checks whether or not it makes sense to apply a volume “balance” with this mapping, i.e. if
    /// there are left/right channels available.
    #[inline]
    pub fn can_balance(&self) -> bool {
        unsafe { capi::pa_channel_map_can_balance(self.as_ref()) != 0 }
    }

    /// Checks whether or not it makes sense to apply a volume “fade” (i.e. “balance” between front
    /// and rear) with this mapping, i.e. if there are front/rear channels available.
    #[inline]
    pub fn can_fade(&self) -> bool {
        unsafe { capi::pa_channel_map_can_fade(self.as_ref()) != 0 }
    }

    /// Checks whether or not it makes sense to apply a volume “LFE balance” (i.e. “balance” between
    /// LFE and non-LFE channels) with this mapping, i.e. if there are LFE and non-LFE channels
    /// available.
    #[inline]
    pub fn can_lfe_balance(&self) -> bool {
        unsafe { capi::pa_channel_map_can_lfe_balance(self.as_ref()) != 0 }
    }

    /// Tries to find a well-known channel mapping name for this channel mapping, i.e. “stereo”,
    /// “surround-71” and so on. This name can be parsed with
    /// [`new_from_string`](#method.new_from_string).
    pub fn to_name(&self) -> Option<Cow<'static, str>> {
        let ptr = unsafe { capi::pa_channel_map_to_name(self.as_ref()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy() })
    }

    /// Similar to [`to_name`](#method.to_name), but returning prettier, human readable text labels,
    /// i.e. “Stereo”, “Surround 7.1” and so on.
    pub fn to_pretty_name(&self) -> Option<String> {
        let ptr = unsafe { capi::pa_channel_map_to_pretty_name(self.as_ref()) };
        if ptr.is_null() {
            return None;
        }
        Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
    }

    /// Checks whether or not the specified channel position is available at least once in the map.
    #[inline]
    pub fn has_position(&self, p: Position) -> bool {
        unsafe { capi::pa_channel_map_has_position(self.as_ref(), p.into()) != 0 }
    }

    /// Generates a bit mask from a map.
    #[inline]
    pub fn get_mask(&self) -> PositionMask {
        unsafe { capi::pa_channel_map_mask(self.as_ref()) }
    }
}
