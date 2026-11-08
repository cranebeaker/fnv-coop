//! Direct3D 9 on wgpu (ADR-0007), part 1: the converter of the game's shader bytecode.
//!
//! The game's renderer is translated from the exe and draws through Direct3D 9; its look is the
//! game's compiled shaders (`Data\Shaders\shaderpackageNNN.sdp`, read from the player's install).
//! `sm` reads that bytecode, lists it and converts it to WGSL with Direct3D 9's meaning of each
//! instruction; naga validates the result as wgpu will. The backend that runs the game's calls on
//! wgpu (a `d3d9_stream::Backend`) builds on it.

pub mod sm;
