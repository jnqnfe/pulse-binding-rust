//! Global definitions

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
use std::os::raw::c_void;
use timeval::Timeval;

pub use capi::PA_INVALID_INDEX as INVALID_INDEX;
pub use capi::pa_device_type_t as Device;
pub use capi::pa_port_available_t as PortAvailable;

pub type FreeCb = extern "C" fn(p: *mut c_void);

/// Playback and record buffer metrics
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct BufferAttr {
    /// Maximum length of the buffer in bytes.
    ///
    /// Setting this to `std::u32::MAX` will initialize this to the maximum value supported by the
    /// server, which is recommended. In strict low-latency playback scenarios you might want to set
    /// this to a lower value, likely together with the [`stream::flags::ADJUST_LATENCY`] flag. If
    /// you do so, you ensure that the latency doesn't grow beyond what is acceptable for the use
    /// case, at the cost of getting more underruns if the latency is lower than what the server can
    /// reliably handle.
    ///
    /// [`stream::flags::ADJUST_LATENCY`]: ../stream/flags/constant.ADJUST_LATENCY.html
    pub maxlength: u32,

    /// Target length of the buffer (playback only). The server tries to assure that at least
    /// `tlength` bytes are always available in the per-stream server-side playback buffer. The
    /// server will only send requests for more data as long as the buffer has less than this number
    /// of bytes of data.
    ///
    /// It is recommended to set this to `std::u32::MAX`, which will initialize this to a value that
    /// is deemed sensible by the server. However, this value will default to something like 2s; for
    /// applications that have specific latency requirements this value should be set to the maximum
    /// latency that the application can deal with.
    ///
    /// When [`stream::flags::ADJUST_LATENCY`] is not set this value will influence only the
    /// per-stream playback buffer size. When [`stream::flags::ADJUST_LATENCY`] is set, the overall
    /// latency of the sink plus the playback buffer size is configured to this value. Set
    /// [`stream::flags::ADJUST_LATENCY`] if you are interested in adjusting the overall latency.
    /// Don't set it if you are interested in configuring the server-side per-stream playback buffer
    /// size.
    ///
    /// [`stream::flags::ADJUST_LATENCY`]: ../stream/flags/constant.ADJUST_LATENCY.html
    pub tlength: u32,

    /// Pre-buffering (playback only). The server does not start with playback before at least
    /// `prebuf` bytes are available in the buffer. It is recommended to set this to
    /// `std::u32::MAX`, which will initialize this to the same value as `tlength`, whatever that
    /// may be.
    ///
    /// Initialize to `0` to enable manual start/stop control of the stream. This means that
    /// playback will not stop on underrun and playback will not start automatically, instead
    /// [`stream::Stream::cork`] needs to be called explicitly. If you set this value to `0` you
    /// should also set [`stream::flags::START_CORKED`]. Should underrun occur, the read index of
    /// the output buffer overtakes the write index, and hence the fill level of the buffer is
    /// negative.
    ///
    /// Start of playback can be forced using [`stream::Stream::trigger`] even though the prebuffer
    /// size hasn't been reached. If a buffer underrun occurs, this prebuffering will be again
    /// enabled.
    ///
    /// [`stream::Stream::cork`]: ../stream/struct.Stream.html#method.cork
    /// [`stream::Stream::trigger`]: ../stream/struct.Stream.html#method.trigger
    /// [`stream::flags::START_CORKED`]: ../stream/flags/constant.START_CORKED.html
    pub prebuf: u32,

    /// Minimum request (playback only). The server does not request less than `minreq` bytes from
    /// the client, instead it waits until the buffer is free enough to request more bytes at once.
    ///
    /// It is recommended to set this to `std::u32::MAX`, which will initialize this to a value that
    /// is deemed sensible by the server. This should be set to a value that gives PulseAudio enough
    /// time to move the data from the per-stream playback buffer into the hardware playback buffer.
    pub minreq: u32,

    /// Fragment size (recording only). The server sends data in blocks of `fragsize` bytes size.
    ///
    /// Large values diminish interactivity with other operations on the connection context but
    /// decrease control overhead. It is recommended to set this to `std::u32::MAX`, which will
    /// initialize this to a value that is deemed sensible by the server. However, this value will
    /// default to something like 2s; For applications that have specific latency requirements this
    /// value should be set to the maximum latency that the application can deal with.
    ///
    /// If [`stream::flags::ADJUST_LATENCY`] is set the overall source latency will be adjusted
    /// according to this value. If it is not set the source latency is left unmodified.
    ///
    /// [`stream::flags::ADJUST_LATENCY`]: ../stream/flags/constant.ADJUST_LATENCY.html
    pub fragsize: u32,
}

