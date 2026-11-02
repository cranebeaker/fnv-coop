//! The game's own shaders, so the renderer can do exactly what they do.
//!
//! New Vegas keeps its compiled shaders in `Data\Shaders\shaderpackageNNN.sdp`:
//! one package per class of graphics card, each holding a few hundred named
//! Direct3D 9 vertex and pixel shaders. [`Package`] reads a package and
//! [`disassemble`] turns a shader's bytecode into readable instructions,
//! with the names the game gives its constants (`AmbientColor`,
//! `LightData`, ...), which is what the lighting code is ported from.
//!
//! ```no_run
//! let bytes = std::fs::read("Data/Shaders/shaderpackage019.sdp")?;
//! let package = shaders::Package::parse(&bytes)?;
//! for shader in &package.shaders {
//!     let listing = shaders::disassemble(&shader.bytecode)?;
//!     println!("{}: {}", shader.name, listing.version);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

mod disasm;
mod package;

pub use disasm::{disassemble, Constant, Disassembly, RegisterSet, ShaderKind, Version};
pub use package::{Package, Shader};

use std::fmt;

/// Why a package or shader couldn't be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Not a shader package, or cut short.
    NotAPackage(String),
    /// The bytecode doesn't follow the Direct3D 9 format.
    BadBytecode(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotAPackage(why) => write!(f, "not a shader package: {why}"),
            Error::BadBytecode(why) => write!(f, "unreadable shader: {why}"),
        }
    }
}

impl std::error::Error for Error {}
