//! coop-engine: the co-op adapter for the game program by address.
//!
//! The upstream engine runs the game's own program code (FalloutNV.exe 1.4.0.525) on a model
//! of its memory. This crate sees that program only through [`Program`]: read, write and call
//! by program address. It names no engine type. One glue module outside this crate implements
//! [`Program`] for the engine. The same adapter can run inside the original program.
//!
//! State is read and written between the game's frames. The host's game runs the story's
//! scripts. A guest's game gets the state as plain values.
//!
//! Each address and offset is in [`addr`], with its source.

pub mod addr;
pub mod audit;
pub mod avatar;
pub mod changes;
pub mod clock;
pub mod console;
pub mod fake;
pub mod files;
pub mod game;
pub mod gate;
pub mod peer;
pub mod program;
pub mod quest;
pub mod record;
pub mod script;
pub mod shared;

pub use program::{Program, Ret};
