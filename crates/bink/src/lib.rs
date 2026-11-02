//! A Bink 1 movie decoder, for the game's movies (`Data\Video`; the base
//! game has one, `FNVIntro.bik`, revision `i`, 1280x720 at 30 frames per
//! second, which the opening quest plays with `PlayBink`).
//!
//! [`Movie`] reads a `.bik` file's header and frame index and hands out each
//! frame's packets; [`Decoder`] turns the video part of each packet, in
//! order, into Y, U and V planes (U and V at half width and height).
//!
//! The game plays its movies through RAD's `binkw32.dll`, which ships in
//! the install, and the aim is to produce exactly what that library
//! produces. Its fixed tables (coefficient order, run patterns, Huffman
//! codes and both dequantization tables) were read out of that DLL (see
//! `tables`), and the transform follows the DLL's own code. The bitstream
//! layout follows the public descriptions of the format.
//!
//! Checked against the library: `research/nv-oracle`'s `nv-bink` decodes a
//! movie with the install's `binkw32.dll` and records the SHA-256 of each
//! frame's planes, and `nvinspect <movie.bik> frames` records the same for
//! this decoder, so the two files can be compared line by line. The
//! results of that comparison are kept in `docs/MOVIES.md`.
//!
//! [`AudioDecoder`] decodes an audio track's packets (the DCT variant) into
//! interleaved 16-bit samples.

mod audio;
mod bits;
mod color;
mod container;
mod tables;
mod video;

pub use audio::{AudioDecoder, AudioError};
pub use container::{
    AudioTrack, Error, Movie, Packet, AUDIO_16BIT, AUDIO_DCT, AUDIO_STEREO, FLAG_ALPHA, FLAG_GRAY,
};
pub use video::{DecodeError, Decoder};

#[cfg(test)]
mod tests;
