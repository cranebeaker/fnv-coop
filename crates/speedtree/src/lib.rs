//! The game's SpeedTree trees (`TREE` records' `.spt` models), read and
//! grown as the game's embedded SpeedTreeRT does it, from its code
//! (`%USERPROFILE%\nv-re\findings\trees.md` has the addresses).
//!
//! A `.spt` file holds no geometry, only parameters: curves (`spline`)
//! and numbers for each level of branches, the leaves, fronds, levels of
//! detail and wind. The tree is grown from them and a seed with the
//! library's own random numbers (`random`), so the same file and seed
//! always give the same tree.

//!
//! From the grown tree the game builds a branch mesh and a leaf mesh for
//! each level of detail (`mesh`), picks the levels each frame by distance
//! (`lod`) and sways them with the weather's wind (`wind`).

pub mod lod;
pub mod math;
pub mod mesh;
pub mod random;
pub(crate) mod reader;
pub mod spline;
pub mod spt;
pub mod tree;
pub mod wind;

pub use reader::SptError;
pub use spt::SptFile;
