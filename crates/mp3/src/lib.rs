//! An MP3 decoder: MPEG-1, MPEG-2 and MPEG-2.5 Layer III, mono and stereo
//! (including joint stereo's mid/side and intensity coding), every
//! bitrate including free format, checksummed frames, and the bit
//! reservoir. The game's music (`Data\Music`, 199 files) is all MPEG-1
//! Layer III: most of it 192 kbit/s at 48 kHz, the Fallout 1 and 2 tracks
//! 128 kbit/s at 44.1 kHz.
//!
//! ID3v2 tags at the start and ID3v1, APEv2 and Lyrics3 tags at the end
//! are skipped, as is junk between frames. A Xing/Info frame is read and
//! not played; with LAME's tag in it, the encoder's delay and padding are
//! trimmed so tracks play (and loop) without gaps.
//!
//! The steps follow ISO 11172-3 (and 13818-3 for the lower sample rates)
//! in double precision: Huffman decoding, requantization, stereo
//! processing, reordering, alias reduction, the IMDCT with its four
//! windows, overlap-add and the polyphase synthesis filterbank. The
//! fixed tables (Huffman codes, band edges, synthesis window) were checked
//! value for value against the copies inside Windows' own MP3 codecs.
//!
//! Checked against Windows' Fraunhofer MP3 decoder (the ACM codec; the
//! game plays its music through DirectShow, so through Windows' decoders):
//! on all 199 tracks, every sample agrees to within one step of 16-bit
//! output (99.97% exactly, once rounded the same way), with the same
//! delay; and so do test streams using what the game's files don't
//! (mid/side and intensity stereo, mixed blocks, mono, MPEG-2 and 2.5,
//! checksums). Three corners where decoders disagree follow Fraunhofer's:
//! the top intensity band, the second Huffman region of MPEG-2 mixed
//! blocks, and alias reduction in 8 kHz mixed blocks (see `layer3`).
//!
//! Streaming, a frame at a time:
//!
//! ```no_run
//! let mut decoder = mp3::Decoder::new(std::fs::read("mus_SCR_DocMitchell.mp3")?)?;
//! let (rate, channels) = (decoder.sample_rate(), decoder.channels());
//! let mut buffer = vec![0i16; 4096];
//! loop {
//!     let n = decoder.read_i16(&mut buffer);
//!     if n == 0 {
//!         break; // or decoder.rewind() to loop
//!     }
//!     // play &buffer[..n]: interleaved, `channels` per sample frame
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Or a whole file: [`decode_all`], and [`wav::write`] to save it.

mod bits;
mod decoder;
mod error;
mod header;
mod huffman;
mod layer3;
mod stream;
mod synth;
mod tables;
pub mod wav;

pub use decoder::{decode_all, to_i16, Decoder, Frame, Pcm, Problem, StreamInfo, DECODER_DELAY};
pub use error::{Error, Result};
pub use header::{ChannelMode, FrameHeader, Version};
pub use stream::{scan, LameTag, Scan, Tags, VbriHeader, XingHeader};

#[cfg(test)]
mod tests;