/// A structure for all kinds of timing information of a stream.
///
/// See [`stream::Stream::update_timing_info`] and [`stream::Stream::get_timing_info`].
///
/// The total output latency a sample that is written with [`stream::Stream::write`] takes to be
/// played may be estimated by:
///
/// ``
/// sink_usec + buffer_usec + transport_usec
/// ``
///
/// (Where `buffer_usec` is defined as the result of passing ``write_index - read_index`` to
/// [`sample::Spec::bytes_to_usec`]). The output buffer which `buffer_usec` relates to may be
/// manipulated freely (with [`stream::Stream::write`]'s `seek` argument, [`stream::Stream::flush`]
/// and friends), the buffers `sink_usec` and `source_usec` relate to are first-in first-out (FIFO)
/// buffers which cannot be flushed or manipulated in any way. The total input latency a sample that
/// is recorded takes to be delivered to the application is:
///
/// ``
/// source_usec + buffer_usec + transport_usec - sink_usec
/// ``
///
/// (Take care of sign issues!). When connected to a monitor source `sink_usec` contains the latency
/// of the owning sink. The two latency estimations described here are implemented in
/// [`stream::Stream::get_latency`].
///
/// All time values are in the sound card clock domain, unless noted otherwise. The sound card clock
/// usually runs at a slightly different rate than the system clock.
///
/// Please note that this structure can be extended as part of evolutionary API updates at any time
/// in any new release.
///
/// [`sample::Spec::bytes_to_usec`]: ../sample/struct.Spec.html#method.bytes_to_usec
/// [`stream::Stream::update_timing_info`]: ../stream/struct.Stream.html#method.update_timing_info
/// [`stream::Stream::get_timing_info`]: ../stream/struct.Stream.html#method.get_timing_info
/// [`stream::Stream::write`]: ../stream/struct.Stream.html#method.write
/// [`stream::Stream::flush`]: ../stream/struct.Stream.html#method.flush
/// [`stream::Stream::get_latency`]: ../stream/struct.Stream.html#method.get_latency
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimingInfo {
    /// The system clock time when this timing info structure was current.
    pub timestamp: Timeval,

    /// Non-zero if the local and the remote machine have synchronized clocks. If synchronized
    /// clocks are detected `transport_usec` becomes much more reliable. However, the code that
    /// detects synchronized clocks is very limited and unreliable itself.
    pub synchronized_clocks: i32,

    /// Time in usecs a sample takes to be played on the sink. For playback streams and record
    /// streams connected to a monitor source.
    pub sink_usec: ::sample::Usecs,

    /// Time in usecs a sample takes from being recorded to being delivered to the application. Only
    /// for record streams.
    pub source_usec: ::sample::Usecs,

    /// Estimated time in usecs a sample takes to be transferred to/from the daemon. For both
    /// playback and record streams.
    pub transport_usec: ::sample::Usecs,

    /// Non-zero when the stream is currently not underrun and data is being passed on to the
    /// device. Only for playback streams. This field does not say whether the data is actually
    /// already being played. To determine this check whether `since_underrun` (converted to usec)
    /// is larger than `sink_usec`.
    pub playing: i32,

    /// Non-zero if `write_index` is not up-to-date because a local write command that corrupted it
    /// has been issued in the time since this latency info was current. Only write commands with
    /// [`stream::SeekMode::RelativeOnRead`] and [`stream::SeekMode::RelativeEnd`] can corrupt
    /// `write_index`.
    ///
    /// [`stream::SeekMode::RelativeOnRead`]: ../stream/enum.SeekMode.html#RelativeOnRead.v
    /// [`stream::SeekMode::RelativeEnd`]: ../stream/enum.SeekMode.html#RelativeEnd.v
    pub write_index_corrupt: i32,

    /// Current write index into the playback buffer in bytes.
    ///
    /// Think twice before using this for seeking purposes: it might be out of date at the time you
    /// want to use it. Consider using [`stream::SeekMode::Relative`] instead.
    ///
    /// [`stream::SeekMode::Relative`]: ../stream/enum.SeekMode.html#Relative.v
    pub write_index: i64,

    /// Non-zero if `read_index` is not up-to-date because a local pause or flush request that
    /// corrupted it has been issued in the time since this latency info was current.
    pub read_index_corrupt: i32,

    /// Current read index into the playback buffer in bytes.
    ///
    /// Think twice before using this for seeking purposes: it might be out of date at the time you
    /// want to use it. Consider using [`stream::SeekMode::RelativeOnRead`] instead.
    ///
    /// [`stream::SeekMode::RelativeOnRead`]: ../stream/enum.SeekMode.html#RelativeOnRead.v
    pub read_index: i64,

    /// The configured latency for the sink.
    pub configured_sink_usec: ::sample::Usecs,

    /// The configured latency for the source.
    pub configured_source_usec: ::sample::Usecs,

    /// Bytes that were handed to the sink since the last underrun happened, or since playback
    /// started again after the last underrun. `playing` will tell you which case it is.
    pub since_underrun: i64,
}

