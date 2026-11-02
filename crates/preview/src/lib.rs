//! Drawing game data into images on the CPU, with no GPU or window: a
//! small software rasterizer and the code that turns a cell into draw calls.
//! Used by `nvinspect render-cell` to check placement against the game
//! before the real-time viewer depends on it.

pub mod actor;
pub mod cell;
pub mod furniture;
pub mod local_map;
pub mod ragdoll;
pub mod raster;
