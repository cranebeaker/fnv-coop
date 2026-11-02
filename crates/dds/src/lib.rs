//! Reader for DDS textures as used by Fallout 3 and New Vegas.
//!
//! Handles the block-compressed formats DXT1, DXT3 and DXT5 (BC1-BC3),
//! ATI1 and ATI2 (BC4/BC5), common uncompressed layouts (A8R8G8B8, R5G6B5,
//! L8 and so on), mip chains and cube maps.
//!
//! A renderer uploads [`Dds::level_data`] straight to the GPU for the
//! compressed formats; [`Dds::decode_rgba`] decodes any level to plain RGBA
//! on the CPU, and [`png::write_rgba`] saves that for viewing.
//!
//! ```no_run
//! let dds = dds::Dds::parse(std::fs::read("crate01.dds")?)?;
//! let rgba = dds.decode_rgba(0, 0)?;
//! let mut file = std::fs::File::create("crate01.png")?;
//! dds::png::write_rgba(&mut file, dds.width(), dds.height(), &rgba)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod bc;
mod deflate;
mod error;
mod file;
pub mod png;

pub use error::{Error, Result};
pub use file::{Dds, Format, PixelMasks};