/// A structure for the spawn API.
///
/// This may be used to integrate auto spawned daemons into your application. For more information
/// see [`context::Context::connect`]. When spawning a new child process the `waitpid()` is used on
/// the child's PID. The spawn routine will not block or ignore SIGCHLD signals, since this cannot
/// be done in a thread compatible way. You might have to do this in prefork/postfork.
///
/// [`context::Context::connect`]: ../context/struct.Context.html#method.connect
#[repr(C)]
#[derive(Debug)]
pub struct SpawnApi {
    /// Is called just before the fork in the parent process.
    pub prefork: Option<extern "C" fn()>,

    /// Is called immediately after the fork in the parent process.
    pub postfork: Option<extern "C" fn()>,

    /// Is called immediately after the fork in the child process.
    ///
    /// It is not safe to close all file descriptors in this function unconditionally, since a UNIX
    /// socket (created using socketpair()) is passed to the new process.
    pub atfork: Option<extern "C" fn()>,
}

pub type SinkFlagSet = capi::def::pa_sink_flags_t;

/// Special sink flags.
pub mod sink_flags {
    use capi;
    use super::SinkFlagSet;

    /// Flag to pass when no specific options are needed
    pub const NOFLAGS: SinkFlagSet = capi::PA_SINK_NOFLAGS;

    /// Supports hardware volume control. This is a dynamic flag and may change at runtime after the
    /// sink has initialized.
    pub const HW_VOLUME_CTRL: SinkFlagSet = capi::PA_SINK_HW_VOLUME_CTRL;

    /// Supports latency querying
    pub const LATENCY: SinkFlagSet = capi::PA_SINK_LATENCY;

    /// Is a hardware sink of some kind, in contrast to "virtual"/software sinks.
    pub const HARDWARE: SinkFlagSet = capi::PA_SINK_HARDWARE;

    /// Is a networked sink of some kind.
    pub const NETWORK: SinkFlagSet = capi::PA_SINK_NETWORK;

    /// Supports hardware mute control. This is a dynamic flag and may change at runtime after the
    /// sink has initialized.
    pub const HW_MUTE_CTRL: SinkFlagSet = capi::PA_SINK_HW_MUTE_CTRL;

    /// Volume can be translated to dB with [`::volume::sw_volume_to_db`]. This is a dynamic flag
    /// and may change at runtime after the sink has initialized.
    ///
    /// [`::volume::sw_volume_to_db`]: ../../volume/fn.sw_volume_to_db.html
    pub const DECIBEL_VOLUME: SinkFlagSet = capi::PA_SINK_DECIBEL_VOLUME;

    /// This sink is in flat volume mode, i.e. always the maximum of the volume  of all connected
    /// inputs.
    pub const FLAT_VOLUME: SinkFlagSet = capi::PA_SINK_FLAT_VOLUME;

    /// The latency can be adjusted dynamically depending on the needs of the connected streams.
    pub const DYNAMIC_LATENCY: SinkFlagSet = capi::PA_SINK_DYNAMIC_LATENCY;

