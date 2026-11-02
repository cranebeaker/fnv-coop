//! Transport-agnostic voice chat building blocks.
//!
//! Pipeline (sender): PCM frame -> [`vad::Gate`] -> [`codec::Codec`] ->
//! [`envelope::VoicePacket`] -> bytes -> any unreliable datagram transport.
//!
//! Pipeline (receiver): bytes -> [`envelope::VoicePacket`] -> per speaker
//! [`jitter::JitterBuffer`] -> [`codec::Codec`] decode (or conceal) ->
//! [`mixer::Mixer`] -> stereo f32 out.
//!
//! The crate has no dependency on `coop-core`. See [`envelope`] for the adapter
//! contract used to carry packets inside whatever message type `coop-core`
//! (or its `social` module) defines.

pub mod codec;
pub mod envelope;
pub mod jitter;
pub mod mixer;
pub mod sim;
pub mod vad;

#[cfg(feature = "opus")]
pub mod opus_codec;

#[cfg(feature = "capture")]
pub mod capture;

/// Frame length in milliseconds used by the built-in codecs and the simulator.
pub const FRAME_MS: u32 = 20;