    /// The sink allows setting what formats are supported by the connected hardware. The actual
    /// functionality to do this might be provided by an extension.
    pub const SET_FORMATS: SinkFlagSet = capi::PA_SINK_SET_FORMATS;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SinkState {
    /// This state is used when the server does not support sink state introspection.
    Invalid = -1,

    /// Running, sink is playing and used by at least one non-corked sink-input.
    Running = 0,

    /// When idle, the sink is playing but there is no non-corked sink-input attached to it.
    Idle = 1,

    /// When suspended, actual sink access can be closed, for instance.
    Suspended = 2,
}

impl From<SinkState> for capi::pa_sink_state_t {
    fn from(s: SinkState) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl From<capi::pa_sink_state_t> for SinkState {
    fn from(s: capi::pa_sink_state_t) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl SinkState {
    /// Returns `true` if sink is playing: running or idle.
    pub fn is_opened(self) -> bool {
        self == SinkState::Running ||
        self == SinkState::Idle
    }

    /// Returns `true` if sink is running.
    pub fn is_running(self) -> bool {
        self == SinkState::Running
    }
}

pub type SourceFlagSet = capi::def::pa_source_flags_t;

/// Special source flags.
pub mod source_flags {
    use capi;
    use super::SourceFlagSet;

    /// Flag to pass when no specific options are needed
    pub const NOFLAGS: SourceFlagSet = capi::PA_SOURCE_NOFLAGS;

    /// Supports hardware volume control. This is a dynamic flag and may change at runtime after the
    /// source has initialized.
    pub const HW_VOLUME_CTRL: SourceFlagSet = capi::PA_SOURCE_HW_VOLUME_CTRL;

    /// Supports latency querying
    pub const LATENCY: SourceFlagSet = capi::PA_SOURCE_LATENCY;

    /// Is a hardware source of some kind, in contrast to "virtual"/software source.
    pub const HARDWARE: SourceFlagSet = capi::PA_SOURCE_HARDWARE;

    /// Is a networked source of some kind.
    pub const NETWORK: SourceFlagSet = capi::PA_SOURCE_NETWORK;

    /// Supports hardware mute control. This is a dynamic flag and may change at runtime after the
    /// source has initialized.
    pub const HW_MUTE_CTRL: SourceFlagSet = capi::PA_SOURCE_HW_MUTE_CTRL;

    /// Volume can be translated to dB with [`::volume::sw_volume_to_db`]. This is a dynamic flag
    /// and may change at runtime after the sink has initialized. Volume can be translated to dB
    /// with [`::volume::sw_volume_to_db`]. This is a dynamic flag and may change at runtime after
    /// the source has initialized.
    ///
    /// [`::volume::sw_volume_to_db`]: ../../volume/fn.sw_volume_to_db.html
    pub const DECIBEL_VOLUME: SourceFlagSet = capi::PA_SOURCE_DECIBEL_VOLUME;

    /// The latency can be adjusted dynamically depending on the needs of the connected streams.
    pub const DYNAMIC_LATENCY: SourceFlagSet = capi::PA_SOURCE_DYNAMIC_LATENCY;

    /// This source is in flat volume mode, i.e. always the maximum of the volume of all connected
    /// outputs.
    pub const FLAT_VOLUME: SourceFlagSet = capi::PA_SOURCE_FLAT_VOLUME;
}

/// Source state.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SourceState {
    /// This state is used when the server does not support source state introspection.
    Invalid = -1,

    /// Running, source is recording and used by at least one non-corked source-output.
    Running = 0,

    /// When idle, the source is still recording but there is no non-corked source-output.
    Idle = 1,

    /// When suspended, actual source access can be closed, for instance.
    Suspended = 2,
}

impl From<SourceState> for capi::pa_source_state_t {
    fn from(s: SourceState) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl From<capi::pa_source_state_t> for SourceState {
    fn from(s: capi::pa_source_state_t) -> Self {
        unsafe { std::mem::transmute(s) }
    }
}

impl SourceState {
    /// Returns `true` if source is recording: running or idle.
    pub fn is_opened(self) -> bool {
        self == SourceState::Running ||
        self == SourceState::Idle
    }

    /// Returns `true` if source is running.
    pub fn is_running(self) -> bool {
        self == SourceState::Running
    }
}
